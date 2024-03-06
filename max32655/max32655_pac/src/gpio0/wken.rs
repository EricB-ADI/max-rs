#[doc = "Register `WKEN` reader"]
pub struct R(crate::R<WKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKEN` writer"]
pub struct W(crate::W<WKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKEN_SPEC>;
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
impl From<crate::W<WKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_WKEN` reader - Mask of all of the pins on the port."]
pub type GPIO_WKEN_R = crate::FieldReader<u32, GPIO_WKEN_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_WKEN_A {
    #[doc = "0: PMU wakeup for this GPIO is disabled."]
    DIS = 0,
    #[doc = "1: PMU wakeup for this GPIO is enabled."]
    EN = 1,
}
impl From<GPIO_WKEN_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_WKEN_A) -> Self {
        variant as _
    }
}
impl GPIO_WKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_WKEN_A> {
        match self.bits {
            0 => Some(GPIO_WKEN_A::DIS),
            1 => Some(GPIO_WKEN_A::EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO_WKEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GPIO_WKEN_A::EN
    }
}
#[doc = "Field `GPIO_WKEN` writer - Mask of all of the pins on the port."]
pub type GPIO_WKEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WKEN_SPEC, u32, GPIO_WKEN_A, 32, O>;
impl<'a, const O: u8> GPIO_WKEN_W<'a, O> {
    #[doc = "PMU wakeup for this GPIO is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO_WKEN_A::DIS)
    }
    #[doc = "PMU wakeup for this GPIO is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(GPIO_WKEN_A::EN)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_wken(&self) -> GPIO_WKEN_R {
        GPIO_WKEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_wken(&mut self) -> GPIO_WKEN_W<0> {
        GPIO_WKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Wake Enable Register. Each bit in this register controls the PMU wakeup enable for the associated GPIO pin in this port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wken](index.html) module"]
pub struct WKEN_SPEC;
impl crate::RegisterSpec for WKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wken::R](R) reader structure"]
impl crate::Readable for WKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wken::W](W) writer structure"]
impl crate::Writable for WKEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WKEN to value 0"]
impl crate::Resettable for WKEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
