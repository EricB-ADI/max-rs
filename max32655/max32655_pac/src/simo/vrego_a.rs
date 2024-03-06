#[doc = "Register `VREGO_A` reader"]
pub struct R(crate::R<VREGO_A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREGO_A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREGO_A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREGO_A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREGO_A` writer"]
pub struct W(crate::W<VREGO_A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREGO_A_SPEC>;
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
impl From<crate::W<VREGO_A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREGO_A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VSETA` reader - Regulator Output Voltage Setting"]
pub type VSETA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VSETA` writer - Regulator Output Voltage Setting"]
pub type VSETA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREGO_A_SPEC, u8, u8, 7, O>;
#[doc = "Field `RANGEA` reader - Regulator Output Range Set"]
pub type RANGEA_R = crate::BitReader<RANGEA_A>;
#[doc = "Regulator Output Range Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RANGEA_A {
    #[doc = "0: Low output voltage range"]
    LOW = 0,
    #[doc = "1: High output voltage range"]
    HIGH = 1,
}
impl From<RANGEA_A> for bool {
    #[inline(always)]
    fn from(variant: RANGEA_A) -> Self {
        variant as u8 != 0
    }
}
impl RANGEA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RANGEA_A {
        match self.bits {
            false => RANGEA_A::LOW,
            true => RANGEA_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == RANGEA_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == RANGEA_A::HIGH
    }
}
#[doc = "Field `RANGEA` writer - Regulator Output Range Set"]
pub type RANGEA_W<'a, const O: u8> = crate::BitWriter<'a, u32, VREGO_A_SPEC, RANGEA_A, O>;
impl<'a, const O: u8> RANGEA_W<'a, O> {
    #[doc = "Low output voltage range"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(RANGEA_A::LOW)
    }
    #[doc = "High output voltage range"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(RANGEA_A::HIGH)
    }
}
impl R {
    #[doc = "Bits 0:6 - Regulator Output Voltage Setting"]
    #[inline(always)]
    pub fn vseta(&self) -> VSETA_R {
        VSETA_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Regulator Output Range Set"]
    #[inline(always)]
    pub fn rangea(&self) -> RANGEA_R {
        RANGEA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Regulator Output Voltage Setting"]
    #[inline(always)]
    #[must_use]
    pub fn vseta(&mut self) -> VSETA_W<0> {
        VSETA_W::new(self)
    }
    #[doc = "Bit 7 - Regulator Output Range Set"]
    #[inline(always)]
    #[must_use]
    pub fn rangea(&mut self) -> RANGEA_W<7> {
        RANGEA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buck Voltage Regulator A Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vrego_a](index.html) module"]
pub struct VREGO_A_SPEC;
impl crate::RegisterSpec for VREGO_A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vrego_a::R](R) reader structure"]
impl crate::Readable for VREGO_A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vrego_a::W](W) writer structure"]
impl crate::Writable for VREGO_A_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREGO_A to value 0"]
impl crate::Resettable for VREGO_A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
