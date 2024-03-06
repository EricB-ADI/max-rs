#[doc = "Register `KEY5` reader"]
pub struct R(crate::R<KEY5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY5` writer"]
pub struct W(crate::W<KEY5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY5_SPEC>;
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
impl From<crate::W<KEY5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY5_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Key 5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key5](index.html) module"]
pub struct KEY5_SPEC;
impl crate::RegisterSpec for KEY5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key5::R](R) reader structure"]
impl crate::Readable for KEY5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key5::W](W) writer structure"]
impl crate::Writable for KEY5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY5 to value 0"]
impl crate::Resettable for KEY5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
