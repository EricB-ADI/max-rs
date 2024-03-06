#[doc = "Register `EN2_CLR` reader"]
pub struct R(crate::R<EN2_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EN2_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EN2_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EN2_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EN2_CLR` writer"]
pub struct W(crate::W<EN2_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EN2_CLR_SPEC>;
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
impl From<crate::W<EN2_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EN2_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALL` reader - Mask of all of the pins on the port."]
pub type ALL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ALL` writer - Mask of all of the pins on the port."]
pub type ALL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EN2_CLR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn all(&self) -> ALL_R {
        ALL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn all(&mut self) -> ALL_W<0> {
        ALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Wake Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN2 to 0, without affecting other bits in that register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en2_clr](index.html) module"]
pub struct EN2_CLR_SPEC;
impl crate::RegisterSpec for EN2_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [en2_clr::R](R) reader structure"]
impl crate::Readable for EN2_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [en2_clr::W](W) writer structure"]
impl crate::Writable for EN2_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EN2_CLR to value 0"]
impl crate::Resettable for EN2_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
