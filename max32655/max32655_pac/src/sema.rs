#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - Read to test and set, returns prior value. Write 0 to clear semaphore."]
    pub semaphores: [SEMAPHORES; 8],
    _reserved1: [u8; 0x20],
    #[doc = "0x40 - Semaphore IRQ0 register."]
    pub irq0: IRQ0,
    #[doc = "0x44 - Semaphore Mailbox 0 register."]
    pub mail0: MAIL0,
    #[doc = "0x48 - Semaphore IRQ1 register."]
    pub irq1: IRQ1,
    #[doc = "0x4c - Semaphore Mailbox 1 register."]
    pub mail1: MAIL1,
    _reserved5: [u8; 0xb0],
    #[doc = "0x100 - Semaphore status bits. 0 indicates the semaphore is free, 1 indicates taken."]
    pub status: STATUS,
}
#[doc = "SEMAPHORES (rw) register accessor: an alias for `Reg<SEMAPHORES_SPEC>`"]
pub type SEMAPHORES = crate::Reg<semaphores::SEMAPHORES_SPEC>;
#[doc = "Read to test and set, returns prior value. Write 0 to clear semaphore."]
pub mod semaphores;
#[doc = "irq0 (rw) register accessor: an alias for `Reg<IRQ0_SPEC>`"]
pub type IRQ0 = crate::Reg<irq0::IRQ0_SPEC>;
#[doc = "Semaphore IRQ0 register."]
pub mod irq0;
#[doc = "mail0 (rw) register accessor: an alias for `Reg<MAIL0_SPEC>`"]
pub type MAIL0 = crate::Reg<mail0::MAIL0_SPEC>;
#[doc = "Semaphore Mailbox 0 register."]
pub mod mail0;
#[doc = "irq1 (rw) register accessor: an alias for `Reg<IRQ1_SPEC>`"]
pub type IRQ1 = crate::Reg<irq1::IRQ1_SPEC>;
#[doc = "Semaphore IRQ1 register."]
pub mod irq1;
#[doc = "mail1 (rw) register accessor: an alias for `Reg<MAIL1_SPEC>`"]
pub type MAIL1 = crate::Reg<mail1::MAIL1_SPEC>;
#[doc = "Semaphore Mailbox 1 register."]
pub mod mail1;
#[doc = "status (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Semaphore status bits. 0 indicates the semaphore is free, 1 indicates taken."]
pub mod status;
