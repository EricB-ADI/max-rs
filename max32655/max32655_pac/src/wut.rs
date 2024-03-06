#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Count. This register stores the current timer count."]
    pub cnt: CNT,
    #[doc = "0x04 - Compare. This register stores the compare value, which is used to set the maximum count value to initiate a reload of the timer to 0x0001."]
    pub cmp: CMP,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Clear Interrupt. Writing a value (0 or 1) to a bit in this register clears the associated interrupt."]
    pub intr: INTR,
    #[doc = "0x10 - Timer Control Register."]
    pub ctrl: CTRL,
    #[doc = "0x14 - Timer Non-Overlapping Compare Register."]
    pub nolcmp: NOLCMP,
    #[doc = "0x18 - Preset register."]
    pub preset: PRESET,
    #[doc = "0x1c - Reload register."]
    pub reload: RELOAD,
    #[doc = "0x20 - Snapshot register."]
    pub snapshot: SNAPSHOT,
}
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Count. This register stores the current timer count."]
pub mod cnt;
#[doc = "CMP (rw) register accessor: an alias for `Reg<CMP_SPEC>`"]
pub type CMP = crate::Reg<cmp::CMP_SPEC>;
#[doc = "Compare. This register stores the compare value, which is used to set the maximum count value to initiate a reload of the timer to 0x0001."]
pub mod cmp;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Clear Interrupt. Writing a value (0 or 1) to a bit in this register clears the associated interrupt."]
pub mod intr;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Timer Control Register."]
pub mod ctrl;
#[doc = "NOLCMP (rw) register accessor: an alias for `Reg<NOLCMP_SPEC>`"]
pub type NOLCMP = crate::Reg<nolcmp::NOLCMP_SPEC>;
#[doc = "Timer Non-Overlapping Compare Register."]
pub mod nolcmp;
#[doc = "PRESET (rw) register accessor: an alias for `Reg<PRESET_SPEC>`"]
pub type PRESET = crate::Reg<preset::PRESET_SPEC>;
#[doc = "Preset register."]
pub mod preset;
#[doc = "RELOAD (rw) register accessor: an alias for `Reg<RELOAD_SPEC>`"]
pub type RELOAD = crate::Reg<reload::RELOAD_SPEC>;
#[doc = "Reload register."]
pub mod reload;
#[doc = "SNAPSHOT (rw) register accessor: an alias for `Reg<SNAPSHOT_SPEC>`"]
pub type SNAPSHOT = crate::Reg<snapshot::SNAPSHOT_SPEC>;
#[doc = "Snapshot register."]
pub mod snapshot;
