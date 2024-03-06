#[doc = "Register `CMP` reader"]
pub struct R(crate::R<CMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMP` writer"]
pub struct W(crate::W<CMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMP_SPEC>;
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
impl From<crate::W<CMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPARE` reader - Timer Compare Value."]
pub type COMPARE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COMPARE` writer - Timer Compare Value."]
pub type COMPARE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMP_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Timer Compare Value."]
    #[inline(always)]
    pub fn compare(&self) -> COMPARE_R {
        COMPARE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer Compare Value."]
    #[inline(always)]
    #[must_use]
    pub fn compare(&mut self) -> COMPARE_W<0> {
        COMPARE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare. This register stores the compare value, which is used to set the maximum count value to initiate a reload of the timer to 0x0001.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp](index.html) module"]
pub struct CMP_SPEC;
impl crate::RegisterSpec for CMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmp::R](R) reader structure"]
impl crate::Readable for CMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmp::W](W) writer structure"]
impl crate::Writable for CMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMP to value 0xffff"]
impl crate::Resettable for CMP_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
