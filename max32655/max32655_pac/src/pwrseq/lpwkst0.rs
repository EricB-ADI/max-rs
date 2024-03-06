#[doc = "Register `LPWKST0` reader"]
pub struct R(crate::R<LPWKST0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPWKST0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPWKST0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPWKST0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPWKST0` writer"]
pub struct W(crate::W<LPWKST0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPWKST0_SPEC>;
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
impl From<crate::W<LPWKST0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPWKST0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKEST` reader - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin (s) transition (s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
pub type WAKEST_R = crate::BitReader<bool>;
#[doc = "Field `WAKEST` writer - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin (s) transition (s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
pub type WAKEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPWKST0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin (s) transition (s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
    #[inline(always)]
    pub fn wakest(&self) -> WAKEST_R {
        WAKEST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin (s) transition (s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
    #[inline(always)]
    #[must_use]
    pub fn wakest(&mut self) -> WAKEST_W<0> {
        WAKEST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Power I/O Wakeup Status Register 0. This register indicates the low power wakeup status for GPIO0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpwkst0](index.html) module"]
pub struct LPWKST0_SPEC;
impl crate::RegisterSpec for LPWKST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpwkst0::R](R) reader structure"]
impl crate::Readable for LPWKST0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpwkst0::W](W) writer structure"]
impl crate::Writable for LPWKST0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPWKST0 to value 0"]
impl crate::Resettable for LPWKST0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
