#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pulse Train Configuration"]
    pub rate_length: RATE_LENGTH,
    #[doc = "0x04 - Write the repeating bit pattern that is shifted out, LSB first, when configured in Pulse Train mode. See PT_RATE_LENGTH.mode for setting the length."]
    pub train: TRAIN,
    #[doc = "0x08 - Pulse Train Loop Count"]
    pub loop_: LOOP,
    #[doc = "0x0c - Pulse Train Auto-Restart Configuration."]
    pub restart: RESTART,
}
#[doc = "RATE_LENGTH (rw) register accessor: an alias for `Reg<RATE_LENGTH_SPEC>`"]
pub type RATE_LENGTH = crate::Reg<rate_length::RATE_LENGTH_SPEC>;
#[doc = "Pulse Train Configuration"]
pub mod rate_length;
#[doc = "TRAIN (rw) register accessor: an alias for `Reg<TRAIN_SPEC>`"]
pub type TRAIN = crate::Reg<train::TRAIN_SPEC>;
#[doc = "Write the repeating bit pattern that is shifted out, LSB first, when configured in Pulse Train mode. See PT_RATE_LENGTH.mode for setting the length."]
pub mod train;
#[doc = "LOOP (rw) register accessor: an alias for `Reg<LOOP_SPEC>`"]
pub type LOOP = crate::Reg<loop_::LOOP_SPEC>;
#[doc = "Pulse Train Loop Count"]
pub mod loop_;
#[doc = "RESTART (rw) register accessor: an alias for `Reg<RESTART_SPEC>`"]
pub type RESTART = crate::Reg<restart::RESTART_SPEC>;
#[doc = "Pulse Train Auto-Restart Configuration."]
pub mod restart;
