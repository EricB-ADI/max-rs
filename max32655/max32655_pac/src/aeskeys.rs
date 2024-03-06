#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AES Key 0."]
    pub key0: KEY0,
    #[doc = "0x04 - AES Key 1."]
    pub key1: KEY1,
    #[doc = "0x08 - AES Key 2."]
    pub key2: KEY2,
    #[doc = "0x0c - AES Key 3."]
    pub key3: KEY3,
    #[doc = "0x10 - AES Key 4."]
    pub key4: KEY4,
    #[doc = "0x14 - AES Key 5."]
    pub key5: KEY5,
    #[doc = "0x18 - AES Key 6."]
    pub key6: KEY6,
    #[doc = "0x1c - AES Key 7."]
    pub key7: KEY7,
}
#[doc = "KEY0 (rw) register accessor: an alias for `Reg<KEY0_SPEC>`"]
pub type KEY0 = crate::Reg<key0::KEY0_SPEC>;
#[doc = "AES Key 0."]
pub mod key0;
#[doc = "KEY1 (rw) register accessor: an alias for `Reg<KEY1_SPEC>`"]
pub type KEY1 = crate::Reg<key1::KEY1_SPEC>;
#[doc = "AES Key 1."]
pub mod key1;
#[doc = "KEY2 (rw) register accessor: an alias for `Reg<KEY2_SPEC>`"]
pub type KEY2 = crate::Reg<key2::KEY2_SPEC>;
#[doc = "AES Key 2."]
pub mod key2;
#[doc = "KEY3 (rw) register accessor: an alias for `Reg<KEY3_SPEC>`"]
pub type KEY3 = crate::Reg<key3::KEY3_SPEC>;
#[doc = "AES Key 3."]
pub mod key3;
#[doc = "KEY4 (rw) register accessor: an alias for `Reg<KEY4_SPEC>`"]
pub type KEY4 = crate::Reg<key4::KEY4_SPEC>;
#[doc = "AES Key 4."]
pub mod key4;
#[doc = "KEY5 (rw) register accessor: an alias for `Reg<KEY5_SPEC>`"]
pub type KEY5 = crate::Reg<key5::KEY5_SPEC>;
#[doc = "AES Key 5."]
pub mod key5;
#[doc = "KEY6 (rw) register accessor: an alias for `Reg<KEY6_SPEC>`"]
pub type KEY6 = crate::Reg<key6::KEY6_SPEC>;
#[doc = "AES Key 6."]
pub mod key6;
#[doc = "KEY7 (rw) register accessor: an alias for `Reg<KEY7_SPEC>`"]
pub type KEY7 = crate::Reg<key7::KEY7_SPEC>;
#[doc = "AES Key 7."]
pub mod key7;
