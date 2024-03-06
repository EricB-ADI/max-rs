#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - 1-Wire Master Configuration."]
    pub cfg: CFG,
    #[doc = "0x04 - 1-Wire Master Clock Divisor."]
    pub clk_div_1us: CLK_DIV_1US,
    #[doc = "0x08 - 1-Wire Master Control/Status."]
    pub ctrl_stat: CTRL_STAT,
    #[doc = "0x0c - 1-Wire Master Data Buffer."]
    pub data: DATA,
    #[doc = "0x10 - 1-Wire Master Interrupt Flags."]
    pub intfl: INTFL,
    #[doc = "0x14 - 1-Wire Master Interrupt Enables."]
    pub inten: INTEN,
}
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "1-Wire Master Configuration."]
pub mod cfg;
#[doc = "CLK_DIV_1US (rw) register accessor: an alias for `Reg<CLK_DIV_1US_SPEC>`"]
pub type CLK_DIV_1US = crate::Reg<clk_div_1us::CLK_DIV_1US_SPEC>;
#[doc = "1-Wire Master Clock Divisor."]
pub mod clk_div_1us;
#[doc = "CTRL_STAT (rw) register accessor: an alias for `Reg<CTRL_STAT_SPEC>`"]
pub type CTRL_STAT = crate::Reg<ctrl_stat::CTRL_STAT_SPEC>;
#[doc = "1-Wire Master Control/Status."]
pub mod ctrl_stat;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "1-Wire Master Data Buffer."]
pub mod data;
#[doc = "INTFL (rw) register accessor: an alias for `Reg<INTFL_SPEC>`"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "1-Wire Master Interrupt Flags."]
pub mod intfl;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "1-Wire Master Interrupt Enables."]
pub mod inten;
