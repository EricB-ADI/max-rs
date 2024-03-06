#[doc = "Register `DS1` reader"]
pub struct R(crate::R<DS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DS1` writer"]
pub struct W(crate::W<DS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DS1_SPEC>;
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
impl From<crate::W<DS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_DS1` reader - Mask of all of the pins on the port."]
pub type GPIO_DS1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GPIO_DS1` writer - Mask of all of the pins on the port."]
pub type GPIO_DS1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DS1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_ds1(&self) -> GPIO_DS1_R {
        GPIO_DS1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_ds1(&mut self) -> GPIO_DS1_W<0> {
        GPIO_DS1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Drive Strength 1 Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ds1](index.html) module"]
pub struct DS1_SPEC;
impl crate::RegisterSpec for DS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ds1::R](R) reader structure"]
impl crate::Readable for DS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ds1::W](W) writer structure"]
impl crate::Writable for DS1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DS1 to value 0"]
impl crate::Resettable for DS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
