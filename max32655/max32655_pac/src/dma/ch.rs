#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - DMA Channel Control Register."]
    pub ctrl: CTRL,
    #[doc = "0x04 - DMA Channel Status Register."]
    pub status: STATUS,
    #[doc = "0x08 - Source Device Address. If SRCINC=1, the counter bits are incremented by 1,2, or 4, depending on the data width of each AHB cycle. For peripheral transfers, some or all of the actual address bits are fixed. If SRCINC=0, this register remains constant. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with the contents of DMA_SRC_RLD."]
    pub src: SRC,
    #[doc = "0x0c - Destination Device Address. For peripheral transfers, some or all of the actual address bits are fixed. If DSTINC=1, this register is incremented on every AHB write out of the DMA FIFO. They are incremented by 1, 2, or 4, depending on the data width of each AHB cycle. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with DMA_DST_RLD."]
    pub dst: DST,
    #[doc = "0x10 - DMA Counter. The user loads this register with the number of bytes to transfer. This counter decreases on every AHB cycle into the DMA FIFO. The decrement will be 1, 2, or 4 depending on the data width of each AHB cycle. When the counter reaches 0, a count-to-zero condition is triggered."]
    pub cnt: CNT,
    #[doc = "0x14 - Source Address Reload Value. The value of this register is loaded into DMA0_SRC upon a count-to-zero condition."]
    pub srcrld: SRCRLD,
    #[doc = "0x18 - Destination Address Reload Value. The value of this register is loaded into DMA0_DST upon a count-to-zero condition."]
    pub dstrld: DSTRLD,
    #[doc = "0x1c - DMA Channel Count Reload Register."]
    pub cntrld: CNTRLD,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "DMA Channel Control Register."]
pub mod ctrl;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "DMA Channel Status Register."]
pub mod status;
#[doc = "SRC (rw) register accessor: an alias for `Reg<SRC_SPEC>`"]
pub type SRC = crate::Reg<src::SRC_SPEC>;
#[doc = "Source Device Address. If SRCINC=1, the counter bits are incremented by 1,2, or 4, depending on the data width of each AHB cycle. For peripheral transfers, some or all of the actual address bits are fixed. If SRCINC=0, this register remains constant. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with the contents of DMA_SRC_RLD."]
pub mod src;
#[doc = "DST (rw) register accessor: an alias for `Reg<DST_SPEC>`"]
pub type DST = crate::Reg<dst::DST_SPEC>;
#[doc = "Destination Device Address. For peripheral transfers, some or all of the actual address bits are fixed. If DSTINC=1, this register is incremented on every AHB write out of the DMA FIFO. They are incremented by 1, 2, or 4, depending on the data width of each AHB cycle. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with DMA_DST_RLD."]
pub mod dst;
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "DMA Counter. The user loads this register with the number of bytes to transfer. This counter decreases on every AHB cycle into the DMA FIFO. The decrement will be 1, 2, or 4 depending on the data width of each AHB cycle. When the counter reaches 0, a count-to-zero condition is triggered."]
pub mod cnt;
#[doc = "SRCRLD (rw) register accessor: an alias for `Reg<SRCRLD_SPEC>`"]
pub type SRCRLD = crate::Reg<srcrld::SRCRLD_SPEC>;
#[doc = "Source Address Reload Value. The value of this register is loaded into DMA0_SRC upon a count-to-zero condition."]
pub mod srcrld;
#[doc = "DSTRLD (rw) register accessor: an alias for `Reg<DSTRLD_SPEC>`"]
pub type DSTRLD = crate::Reg<dstrld::DSTRLD_SPEC>;
#[doc = "Destination Address Reload Value. The value of this register is loaded into DMA0_DST upon a count-to-zero condition."]
pub mod dstrld;
#[doc = "CNTRLD (rw) register accessor: an alias for `Reg<CNTRLD_SPEC>`"]
pub type CNTRLD = crate::Reg<cntrld::CNTRLD_SPEC>;
#[doc = "DMA Channel Count Reload Register."]
pub mod cntrld;
