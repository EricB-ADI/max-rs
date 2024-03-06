#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AES Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - AES Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - AES Interrupt Flag Register"]
    pub intfl: INTFL,
    #[doc = "0x0c - AES Interrupt Enable Register"]
    pub inten: INTEN,
    #[doc = "0x10 - AES Data Register"]
    pub fifo: FIFO,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "AES Control Register"]
pub mod ctrl;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "AES Status Register"]
pub mod status;
#[doc = "INTFL (rw) register accessor: an alias for `Reg<INTFL_SPEC>`"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "AES Interrupt Flag Register"]
pub mod intfl;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "AES Interrupt Enable Register"]
pub mod inten;
#[doc = "FIFO (rw) register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "AES Data Register"]
pub mod fifo;
