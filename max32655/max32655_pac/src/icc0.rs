#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache ID Register."]
    pub info: INFO,
    #[doc = "0x04 - Memory Configuration Register."]
    pub sz: SZ,
    _reserved2: [u8; 0xf8],
    #[doc = "0x100 - Cache Control and Status Register."]
    pub ctrl: CTRL,
    _reserved3: [u8; 0x05fc],
    #[doc = "0x700 - Invalidate All Registers."]
    pub invalidate: INVALIDATE,
}
#[doc = "INFO (r) register accessor: an alias for `Reg<INFO_SPEC>`"]
pub type INFO = crate::Reg<info::INFO_SPEC>;
#[doc = "Cache ID Register."]
pub mod info;
#[doc = "SZ (r) register accessor: an alias for `Reg<SZ_SPEC>`"]
pub type SZ = crate::Reg<sz::SZ_SPEC>;
#[doc = "Memory Configuration Register."]
pub mod sz;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Cache Control and Status Register."]
pub mod ctrl;
#[doc = "INVALIDATE (rw) register accessor: an alias for `Reg<INVALIDATE_SPEC>`"]
pub type INVALIDATE = crate::Reg<invalidate::INVALIDATE_SPEC>;
#[doc = "Invalidate All Registers."]
pub mod invalidate;
