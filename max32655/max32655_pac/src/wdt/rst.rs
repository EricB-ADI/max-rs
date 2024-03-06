#[doc = "Register `RST` writer"]
pub struct W(crate::W<RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RST_SPEC>;
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
impl From<crate::W<RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Writing the watchdog counter 'reset sequence' to this register resets the watchdog counter. If the watchdog count exceeds INT_PERIOD_UPPER_LIMIT then a watchdog interrupt will occur, if enabled. If the watchdog count exceeds RST_PERIOD_UPPER_LIMIT then a watchdog reset will occur, if enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RESET_AW {
    #[doc = "165: The first value to be written to reset the WDT."]
    SEQ0 = 165,
    #[doc = "90: The second value to be written to reset the WDT."]
    SEQ1 = 90,
}
impl From<RESET_AW> for u8 {
    #[inline(always)]
    fn from(variant: RESET_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `RESET` writer - Writing the watchdog counter 'reset sequence' to this register resets the watchdog counter. If the watchdog count exceeds INT_PERIOD_UPPER_LIMIT then a watchdog interrupt will occur, if enabled. If the watchdog count exceeds RST_PERIOD_UPPER_LIMIT then a watchdog reset will occur, if enabled."]
pub type RESET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RST_SPEC, u8, RESET_AW, 8, O>;
impl<'a, const O: u8> RESET_W<'a, O> {
    #[doc = "The first value to be written to reset the WDT."]
    #[inline(always)]
    pub fn seq0(self) -> &'a mut W {
        self.variant(RESET_AW::SEQ0)
    }
    #[doc = "The second value to be written to reset the WDT."]
    #[inline(always)]
    pub fn seq1(self) -> &'a mut W {
        self.variant(RESET_AW::SEQ1)
    }
}
impl W {
    #[doc = "Bits 0:7 - Writing the watchdog counter 'reset sequence' to this register resets the watchdog counter. If the watchdog count exceeds INT_PERIOD_UPPER_LIMIT then a watchdog interrupt will occur, if enabled. If the watchdog count exceeds RST_PERIOD_UPPER_LIMIT then a watchdog reset will occur, if enabled."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<0> {
        RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Windowed Watchdog Timer Reset Register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst](index.html) module"]
pub struct RST_SPEC;
impl crate::RegisterSpec for RST_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rst::W](W) writer structure"]
impl crate::Writable for RST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RST to value 0"]
impl crate::Resettable for RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
