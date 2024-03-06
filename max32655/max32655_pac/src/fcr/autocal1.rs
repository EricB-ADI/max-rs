#[doc = "Register `AUTOCAL1` reader"]
pub struct R(crate::R<AUTOCAL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTOCAL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTOCAL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTOCAL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUTOCAL1` writer"]
pub struct W(crate::W<AUTOCAL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUTOCAL1_SPEC>;
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
impl From<crate::W<AUTOCAL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUTOCAL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INITTRM` reader - Initial Trim Setting."]
pub type INITTRM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INITTRM` writer - Initial Trim Setting."]
pub type INITTRM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUTOCAL1_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - Initial Trim Setting."]
    #[inline(always)]
    pub fn inittrm(&self) -> INITTRM_R {
        INITTRM_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Initial Trim Setting."]
    #[inline(always)]
    #[must_use]
    pub fn inittrm(&mut self) -> INITTRM_W<0> {
        INITTRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Automatic Calibration 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autocal1](index.html) module"]
pub struct AUTOCAL1_SPEC;
impl crate::RegisterSpec for AUTOCAL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [autocal1::R](R) reader structure"]
impl crate::Readable for AUTOCAL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [autocal1::W](W) writer structure"]
impl crate::Writable for AUTOCAL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUTOCAL1 to value 0"]
impl crate::Resettable for AUTOCAL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
