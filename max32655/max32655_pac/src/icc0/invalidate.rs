#[doc = "Register `INVALIDATE` reader"]
pub struct R(crate::R<INVALIDATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INVALIDATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INVALIDATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INVALIDATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INVALIDATE` writer"]
pub struct W(crate::W<INVALIDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INVALIDATE_SPEC>;
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
impl From<crate::W<INVALIDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INVALIDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INVALID` reader - Invalidate."]
pub type INVALID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INVALID` writer - Invalidate."]
pub type INVALID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INVALIDATE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Invalidate."]
    #[inline(always)]
    pub fn invalid(&self) -> INVALID_R {
        INVALID_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Invalidate."]
    #[inline(always)]
    #[must_use]
    pub fn invalid(&mut self) -> INVALID_W<0> {
        INVALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Invalidate All Registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [invalidate](index.html) module"]
pub struct INVALIDATE_SPEC;
impl crate::RegisterSpec for INVALIDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [invalidate::R](R) reader structure"]
impl crate::Readable for INVALIDATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [invalidate::W](W) writer structure"]
impl crate::Writable for INVALIDATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INVALIDATE to value 0"]
impl crate::Resettable for INVALIDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
