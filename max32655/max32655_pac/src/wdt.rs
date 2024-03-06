#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Timer Control Register."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Windowed Watchdog Timer Reset Register."]
    pub rst: RST,
    #[doc = "0x08 - Windowed Watchdog Timer Clock Select Register."]
    pub clksel: CLKSEL,
    #[doc = "0x0c - Windowed Watchdog Timer Count Register."]
    pub cnt: CNT,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Watchdog Timer Control Register."]
pub mod ctrl;
#[doc = "RST (w) register accessor: an alias for `Reg<RST_SPEC>`"]
pub type RST = crate::Reg<rst::RST_SPEC>;
#[doc = "Windowed Watchdog Timer Reset Register."]
pub mod rst;
#[doc = "CLKSEL (rw) register accessor: an alias for `Reg<CLKSEL_SPEC>`"]
pub type CLKSEL = crate::Reg<clksel::CLKSEL_SPEC>;
#[doc = "Windowed Watchdog Timer Clock Select Register."]
pub mod clksel;
#[doc = "CNT (r) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Windowed Watchdog Timer Count Register."]
pub mod cnt;
