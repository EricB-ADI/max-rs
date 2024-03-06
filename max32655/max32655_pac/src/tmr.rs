#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Counter Register."]
    pub cnt: CNT,
    #[doc = "0x04 - Timer Compare Register."]
    pub cmp: CMP,
    #[doc = "0x08 - Timer PWM Register."]
    pub pwm: PWM,
    #[doc = "0x0c - Timer Interrupt Status Register."]
    pub intfl: INTFL,
    #[doc = "0x10 - Timer Control Register."]
    pub ctrl0: CTRL0,
    #[doc = "0x14 - Timer Non-Overlapping Compare Register."]
    pub nolcmp: NOLCMP,
    #[doc = "0x18 - Timer Configuration Register."]
    pub ctrl1: CTRL1,
    #[doc = "0x1c - Timer Wakeup Status Register."]
    pub wkfl: WKFL,
}
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Timer Counter Register."]
pub mod cnt;
#[doc = "CMP (rw) register accessor: an alias for `Reg<CMP_SPEC>`"]
pub type CMP = crate::Reg<cmp::CMP_SPEC>;
#[doc = "Timer Compare Register."]
pub mod cmp;
#[doc = "PWM (rw) register accessor: an alias for `Reg<PWM_SPEC>`"]
pub type PWM = crate::Reg<pwm::PWM_SPEC>;
#[doc = "Timer PWM Register."]
pub mod pwm;
#[doc = "INTFL (rw) register accessor: an alias for `Reg<INTFL_SPEC>`"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "Timer Interrupt Status Register."]
pub mod intfl;
#[doc = "CTRL0 (rw) register accessor: an alias for `Reg<CTRL0_SPEC>`"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "Timer Control Register."]
pub mod ctrl0;
#[doc = "NOLCMP (rw) register accessor: an alias for `Reg<NOLCMP_SPEC>`"]
pub type NOLCMP = crate::Reg<nolcmp::NOLCMP_SPEC>;
#[doc = "Timer Non-Overlapping Compare Register."]
pub mod nolcmp;
#[doc = "CTRL1 (rw) register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Timer Configuration Register."]
pub mod ctrl1;
#[doc = "WKFL (rw) register accessor: an alias for `Reg<WKFL_SPEC>`"]
pub type WKFL = crate::Reg<wkfl::WKFL_SPEC>;
#[doc = "Timer Wakeup Status Register."]
pub mod wkfl;
