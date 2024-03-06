#[doc = "Register `VREGO_C` reader"]
pub struct R(crate::R<VREGO_C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREGO_C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREGO_C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREGO_C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREGO_C` writer"]
pub struct W(crate::W<VREGO_C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREGO_C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<VREGO_C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREGO_C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VSETC` reader - Regulator Output Voltage Setting"]
pub type VSETC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VSETC` writer - Regulator Output Voltage Setting"]
pub type VSETC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREGO_C_SPEC, u8, u8, 7, O>;
#[doc = "Field `RANGEC` reader - Regulator Output Range Set"]
pub type RANGEC_R = crate::BitReader<RANGEC_A>;
#[doc = "Regulator Output Range Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RANGEC_A {
    #[doc = "0: Low output voltage range"]
    LOW = 0,
    #[doc = "1: High output voltage range"]
    HIGH = 1,
}
impl From<RANGEC_A> for bool {
    #[inline(always)]
    fn from(variant: RANGEC_A) -> Self {
        variant as u8 != 0
    }
}
impl RANGEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RANGEC_A {
        match self.bits {
            false => RANGEC_A::LOW,
            true => RANGEC_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == RANGEC_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == RANGEC_A::HIGH
    }
}
#[doc = "Field `RANGEC` writer - Regulator Output Range Set"]
pub type RANGEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, VREGO_C_SPEC, RANGEC_A, O>;
impl<'a, const O: u8> RANGEC_W<'a, O> {
    #[doc = "Low output voltage range"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(RANGEC_A::LOW)
    }
    #[doc = "High output voltage range"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(RANGEC_A::HIGH)
    }
}
impl R {
    #[doc = "Bits 0:6 - Regulator Output Voltage Setting"]
    #[inline(always)]
    pub fn vsetc(&self) -> VSETC_R {
        VSETC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Regulator Output Range Set"]
    #[inline(always)]
    pub fn rangec(&self) -> RANGEC_R {
        RANGEC_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Regulator Output Voltage Setting"]
    #[inline(always)]
    #[must_use]
    pub fn vsetc(&mut self) -> VSETC_W<0> {
        VSETC_W::new(self)
    }
    #[doc = "Bit 7 - Regulator Output Range Set"]
    #[inline(always)]
    #[must_use]
    pub fn rangec(&mut self) -> RANGEC_W<7> {
        RANGEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buck Voltage Regulator C Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vrego_c](index.html) module"]
pub struct VREGO_C_SPEC;
impl crate::RegisterSpec for VREGO_C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vrego_c::R](R) reader structure"]
impl crate::Readable for VREGO_C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vrego_c::W](W) writer structure"]
impl crate::Writable for VREGO_C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREGO_C to value 0"]
impl crate::Resettable for VREGO_C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
