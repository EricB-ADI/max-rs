#[doc = "Register `INTEN_CLR` reader"]
pub struct R(crate::R<INTEN_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN_CLR` writer"]
pub struct W(crate::W<INTEN_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_CLR_SPEC>;
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
impl From<crate::W<INTEN_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_INTEN_CLR` reader - Mask of all of the pins on the port."]
pub type GPIO_INTEN_CLR_R = crate::FieldReader<u32, GPIO_INTEN_CLR_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_INTEN_CLR_A {
    #[doc = "0: No Effect."]
    NO = 0,
    #[doc = "1: Clear GPIO_INT_EN bit in this position to '0'"]
    CLEAR = 1,
}
impl From<GPIO_INTEN_CLR_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INTEN_CLR_A) -> Self {
        variant as _
    }
}
impl GPIO_INTEN_CLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_INTEN_CLR_A> {
        match self.bits {
            0 => Some(GPIO_INTEN_CLR_A::NO),
            1 => Some(GPIO_INTEN_CLR_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == GPIO_INTEN_CLR_A::NO
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO_INTEN_CLR_A::CLEAR
    }
}
#[doc = "Field `GPIO_INTEN_CLR` writer - Mask of all of the pins on the port."]
pub type GPIO_INTEN_CLR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTEN_CLR_SPEC, u32, GPIO_INTEN_CLR_A, 32, O>;
impl<'a, const O: u8> GPIO_INTEN_CLR_W<'a, O> {
    #[doc = "No Effect."]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(GPIO_INTEN_CLR_A::NO)
    }
    #[doc = "Clear GPIO_INT_EN bit in this position to '0'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO_INTEN_CLR_A::CLEAR)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_inten_clr(&self) -> GPIO_INTEN_CLR_R {
        GPIO_INTEN_CLR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_inten_clr(&mut self) -> GPIO_INTEN_CLR_W<0> {
        GPIO_INTEN_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Interrupt Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_EN to 0, without affecting other bits in that register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten_clr](index.html) module"]
pub struct INTEN_CLR_SPEC;
impl crate::RegisterSpec for INTEN_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten_clr::R](R) reader structure"]
impl crate::Readable for INTEN_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten_clr::W](W) writer structure"]
impl crate::Writable for INTEN_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN_CLR to value 0"]
impl crate::Resettable for INTEN_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
