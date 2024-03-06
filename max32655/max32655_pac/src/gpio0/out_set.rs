#[doc = "Register `OUT_SET` writer"]
pub struct W(crate::W<OUT_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_SET_SPEC>;
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
impl From<crate::W<OUT_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_OUT_SET_AW {
    #[doc = "0: No Effect."]
    NO = 0,
    #[doc = "1: Set GPIO_OUT bit in this position to '1'"]
    SET = 1,
}
impl From<GPIO_OUT_SET_AW> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_OUT_SET_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO_OUT_SET` writer - Mask of all of the pins on the port."]
pub type GPIO_OUT_SET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUT_SET_SPEC, u32, GPIO_OUT_SET_AW, 32, O>;
impl<'a, const O: u8> GPIO_OUT_SET_W<'a, O> {
    #[doc = "No Effect."]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(GPIO_OUT_SET_AW::NO)
    }
    #[doc = "Set GPIO_OUT bit in this position to '1'"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(GPIO_OUT_SET_AW::SET)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_out_set(&mut self) -> GPIO_OUT_SET_W<0> {
        GPIO_OUT_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Output Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT to 1, without affecting other bits in that register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_set](index.html) module"]
pub struct OUT_SET_SPEC;
impl crate::RegisterSpec for OUT_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [out_set::W](W) writer structure"]
impl crate::Writable for OUT_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_SET to value 0"]
impl crate::Resettable for OUT_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
