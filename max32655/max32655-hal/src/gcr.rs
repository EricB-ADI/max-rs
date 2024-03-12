use crate::pac;
pub fn steal_gcr() -> pac::GCR {
    unsafe { pac::Peripherals::steal().GCR }
}
