#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPHYST` reader - Comparator hysteresis control."]
pub type CMPHYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMPHYST` writer - Comparator hysteresis control."]
pub type CMPHYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `INRO_EN` reader - INRO Enable."]
pub type INRO_EN_R = crate::BitReader<bool>;
#[doc = "Field `INRO_EN` writer - INRO Enable."]
pub type INRO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ERTCO_EN` reader - ERTCO Enable."]
pub type ERTCO_EN_R = crate::BitReader<bool>;
#[doc = "Field `ERTCO_EN` writer - ERTCO Enable."]
pub type ERTCO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `IBRO_EN` reader - IBRO Enable."]
pub type IBRO_EN_R = crate::BitReader<bool>;
#[doc = "Field `IBRO_EN` writer - IBRO Enable."]
pub type IBRO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `32KOSC_EN` reader - Enable 32K Oscillator input."]
pub type _32KOSC_EN_R = crate::BitReader<bool>;
#[doc = "Field `32KOSC_EN` writer - Enable 32K Oscillator input."]
pub type _32KOSC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SIMO_CLKSCL_EN` reader - SIMO Clock Scaling Enable."]
pub type SIMO_CLKSCL_EN_R = crate::BitReader<bool>;
#[doc = "Field `SIMO_CLKSCL_EN` writer - SIMO Clock Scaling Enable."]
pub type SIMO_CLKSCL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SIMO_RSTD` reader - SIMO System Reset Disable."]
pub type SIMO_RSTD_R = crate::BitReader<bool>;
#[doc = "Field `SIMO_RSTD` writer - SIMO System Reset Disable."]
pub type SIMO_RSTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Comparator hysteresis control."]
    #[inline(always)]
    pub fn cmphyst(&self) -> CMPHYST_R {
        CMPHYST_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - INRO Enable."]
    #[inline(always)]
    pub fn inro_en(&self) -> INRO_EN_R {
        INRO_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ERTCO Enable."]
    #[inline(always)]
    pub fn ertco_en(&self) -> ERTCO_EN_R {
        ERTCO_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IBRO Enable."]
    #[inline(always)]
    pub fn ibro_en(&self) -> IBRO_EN_R {
        IBRO_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable 32K Oscillator input."]
    #[inline(always)]
    pub fn _32kosc_en(&self) -> _32KOSC_EN_R {
        _32KOSC_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - SIMO Clock Scaling Enable."]
    #[inline(always)]
    pub fn simo_clkscl_en(&self) -> SIMO_CLKSCL_EN_R {
        SIMO_CLKSCL_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SIMO System Reset Disable."]
    #[inline(always)]
    pub fn simo_rstd(&self) -> SIMO_RSTD_R {
        SIMO_RSTD_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparator hysteresis control."]
    #[inline(always)]
    #[must_use]
    pub fn cmphyst(&mut self) -> CMPHYST_W<0> {
        CMPHYST_W::new(self)
    }
    #[doc = "Bit 2 - INRO Enable."]
    #[inline(always)]
    #[must_use]
    pub fn inro_en(&mut self) -> INRO_EN_W<2> {
        INRO_EN_W::new(self)
    }
    #[doc = "Bit 3 - ERTCO Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ertco_en(&mut self) -> ERTCO_EN_W<3> {
        ERTCO_EN_W::new(self)
    }
    #[doc = "Bit 4 - IBRO Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ibro_en(&mut self) -> IBRO_EN_W<4> {
        IBRO_EN_W::new(self)
    }
    #[doc = "Bit 5 - Enable 32K Oscillator input."]
    #[inline(always)]
    #[must_use]
    pub fn _32kosc_en(&mut self) -> _32KOSC_EN_W<5> {
        _32KOSC_EN_W::new(self)
    }
    #[doc = "Bit 8 - SIMO Clock Scaling Enable."]
    #[inline(always)]
    #[must_use]
    pub fn simo_clkscl_en(&mut self) -> SIMO_CLKSCL_EN_W<8> {
        SIMO_CLKSCL_EN_W::new(self)
    }
    #[doc = "Bit 9 - SIMO System Reset Disable."]
    #[inline(always)]
    #[must_use]
    pub fn simo_rstd(&mut self) -> SIMO_RSTD_W<9> {
        SIMO_RSTD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Miscellaneous Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
