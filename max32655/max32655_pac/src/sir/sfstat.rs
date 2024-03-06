#[doc = "Register `SFSTAT` reader"]
pub struct R(crate::R<SFSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SECBOOT` reader - Security Boot."]
pub type SECBOOT_R = crate::BitReader<bool>;
#[doc = "Field `TRNG` reader - TRNG Function."]
pub type TRNG_R = crate::BitReader<TRNG_A>;
#[doc = "TRNG Function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRNG_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<TRNG_A> for bool {
    #[inline(always)]
    fn from(variant: TRNG_A) -> Self {
        variant as u8 != 0
    }
}
impl TRNG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRNG_A {
        match self.bits {
            false => TRNG_A::NO,
            true => TRNG_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == TRNG_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == TRNG_A::YES
    }
}
#[doc = "Field `AES` reader - AES Block."]
pub type AES_R = crate::BitReader<AES_A>;
#[doc = "AES Block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AES_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<AES_A> for bool {
    #[inline(always)]
    fn from(variant: AES_A) -> Self {
        variant as u8 != 0
    }
}
impl AES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AES_A {
        match self.bits {
            false => AES_A::NO,
            true => AES_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == AES_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == AES_A::YES
    }
}
impl R {
    #[doc = "Bit 0 - Security Boot."]
    #[inline(always)]
    pub fn secboot(&self) -> SECBOOT_R {
        SECBOOT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - TRNG Function."]
    #[inline(always)]
    pub fn trng(&self) -> TRNG_R {
        TRNG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AES Block."]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Security function status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfstat](index.html) module"]
pub struct SFSTAT_SPEC;
impl crate::RegisterSpec for SFSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfstat::R](R) reader structure"]
impl crate::Readable for SFSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SFSTAT to value 0"]
impl crate::Resettable for SFSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
