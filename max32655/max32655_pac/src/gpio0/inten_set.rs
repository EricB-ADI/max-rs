#[doc = "Register `INTEN_SET` reader"]
pub struct R(crate::R<INTEN_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN_SET` writer"]
pub struct W(crate::W<INTEN_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SET_SPEC>;
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
impl From<crate::W<INTEN_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_INTEN_SET` reader - Mask of all of the pins on the port."]
pub type GPIO_INTEN_SET_R = crate::FieldReader<u32, GPIO_INTEN_SET_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_INTEN_SET_A {
    #[doc = "0: No effect."]
    NO = 0,
    #[doc = "1: Set GPIO_INT_EN bit in this position to '1'"]
    SET = 1,
}
impl From<GPIO_INTEN_SET_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INTEN_SET_A) -> Self {
        variant as _
    }
}
impl GPIO_INTEN_SET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_INTEN_SET_A> {
        match self.bits {
            0 => Some(GPIO_INTEN_SET_A::NO),
            1 => Some(GPIO_INTEN_SET_A::SET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == GPIO_INTEN_SET_A::NO
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == GPIO_INTEN_SET_A::SET
    }
}
#[doc = "Field `GPIO_INTEN_SET` writer - Mask of all of the pins on the port."]
pub type GPIO_INTEN_SET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTEN_SET_SPEC, u32, GPIO_INTEN_SET_A, 32, O>;
impl<'a, const O: u8> GPIO_INTEN_SET_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(GPIO_INTEN_SET_A::NO)
    }
    #[doc = "Set GPIO_INT_EN bit in this position to '1'"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(GPIO_INTEN_SET_A::SET)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_inten_set(&self) -> GPIO_INTEN_SET_R {
        GPIO_INTEN_SET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_inten_set(&mut self) -> GPIO_INTEN_SET_W<0> {
        GPIO_INTEN_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Interrupt Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_INT_EN to 1, without affecting other bits in that register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten_set](index.html) module"]
pub struct INTEN_SET_SPEC;
impl crate::RegisterSpec for INTEN_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten_set::R](R) reader structure"]
impl crate::Readable for INTEN_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten_set::W](W) writer structure"]
impl crate::Writable for INTEN_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN_SET to value 0"]
impl crate::Resettable for INTEN_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
