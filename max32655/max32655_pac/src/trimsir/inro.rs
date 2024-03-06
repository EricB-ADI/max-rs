#[doc = "Register `INRO` reader"]
pub struct R(crate::R<INRO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INRO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INRO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INRO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INRO` writer"]
pub struct W(crate::W<INRO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INRO_SPEC>;
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
impl From<crate::W<INRO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INRO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIM16K` reader - INRO 16KHz Trim."]
pub type TRIM16K_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM16K` writer - INRO 16KHz Trim."]
pub type TRIM16K_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INRO_SPEC, u8, u8, 3, O>;
#[doc = "Field `TRIM30K` reader - INRO 30KHz Trim."]
pub type TRIM30K_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM30K` writer - INRO 30KHz Trim."]
pub type TRIM30K_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INRO_SPEC, u8, u8, 3, O>;
#[doc = "Field `LPCLKSEL` reader - INRO Low Power Mode Clock Select."]
pub type LPCLKSEL_R = crate::FieldReader<u8, LPCLKSEL_A>;
#[doc = "INRO Low Power Mode Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPCLKSEL_A {
    #[doc = "0: `0`"]
    _8KHZ = 0,
    #[doc = "1: `1`"]
    _16KHZ = 1,
    #[doc = "2: `10`"]
    _30KHZ = 2,
}
impl From<LPCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPCLKSEL_A) -> Self {
        variant as _
    }
}
impl LPCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPCLKSEL_A> {
        match self.bits {
            0 => Some(LPCLKSEL_A::_8KHZ),
            1 => Some(LPCLKSEL_A::_16KHZ),
            2 => Some(LPCLKSEL_A::_30KHZ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8KHZ`"]
    #[inline(always)]
    pub fn is_8khz(&self) -> bool {
        *self == LPCLKSEL_A::_8KHZ
    }
    #[doc = "Checks if the value of the field is `_16KHZ`"]
    #[inline(always)]
    pub fn is_16khz(&self) -> bool {
        *self == LPCLKSEL_A::_16KHZ
    }
    #[doc = "Checks if the value of the field is `_30KHZ`"]
    #[inline(always)]
    pub fn is_30khz(&self) -> bool {
        *self == LPCLKSEL_A::_30KHZ
    }
}
#[doc = "Field `LPCLKSEL` writer - INRO Low Power Mode Clock Select."]
pub type LPCLKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INRO_SPEC, u8, LPCLKSEL_A, 2, O>;
impl<'a, const O: u8> LPCLKSEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _8khz(self) -> &'a mut W {
        self.variant(LPCLKSEL_A::_8KHZ)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _16khz(self) -> &'a mut W {
        self.variant(LPCLKSEL_A::_16KHZ)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _30khz(self) -> &'a mut W {
        self.variant(LPCLKSEL_A::_30KHZ)
    }
}
impl R {
    #[doc = "Bits 0:2 - INRO 16KHz Trim."]
    #[inline(always)]
    pub fn trim16k(&self) -> TRIM16K_R {
        TRIM16K_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - INRO 30KHz Trim."]
    #[inline(always)]
    pub fn trim30k(&self) -> TRIM30K_R {
        TRIM30K_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - INRO Low Power Mode Clock Select."]
    #[inline(always)]
    pub fn lpclksel(&self) -> LPCLKSEL_R {
        LPCLKSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - INRO 16KHz Trim."]
    #[inline(always)]
    #[must_use]
    pub fn trim16k(&mut self) -> TRIM16K_W<0> {
        TRIM16K_W::new(self)
    }
    #[doc = "Bits 3:5 - INRO 30KHz Trim."]
    #[inline(always)]
    #[must_use]
    pub fn trim30k(&mut self) -> TRIM30K_W<3> {
        TRIM30K_W::new(self)
    }
    #[doc = "Bits 6:7 - INRO Low Power Mode Clock Select."]
    #[inline(always)]
    #[must_use]
    pub fn lpclksel(&mut self) -> LPCLKSEL_W<6> {
        LPCLKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Trim System Initialization Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inro](index.html) module"]
pub struct INRO_SPEC;
impl crate::RegisterSpec for INRO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inro::R](R) reader structure"]
impl crate::Readable for INRO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inro::W](W) writer structure"]
impl crate::Writable for INRO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INRO to value 0"]
impl crate::Resettable for INRO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
