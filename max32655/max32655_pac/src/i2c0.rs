#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register0."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status Register."]
    pub status: STATUS,
    #[doc = "0x08 - Interrupt Status Register."]
    pub intfl0: INTFL0,
    #[doc = "0x0c - Interrupt Enable Register."]
    pub inten0: INTEN0,
    #[doc = "0x10 - Interrupt Status Register 1."]
    pub intfl1: INTFL1,
    #[doc = "0x14 - Interrupt Staus Register 1."]
    pub inten1: INTEN1,
    #[doc = "0x18 - FIFO Configuration Register."]
    pub fifolen: FIFOLEN,
    #[doc = "0x1c - Receive Control Register 0."]
    pub rxctrl0: RXCTRL0,
    #[doc = "0x20 - Receive Control Register 1."]
    pub rxctrl1: RXCTRL1,
    #[doc = "0x24 - Transmit Control Register 0."]
    pub txctrl0: TXCTRL0,
    #[doc = "0x28 - Transmit Control Register 1."]
    pub txctrl1: TXCTRL1,
    #[doc = "0x2c - Data Register."]
    pub fifo: FIFO,
    #[doc = "0x30 - Master Control Register."]
    pub mstctrl: MSTCTRL,
    #[doc = "0x34 - Clock Low Register."]
    pub clklo: CLKLO,
    #[doc = "0x38 - Clock high Register."]
    pub clkhi: CLKHI,
    #[doc = "0x3c - Clock high Register."]
    pub hsclk: HSCLK,
    #[doc = "0x40 - Timeout Register"]
    pub timeout: TIMEOUT,
    _reserved17: [u8; 0x04],
    #[doc = "0x48 - DMA Register."]
    pub dma: DMA,
    _reserved_18_slave0: [u8; 0x10],
}
impl RegisterBlock {
    #[doc = "0x4c - Slave Address Register."]
    #[inline(always)]
    pub const fn slave0(&self) -> &SLAVE0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(76usize).cast() }
    }
    #[doc = "0x4c..0x5c - Slave Address Register."]
    #[inline(always)]
    pub const fn slave_multi(&self) -> &[SLAVE_MULTI; 4] {
        unsafe { &*(self as *const Self).cast::<u8>().add(76usize).cast() }
    }
    #[doc = "0x50 - Slave Address Register."]
    #[inline(always)]
    pub const fn slave1(&self) -> &SLAVE1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(80usize).cast() }
    }
    #[doc = "0x54 - Slave Address Register."]
    #[inline(always)]
    pub const fn slave2(&self) -> &SLAVE2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(84usize).cast() }
    }
    #[doc = "0x58 - Slave Address Register."]
    #[inline(always)]
    pub const fn slave3(&self) -> &SLAVE3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(88usize).cast() }
    }
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register0."]
pub mod ctrl;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register."]
pub mod status;
#[doc = "INTFL0 (rw) register accessor: an alias for `Reg<INTFL0_SPEC>`"]
pub type INTFL0 = crate::Reg<intfl0::INTFL0_SPEC>;
#[doc = "Interrupt Status Register."]
pub mod intfl0;
#[doc = "INTEN0 (rw) register accessor: an alias for `Reg<INTEN0_SPEC>`"]
pub type INTEN0 = crate::Reg<inten0::INTEN0_SPEC>;
#[doc = "Interrupt Enable Register."]
pub mod inten0;
#[doc = "INTFL1 (rw) register accessor: an alias for `Reg<INTFL1_SPEC>`"]
pub type INTFL1 = crate::Reg<intfl1::INTFL1_SPEC>;
#[doc = "Interrupt Status Register 1."]
pub mod intfl1;
#[doc = "INTEN1 (rw) register accessor: an alias for `Reg<INTEN1_SPEC>`"]
pub type INTEN1 = crate::Reg<inten1::INTEN1_SPEC>;
#[doc = "Interrupt Staus Register 1."]
pub mod inten1;
#[doc = "FIFOLEN (rw) register accessor: an alias for `Reg<FIFOLEN_SPEC>`"]
pub type FIFOLEN = crate::Reg<fifolen::FIFOLEN_SPEC>;
#[doc = "FIFO Configuration Register."]
pub mod fifolen;
#[doc = "RXCTRL0 (rw) register accessor: an alias for `Reg<RXCTRL0_SPEC>`"]
pub type RXCTRL0 = crate::Reg<rxctrl0::RXCTRL0_SPEC>;
#[doc = "Receive Control Register 0."]
pub mod rxctrl0;
#[doc = "RXCTRL1 (rw) register accessor: an alias for `Reg<RXCTRL1_SPEC>`"]
pub type RXCTRL1 = crate::Reg<rxctrl1::RXCTRL1_SPEC>;
#[doc = "Receive Control Register 1."]
pub mod rxctrl1;
#[doc = "TXCTRL0 (rw) register accessor: an alias for `Reg<TXCTRL0_SPEC>`"]
pub type TXCTRL0 = crate::Reg<txctrl0::TXCTRL0_SPEC>;
#[doc = "Transmit Control Register 0."]
pub mod txctrl0;
#[doc = "TXCTRL1 (rw) register accessor: an alias for `Reg<TXCTRL1_SPEC>`"]
pub type TXCTRL1 = crate::Reg<txctrl1::TXCTRL1_SPEC>;
#[doc = "Transmit Control Register 1."]
pub mod txctrl1;
#[doc = "FIFO (rw) register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "Data Register."]
pub mod fifo;
#[doc = "MSTCTRL (rw) register accessor: an alias for `Reg<MSTCTRL_SPEC>`"]
pub type MSTCTRL = crate::Reg<mstctrl::MSTCTRL_SPEC>;
#[doc = "Master Control Register."]
pub mod mstctrl;
#[doc = "CLKLO (rw) register accessor: an alias for `Reg<CLKLO_SPEC>`"]
pub type CLKLO = crate::Reg<clklo::CLKLO_SPEC>;
#[doc = "Clock Low Register."]
pub mod clklo;
#[doc = "CLKHI (rw) register accessor: an alias for `Reg<CLKHI_SPEC>`"]
pub type CLKHI = crate::Reg<clkhi::CLKHI_SPEC>;
#[doc = "Clock high Register."]
pub mod clkhi;
#[doc = "HSCLK (rw) register accessor: an alias for `Reg<HSCLK_SPEC>`"]
pub type HSCLK = crate::Reg<hsclk::HSCLK_SPEC>;
#[doc = "Clock high Register."]
pub mod hsclk;
#[doc = "TIMEOUT (rw) register accessor: an alias for `Reg<TIMEOUT_SPEC>`"]
pub type TIMEOUT = crate::Reg<timeout::TIMEOUT_SPEC>;
#[doc = "Timeout Register"]
pub mod timeout;
#[doc = "DMA (rw) register accessor: an alias for `Reg<DMA_SPEC>`"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "DMA Register."]
pub mod dma;
#[doc = "SLAVE_MULTI (rw) register accessor: an alias for `Reg<SLAVE_MULTI_SPEC>`"]
pub type SLAVE_MULTI = crate::Reg<slave_multi::SLAVE_MULTI_SPEC>;
#[doc = "Slave Address Register."]
pub mod slave_multi;
#[doc = "SLAVE0 (rw) register accessor: an alias for `Reg<SLAVE0_SPEC>`"]
pub type SLAVE0 = crate::Reg<slave0::SLAVE0_SPEC>;
#[doc = "Slave Address Register."]
pub mod slave0;
#[doc = "SLAVE1 (rw) register accessor: an alias for `Reg<SLAVE1_SPEC>`"]
pub type SLAVE1 = crate::Reg<slave1::SLAVE1_SPEC>;
#[doc = "Slave Address Register."]
pub mod slave1;
#[doc = "SLAVE2 (rw) register accessor: an alias for `Reg<SLAVE2_SPEC>`"]
pub type SLAVE2 = crate::Reg<slave2::SLAVE2_SPEC>;
#[doc = "Slave Address Register."]
pub mod slave2;
#[doc = "SLAVE3 (rw) register accessor: an alias for `Reg<SLAVE3_SPEC>`"]
pub type SLAVE3 = crate::Reg<slave3::SLAVE3_SPEC>;
#[doc = "Slave Address Register."]
pub mod slave3;
