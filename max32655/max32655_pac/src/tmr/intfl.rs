#[doc = "Register `INTFL` reader"]
pub struct R(crate::R<INTFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFL` writer"]
pub struct W(crate::W<INTFL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFL_SPEC>;
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
impl From<crate::W<INTFL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQ_A` reader - Interrupt Flag for Timer A."]
pub type IRQ_A_R = crate::BitReader<bool>;
#[doc = "Field `IRQ_A` writer - Interrupt Flag for Timer A."]
pub type IRQ_A_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFL_SPEC, bool, O>;
#[doc = "Field `WRDONE_A` reader - Write Done Flag for Timer A indicating the write is complete from APB to CLK_TMR domain."]
pub type WRDONE_A_R = crate::BitReader<bool>;
#[doc = "Field `WRDONE_A` writer - Write Done Flag for Timer A indicating the write is complete from APB to CLK_TMR domain."]
pub type WRDONE_A_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFL_SPEC, bool, O>;
#[doc = "Field `WR_DIS_A` reader - Write Disable to CNT/PWM for Timer A in the non-cascaded dual timer configuration."]
pub type WR_DIS_A_R = crate::BitReader<bool>;
#[doc = "Field `WR_DIS_A` writer - Write Disable to CNT/PWM for Timer A in the non-cascaded dual timer configuration."]
pub type WR_DIS_A_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFL_SPEC, bool, O>;
#[doc = "Field `IRQ_B` reader - Interrupt Flag for Timer B."]
pub type IRQ_B_R = crate::BitReader<bool>;
#[doc = "Field `IRQ_B` writer - Interrupt Flag for Timer B."]
pub type IRQ_B_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFL_SPEC, bool, O>;
#[doc = "Field `WRDONE_B` reader - Write Done Flag for Timer B indicating the write is complete from APB to CLK_TMR domain."]
pub type WRDONE_B_R = crate::BitReader<bool>;
#[doc = "Field `WRDONE_B` writer - Write Done Flag for Timer B indicating the write is complete from APB to CLK_TMR domain."]
pub type WRDONE_B_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFL_SPEC, bool, O>;
#[doc = "Field `WR_DIS_B` reader - Write Disable to CNT/PWM for Timer B in the non-cascaded dual timer configuration."]
pub type WR_DIS_B_R = crate::BitReader<bool>;
#[doc = "Field `WR_DIS_B` writer - Write Disable to CNT/PWM for Timer B in the non-cascaded dual timer configuration."]
pub type WR_DIS_B_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt Flag for Timer A."]
    #[inline(always)]
    pub fn irq_a(&self) -> IRQ_A_R {
        IRQ_A_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Write Done Flag for Timer A indicating the write is complete from APB to CLK_TMR domain."]
    #[inline(always)]
    pub fn wrdone_a(&self) -> WRDONE_A_R {
        WRDONE_A_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write Disable to CNT/PWM for Timer A in the non-cascaded dual timer configuration."]
    #[inline(always)]
    pub fn wr_dis_a(&self) -> WR_DIS_A_R {
        WR_DIS_A_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Flag for Timer B."]
    #[inline(always)]
    pub fn irq_b(&self) -> IRQ_B_R {
        IRQ_B_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Write Done Flag for Timer B indicating the write is complete from APB to CLK_TMR domain."]
    #[inline(always)]
    pub fn wrdone_b(&self) -> WRDONE_B_R {
        WRDONE_B_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write Disable to CNT/PWM for Timer B in the non-cascaded dual timer configuration."]
    #[inline(always)]
    pub fn wr_dis_b(&self) -> WR_DIS_B_R {
        WR_DIS_B_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Flag for Timer A."]
    #[inline(always)]
    #[must_use]
    pub fn irq_a(&mut self) -> IRQ_A_W<0> {
        IRQ_A_W::new(self)
    }
    #[doc = "Bit 8 - Write Done Flag for Timer A indicating the write is complete from APB to CLK_TMR domain."]
    #[inline(always)]
    #[must_use]
    pub fn wrdone_a(&mut self) -> WRDONE_A_W<8> {
        WRDONE_A_W::new(self)
    }
    #[doc = "Bit 9 - Write Disable to CNT/PWM for Timer A in the non-cascaded dual timer configuration."]
    #[inline(always)]
    #[must_use]
    pub fn wr_dis_a(&mut self) -> WR_DIS_A_W<9> {
        WR_DIS_A_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt Flag for Timer B."]
    #[inline(always)]
    #[must_use]
    pub fn irq_b(&mut self) -> IRQ_B_W<16> {
        IRQ_B_W::new(self)
    }
    #[doc = "Bit 24 - Write Done Flag for Timer B indicating the write is complete from APB to CLK_TMR domain."]
    #[inline(always)]
    #[must_use]
    pub fn wrdone_b(&mut self) -> WRDONE_B_W<24> {
        WRDONE_B_W::new(self)
    }
    #[doc = "Bit 25 - Write Disable to CNT/PWM for Timer B in the non-cascaded dual timer configuration."]
    #[inline(always)]
    #[must_use]
    pub fn wr_dis_b(&mut self) -> WR_DIS_B_W<25> {
        WR_DIS_B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Interrupt Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intfl](index.html) module"]
pub struct INTFL_SPEC;
impl crate::RegisterSpec for INTFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intfl::R](R) reader structure"]
impl crate::Readable for INTFL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intfl::W](W) writer structure"]
impl crate::Writable for INTFL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFL to value 0"]
impl crate::Resettable for INTFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
