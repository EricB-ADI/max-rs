#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global Enable/Disable Controls for All Pulse Trains"]
    pub enable: ENABLE,
    #[doc = "0x04 - Global Resync (All Pulse Trains) Control"]
    pub resync: RESYNC,
    #[doc = "0x08 - Pulse Train Stop Interrupt Flags"]
    pub stop_intfl: STOP_INTFL,
    #[doc = "0x0c - Pulse Train Stop Interrupt Enable/Disable"]
    pub stop_inten: STOP_INTEN,
    #[doc = "0x10 - Pulse Train Global Safe Enable."]
    pub safe_en: SAFE_EN,
    #[doc = "0x14 - Pulse Train Global Safe Disable."]
    pub safe_dis: SAFE_DIS,
    #[doc = "0x18 - Pulse Train Ready Interrupt Flags"]
    pub ready_intfl: READY_INTFL,
    #[doc = "0x1c - Pulse Train Ready Interrupt Enable/Disable"]
    pub ready_inten: READY_INTEN,
}
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Global Enable/Disable Controls for All Pulse Trains"]
pub mod enable;
#[doc = "RESYNC (rw) register accessor: an alias for `Reg<RESYNC_SPEC>`"]
pub type RESYNC = crate::Reg<resync::RESYNC_SPEC>;
#[doc = "Global Resync (All Pulse Trains) Control"]
pub mod resync;
#[doc = "STOP_INTFL (rw) register accessor: an alias for `Reg<STOP_INTFL_SPEC>`"]
pub type STOP_INTFL = crate::Reg<stop_intfl::STOP_INTFL_SPEC>;
#[doc = "Pulse Train Stop Interrupt Flags"]
pub mod stop_intfl;
#[doc = "STOP_INTEN (rw) register accessor: an alias for `Reg<STOP_INTEN_SPEC>`"]
pub type STOP_INTEN = crate::Reg<stop_inten::STOP_INTEN_SPEC>;
#[doc = "Pulse Train Stop Interrupt Enable/Disable"]
pub mod stop_inten;
#[doc = "SAFE_EN (w) register accessor: an alias for `Reg<SAFE_EN_SPEC>`"]
pub type SAFE_EN = crate::Reg<safe_en::SAFE_EN_SPEC>;
#[doc = "Pulse Train Global Safe Enable."]
pub mod safe_en;
#[doc = "SAFE_DIS (w) register accessor: an alias for `Reg<SAFE_DIS_SPEC>`"]
pub type SAFE_DIS = crate::Reg<safe_dis::SAFE_DIS_SPEC>;
#[doc = "Pulse Train Global Safe Disable."]
pub mod safe_dis;
#[doc = "READY_INTFL (rw) register accessor: an alias for `Reg<READY_INTFL_SPEC>`"]
pub type READY_INTFL = crate::Reg<ready_intfl::READY_INTFL_SPEC>;
#[doc = "Pulse Train Ready Interrupt Flags"]
pub mod ready_intfl;
#[doc = "READY_INTEN (rw) register accessor: an alias for `Reg<READY_INTEN_SPEC>`"]
pub type READY_INTEN = crate::Reg<ready_inten::READY_INTEN_SPEC>;
#[doc = "Pulse Train Ready Interrupt Enable/Disable"]
pub mod ready_inten;
