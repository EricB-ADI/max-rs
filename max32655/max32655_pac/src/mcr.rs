#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ECC Enable Register"]
    pub eccen: ECCEN,
    #[doc = "0x04 - IPO Manual Register"]
    pub ipo_mtrim: IPO_MTRIM,
    #[doc = "0x08 - Output Enable Register"]
    pub outen: OUTEN,
    #[doc = "0x0c - Comparator Control Register."]
    pub cmp_ctrl: CMP_CTRL,
    #[doc = "0x10 - Miscellaneous Control Register."]
    pub ctrl: CTRL,
    _reserved5: [u8; 0x0c],
    #[doc = "0x20 - GPIO3 Pin Control Register."]
    pub gpio3_ctrl: GPIO3_CTRL,
}
#[doc = "ECCEN (rw) register accessor: an alias for `Reg<ECCEN_SPEC>`"]
pub type ECCEN = crate::Reg<eccen::ECCEN_SPEC>;
#[doc = "ECC Enable Register"]
pub mod eccen;
#[doc = "IPO_MTRIM (rw) register accessor: an alias for `Reg<IPO_MTRIM_SPEC>`"]
pub type IPO_MTRIM = crate::Reg<ipo_mtrim::IPO_MTRIM_SPEC>;
#[doc = "IPO Manual Register"]
pub mod ipo_mtrim;
#[doc = "OUTEN (rw) register accessor: an alias for `Reg<OUTEN_SPEC>`"]
pub type OUTEN = crate::Reg<outen::OUTEN_SPEC>;
#[doc = "Output Enable Register"]
pub mod outen;
#[doc = "CMP_CTRL (rw) register accessor: an alias for `Reg<CMP_CTRL_SPEC>`"]
pub type CMP_CTRL = crate::Reg<cmp_ctrl::CMP_CTRL_SPEC>;
#[doc = "Comparator Control Register."]
pub mod cmp_ctrl;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Miscellaneous Control Register."]
pub mod ctrl;
#[doc = "GPIO3_CTRL (rw) register accessor: an alias for `Reg<GPIO3_CTRL_SPEC>`"]
pub type GPIO3_CTRL = crate::Reg<gpio3_ctrl::GPIO3_CTRL_SPEC>;
#[doc = "GPIO3 Pin Control Register."]
pub mod gpio3_ctrl;
