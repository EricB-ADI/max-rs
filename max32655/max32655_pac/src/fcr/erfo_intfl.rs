#[doc = "Register `ERFO_INTFL` reader"]
pub struct R(crate::R<ERFO_INTFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERFO_INTFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERFO_INTFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERFO_INTFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERFO_INTFL` writer"]
pub struct W(crate::W<ERFO_INTFL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERFO_INTFL_SPEC>;
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
impl From<crate::W<ERFO_INTFL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERFO_INTFL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rdy` reader - Ready interrupt flag."]
pub type RDY_R = crate::BitReader<bool>;
#[doc = "Field `rdy` writer - Ready interrupt flag."]
pub type RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERFO_INTFL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Ready interrupt flag."]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ready interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn rdy(&mut self) -> RDY_W<0> {
        RDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ERFO Ready Interrupt Flag register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erfo_intfl](index.html) module"]
pub struct ERFO_INTFL_SPEC;
impl crate::RegisterSpec for ERFO_INTFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [erfo_intfl::R](R) reader structure"]
impl crate::Readable for ERFO_INTFL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [erfo_intfl::W](W) writer structure"]
impl crate::Writable for ERFO_INTFL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERFO_INTFL to value 0"]
impl crate::Resettable for ERFO_INTFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
