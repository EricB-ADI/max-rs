#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Low Power Control Register."]
    pub lpcn: LPCN,
    #[doc = "0x04 - Low Power I/O Wakeup Status Register 0. This register indicates the low power wakeup status for GPIO0."]
    pub lpwkst0: LPWKST0,
    #[doc = "0x08 - Low Power I/O Wakeup Enable Register 0. This register enables low power wakeup functionality for GPIO0."]
    pub lpwken0: LPWKEN0,
    #[doc = "0x0c - Low Power I/O Wakeup Status Register 1. This register indicates the low power wakeup status for GPIO1."]
    pub lpwkst1: LPWKST1,
    #[doc = "0x10 - Low Power I/O Wakeup Enable Register 1. This register enables low power wakeup functionality for GPIO1."]
    pub lpwken1: LPWKEN1,
    #[doc = "0x14 - Low Power I/O Wakeup Status Register 2. This register indicates the low power wakeup status for GPIO2."]
    pub lpwkst2: LPWKST2,
    #[doc = "0x18 - Low Power I/O Wakeup Enable Register 2. This register enables low power wakeup functionality for GPIO2."]
    pub lpwken2: LPWKEN2,
    #[doc = "0x1c - Low Power I/O Wakeup Status Register 3. This register indicates the low power wakeup status for GPIO3."]
    pub lpwkst3: LPWKST3,
    #[doc = "0x20 - Low Power I/O Wakeup Enable Register 3. This register enables low power wakeup functionality for GPIO3."]
    pub lpwken3: LPWKEN3,
    _reserved9: [u8; 0x0c],
    #[doc = "0x30 - Low Power Peripheral Wakeup Status Register."]
    pub lppwst: LPPWST,
    #[doc = "0x34 - Low Power Peripheral Wakeup Enable Register."]
    pub lppwen: LPPWEN,
    _reserved11: [u8; 0x0c],
    #[doc = "0x44 - Low-Power VBTLE Power Down Register."]
    pub vbtlepd: VBTLEPD,
    #[doc = "0x48 - General Purpose Register 0"]
    pub gp0: GP0,
    #[doc = "0x4c - General Purpose Register 1"]
    pub gp1: GP1,
}
#[doc = "LPCN (rw) register accessor: an alias for `Reg<LPCN_SPEC>`"]
pub type LPCN = crate::Reg<lpcn::LPCN_SPEC>;
#[doc = "Low Power Control Register."]
pub mod lpcn;
#[doc = "LPWKST0 (rw) register accessor: an alias for `Reg<LPWKST0_SPEC>`"]
pub type LPWKST0 = crate::Reg<lpwkst0::LPWKST0_SPEC>;
#[doc = "Low Power I/O Wakeup Status Register 0. This register indicates the low power wakeup status for GPIO0."]
pub mod lpwkst0;
#[doc = "LPWKEN0 (rw) register accessor: an alias for `Reg<LPWKEN0_SPEC>`"]
pub type LPWKEN0 = crate::Reg<lpwken0::LPWKEN0_SPEC>;
#[doc = "Low Power I/O Wakeup Enable Register 0. This register enables low power wakeup functionality for GPIO0."]
pub mod lpwken0;
pub use lpwken0 as lpwken1;
pub use lpwken0 as lpwken2;
pub use lpwken0 as lpwken3;
pub use lpwkst0 as lpwkst1;
pub use lpwkst0 as lpwkst2;
pub use lpwkst0 as lpwkst3;
pub use LPWKEN0 as LPWKEN1;
pub use LPWKEN0 as LPWKEN2;
pub use LPWKEN0 as LPWKEN3;
pub use LPWKST0 as LPWKST1;
pub use LPWKST0 as LPWKST2;
pub use LPWKST0 as LPWKST3;
#[doc = "LPPWST (rw) register accessor: an alias for `Reg<LPPWST_SPEC>`"]
pub type LPPWST = crate::Reg<lppwst::LPPWST_SPEC>;
#[doc = "Low Power Peripheral Wakeup Status Register."]
pub mod lppwst;
#[doc = "LPPWEN (rw) register accessor: an alias for `Reg<LPPWEN_SPEC>`"]
pub type LPPWEN = crate::Reg<lppwen::LPPWEN_SPEC>;
#[doc = "Low Power Peripheral Wakeup Enable Register."]
pub mod lppwen;
#[doc = "VBTLEPD (rw) register accessor: an alias for `Reg<VBTLEPD_SPEC>`"]
pub type VBTLEPD = crate::Reg<vbtlepd::VBTLEPD_SPEC>;
#[doc = "Low-Power VBTLE Power Down Register."]
pub mod vbtlepd;
#[doc = "GP0 (rw) register accessor: an alias for `Reg<GP0_SPEC>`"]
pub type GP0 = crate::Reg<gp0::GP0_SPEC>;
#[doc = "General Purpose Register 0"]
pub mod gp0;
#[doc = "GP1 (rw) register accessor: an alias for `Reg<GP1_SPEC>`"]
pub type GP1 = crate::Reg<gp1::GP1_SPEC>;
#[doc = "General Purpose Register 1"]
pub mod gp1;
