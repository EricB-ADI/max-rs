#[doc = "Register `RLR0` reader"]
pub struct R(crate::R<RLR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RLR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RLR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RLR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RLR0` writer"]
pub struct W(crate::W<RLR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RLR0_SPEC>;
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
impl From<crate::W<RLR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RLR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RLR0` reader - Access control."]
pub type RLR0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RLR0` writer - Access control."]
pub type RLR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RLR0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn rlr0(&self) -> RLR0_R {
        RLR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    #[must_use]
    pub fn rlr0(&mut self) -> RLR0_W<0> {
        RLR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RLR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr0](index.html) module"]
pub struct RLR0_SPEC;
impl crate::RegisterSpec for RLR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rlr0::R](R) reader structure"]
impl crate::Readable for RLR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rlr0::W](W) writer structure"]
impl crate::Writable for RLR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RLR0 to value 0"]
impl crate::Resettable for RLR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
