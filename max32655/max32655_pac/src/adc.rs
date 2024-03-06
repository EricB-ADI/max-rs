#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Control"]
    pub ctrl: CTRL,
    #[doc = "0x04 - ADC Status"]
    pub status: STATUS,
    #[doc = "0x08 - ADC Output Data"]
    pub data: DATA,
    #[doc = "0x0c - ADC Interrupt Control Register"]
    pub intr: INTR,
    #[doc = "0x10..0x20 - ADC Limit"]
    pub limit: [LIMIT; 4],
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "ADC Control"]
pub mod ctrl;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "ADC Status"]
pub mod status;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "ADC Output Data"]
pub mod data;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "ADC Interrupt Control Register"]
pub mod intr;
#[doc = "LIMIT (rw) register accessor: an alias for `Reg<LIMIT_SPEC>`"]
pub type LIMIT = crate::Reg<limit::LIMIT_SPEC>;
#[doc = "ADC Limit"]
pub mod limit;
