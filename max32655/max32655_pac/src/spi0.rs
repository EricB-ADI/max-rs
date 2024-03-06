#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_fifo8: [u8; 0x04],
    #[doc = "0x04 - Register for controlling SPI peripheral."]
    pub ctrl0: CTRL0,
    #[doc = "0x08 - Register for controlling SPI peripheral."]
    pub ctrl1: CTRL1,
    #[doc = "0x0c - Register for controlling SPI peripheral."]
    pub ctrl2: CTRL2,
    #[doc = "0x10 - Register for controlling SPI peripheral/Slave Select Timing."]
    pub sstime: SSTIME,
    #[doc = "0x14 - Register for controlling SPI clock rate."]
    pub clkctrl: CLKCTRL,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - Register for controlling DMA."]
    pub dma: DMA,
    #[doc = "0x20 - Register for reading and clearing interrupt flags. All bits are write 1 to clear."]
    pub intfl: INTFL,
    #[doc = "0x24 - Register for enabling interrupts."]
    pub inten: INTEN,
    #[doc = "0x28 - Register for wake up flags. All bits in this register are write 1 to clear."]
    pub wkfl: WKFL,
    #[doc = "0x2c - Register for wake up enable."]
    pub wken: WKEN,
    #[doc = "0x30 - SPI Status register."]
    pub stat: STAT,
}
impl RegisterBlock {
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub const fn fifo8(&self) -> &[FIFO8; 4] {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub const fn fifo16(&self) -> &[FIFO16; 2] {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub const fn fifo32(&self) -> &FIFO32 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
}
#[doc = "FIFO32 (rw) register accessor: an alias for `Reg<FIFO32_SPEC>`"]
pub type FIFO32 = crate::Reg<fifo32::FIFO32_SPEC>;
#[doc = "Register for reading and writing the FIFO."]
pub mod fifo32;
#[doc = "FIFO16 (rw) register accessor: an alias for `Reg<FIFO16_SPEC>`"]
pub type FIFO16 = crate::Reg<fifo16::FIFO16_SPEC>;
#[doc = "Register for reading and writing the FIFO."]
pub mod fifo16;
#[doc = "FIFO8 (rw) register accessor: an alias for `Reg<FIFO8_SPEC>`"]
pub type FIFO8 = crate::Reg<fifo8::FIFO8_SPEC>;
#[doc = "Register for reading and writing the FIFO."]
pub mod fifo8;
#[doc = "CTRL0 (rw) register accessor: an alias for `Reg<CTRL0_SPEC>`"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "Register for controlling SPI peripheral."]
pub mod ctrl0;
#[doc = "CTRL1 (rw) register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Register for controlling SPI peripheral."]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Register for controlling SPI peripheral."]
pub mod ctrl2;
#[doc = "SSTIME (rw) register accessor: an alias for `Reg<SSTIME_SPEC>`"]
pub type SSTIME = crate::Reg<sstime::SSTIME_SPEC>;
#[doc = "Register for controlling SPI peripheral/Slave Select Timing."]
pub mod sstime;
#[doc = "CLKCTRL (rw) register accessor: an alias for `Reg<CLKCTRL_SPEC>`"]
pub type CLKCTRL = crate::Reg<clkctrl::CLKCTRL_SPEC>;
#[doc = "Register for controlling SPI clock rate."]
pub mod clkctrl;
#[doc = "DMA (rw) register accessor: an alias for `Reg<DMA_SPEC>`"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "Register for controlling DMA."]
pub mod dma;
#[doc = "INTFL (rw) register accessor: an alias for `Reg<INTFL_SPEC>`"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "Register for reading and clearing interrupt flags. All bits are write 1 to clear."]
pub mod intfl;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Register for enabling interrupts."]
pub mod inten;
#[doc = "WKFL (rw) register accessor: an alias for `Reg<WKFL_SPEC>`"]
pub type WKFL = crate::Reg<wkfl::WKFL_SPEC>;
#[doc = "Register for wake up flags. All bits in this register are write 1 to clear."]
pub mod wkfl;
#[doc = "WKEN (rw) register accessor: an alias for `Reg<WKEN_SPEC>`"]
pub type WKEN = crate::Reg<wken::WKEN_SPEC>;
#[doc = "Register for wake up enable."]
pub mod wken;
#[doc = "STAT (r) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "SPI Status register."]
pub mod stat;
