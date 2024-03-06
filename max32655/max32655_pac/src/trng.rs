#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TRNG Control Register."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Data. The content of this register is valid only when RNG_IS = 1. When TRNG is disabled, read returns 0x0000 0000."]
    pub status: STATUS,
    #[doc = "0x08 - Data. The content of this register is valid only when RNG_IS = 1. When TRNG is disabled, read returns 0x0000 0000."]
    pub data: DATA,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "TRNG Control Register."]
pub mod ctrl;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Data. The content of this register is valid only when RNG_IS = 1. When TRNG is disabled, read returns 0x0000 0000."]
pub mod status;
#[doc = "DATA (r) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data. The content of this register is valid only when RNG_IS = 1. When TRNG is disabled, read returns 0x0000 0000."]
pub mod data;
