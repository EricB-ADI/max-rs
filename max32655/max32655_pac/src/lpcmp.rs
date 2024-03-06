#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x0c - Comparator Control Register."]
    pub ctrl: [CTRL; 3],
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Comparator Control Register."]
pub mod ctrl;
