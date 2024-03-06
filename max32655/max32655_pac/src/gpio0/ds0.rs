#[doc = "Register `DS0` reader"]
pub struct R(crate::R<DS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DS0` writer"]
pub struct W(crate::W<DS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DS0_SPEC>;
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
impl From<crate::W<DS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_DS0` reader - Mask of all of the pins on the port."]
pub type GPIO_DS0_R = crate::FieldReader<u32, GPIO_DS0_A>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_DS0_A {
    #[doc = "0: GPIO port pin is in low-drive mode."]
    LD = 0,
    #[doc = "1: GPIO port pin is in high-drive mode."]
    HD = 1,
}
impl From<GPIO_DS0_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_DS0_A) -> Self {
        variant as _
    }
}
impl GPIO_DS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_DS0_A> {
        match self.bits {
            0 => Some(GPIO_DS0_A::LD),
            1 => Some(GPIO_DS0_A::HD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LD`"]
    #[inline(always)]
    pub fn is_ld(&self) -> bool {
        *self == GPIO_DS0_A::LD
    }
    #[doc = "Checks if the value of the field is `HD`"]
    #[inline(always)]
    pub fn is_hd(&self) -> bool {
        *self == GPIO_DS0_A::HD
    }
}
#[doc = "Field `GPIO_DS0` writer - Mask of all of the pins on the port."]
pub type GPIO_DS0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DS0_SPEC, u32, GPIO_DS0_A, 32, O>;
impl<'a, const O: u8> GPIO_DS0_W<'a, O> {
    #[doc = "GPIO port pin is in low-drive mode."]
    #[inline(always)]
    pub fn ld(self) -> &'a mut W {
        self.variant(GPIO_DS0_A::LD)
    }
    #[doc = "GPIO port pin is in high-drive mode."]
    #[inline(always)]
    pub fn hd(self) -> &'a mut W {
        self.variant(GPIO_DS0_A::HD)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_ds0(&self) -> GPIO_DS0_R {
        GPIO_DS0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_ds0(&mut self) -> GPIO_DS0_W<0> {
        GPIO_DS0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Drive Strength Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ds0](index.html) module"]
pub struct DS0_SPEC;
impl crate::RegisterSpec for DS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ds0::R](R) reader structure"]
impl crate::Readable for DS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ds0::W](W) writer structure"]
impl crate::Writable for DS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DS0 to value 0"]
impl crate::Resettable for DS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
