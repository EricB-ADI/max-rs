#[doc = "Register `AUTOCAL2` reader"]
pub struct R(crate::R<AUTOCAL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTOCAL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTOCAL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTOCAL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUTOCAL2` writer"]
pub struct W(crate::W<AUTOCAL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUTOCAL2_SPEC>;
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
impl From<crate::W<AUTOCAL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUTOCAL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DONECNT` reader - Auto-callibration Done Counter Setting."]
pub type DONECNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DONECNT` writer - Auto-callibration Done Counter Setting."]
pub type DONECNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUTOCAL2_SPEC, u8, u8, 8, O>;
#[doc = "Field `ACDIV` reader - Auto-callibration Div Setting."]
pub type ACDIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ACDIV` writer - Auto-callibration Div Setting."]
pub type ACDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUTOCAL2_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:7 - Auto-callibration Done Counter Setting."]
    #[inline(always)]
    pub fn donecnt(&self) -> DONECNT_R {
        DONECNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:20 - Auto-callibration Div Setting."]
    #[inline(always)]
    pub fn acdiv(&self) -> ACDIV_R {
        ACDIV_R::new(((self.bits >> 8) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Auto-callibration Done Counter Setting."]
    #[inline(always)]
    #[must_use]
    pub fn donecnt(&mut self) -> DONECNT_W<0> {
        DONECNT_W::new(self)
    }
    #[doc = "Bits 8:20 - Auto-callibration Div Setting."]
    #[inline(always)]
    #[must_use]
    pub fn acdiv(&mut self) -> ACDIV_W<8> {
        ACDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Automatic Calibration 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autocal2](index.html) module"]
pub struct AUTOCAL2_SPEC;
impl crate::RegisterSpec for AUTOCAL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [autocal2::R](R) reader structure"]
impl crate::Readable for AUTOCAL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [autocal2::W](W) writer structure"]
impl crate::Writable for AUTOCAL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUTOCAL2 to value 0"]
impl crate::Resettable for AUTOCAL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
