#[doc = "Register `SRSEL` reader"]
pub struct R(crate::R<SRSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRSEL` writer"]
pub struct W(crate::W<SRSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRSEL_SPEC>;
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
impl From<crate::W<SRSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_SRSEL` reader - Mask of all of the pins on the port."]
pub type GPIO_SRSEL_R = crate::FieldReader<u32, GPIO_SRSEL_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_SRSEL_A {
    #[doc = "0: Fast Slew Rate selected."]
    FAST = 0,
    #[doc = "1: Slow Slew Rate selected."]
    SLOW = 1,
}
impl From<GPIO_SRSEL_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_SRSEL_A) -> Self {
        variant as _
    }
}
impl GPIO_SRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_SRSEL_A> {
        match self.bits {
            0 => Some(GPIO_SRSEL_A::FAST),
            1 => Some(GPIO_SRSEL_A::SLOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == GPIO_SRSEL_A::FAST
    }
    #[doc = "Checks if the value of the field is `SLOW`"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == GPIO_SRSEL_A::SLOW
    }
}
#[doc = "Field `GPIO_SRSEL` writer - Mask of all of the pins on the port."]
pub type GPIO_SRSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRSEL_SPEC, u32, GPIO_SRSEL_A, 32, O>;
impl<'a, const O: u8> GPIO_SRSEL_W<'a, O> {
    #[doc = "Fast Slew Rate selected."]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(GPIO_SRSEL_A::FAST)
    }
    #[doc = "Slow Slew Rate selected."]
    #[inline(always)]
    pub fn slow(self) -> &'a mut W {
        self.variant(GPIO_SRSEL_A::SLOW)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_srsel(&self) -> GPIO_SRSEL_R {
        GPIO_SRSEL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_srsel(&mut self) -> GPIO_SRSEL_W<0> {
        GPIO_SRSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Slew Rate Enable Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srsel](index.html) module"]
pub struct SRSEL_SPEC;
impl crate::RegisterSpec for SRSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srsel::R](R) reader structure"]
impl crate::Readable for SRSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srsel::W](W) writer structure"]
impl crate::Writable for SRSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRSEL to value 0"]
impl crate::Resettable for SRSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
