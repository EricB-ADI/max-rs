#[doc = "Register `DUALEDGE` reader"]
pub struct R(crate::R<DUALEDGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DUALEDGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DUALEDGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DUALEDGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DUALEDGE` writer"]
pub struct W(crate::W<DUALEDGE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DUALEDGE_SPEC>;
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
impl From<crate::W<DUALEDGE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DUALEDGE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_DUALEDGE` reader - Mask of all of the pins on the port."]
pub type GPIO_DUALEDGE_R = crate::FieldReader<u32, GPIO_DUALEDGE_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_DUALEDGE_A {
    #[doc = "0: No Effect."]
    NO = 0,
    #[doc = "1: Dual Edge mode is enabled. If edge-triggered interrupts are enabled on this GPIO pin, then both rising and falling edges will trigger interrupts regardless of the GPIO_INT_POL setting."]
    EN = 1,
}
impl From<GPIO_DUALEDGE_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_DUALEDGE_A) -> Self {
        variant as _
    }
}
impl GPIO_DUALEDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_DUALEDGE_A> {
        match self.bits {
            0 => Some(GPIO_DUALEDGE_A::NO),
            1 => Some(GPIO_DUALEDGE_A::EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == GPIO_DUALEDGE_A::NO
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GPIO_DUALEDGE_A::EN
    }
}
#[doc = "Field `GPIO_DUALEDGE` writer - Mask of all of the pins on the port."]
pub type GPIO_DUALEDGE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DUALEDGE_SPEC, u32, GPIO_DUALEDGE_A, 32, O>;
impl<'a, const O: u8> GPIO_DUALEDGE_W<'a, O> {
    #[doc = "No Effect."]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(GPIO_DUALEDGE_A::NO)
    }
    #[doc = "Dual Edge mode is enabled. If edge-triggered interrupts are enabled on this GPIO pin, then both rising and falling edges will trigger interrupts regardless of the GPIO_INT_POL setting."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(GPIO_DUALEDGE_A::EN)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_dualedge(&self) -> GPIO_DUALEDGE_R {
        GPIO_DUALEDGE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_dualedge(&mut self) -> GPIO_DUALEDGE_W<0> {
        GPIO_DUALEDGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Interrupt Dual Edge Mode Register. Each bit in this register selects dual edge mode for the associated GPIO pin in this port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dualedge](index.html) module"]
pub struct DUALEDGE_SPEC;
impl crate::RegisterSpec for DUALEDGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dualedge::R](R) reader structure"]
impl crate::Readable for DUALEDGE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dualedge::W](W) writer structure"]
impl crate::Writable for DUALEDGE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DUALEDGE to value 0"]
impl crate::Resettable for DUALEDGE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
