#[doc = "Register `EN1` reader"]
pub struct R(crate::R<EN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EN1` writer"]
pub struct W(crate::W<EN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EN1_SPEC>;
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
impl From<crate::W<EN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_EN1` reader - Mask of all of the pins on the port."]
pub type GPIO_EN1_R = crate::FieldReader<u32, GPIO_EN1_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_EN1_A {
    #[doc = "0: Primary function selected."]
    PRIMARY = 0,
    #[doc = "1: Secondary function selected."]
    SECONDARY = 1,
}
impl From<GPIO_EN1_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_EN1_A) -> Self {
        variant as _
    }
}
impl GPIO_EN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_EN1_A> {
        match self.bits {
            0 => Some(GPIO_EN1_A::PRIMARY),
            1 => Some(GPIO_EN1_A::SECONDARY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRIMARY`"]
    #[inline(always)]
    pub fn is_primary(&self) -> bool {
        *self == GPIO_EN1_A::PRIMARY
    }
    #[doc = "Checks if the value of the field is `SECONDARY`"]
    #[inline(always)]
    pub fn is_secondary(&self) -> bool {
        *self == GPIO_EN1_A::SECONDARY
    }
}
#[doc = "Field `GPIO_EN1` writer - Mask of all of the pins on the port."]
pub type GPIO_EN1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EN1_SPEC, u32, GPIO_EN1_A, 32, O>;
impl<'a, const O: u8> GPIO_EN1_W<'a, O> {
    #[doc = "Primary function selected."]
    #[inline(always)]
    pub fn primary(self) -> &'a mut W {
        self.variant(GPIO_EN1_A::PRIMARY)
    }
    #[doc = "Secondary function selected."]
    #[inline(always)]
    pub fn secondary(self) -> &'a mut W {
        self.variant(GPIO_EN1_A::SECONDARY)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_en1(&self) -> GPIO_EN1_R {
        GPIO_EN1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_en1(&mut self) -> GPIO_EN1_W<0> {
        GPIO_EN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en1](index.html) module"]
pub struct EN1_SPEC;
impl crate::RegisterSpec for EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [en1::R](R) reader structure"]
impl crate::Readable for EN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [en1::W](W) writer structure"]
impl crate::Writable for EN1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EN1 to value 0"]
impl crate::Resettable for EN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
