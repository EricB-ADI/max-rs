#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - Low Power Reset Register."]
    pub rst: RST,
    #[doc = "0x0c - Low Power Peripheral Clock Disable Register."]
    pub pclkdis: PCLKDIS,
}
#[doc = "RST (rw) register accessor: an alias for `Reg<RST_SPEC>`"]
pub type RST = crate::Reg<rst::RST_SPEC>;
#[doc = "Low Power Reset Register."]
pub mod rst;
#[doc = "PCLKDIS (rw) register accessor: an alias for `Reg<PCLKDIS_SPEC>`"]
pub type PCLKDIS = crate::Reg<pclkdis::PCLKDIS_SPEC>;
#[doc = "Low Power Peripheral Clock Disable Register."]
pub mod pclkdis;
