#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Initialization Status Register."]
    pub sistat: SISTAT,
    #[doc = "0x04 - Read-only field set by the SIB block if a CRC error occurs during the read of the OTP memory. Contains the failing address in OTP memory (when CRCERR equals 1)."]
    pub addr: ADDR,
    _reserved2: [u8; 0x40],
    #[doc = "0x48 - BTLE LDO Trim register."]
    pub btle_ldo_trim: BTLE_LDO_TRIM,
    _reserved3: [u8; 0xb4],
    #[doc = "0x100 - funcstat register."]
    pub fstat: FSTAT,
    #[doc = "0x104 - Security function status register."]
    pub sfstat: SFSTAT,
}
#[doc = "SISTAT (r) register accessor: an alias for `Reg<SISTAT_SPEC>`"]
pub type SISTAT = crate::Reg<sistat::SISTAT_SPEC>;
#[doc = "System Initialization Status Register."]
pub mod sistat;
#[doc = "ADDR (r) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Read-only field set by the SIB block if a CRC error occurs during the read of the OTP memory. Contains the failing address in OTP memory (when CRCERR equals 1)."]
pub mod addr;
#[doc = "BTLE_LDO_TRIM (rw) register accessor: an alias for `Reg<BTLE_LDO_TRIM_SPEC>`"]
pub type BTLE_LDO_TRIM = crate::Reg<btle_ldo_trim::BTLE_LDO_TRIM_SPEC>;
#[doc = "BTLE LDO Trim register."]
pub mod btle_ldo_trim;
#[doc = "FSTAT (r) register accessor: an alias for `Reg<FSTAT_SPEC>`"]
pub type FSTAT = crate::Reg<fstat::FSTAT_SPEC>;
#[doc = "funcstat register."]
pub mod fstat;
#[doc = "SFSTAT (r) register accessor: an alias for `Reg<SFSTAT_SPEC>`"]
pub type SFSTAT = crate::Reg<sfstat::SFSTAT_SPEC>;
#[doc = "Security function status register."]
pub mod sfstat;
