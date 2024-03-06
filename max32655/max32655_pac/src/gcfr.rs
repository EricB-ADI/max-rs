#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Register 0."]
    pub reg0: REG0,
    #[doc = "0x04 - Register 1."]
    pub reg1: REG1,
}
#[doc = "REG0 (rw) register accessor: an alias for `Reg<REG0_SPEC>`"]
pub type REG0 = crate::Reg<reg0::REG0_SPEC>;
#[doc = "Register 0."]
pub mod reg0;
#[doc = "REG1 (rw) register accessor: an alias for `Reg<REG1_SPEC>`"]
pub type REG1 = crate::Reg<reg1::REG1_SPEC>;
#[doc = "Register 1."]
pub mod reg1;
