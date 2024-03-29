#[doc = "Register `PADCTRL1` reader"]
pub struct R(crate::R<PADCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADCTRL1` writer"]
pub struct W(crate::W<PADCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADCTRL1_SPEC>;
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
impl From<crate::W<PADCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_PADCTRL1` reader - The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
pub type GPIO_PADCTRL1_R = crate::FieldReader<u32, GPIO_PADCTRL1_A>;
#[doc = "The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_PADCTRL1_A {
    #[doc = "0: High Impedance."]
    IMPEDANCE = 0,
    #[doc = "1: Weak pull-up mode."]
    PU = 1,
    #[doc = "2: weak pull-down mode."]
    PD = 2,
}
impl From<GPIO_PADCTRL1_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_PADCTRL1_A) -> Self {
        variant as _
    }
}
impl GPIO_PADCTRL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_PADCTRL1_A> {
        match self.bits {
            0 => Some(GPIO_PADCTRL1_A::IMPEDANCE),
            1 => Some(GPIO_PADCTRL1_A::PU),
            2 => Some(GPIO_PADCTRL1_A::PD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IMPEDANCE`"]
    #[inline(always)]
    pub fn is_impedance(&self) -> bool {
        *self == GPIO_PADCTRL1_A::IMPEDANCE
    }
    #[doc = "Checks if the value of the field is `PU`"]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == GPIO_PADCTRL1_A::PU
    }
    #[doc = "Checks if the value of the field is `PD`"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == GPIO_PADCTRL1_A::PD
    }
}
#[doc = "Field `GPIO_PADCTRL1` writer - The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
pub type GPIO_PADCTRL1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PADCTRL1_SPEC, u32, GPIO_PADCTRL1_A, 32, O>;
impl<'a, const O: u8> GPIO_PADCTRL1_W<'a, O> {
    #[doc = "High Impedance."]
    #[inline(always)]
    pub fn impedance(self) -> &'a mut W {
        self.variant(GPIO_PADCTRL1_A::IMPEDANCE)
    }
    #[doc = "Weak pull-up mode."]
    #[inline(always)]
    pub fn pu(self) -> &'a mut W {
        self.variant(GPIO_PADCTRL1_A::PU)
    }
    #[doc = "weak pull-down mode."]
    #[inline(always)]
    pub fn pd(self) -> &'a mut W {
        self.variant(GPIO_PADCTRL1_A::PD)
    }
}
impl R {
    #[doc = "Bits 0:31 - The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
    #[inline(always)]
    pub fn gpio_padctrl1(&self) -> GPIO_PADCTRL1_R {
        GPIO_PADCTRL1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_padctrl1(&mut self) -> GPIO_PADCTRL1_W<0> {
        GPIO_PADCTRL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Input Mode Config 2. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padctrl1](index.html) module"]
pub struct PADCTRL1_SPEC;
impl crate::RegisterSpec for PADCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padctrl1::R](R) reader structure"]
impl crate::Readable for PADCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padctrl1::W](W) writer structure"]
impl crate::Writable for PADCTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PADCTRL1 to value 0"]
impl crate::Resettable for PADCTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
