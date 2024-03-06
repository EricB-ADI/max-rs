#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status register"]
    pub status: STATUS,
    #[doc = "0x08 - Interrupt Enable control register"]
    pub int_en: INT_EN,
    #[doc = "0x0c - Interrupt status flags Control register"]
    pub int_fl: INT_FL,
    #[doc = "0x10 - Clock Divider register"]
    pub clkdiv: CLKDIV,
    #[doc = "0x14 - Over Sampling Rate register"]
    pub osr: OSR,
    #[doc = "0x18 - TX FIFO Output Peek register"]
    pub txpeek: TXPEEK,
    #[doc = "0x1c - Pin register"]
    pub pnr: PNR,
    #[doc = "0x20 - FIFO Read/Write register"]
    pub fifo: FIFO,
    _reserved9: [u8; 0x0c],
    #[doc = "0x30 - DMA Configuration register"]
    pub dma: DMA,
    #[doc = "0x34 - Wake up enable Control register"]
    pub wken: WKEN,
    #[doc = "0x38 - Wake up Flags register"]
    pub wkfl: WKFL,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register"]
pub mod status;
#[doc = "INT_EN (rw) register accessor: an alias for `Reg<INT_EN_SPEC>`"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "Interrupt Enable control register"]
pub mod int_en;
#[doc = "INT_FL (rw) register accessor: an alias for `Reg<INT_FL_SPEC>`"]
pub type INT_FL = crate::Reg<int_fl::INT_FL_SPEC>;
#[doc = "Interrupt status flags Control register"]
pub mod int_fl;
#[doc = "CLKDIV (rw) register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock Divider register"]
pub mod clkdiv;
#[doc = "OSR (rw) register accessor: an alias for `Reg<OSR_SPEC>`"]
pub type OSR = crate::Reg<osr::OSR_SPEC>;
#[doc = "Over Sampling Rate register"]
pub mod osr;
#[doc = "TXPEEK (rw) register accessor: an alias for `Reg<TXPEEK_SPEC>`"]
pub type TXPEEK = crate::Reg<txpeek::TXPEEK_SPEC>;
#[doc = "TX FIFO Output Peek register"]
pub mod txpeek;
#[doc = "PNR (rw) register accessor: an alias for `Reg<PNR_SPEC>`"]
pub type PNR = crate::Reg<pnr::PNR_SPEC>;
#[doc = "Pin register"]
pub mod pnr;
#[doc = "FIFO (rw) register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "FIFO Read/Write register"]
pub mod fifo;
#[doc = "DMA (rw) register accessor: an alias for `Reg<DMA_SPEC>`"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "DMA Configuration register"]
pub mod dma;
#[doc = "WKEN (rw) register accessor: an alias for `Reg<WKEN_SPEC>`"]
pub type WKEN = crate::Reg<wken::WKEN_SPEC>;
#[doc = "Wake up enable Control register"]
pub mod wken;
#[doc = "WKFL (rw) register accessor: an alias for `Reg<WKFL_SPEC>`"]
pub type WKFL = crate::Reg<wkfl::WKFL_SPEC>;
#[doc = "Wake up Flags register"]
pub mod wkfl;
