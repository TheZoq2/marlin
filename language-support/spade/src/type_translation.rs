pub trait SpadeType {
    fn size(&self) -> usize;
    fn backward_size(&self) -> usize;

    /// Update the value of this type based on the `bits`. The `start_bit` and `end_bit` parameters
    /// are the bit offsets at which this value starts in `bits`. `start_bit` must be respected, but
    /// `end_bit` can be ignored _if_ the type knows its own size. It is there for types like `uN` which
    /// do not know the size of their underlying Spade value
    fn update_value(
        &mut self,
        bit_offset: usize,
        bits: &[u32],
    );
}

pub struct SpadeUint<const N: u64> {
    inner: u64,
}

impl<const N: u64> SpadeType for SpadeUint<N> {
    fn size(&self) -> usize {
        N as usize
    }

    fn backward_size(&self) -> usize {
        0
    }

    fn update_value(
        &mut self,
        bit_offset: usize,
        bits: &[u32],
    ) {
        let start_idx = bit_offset / 32;
        let shift_amount = bit_offset % 32;

        let from_0 = bits[start_idx] >> shift_amount;
        let from_1 = if N > 32 {
            bits[start_idx + 1] >> shift_amount
        } else {
            0
        };
        let remaining = if shift_amount != 0 {
            if N > 32 {
                bits[start_idx + 2] >> shift_amount
            } else {
                bits[start_idx + 1] >> shift_amount
            }
        } else {
            0
        };
    }
}

macro_rules! uint_methods {
    ($ty:ty) => {
        impl<const N: u64> From<$ty> for SpadeUint<N> {
            fn from(value: $ty) -> Self {
                // TODO: Panic if the value does not fit
                SpadeUint::<N>{inner: value as u64}
            }
        }

        impl<const N: u64> PartialEq<$ty> for SpadeUint<N> {
            fn eq(&self, other: &$ty) -> bool {
                self.inner == *other as u64
            }
        }
    }
}
uint_methods!(u8);
uint_methods!(u16);
uint_methods!(u32);
uint_methods!(u64);
