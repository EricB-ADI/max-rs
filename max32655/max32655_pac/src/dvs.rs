#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctl: CTL,
    #[doc = "0x04 - Status Fields"]
    pub stat: STAT,
    #[doc = "0x08 - Direct control of target voltage"]
    pub direct: DIRECT,
    #[doc = "0x0c - Monitor Delay"]
    pub mon: MON,
    #[doc = "0x10 - Up Delay Register"]
    pub adj_up: ADJ_UP,
    #[doc = "0x14 - Down Delay Register"]
    pub adj_dwn: ADJ_DWN,
    #[doc = "0x18 - Up Delay Register"]
    pub thres_cmp: THRES_CMP,
    #[doc = "0x1c..0x30 - DVS Tap Select Register"]
    pub tap_sel: [TAP_SEL; 5],
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control Register"]
pub mod ctl;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status Fields"]
pub mod stat;
#[doc = "DIRECT (rw) register accessor: an alias for `Reg<DIRECT_SPEC>`"]
pub type DIRECT = crate::Reg<direct::DIRECT_SPEC>;
#[doc = "Direct control of target voltage"]
pub mod direct;
#[doc = "MON (rw) register accessor: an alias for `Reg<MON_SPEC>`"]
pub type MON = crate::Reg<mon::MON_SPEC>;
#[doc = "Monitor Delay"]
pub mod mon;
#[doc = "ADJ_UP (rw) register accessor: an alias for `Reg<ADJ_UP_SPEC>`"]
pub type ADJ_UP = crate::Reg<adj_up::ADJ_UP_SPEC>;
#[doc = "Up Delay Register"]
pub mod adj_up;
#[doc = "ADJ_DWN (rw) register accessor: an alias for `Reg<ADJ_DWN_SPEC>`"]
pub type ADJ_DWN = crate::Reg<adj_dwn::ADJ_DWN_SPEC>;
#[doc = "Down Delay Register"]
pub mod adj_dwn;
#[doc = "THRES_CMP (rw) register accessor: an alias for `Reg<THRES_CMP_SPEC>`"]
pub type THRES_CMP = crate::Reg<thres_cmp::THRES_CMP_SPEC>;
#[doc = "Up Delay Register"]
pub mod thres_cmp;
#[doc = "TAP_SEL (rw) register accessor: an alias for `Reg<TAP_SEL_SPEC>`"]
pub type TAP_SEL = crate::Reg<tap_sel::TAP_SEL_SPEC>;
#[doc = "DVS Tap Select Register"]
pub mod tap_sel;
