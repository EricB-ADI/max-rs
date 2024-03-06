#[doc = "Register `OUT` reader"]
pub struct R(crate::R<OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT` writer"]
pub struct W(crate::W<OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_SPEC>;
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
impl From<crate::W<OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_OUT` reader - Mask of all of the pins on the port."]
pub type GPIO_OUT_R = crate::FieldReader<u32, GPIO_OUT_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_OUT_A {
    #[doc = "0: Drive Logic 0 (low) on GPIO output."]
    LOW = 0,
    #[doc = "1: Drive logic 1 (high) on GPIO output."]
    HIGH = 1,
}
impl From<GPIO_OUT_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_OUT_A) -> Self {
        variant as _
    }
}
impl GPIO_OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_OUT_A> {
        match self.bits {
            0 => Some(GPIO_OUT_A::LOW),
            1 => Some(GPIO_OUT_A::HIGH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == GPIO_OUT_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == GPIO_OUT_A::HIGH
    }
}
#[doc = "Field `GPIO_OUT` writer - Mask of all of the pins on the port."]
pub type GPIO_OUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUT_SPEC, u32, GPIO_OUT_A, 32, O>;
impl<'a, const O: u8> GPIO_OUT_W<'a, O> {
    #[doc = "Drive Logic 0 (low) on GPIO output."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(GPIO_OUT_A::LOW)
    }
    #[doc = "Drive logic 1 (high) on GPIO output."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(GPIO_OUT_A::HIGH)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_out(&self) -> GPIO_OUT_R {
        GPIO_OUT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_out(&mut self) -> GPIO_OUT_W<0> {
        GPIO_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Output Register. Each bit controls the GPIO_OUT setting for one pin in the associated port. This register can be written either directly, or by using the GPIO_OUT_SET and GPIO_OUT_CLR registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out](index.html) module"]
pub struct OUT_SPEC;
impl crate::RegisterSpec for OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out::R](R) reader structure"]
impl crate::Readable for OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out::W](W) writer structure"]
impl crate::Writable for OUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT to value 0"]
impl crate::Resettable for OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
