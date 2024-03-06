#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_INTEN` reader - Mask of all of the pins on the port."]
pub type GPIO_INTEN_R = crate::FieldReader<u32, GPIO_INTEN_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_INTEN_A {
    #[doc = "0: Interrupts are disabled for this GPIO pin."]
    DIS = 0,
    #[doc = "1: Interrupts are enabled for this GPIO pin."]
    EN = 1,
}
impl From<GPIO_INTEN_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_INTEN_A) -> Self {
        variant as _
    }
}
impl GPIO_INTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_INTEN_A> {
        match self.bits {
            0 => Some(GPIO_INTEN_A::DIS),
            1 => Some(GPIO_INTEN_A::EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO_INTEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GPIO_INTEN_A::EN
    }
}
#[doc = "Field `GPIO_INTEN` writer - Mask of all of the pins on the port."]
pub type GPIO_INTEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTEN_SPEC, u32, GPIO_INTEN_A, 32, O>;
impl<'a, const O: u8> GPIO_INTEN_W<'a, O> {
    #[doc = "Interrupts are disabled for this GPIO pin."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO_INTEN_A::DIS)
    }
    #[doc = "Interrupts are enabled for this GPIO pin."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(GPIO_INTEN_A::EN)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_inten(&self) -> GPIO_INTEN_R {
        GPIO_INTEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_inten(&mut self) -> GPIO_INTEN_W<0> {
        GPIO_INTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Interrupt Enable Register. Each bit in this register controls the GPIO interrupt enable for the associated pin on the GPIO port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
