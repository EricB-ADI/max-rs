#[doc = "Register `HYSEN` reader"]
pub struct R(crate::R<HYSEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HYSEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HYSEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HYSEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HYSEN` writer"]
pub struct W(crate::W<HYSEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HYSEN_SPEC>;
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
impl From<crate::W<HYSEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HYSEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_HYSEN` reader - Mask of all of the pins on the port."]
pub type GPIO_HYSEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GPIO_HYSEN` writer - Mask of all of the pins on the port."]
pub type GPIO_HYSEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HYSEN_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_hysen(&self) -> GPIO_HYSEN_R {
        GPIO_HYSEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_hysen(&mut self) -> GPIO_HYSEN_W<0> {
        GPIO_HYSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Input Hysteresis Enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hysen](index.html) module"]
pub struct HYSEN_SPEC;
impl crate::RegisterSpec for HYSEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hysen::R](R) reader structure"]
impl crate::Readable for HYSEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hysen::W](W) writer structure"]
impl crate::Writable for HYSEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HYSEN to value 0"]
impl crate::Resettable for HYSEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
