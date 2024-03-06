#[doc = "Register `CTRL1CH0` reader"]
pub struct R(crate::R<CTRL1CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL1CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL1CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL1CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL1CH0` writer"]
pub struct W(crate::W<CTRL1CH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL1CH0_SPEC>;
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
impl From<crate::W<CTRL1CH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL1CH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BITS_WORD` reader - I2S word length."]
pub type BITS_WORD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BITS_WORD` writer - I2S word length."]
pub type BITS_WORD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL1CH0_SPEC, u8, u8, 5, O>;
#[doc = "Field `EN` reader - I2S clock enable."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - I2S clock enable."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL1CH0_SPEC, bool, O>;
#[doc = "Field `SMP_SIZE` reader - I2S sample size length."]
pub type SMP_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP_SIZE` writer - I2S sample size length."]
pub type SMP_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL1CH0_SPEC, u8, u8, 5, O>;
#[doc = "Field `EXTCLK_EN` reader - Selects the clock source for master mode operations in the ME17B."]
pub type EXTCLK_EN_R = crate::BitReader<EXTCLK_EN_A>;
#[doc = "Selects the clock source for master mode operations in the ME17B.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTCLK_EN_A {
    #[doc = "0: Use ERFO."]
    ERFO = 0,
    #[doc = "1: Use External Clock."]
    EXT = 1,
}
impl From<EXTCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EXTCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTCLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTCLK_EN_A {
        match self.bits {
            false => EXTCLK_EN_A::ERFO,
            true => EXTCLK_EN_A::EXT,
        }
    }
    #[doc = "Checks if the value of the field is `ERFO`"]
    #[inline(always)]
    pub fn is_erfo(&self) -> bool {
        *self == EXTCLK_EN_A::ERFO
    }
    #[doc = "Checks if the value of the field is `EXT`"]
    #[inline(always)]
    pub fn is_ext(&self) -> bool {
        *self == EXTCLK_EN_A::EXT
    }
}
#[doc = "Field `EXTCLK_EN` writer - Selects the clock source for master mode operations in the ME17B."]
pub type EXTCLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL1CH0_SPEC, EXTCLK_EN_A, O>;
impl<'a, const O: u8> EXTCLK_EN_W<'a, O> {
    #[doc = "Use ERFO."]
    #[inline(always)]
    pub fn erfo(self) -> &'a mut W {
        self.variant(EXTCLK_EN_A::ERFO)
    }
    #[doc = "Use External Clock."]
    #[inline(always)]
    pub fn ext(self) -> &'a mut W {
        self.variant(EXTCLK_EN_A::EXT)
    }
}
#[doc = "Field `ADJUST` reader - LSB/MSB Justify."]
pub type ADJUST_R = crate::BitReader<bool>;
#[doc = "Field `ADJUST` writer - LSB/MSB Justify."]
pub type ADJUST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL1CH0_SPEC, bool, O>;
#[doc = "Field `CLKDIV` reader - I2S clock frequency divisor."]
pub type CLKDIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLKDIV` writer - I2S clock frequency divisor."]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL1CH0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:4 - I2S word length."]
    #[inline(always)]
    pub fn bits_word(&self) -> BITS_WORD_R {
        BITS_WORD_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - I2S clock enable."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:13 - I2S sample size length."]
    #[inline(always)]
    pub fn smp_size(&self) -> SMP_SIZE_R {
        SMP_SIZE_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Selects the clock source for master mode operations in the ME17B."]
    #[inline(always)]
    pub fn extclk_en(&self) -> EXTCLK_EN_R {
        EXTCLK_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LSB/MSB Justify."]
    #[inline(always)]
    pub fn adjust(&self) -> ADJUST_R {
        ADJUST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - I2S clock frequency divisor."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - I2S word length."]
    #[inline(always)]
    #[must_use]
    pub fn bits_word(&mut self) -> BITS_WORD_W<0> {
        BITS_WORD_W::new(self)
    }
    #[doc = "Bit 8 - I2S clock enable."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<8> {
        EN_W::new(self)
    }
    #[doc = "Bits 9:13 - I2S sample size length."]
    #[inline(always)]
    #[must_use]
    pub fn smp_size(&mut self) -> SMP_SIZE_W<9> {
        SMP_SIZE_W::new(self)
    }
    #[doc = "Bit 14 - Selects the clock source for master mode operations in the ME17B."]
    #[inline(always)]
    #[must_use]
    pub fn extclk_en(&mut self) -> EXTCLK_EN_W<14> {
        EXTCLK_EN_W::new(self)
    }
    #[doc = "Bit 15 - LSB/MSB Justify."]
    #[inline(always)]
    #[must_use]
    pub fn adjust(&mut self) -> ADJUST_W<15> {
        ADJUST_W::new(self)
    }
    #[doc = "Bits 16:31 - I2S clock frequency divisor."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<16> {
        CLKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Local channel Setup.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1ch0](index.html) module"]
pub struct CTRL1CH0_SPEC;
impl crate::RegisterSpec for CTRL1CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl1ch0::R](R) reader structure"]
impl crate::Readable for CTRL1CH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl1ch0::W](W) writer structure"]
impl crate::Writable for CTRL1CH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL1CH0 to value 0"]
impl crate::Resettable for CTRL1CH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
