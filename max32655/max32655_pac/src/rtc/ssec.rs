#[doc = "Register `SSEC` reader"]
pub struct R(crate::R<SSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSEC` writer"]
pub struct W(crate::W<SSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSEC_SPEC>;
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
impl From<crate::W<SSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSEC` reader - Sub-Seconds Counter (12-bit)."]
pub type SSEC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SSEC` writer - Sub-Seconds Counter (12-bit)."]
pub type SSEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSEC_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Sub-Seconds Counter (12-bit)."]
    #[inline(always)]
    pub fn ssec(&self) -> SSEC_R {
        SSEC_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Sub-Seconds Counter (12-bit)."]
    #[inline(always)]
    #[must_use]
    pub fn ssec(&mut self) -> SSEC_W<0> {
        SSEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Sub-second Counter. This counter increments at 256Hz. RTC_SEC is incremented when this register rolls over from 0xFF to 0x00.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssec](index.html) module"]
pub struct SSEC_SPEC;
impl crate::RegisterSpec for SSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssec::R](R) reader structure"]
impl crate::Readable for SSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssec::W](W) writer structure"]
impl crate::Writable for SSEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSEC to value 0"]
impl crate::Resettable for SSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
