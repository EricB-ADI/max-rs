#[doc = "Register `INTPOL` reader"]
pub struct R(crate::R<INTPOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTPOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTPOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTPOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTPOL` writer"]
pub struct W(crate::W<INTPOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTPOL_SPEC>;
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
impl From<crate::W<INTPOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTPOL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_INTPOL` reader - Mask of all of the pins on the port."]
pub type GPIO_INTPOL_R = crate::FieldReader<u32, GPIO_INTPOL_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_INTPOL_A {
    #[doc = "0: Interrupts are latched on a falling edge or low level condition for this pin."]
    FALLING = 0,
    #[doc = "1: Interrupts are latched on a rising edge or high condition for this pin."]
    RISING = 1,
}
impl From<GPIO_INTPOL_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INTPOL_A) -> Self {
        variant as _
    }
}
impl GPIO_INTPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_INTPOL_A> {
        match self.bits {
            0 => Some(GPIO_INTPOL_A::FALLING),
            1 => Some(GPIO_INTPOL_A::RISING),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == GPIO_INTPOL_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == GPIO_INTPOL_A::RISING
    }
}
#[doc = "Field `GPIO_INTPOL` writer - Mask of all of the pins on the port."]
pub type GPIO_INTPOL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTPOL_SPEC, u32, GPIO_INTPOL_A, 32, O>;
impl<'a, const O: u8> GPIO_INTPOL_W<'a, O> {
    #[doc = "Interrupts are latched on a falling edge or low level condition for this pin."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(GPIO_INTPOL_A::FALLING)
    }
    #[doc = "Interrupts are latched on a rising edge or high condition for this pin."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(GPIO_INTPOL_A::RISING)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_intpol(&self) -> GPIO_INTPOL_R {
        GPIO_INTPOL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_intpol(&mut self) -> GPIO_INTPOL_W<0> {
        GPIO_INTPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Interrupt Polarity Register. Each bit in this register controls the interrupt polarity setting for one GPIO pin in the associated port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intpol](index.html) module"]
pub struct INTPOL_SPEC;
impl crate::RegisterSpec for INTPOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intpol::R](R) reader structure"]
impl crate::Readable for INTPOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intpol::W](W) writer structure"]
impl crate::Writable for INTPOL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTPOL to value 0"]
impl crate::Resettable for INTPOL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
