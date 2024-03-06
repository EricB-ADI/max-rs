#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Control."]
    pub sysctrl: SYSCTRL,
    #[doc = "0x04 - Reset."]
    pub rst0: RST0,
    #[doc = "0x08 - Clock Control."]
    pub clkctrl: CLKCTRL,
    #[doc = "0x0c - Power Management."]
    pub pm: PM,
    _reserved4: [u8; 0x08],
    #[doc = "0x18 - Peripheral Clock Divider."]
    pub pclkdiv: PCLKDIV,
    _reserved5: [u8; 0x08],
    #[doc = "0x24 - Peripheral Clock Disable."]
    pub pclkdis0: PCLKDIS0,
    #[doc = "0x28 - Memory Clock Control Register."]
    pub memctrl: MEMCTRL,
    #[doc = "0x2c - Memory Zeroize Control."]
    pub memz: MEMZ,
    _reserved8: [u8; 0x10],
    #[doc = "0x40 - System Status Register."]
    pub sysst: SYSST,
    #[doc = "0x44 - Reset 1."]
    pub rst1: RST1,
    #[doc = "0x48 - Peripheral Clock Disable."]
    pub pclkdis1: PCLKDIS1,
    #[doc = "0x4c - Event Enable Register."]
    pub eventen: EVENTEN,
    #[doc = "0x50 - Revision Register."]
    pub revision: REVISION,
    #[doc = "0x54 - System Status Interrupt Enable Register."]
    pub sysie: SYSIE,
    _reserved14: [u8; 0x0c],
    #[doc = "0x64 - ECC Error Register"]
    pub eccerr: ECCERR,
    #[doc = "0x68 - ECC Not Double Error Detect Register"]
    pub eccced: ECCCED,
    #[doc = "0x6c - ECC IRQ Enable Register"]
    pub eccie: ECCIE,
    #[doc = "0x70 - ECC Error Address Register"]
    pub eccaddr: ECCADDR,
    #[doc = "0x74 - BTLE LDO Control Register"]
    pub btleldoctrl: BTLELDOCTRL,
    #[doc = "0x78 - BTLE LDO Delay Register"]
    pub btleldodly: BTLELDODLY,
    _reserved20: [u8; 0x04],
    #[doc = "0x80 - General Purpose Register."]
    pub gpr: GPR,
}
#[doc = "SYSCTRL (rw) register accessor: an alias for `Reg<SYSCTRL_SPEC>`"]
pub type SYSCTRL = crate::Reg<sysctrl::SYSCTRL_SPEC>;
#[doc = "System Control."]
pub mod sysctrl;
#[doc = "RST0 (rw) register accessor: an alias for `Reg<RST0_SPEC>`"]
pub type RST0 = crate::Reg<rst0::RST0_SPEC>;
#[doc = "Reset."]
pub mod rst0;
#[doc = "CLKCTRL (rw) register accessor: an alias for `Reg<CLKCTRL_SPEC>`"]
pub type CLKCTRL = crate::Reg<clkctrl::CLKCTRL_SPEC>;
#[doc = "Clock Control."]
pub mod clkctrl;
#[doc = "PM (rw) register accessor: an alias for `Reg<PM_SPEC>`"]
pub type PM = crate::Reg<pm::PM_SPEC>;
#[doc = "Power Management."]
pub mod pm;
#[doc = "PCLKDIV (rw) register accessor: an alias for `Reg<PCLKDIV_SPEC>`"]
pub type PCLKDIV = crate::Reg<pclkdiv::PCLKDIV_SPEC>;
#[doc = "Peripheral Clock Divider."]
pub mod pclkdiv;
#[doc = "PCLKDIS0 (rw) register accessor: an alias for `Reg<PCLKDIS0_SPEC>`"]
pub type PCLKDIS0 = crate::Reg<pclkdis0::PCLKDIS0_SPEC>;
#[doc = "Peripheral Clock Disable."]
pub mod pclkdis0;
#[doc = "MEMCTRL (rw) register accessor: an alias for `Reg<MEMCTRL_SPEC>`"]
pub type MEMCTRL = crate::Reg<memctrl::MEMCTRL_SPEC>;
#[doc = "Memory Clock Control Register."]
pub mod memctrl;
#[doc = "MEMZ (rw) register accessor: an alias for `Reg<MEMZ_SPEC>`"]
pub type MEMZ = crate::Reg<memz::MEMZ_SPEC>;
#[doc = "Memory Zeroize Control."]
pub mod memz;
#[doc = "SYSST (rw) register accessor: an alias for `Reg<SYSST_SPEC>`"]
pub type SYSST = crate::Reg<sysst::SYSST_SPEC>;
#[doc = "System Status Register."]
pub mod sysst;
#[doc = "RST1 (rw) register accessor: an alias for `Reg<RST1_SPEC>`"]
pub type RST1 = crate::Reg<rst1::RST1_SPEC>;
#[doc = "Reset 1."]
pub mod rst1;
#[doc = "PCLKDIS1 (rw) register accessor: an alias for `Reg<PCLKDIS1_SPEC>`"]
pub type PCLKDIS1 = crate::Reg<pclkdis1::PCLKDIS1_SPEC>;
#[doc = "Peripheral Clock Disable."]
pub mod pclkdis1;
#[doc = "EVENTEN (rw) register accessor: an alias for `Reg<EVENTEN_SPEC>`"]
pub type EVENTEN = crate::Reg<eventen::EVENTEN_SPEC>;
#[doc = "Event Enable Register."]
pub mod eventen;
#[doc = "REVISION (r) register accessor: an alias for `Reg<REVISION_SPEC>`"]
pub type REVISION = crate::Reg<revision::REVISION_SPEC>;
#[doc = "Revision Register."]
pub mod revision;
#[doc = "SYSIE (rw) register accessor: an alias for `Reg<SYSIE_SPEC>`"]
pub type SYSIE = crate::Reg<sysie::SYSIE_SPEC>;
#[doc = "System Status Interrupt Enable Register."]
pub mod sysie;
#[doc = "ECCERR (rw) register accessor: an alias for `Reg<ECCERR_SPEC>`"]
pub type ECCERR = crate::Reg<eccerr::ECCERR_SPEC>;
#[doc = "ECC Error Register"]
pub mod eccerr;
#[doc = "ECCCED (rw) register accessor: an alias for `Reg<ECCCED_SPEC>`"]
pub type ECCCED = crate::Reg<eccced::ECCCED_SPEC>;
#[doc = "ECC Not Double Error Detect Register"]
pub mod eccced;
#[doc = "ECCIE (rw) register accessor: an alias for `Reg<ECCIE_SPEC>`"]
pub type ECCIE = crate::Reg<eccie::ECCIE_SPEC>;
#[doc = "ECC IRQ Enable Register"]
pub mod eccie;
#[doc = "ECCADDR (rw) register accessor: an alias for `Reg<ECCADDR_SPEC>`"]
pub type ECCADDR = crate::Reg<eccaddr::ECCADDR_SPEC>;
#[doc = "ECC Error Address Register"]
pub mod eccaddr;
#[doc = "BTLELDOCTRL (rw) register accessor: an alias for `Reg<BTLELDOCTRL_SPEC>`"]
pub type BTLELDOCTRL = crate::Reg<btleldoctrl::BTLELDOCTRL_SPEC>;
#[doc = "BTLE LDO Control Register"]
pub mod btleldoctrl;
#[doc = "BTLELDODLY (rw) register accessor: an alias for `Reg<BTLELDODLY_SPEC>`"]
pub type BTLELDODLY = crate::Reg<btleldodly::BTLELDODLY_SPEC>;
#[doc = "BTLE LDO Delay Register"]
pub mod btleldodly;
#[doc = "GPR (rw) register accessor: an alias for `Reg<GPR_SPEC>`"]
pub type GPR = crate::Reg<gpr::GPR_SPEC>;
#[doc = "General Purpose Register."]
pub mod gpr;
