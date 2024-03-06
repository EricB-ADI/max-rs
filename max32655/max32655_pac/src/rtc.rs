#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Second Counter. This register contains the 32-bit second counter."]
    pub sec: SEC,
    #[doc = "0x04 - RTC Sub-second Counter. This counter increments at 256Hz. RTC_SEC is incremented when this register rolls over from 0xFF to 0x00."]
    pub ssec: SSEC,
    #[doc = "0x08 - Time-of-day Alarm."]
    pub toda: TODA,
    #[doc = "0x0c - RTC sub-second alarm. This register contains the reload value for the sub-second alarm."]
    pub sseca: SSECA,
    #[doc = "0x10 - RTC Control Register."]
    pub ctrl: CTRL,
    #[doc = "0x14 - RTC Trim Register."]
    pub trim: TRIM,
    #[doc = "0x18 - RTC Oscillator Control Register."]
    pub oscctrl: OSCCTRL,
}
#[doc = "SEC (rw) register accessor: an alias for `Reg<SEC_SPEC>`"]
pub type SEC = crate::Reg<sec::SEC_SPEC>;
#[doc = "RTC Second Counter. This register contains the 32-bit second counter."]
pub mod sec;
#[doc = "SSEC (rw) register accessor: an alias for `Reg<SSEC_SPEC>`"]
pub type SSEC = crate::Reg<ssec::SSEC_SPEC>;
#[doc = "RTC Sub-second Counter. This counter increments at 256Hz. RTC_SEC is incremented when this register rolls over from 0xFF to 0x00."]
pub mod ssec;
#[doc = "TODA (rw) register accessor: an alias for `Reg<TODA_SPEC>`"]
pub type TODA = crate::Reg<toda::TODA_SPEC>;
#[doc = "Time-of-day Alarm."]
pub mod toda;
#[doc = "SSECA (rw) register accessor: an alias for `Reg<SSECA_SPEC>`"]
pub type SSECA = crate::Reg<sseca::SSECA_SPEC>;
#[doc = "RTC sub-second alarm. This register contains the reload value for the sub-second alarm."]
pub mod sseca;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "RTC Control Register."]
pub mod ctrl;
#[doc = "TRIM (rw) register accessor: an alias for `Reg<TRIM_SPEC>`"]
pub type TRIM = crate::Reg<trim::TRIM_SPEC>;
#[doc = "RTC Trim Register."]
pub mod trim;
#[doc = "OSCCTRL (rw) register accessor: an alias for `Reg<OSCCTRL_SPEC>`"]
pub type OSCCTRL = crate::Reg<oscctrl::OSCCTRL_SPEC>;
#[doc = "RTC Oscillator Control Register."]
pub mod oscctrl;
