enum TxState {} enum RxState {} struct TxOutRxIn < T >
           {
               data_valid : marlin :: spade :: type_translation :: Bool, data : T, ack :
               marlin :: spade :: type_translation :: Bool
           } impl < T > marlin :: spade :: type_translation :: FromBits for TxOutRxIn < T
           >
           {
               fn size() -> usize { data_valid.size() + data.size() + ack.size() } fn
               backward_size() -> usize { data_valid.size() + data.size() + ack.size() }
               fn update_value(& mut self, bit_offset : usize, bits : & [u32])
               {
                   let mut local_offset = 0;
                   self.data_valid.update_value(bit_offset + local_offset, bits);
                   local_offset += data_valid.size(); ;
                   self.data.update_value(bit_offset + local_offset, bits); local_offset
                   += data.size(); ;
                   self.ack.update_value(bit_offset + local_offset, bits); local_offset
                   += ack.size();
               }
           } enum RxState {} struct FromSender < T >
           {
               valid : marlin :: spade :: type_translation :: Bool, data : T, ack :
               marlin :: spade :: type_translation :: Bool
           } impl < T > marlin :: spade :: type_translation :: FromBits for FromSender <
           T >
           {
               fn size() -> usize { valid.size() + data.size() + ack.size() } fn
               backward_size() -> usize { valid.size() + data.size() + ack.size() } fn
               update_value(& mut self, bit_offset : usize, bits : & [u32])
               {
                   let mut local_offset = 0;
                   self.valid.update_value(bit_offset + local_offset, bits); local_offset
                   += valid.size(); ;
                   self.data.update_value(bit_offset + local_offset, bits); local_offset
                   += data.size(); ;
                   self.ack.update_value(bit_offset + local_offset, bits); local_offset
                   += ack.size();
               }
           } enum TxState < T > {} enum Option < T > {} struct FifoWrite < D >
           {
               write : std :: option :: Option < D > , full : marlin :: spade ::
               type_translation :: Bool
           } impl < D > marlin :: spade :: type_translation :: FromBits for FifoWrite < D
           >
           {
               fn size() -> usize { write.size() + full.size() } fn backward_size() ->
               usize { write.size() + full.size() } fn
               update_value(& mut self, bit_offset : usize, bits : & [u32])
               {
                   let mut local_offset = 0;
                   self.write.update_value(bit_offset + local_offset, bits); local_offset
                   += write.size(); ;
                   self.full.update_value(bit_offset + local_offset, bits); local_offset
                   += full.size();
               }
           } struct ReadPort < const W : u64, D >
           { addr : marlin :: spade :: type_translation :: SpadeUint < W > , out : D }
           impl < const W : u64, D > marlin :: spade :: type_translation :: FromBits for
           ReadPort < const W : u64, D >
           {
               fn size() -> usize { addr.size() + out.size() } fn backward_size() ->
               usize { addr.size() + out.size() } fn
               update_value(& mut self, bit_offset : usize, bits : & [u32])
               {
                   let mut local_offset = 0;
                   self.addr.update_value(bit_offset + local_offset, bits); local_offset
                   += addr.size(); ;
                   self.out.update_value(bit_offset + local_offset, bits); local_offset
                   += out.size();
               }
           } struct FifoRtoW < const W : u64 >
           {
               read_ptr : marlin :: spade :: type_translation :: SpadeUint < W > ,
               write_ptr : marlin :: spade :: type_translation :: SpadeUint < W >
           } impl < const W : u64 > marlin :: spade :: type_translation :: FromBits for
           FifoRtoW < const W : u64 >
           {
               fn size() -> usize { read_ptr.size() + write_ptr.size() } fn
               backward_size() -> usize { read_ptr.size() + write_ptr.size() } fn
               update_value(& mut self, bit_offset : usize, bits : & [u32])
               {
                   let mut local_offset = 0;
                   self.read_ptr.update_value(bit_offset + local_offset, bits);
                   local_offset += read_ptr.size(); ;
                   self.write_ptr.update_value(bit_offset + local_offset, bits);
                   local_offset += write_ptr.size();
               }
           } struct FifoRead < const W : u64, D >
           {
               read : std :: option :: Option < D > , ack : marlin :: spade ::
               type_translation :: Bool, num_elements : marlin :: spade ::
               type_translation :: SpadeUint < W >
           } impl < const W : u64, D > marlin :: spade :: type_translation :: FromBits
           for FifoRead < const W : u64, D >
           {
               fn size() -> usize { read.size() + ack.size() + num_elements.size() } fn
               backward_size() -> usize
               { read.size() + ack.size() + num_elements.size() } fn
               update_value(& mut self, bit_offset : usize, bits : & [u32])
               {
                   let mut local_offset = 0;
                   self.read.update_value(bit_offset + local_offset, bits); local_offset
                   += read.size(); ;
                   self.ack.update_value(bit_offset + local_offset, bits); local_offset
                   += ack.size(); ;
                   self.num_elements.update_value(bit_offset + local_offset, bits);
                   local_offset += num_elements.size();
               }
           } struct WritePort < const W : u64, D >
           {
               addr : marlin :: spade :: type_translation :: SpadeUint < W > , write :
               std :: option :: Option < D >
           } impl < const W : u64, D > marlin :: spade :: type_translation :: FromBits
           for WritePort < const W : u64, D >
           {
               fn size() -> usize { addr.size() + write.size() } fn backward_size() ->
               usize { addr.size() + write.size() } fn
               update_value(& mut self, bit_offset : usize, bits : & [u32])
               {
                   let mut local_offset = 0;
                   self.addr.update_value(bit_offset + local_offset, bits); local_offset
                   += addr.size(); ;
                   self.write.update_value(bit_offset + local_offset, bits); local_offset
                   += write.size();
               }
           }
