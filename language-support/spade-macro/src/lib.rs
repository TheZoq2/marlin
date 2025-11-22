// Copyright (C) 2024 Ethan Uppal.
//
// This Source Code Form is subject to the terms of the Mozilla Public License,
// v. 2.0. If a copy of the MPL was not distributed with this file, You can
// obtain one at https://mozilla.org/MPL/2.0/.

mod types;

use std::env;

use camino::Utf8PathBuf;
use marlin_verilator::{PortDirection, mangle};
use marlin_verilog_macro_builder::build_verilated_struct;
use num::ToPrimitive;
use proc_macro::TokenStream;

use proc_macro_error::proc_macro_error;
use spade as spade_compiler;
use spade_compiler::compiler_state::CompilerState;
use spade_hir_lowering::{MirLowerable, UnitNameExt};
use types::mirror_types;

// TODO: Move into a more general place
struct MacroArgs {
    pub top: syn::LitStr,
}

impl syn::parse::Parse for MacroArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        syn::custom_keyword!(top);

        input.parse::<top>()?;
        input.parse::<syn::Token![=]>()?;
        let top = input.parse::<syn::LitStr>()?;

        Ok(Self { top })
    }
}

fn search_for_swim_toml(mut start: Utf8PathBuf) -> Option<Utf8PathBuf> {
    while start.parent().is_some() {
        if start.join("swim.toml").is_file() {
            return Some(start.join("swim.toml"));
        }
        start.pop();
    }
    None
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn spade(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as MacroArgs);

    let manifest_directory = Utf8PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").expect("Please use CARGO"),
    );
    let Some(swim_toml) = search_for_swim_toml(manifest_directory) else {
        return syn::Error::new_spanned(args.top, "Could not find swim.toml")
            .into_compile_error()
            .into();
    };
    let mut source_path = swim_toml.clone();
    source_path.pop();

    let verilog_source_path = {
        syn::LitStr::new(
            source_path.join("build/spade.sv").as_str(),
            args.top.span(),
        )
    };

    let state_file_path = source_path.join("build/state.bincode");
    let state_file_content = match std::fs::read(&state_file_path) {
        Ok(state_file) => state_file,
        Err(e) => {
            return syn::Error::new_spanned(
                args.top,
                format!("Failed to read {state_file_path}. {e}"),
            )
            .into_compile_error()
            .into();
        }
    };

    let (compiler_state, _) =
        match bincode::serde::decode_from_slice::<CompilerState, _>(
            &state_file_content,
            bincode::config::standard(),
        ) {
            Ok(state) => state,
            Err(e) => {
                return syn::Error::new_spanned(
                    args.top,
                    format!("Failed to decode build/state.bincode. {e}"),
                )
                .into_compile_error()
                .into();
            }
        };

    let Some(top_unit) =
        compiler_state
            .item_list
            .executables
            .iter()
            .find_map(|(exec, item)| {
                if exec.1.as_strs()
                    == args.top.value().split("::").collect::<Vec<_>>()
                {
                    Some(item)
                } else {
                    None
                }
            })
    else {
        return syn::Error::new_spanned(
            &args.top,
            format!(
                "Spade project did not contain {}.\nRemember to include the project name from `swim.toml` in the path",
                args.top.value()
            ),
        ).into_compile_error().into();
    };

    let top_unit = match top_unit {
        spade_hir::ExecutableItem::Unit(unit) => unit,
        spade_hir::ExecutableItem::ExternUnit(unit_name, loc) => {
            return syn::Error::new_spanned(
                args.top,
                format!("Top unit is an extern unit, which cannot be tested"),
            )
            .into_compile_error()
            .into();
        }
        spade_hir::ExecutableItem::EnumInstance { base_enum, variant } => {
            return syn::Error::new_spanned(
                args.top,
                format!("Top unit is an enum variant, which cannot be tested"),
            )
            .into_compile_error()
            .into();
        }
        spade_hir::ExecutableItem::StructInstance => {
            return syn::Error::new_spanned(
                args.top,
                format!("Top unit is a struct, which cannot be tested"),
            )
            .into_compile_error()
            .into();
        }
    };

    if !(top_unit.head.unit_type_params.is_empty()
        && top_unit.head.scope_type_params.is_empty())
    {
        return syn::Error::new_spanned(
            args.top,
            format!("The module under test cannot be generic. Consider creating a non-generic test harness.")
        ).into_compile_error().into();
    }
    let type_state = compiler_state
        .mir_context
        .get(top_unit.name.name_id())
        .expect("Expected to find a mir_context for the top module")
        .type_state
        .clone();

    let mut ports = vec![];
    for ((name, _hir_type), param) in
        top_unit.inputs.iter().zip(top_unit.head.inputs.0.clone())
    {
        let ty = type_state
            .concrete_type_of_name(
                &name,
                compiler_state.symtab.symtab(),
                &compiler_state.item_list.types,
            )
            .expect("Expected a concrete type for {name}");

        let name = if param.no_mangle.is_some() {
            param.name.0.to_string()
        } else {
            format!("{}_i", param.name.0)
        };

        let mir_ty = ty.to_mir_type();

        let size = mir_ty
            .size()
            .to_usize()
            .expect("Types with more than 2^64 bits are unsupported");
        if size != 0 {
            ports.push((name, size, 0, PortDirection::Input));
        }

        let back_size = mir_ty
            .backward_size()
            .to_usize()
            .expect("Types with more than 2^64 bits are unsupported");
        if back_size != 0 {
            // TODO: Verify that this mangling scheme is correct
            ports.push((
                param.name.0.clone() + "_o",
                back_size,
                0,
                PortDirection::Output,
            ));
        }
    }

    let verilator = build_verilated_struct(
        "spade",
        syn::LitStr::new(
            &mangle(top_unit.name.as_mir().without_escapes()).unwrap(),
            args.top.span(),
        ),
        verilog_source_path,
        ports,
        None,
        None,
        item.into(),
    );

    let type_definitions = mirror_types(&compiler_state);
    // panic!("{type_definitions}");

    quote::quote!{
        #verilator
        #type_definitions
    }.into()
}
