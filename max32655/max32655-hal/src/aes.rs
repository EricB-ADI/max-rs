// use crate::common;
// use crate::pac;
// use crate::MxcError;
// use crate::Sys;

// pub enum KeySize {
//     AES128,
//     AES192,
//     AES256,
// }

// pub enum EncType {
//     EncExtKey,
//     DecExtKey,
//     DecIntKey,
// }

// #[allow(dead_code)]
// pub struct Req<'a> {
//     input: &'a [u32],
//     output: &'a mut [u32],

//     key_size: KeySize,
//     enc_type: EncType,
// }

// pub trait AES {
//     fn init(&self);
//     fn is_busy(&self) -> bool;
//     fn shutdown(&self);
//     fn generate_key(&self);
//     fn set_key_size(&self, key_size: KeySize);
//     fn get_key_size(&self) -> KeySize;
//     fn flush_input_fifo(&self);
//     fn flush_output_fifo(&self);
//     fn start(&self);
//     fn get_flags(&self);
//     fn clear_flags(&self);
//     fn encrypt(&self);
//     fn decrypt(&self);
//     fn set_ext_key(&self, key: &[u8], len: u32);

//     fn wait_until_not_busy(&self) {
//         while self.is_busy() {}
//     }
// }

// impl AES for pac::AES {
//     fn init(&self) {
//         Sys::periph_clock_enable(Sys::PeriphClock::AES);
//         Sys::periph_clock_enable(Sys::PeriphClock::TRNG);

//         self.ctrl.write(|w| unsafe {
//             w.bits(0);
//             self.wait_until_not_busy();
//             w.en().set_bit()
//         })
//     }
//     fn is_busy(&self) -> bool {
//         self.status.read().busy().bit_is_set()
//     }
//     fn shutdown(&self) {
//         self.flush_input_fifo();
//         self.flush_output_fifo();
//         self.wait_until_not_busy();

//         self.ctrl.write(|w| unsafe { w.bits(0) })
//     }
//     fn generate_key(&self) {}
//     fn set_key_size(&self, key_size: KeySize) {}
//     fn get_key_size(&self) -> KeySize {
//         KeySize::AES128
//     }
//     fn flush_input_fifo(&self) {}
//     fn flush_output_fifo(&self) {}
//     fn start(&self) {}
//     fn encrypt(&self) {}
//     fn decrypt(&self) {}
//     fn set_ext_key(&self, key: &[u8], len: u32) {}

//     fn get_flags(&self) {}
//     fn clear_flags(&self) {}
// }
