#[doc = "Register `EN0` reader"]
pub struct R(crate::R<EN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EN0` writer"]
pub struct W(crate::W<EN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EN0_SPEC>;
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
impl From<crate::W<EN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_EN` reader - Mask of all of the pins on the port."]
pub type GPIO_EN_R = crate::FieldReader<u32, GPIO_EN_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_EN_A {
    #[doc = "0: Alternate function enabled."]
    ALTERNATE = 0,
    #[doc = "1: GPIO function is enabled."]
    GPIO = 1,
}
impl From<GPIO_EN_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_EN_A) -> Self {
        variant as _
    }
}
impl GPIO_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_EN_A> {
        match self.bits {
            0 => Some(GPIO_EN_A::ALTERNATE),
            1 => Some(GPIO_EN_A::GPIO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ALTERNATE`"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == GPIO_EN_A::ALTERNATE
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == GPIO_EN_A::GPIO
    }
}
#[doc = "Field `GPIO_EN` writer - Mask of all of the pins on the port."]
pub type GPIO_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EN0_SPEC, u32, GPIO_EN_A, 32, O>;
impl<'a, const O: u8> GPIO_EN_W<'a, O> {
    #[doc = "Alternate function enabled."]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(GPIO_EN_A::ALTERNATE)
    }
    #[doc = "GPIO function is enabled."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(GPIO_EN_A::GPIO)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_en(&mut self) -> GPIO_EN_W<0> {
        GPIO_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Function Enable Register. Each bit controls the GPIO_EN setting for one GPIO pin on the associated port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en0](index.html) module"]
pub struct EN0_SPEC;
impl crate::RegisterSpec for EN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [en0::R](R) reader structure"]
impl crate::Readable for EN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [en0::W](W) writer structure"]
impl crate::Writable for EN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EN0 to value 0"]
impl crate::Resettable for EN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
