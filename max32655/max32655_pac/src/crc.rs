#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC Control"]
    pub ctrl: CTRL,
    _reserved_1_datain8: [u8; 0x04],
    #[doc = "0x08 - CRC Polynomial"]
    pub poly: POLY,
    #[doc = "0x0c - Current CRC Value"]
    pub val: VAL,
}
impl RegisterBlock {
    #[doc = "0x04 - CRC Data Input"]
    #[inline(always)]
    pub const fn datain8(&self) -> &[DATAIN8; 4] {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - CRC Data Input"]
    #[inline(always)]
    pub const fn datain16(&self) -> &[DATAIN16; 2] {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - CRC Data Input"]
    #[inline(always)]
    pub const fn datain32(&self) -> &DATAIN32 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "CRC Control"]
pub mod ctrl;
#[doc = "DATAIN32 (rw) register accessor: an alias for `Reg<DATAIN32_SPEC>`"]
pub type DATAIN32 = crate::Reg<datain32::DATAIN32_SPEC>;
#[doc = "CRC Data Input"]
pub mod datain32;
#[doc = "DATAIN16 (rw) register accessor: an alias for `Reg<DATAIN16_SPEC>`"]
pub type DATAIN16 = crate::Reg<datain16::DATAIN16_SPEC>;
#[doc = "CRC Data Input"]
pub mod datain16;
#[doc = "DATAIN8 (rw) register accessor: an alias for `Reg<DATAIN8_SPEC>`"]
pub type DATAIN8 = crate::Reg<datain8::DATAIN8_SPEC>;
#[doc = "CRC Data Input"]
pub mod datain8;
#[doc = "POLY (rw) register accessor: an alias for `Reg<POLY_SPEC>`"]
pub type POLY = crate::Reg<poly::POLY_SPEC>;
#[doc = "CRC Polynomial"]
pub mod poly;
#[doc = "VAL (rw) register accessor: an alias for `Reg<VAL_SPEC>`"]
pub type VAL = crate::Reg<val::VAL_SPEC>;
#[doc = "Current CRC Value"]
pub mod val;
