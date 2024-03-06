#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Control Register."]
    pub inten: INTEN,
    #[doc = "0x04 - DMA Interrupt Register."]
    pub intfl: INTFL,
    _reserved2: [u8; 0xf8],
    #[doc = "0x100..0x180 - DMA Channel registers."]
    pub ch: [CH; 4],
}
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "DMA Control Register."]
pub mod inten;
#[doc = "INTFL (r) register accessor: an alias for `Reg<INTFL_SPEC>`"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "DMA Interrupt Register."]
pub mod intfl;
#[doc = "DMA Channel registers."]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "DMA Channel registers."]
pub mod ch;
