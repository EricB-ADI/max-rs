#[doc = "Register `LPWKEN0` reader"]
pub struct R(crate::R<LPWKEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPWKEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPWKEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPWKEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPWKEN0` writer"]
pub struct W(crate::W<LPWKEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPWKEN0_SPEC>;
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
impl From<crate::W<LPWKEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPWKEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKEEN` reader - Enable wakeup. These bits allow wakeup from the corresponding GPIO pin (s) on transition (s) from low to high or high to low when PM.GPIOWKEN is set. Wakeup status is indicated in PPWKST register."]
pub type WAKEEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WAKEEN` writer - Enable wakeup. These bits allow wakeup from the corresponding GPIO pin (s) on transition (s) from low to high or high to low when PM.GPIOWKEN is set. Wakeup status is indicated in PPWKST register."]
pub type WAKEEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPWKEN0_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bits 0:30 - Enable wakeup. These bits allow wakeup from the corresponding GPIO pin (s) on transition (s) from low to high or high to low when PM.GPIOWKEN is set. Wakeup status is indicated in PPWKST register."]
    #[inline(always)]
    pub fn wakeen(&self) -> WAKEEN_R {
        WAKEEN_R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - Enable wakeup. These bits allow wakeup from the corresponding GPIO pin (s) on transition (s) from low to high or high to low when PM.GPIOWKEN is set. Wakeup status is indicated in PPWKST register."]
    #[inline(always)]
    #[must_use]
    pub fn wakeen(&mut self) -> WAKEEN_W<0> {
        WAKEEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Power I/O Wakeup Enable Register 0. This register enables low power wakeup functionality for GPIO0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpwken0](index.html) module"]
pub struct LPWKEN0_SPEC;
impl crate::RegisterSpec for LPWKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpwken0::R](R) reader structure"]
impl crate::Readable for LPWKEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpwken0::W](W) writer structure"]
impl crate::Writable for LPWKEN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPWKEN0 to value 0"]
impl crate::Resettable for LPWKEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
