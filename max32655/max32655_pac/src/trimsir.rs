#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - RTC Trim System Initialization Register."]
    pub rtc: RTC,
    _reserved1: [u8; 0x28],
    #[doc = "0x34 - SIMO Trim System Initialization Register."]
    pub simo: SIMO,
    _reserved2: [u8; 0x04],
    #[doc = "0x3c - IPO Low Trim System Initialization Register."]
    pub ipolo: IPOLO,
    #[doc = "0x40 - Control Trim System Initialization Register."]
    pub ctrl: CTRL,
    #[doc = "0x44 - RTC Trim System Initialization Register."]
    pub inro: INRO,
}
#[doc = "RTC (rw) register accessor: an alias for `Reg<RTC_SPEC>`"]
pub type RTC = crate::Reg<rtc::RTC_SPEC>;
#[doc = "RTC Trim System Initialization Register."]
pub mod rtc;
#[doc = "SIMO (r) register accessor: an alias for `Reg<SIMO_SPEC>`"]
pub type SIMO = crate::Reg<simo::SIMO_SPEC>;
#[doc = "SIMO Trim System Initialization Register."]
pub mod simo;
#[doc = "IPOLO (r) register accessor: an alias for `Reg<IPOLO_SPEC>`"]
pub type IPOLO = crate::Reg<ipolo::IPOLO_SPEC>;
#[doc = "IPO Low Trim System Initialization Register."]
pub mod ipolo;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Trim System Initialization Register."]
pub mod ctrl;
#[doc = "INRO (rw) register accessor: an alias for `Reg<INRO_SPEC>`"]
pub type INRO = crate::Reg<inro::INRO_SPEC>;
#[doc = "RTC Trim System Initialization Register."]
pub mod inro;
