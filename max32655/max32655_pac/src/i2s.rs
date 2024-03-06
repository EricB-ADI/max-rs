#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global mode channel."]
    pub ctrl0ch0: CTRL0CH0,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Local channel Setup."]
    pub ctrl1ch0: CTRL1CH0,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20 - Filter."]
    pub filtch0: FILTCH0,
    _reserved3: [u8; 0x0c],
    #[doc = "0x30 - DMA Control."]
    pub dmach0: DMACH0,
    _reserved4: [u8; 0x0c],
    #[doc = "0x40 - I2S Fifo."]
    pub fifoch0: FIFOCH0,
    _reserved5: [u8; 0x0c],
    #[doc = "0x50 - ISR Status."]
    pub intfl: INTFL,
    #[doc = "0x54 - Interrupt Enable."]
    pub inten: INTEN,
    #[doc = "0x58 - Ext Control."]
    pub extsetup: EXTSETUP,
    #[doc = "0x5c - Wakeup Enable."]
    pub wken: WKEN,
    #[doc = "0x60 - Wakeup Flags."]
    pub wkfl: WKFL,
}
#[doc = "CTRL0CH0 (rw) register accessor: an alias for `Reg<CTRL0CH0_SPEC>`"]
pub type CTRL0CH0 = crate::Reg<ctrl0ch0::CTRL0CH0_SPEC>;
#[doc = "Global mode channel."]
pub mod ctrl0ch0;
#[doc = "CTRL1CH0 (rw) register accessor: an alias for `Reg<CTRL1CH0_SPEC>`"]
pub type CTRL1CH0 = crate::Reg<ctrl1ch0::CTRL1CH0_SPEC>;
#[doc = "Local channel Setup."]
pub mod ctrl1ch0;
#[doc = "FILTCH0 (rw) register accessor: an alias for `Reg<FILTCH0_SPEC>`"]
pub type FILTCH0 = crate::Reg<filtch0::FILTCH0_SPEC>;
#[doc = "Filter."]
pub mod filtch0;
#[doc = "DMACH0 (rw) register accessor: an alias for `Reg<DMACH0_SPEC>`"]
pub type DMACH0 = crate::Reg<dmach0::DMACH0_SPEC>;
#[doc = "DMA Control."]
pub mod dmach0;
#[doc = "FIFOCH0 (rw) register accessor: an alias for `Reg<FIFOCH0_SPEC>`"]
pub type FIFOCH0 = crate::Reg<fifoch0::FIFOCH0_SPEC>;
#[doc = "I2S Fifo."]
pub mod fifoch0;
#[doc = "INTFL (rw) register accessor: an alias for `Reg<INTFL_SPEC>`"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "ISR Status."]
pub mod intfl;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt Enable."]
pub mod inten;
#[doc = "EXTSETUP (rw) register accessor: an alias for `Reg<EXTSETUP_SPEC>`"]
pub type EXTSETUP = crate::Reg<extsetup::EXTSETUP_SPEC>;
#[doc = "Ext Control."]
pub mod extsetup;
#[doc = "WKEN (rw) register accessor: an alias for `Reg<WKEN_SPEC>`"]
pub type WKEN = crate::Reg<wken::WKEN_SPEC>;
#[doc = "Wakeup Enable."]
pub mod wken;
#[doc = "WKFL (rw) register accessor: an alias for `Reg<WKFL_SPEC>`"]
pub type WKFL = crate::Reg<wkfl::WKFL_SPEC>;
#[doc = "Wakeup Flags."]
pub mod wkfl;
