#[doc = "Register `SIMO` reader"]
pub struct R(crate::R<SIMO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIMO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIMO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIMO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLKDIV` reader - SIMO Clock Divide."]
pub type CLKDIV_R = crate::FieldReader<u8, CLKDIV_A>;
#[doc = "SIMO Clock Divide.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKDIV_A {
    #[doc = "0: `0`"]
    DIV1 = 0,
    #[doc = "1: `1`"]
    DIV16 = 1,
    #[doc = "3: `11`"]
    DIV32 = 3,
    #[doc = "5: `101`"]
    DIV64 = 5,
    #[doc = "7: `111`"]
    DIV128 = 7,
}
impl From<CLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKDIV_A) -> Self {
        variant as _
    }
}
impl CLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKDIV_A> {
        match self.bits {
            0 => Some(CLKDIV_A::DIV1),
            1 => Some(CLKDIV_A::DIV16),
            3 => Some(CLKDIV_A::DIV32),
            5 => Some(CLKDIV_A::DIV64),
            7 => Some(CLKDIV_A::DIV128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CLKDIV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CLKDIV_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == CLKDIV_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == CLKDIV_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == CLKDIV_A::DIV128
    }
}
impl R {
    #[doc = "Bits 0:2 - SIMO Clock Divide."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 7) as u8)
    }
}
#[doc = "SIMO Trim System Initialization Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [simo](index.html) module"]
pub struct SIMO_SPEC;
impl crate::RegisterSpec for SIMO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [simo::R](R) reader structure"]
impl crate::Readable for SIMO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIMO to value 0"]
impl crate::Resettable for SIMO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
