#[doc = "Register `RST` reader"]
pub struct R(crate::R<RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RST_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Field `GPIO2` reader - Low Power GPIO 2 Reset."]
pub type GPIO2_R = crate::BitReader<RESET_A>;
#[doc = "Low Power GPIO 2 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Starts Reset or indicates reset in progress."]
    BUSY = 1,
}
impl From<RESET_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO2_R {
   
}
#[doc = "Field `GPIO2` writer - Low Power GPIO 2 Reset."]
pub type GPIO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_SPEC, RESET_A, O>;
impl<'a, const O: u8> GPIO2_W<'a, O> {

}
#[doc = "Field `WDT1` reader - Low Power Watchdog Timer 1 Reset."]
pub type WDT1_R = crate::BitReader<RESET_A>;
#[doc = "Low Power Watchdog Timer 1 Reset.\n\nValue on reset: 0"]
impl WDT1_R {

}
#[doc = "Field `WDT1` writer - Low Power Watchdog Timer 1 Reset."]
pub type WDT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_SPEC, RESET_A, O>;
impl<'a, const O: u8> WDT1_W<'a, O> {

}
#[doc = "Field `TMR4` reader - Low Power Timer 4 Reset."]
pub type TMR4_R = crate::BitReader<RESET_A>;
#[doc = "Low Power Timer 4 Reset.\n\nValue on reset: 0"]

impl TMR4_R {

}
#[doc = "Field `TMR4` writer - Low Power Timer 4 Reset."]
pub type TMR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_SPEC, RESET_A, O>;
impl<'a, const O: u8> TMR4_W<'a, O> {

}
#[doc = "Field `TMR5` reader - Low Power Timer 5 Reset."]
pub type TMR5_R = crate::BitReader<RESET_A>;
#[doc = "Low Power Timer 5 Reset.\n\nValue on reset: 0"]

impl TMR5_R {

}
#[doc = "Field `TMR5` writer - Low Power Timer 5 Reset."]
pub type TMR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_SPEC, RESET_A, O>;
impl<'a, const O: u8> TMR5_W<'a, O> {

}
#[doc = "Field `UART3` reader - Low Power UART 3 Reset."]
pub type UART3_R = crate::BitReader<RESET_A>;
#[doc = "Low Power UART 3 Reset.\n\nValue on reset: 0"]

impl UART3_R {

}
#[doc = "Field `UART3` writer - Low Power UART 3 Reset."]
pub type UART3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_SPEC, RESET_A, O>;
impl<'a, const O: u8> UART3_W<'a, O> {

}
#[doc = "Field `LPCOMP` reader - Low Power Comparator Reset."]
pub type LPCOMP_R = crate::BitReader<RESET_A>;
#[doc = "Low Power Comparator Reset.\n\nValue on reset: 0"]

impl LPCOMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_A {
        match self.bits {
            false => RESET_A::RESET_DONE,
            true => RESET_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == RESET_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RESET_A::BUSY
    }
}
#[doc = "Field `LPCOMP` writer - Low Power Comparator Reset."]
pub type LPCOMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_SPEC, RESET_A, O>;
impl<'a, const O: u8> LPCOMP_W<'a, O> {
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn reset_done(self) -> &'a mut W {
        self.variant(RESET_A::RESET_DONE)
    }
    #[doc = "Starts Reset or indicates reset in progress."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(RESET_A::BUSY)
    }
}
impl R {
    #[doc = "Bit 0 - Low Power GPIO 2 Reset."]
    #[inline(always)]
    pub fn gpio2(&self) -> GPIO2_R {
        GPIO2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Power Watchdog Timer 1 Reset."]
    #[inline(always)]
    pub fn wdt1(&self) -> WDT1_R {
        WDT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low Power Timer 4 Reset."]
    #[inline(always)]
    pub fn tmr4(&self) -> TMR4_R {
        TMR4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Low Power Timer 5 Reset."]
    #[inline(always)]
    pub fn tmr5(&self) -> TMR5_R {
        TMR5_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Power UART 3 Reset."]
    #[inline(always)]
    pub fn uart3(&self) -> UART3_R {
        UART3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Low Power Comparator Reset."]
    #[inline(always)]
    pub fn lpcomp(&self) -> LPCOMP_R {
        LPCOMP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Power GPIO 2 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn gpio2(&mut self) -> GPIO2_W<0> {
        GPIO2_W::new(self)
    }
    #[doc = "Bit 1 - Low Power Watchdog Timer 1 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn wdt1(&mut self) -> WDT1_W<1> {
        WDT1_W::new(self)
    }
    #[doc = "Bit 2 - Low Power Timer 4 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn tmr4(&mut self) -> TMR4_W<2> {
        TMR4_W::new(self)
    }
    #[doc = "Bit 3 - Low Power Timer 5 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn tmr5(&mut self) -> TMR5_W<3> {
        TMR5_W::new(self)
    }
    #[doc = "Bit 4 - Low Power UART 3 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn uart3(&mut self) -> UART3_W<4> {
        UART3_W::new(self)
    }
    #[doc = "Bit 6 - Low Power Comparator Reset."]
    #[inline(always)]
    #[must_use]
    pub fn lpcomp(&mut self) -> LPCOMP_W<6> {
        LPCOMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Power Reset Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst](index.html) module"]
pub struct RST_SPEC;
impl crate::RegisterSpec for RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rst::R](R) reader structure"]
impl crate::Readable for RST_SPEC {
    type Reader = R;
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
