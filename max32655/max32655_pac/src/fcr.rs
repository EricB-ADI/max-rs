#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Function Control 0."]
    pub fctrl0: FCTRL0,
    #[doc = "0x04 - Automatic Calibration 0."]
    pub autocal0: AUTOCAL0,
    #[doc = "0x08 - Automatic Calibration 1."]
    pub autocal1: AUTOCAL1,
    #[doc = "0x0c - Automatic Calibration 2"]
    pub autocal2: AUTOCAL2,
    #[doc = "0x10 - RISC-V Boot Address."]
    pub urvbootaddr: URVBOOTADDR,
    #[doc = "0x14 - RISC-V Control Register."]
    pub urvctrl: URVCTRL,
    #[doc = "0x18 - ERFO Kick Start Register."]
    pub erfoks: ERFOKS,
    #[doc = "0x1c - ERFO Ready Interrupt Flag register."]
    pub erfo_intfl: ERFO_INTFL,
    #[doc = "0x20 - ERFO Ready Interrupt Enable register."]
    pub erfo_inten: ERFO_INTEN,
}
#[doc = "FCTRL0 (rw) register accessor: an alias for `Reg<FCTRL0_SPEC>`"]
pub type FCTRL0 = crate::Reg<fctrl0::FCTRL0_SPEC>;
#[doc = "Function Control 0."]
pub mod fctrl0;
#[doc = "AUTOCAL0 (rw) register accessor: an alias for `Reg<AUTOCAL0_SPEC>`"]
pub type AUTOCAL0 = crate::Reg<autocal0::AUTOCAL0_SPEC>;
#[doc = "Automatic Calibration 0."]
pub mod autocal0;
#[doc = "AUTOCAL1 (rw) register accessor: an alias for `Reg<AUTOCAL1_SPEC>`"]
pub type AUTOCAL1 = crate::Reg<autocal1::AUTOCAL1_SPEC>;
#[doc = "Automatic Calibration 1."]
pub mod autocal1;
#[doc = "AUTOCAL2 (rw) register accessor: an alias for `Reg<AUTOCAL2_SPEC>`"]
pub type AUTOCAL2 = crate::Reg<autocal2::AUTOCAL2_SPEC>;
#[doc = "Automatic Calibration 2"]
pub mod autocal2;
#[doc = "URVBOOTADDR (rw) register accessor: an alias for `Reg<URVBOOTADDR_SPEC>`"]
pub type URVBOOTADDR = crate::Reg<urvbootaddr::URVBOOTADDR_SPEC>;
#[doc = "RISC-V Boot Address."]
pub mod urvbootaddr;
#[doc = "URVCTRL (rw) register accessor: an alias for `Reg<URVCTRL_SPEC>`"]
pub type URVCTRL = crate::Reg<urvctrl::URVCTRL_SPEC>;
#[doc = "RISC-V Control Register."]
pub mod urvctrl;
#[doc = "ERFOKS (rw) register accessor: an alias for `Reg<ERFOKS_SPEC>`"]
pub type ERFOKS = crate::Reg<erfoks::ERFOKS_SPEC>;
#[doc = "ERFO Kick Start Register."]
pub mod erfoks;
#[doc = "ERFO_INTFL (rw) register accessor: an alias for `Reg<ERFO_INTFL_SPEC>`"]
pub type ERFO_INTFL = crate::Reg<erfo_intfl::ERFO_INTFL_SPEC>;
#[doc = "ERFO Ready Interrupt Flag register."]
pub mod erfo_intfl;
#[doc = "ERFO_INTEN (rw) register accessor: an alias for `Reg<ERFO_INTEN_SPEC>`"]
pub type ERFO_INTEN = crate::Reg<erfo_inten::ERFO_INTEN_SPEC>;
#[doc = "ERFO Ready Interrupt Enable register."]
pub mod erfo_inten;
