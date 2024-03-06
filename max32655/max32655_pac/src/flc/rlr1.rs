#[doc = "Register `RLR1` reader"]
pub struct R(crate::R<RLR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RLR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RLR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RLR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RLR1` writer"]
pub struct W(crate::W<RLR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RLR1_SPEC>;
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
impl From<crate::W<RLR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RLR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RLR1` reader - Access control."]
pub type RLR1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RLR1` writer - Access control."]
pub type RLR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RLR1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn rlr1(&self) -> RLR1_R {
        RLR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    #[must_use]
    pub fn rlr1(&mut self) -> RLR1_W<0> {
        RLR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RLR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr1](index.html) module"]
pub struct RLR1_SPEC;
impl crate::RegisterSpec for RLR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rlr1::R](R) reader structure"]
impl crate::Readable for RLR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rlr1::W](W) writer structure"]
impl crate::Writable for RLR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RLR1 to value 0"]
impl crate::Resettable for RLR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
