#[doc = "Register `OSCCTRL` reader"]
pub struct R(crate::R<OSCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCCTRL` writer"]
pub struct W(crate::W<OSCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCCTRL_SPEC>;
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
impl From<crate::W<OSCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTER_EN` reader - Enables analog deglitch filter."]
pub type FILTER_EN_R = crate::BitReader<bool>;
#[doc = "Field `FILTER_EN` writer - Enables analog deglitch filter."]
pub type FILTER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCCTRL_SPEC, bool, O>;
#[doc = "Field `IBIAS_SEL` reader - If IBIAS_EN is 1, selects 4x,2x mode."]
pub type IBIAS_SEL_R = crate::BitReader<bool>;
#[doc = "Field `IBIAS_SEL` writer - If IBIAS_EN is 1, selects 4x,2x mode."]
pub type IBIAS_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCCTRL_SPEC, bool, O>;
#[doc = "Field `HYST_EN` reader - Enables high current hysteresis buffer."]
pub type HYST_EN_R = crate::BitReader<bool>;
#[doc = "Field `HYST_EN` writer - Enables high current hysteresis buffer."]
pub type HYST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCCTRL_SPEC, bool, O>;
#[doc = "Field `IBIAS_EN` reader - Enables higher 4x,2x current modes."]
pub type IBIAS_EN_R = crate::BitReader<bool>;
#[doc = "Field `IBIAS_EN` writer - Enables higher 4x,2x current modes."]
pub type IBIAS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCCTRL_SPEC, bool, O>;
#[doc = "Field `BYPASS` reader - RTC Crystal Bypass"]
pub type BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `BYPASS` writer - RTC Crystal Bypass"]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCCTRL_SPEC, bool, O>;
#[doc = "Field `SQW_32K` reader - RTC 32kHz Square Wave Output"]
pub type SQW_32K_R = crate::BitReader<bool>;
#[doc = "Field `SQW_32K` writer - RTC 32kHz Square Wave Output"]
pub type SQW_32K_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enables analog deglitch filter."]
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If IBIAS_EN is 1, selects 4x,2x mode."]
    #[inline(always)]
    pub fn ibias_sel(&self) -> IBIAS_SEL_R {
        IBIAS_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables high current hysteresis buffer."]
    #[inline(always)]
    pub fn hyst_en(&self) -> HYST_EN_R {
        HYST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables higher 4x,2x current modes."]
    #[inline(always)]
    pub fn ibias_en(&self) -> IBIAS_EN_R {
        IBIAS_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC Crystal Bypass"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC 32kHz Square Wave Output"]
    #[inline(always)]
    pub fn sqw_32k(&self) -> SQW_32K_R {
        SQW_32K_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables analog deglitch filter."]
    #[inline(always)]
    #[must_use]
    pub fn filter_en(&mut self) -> FILTER_EN_W<0> {
        FILTER_EN_W::new(self)
    }
    #[doc = "Bit 1 - If IBIAS_EN is 1, selects 4x,2x mode."]
    #[inline(always)]
    #[must_use]
    pub fn ibias_sel(&mut self) -> IBIAS_SEL_W<1> {
        IBIAS_SEL_W::new(self)
    }
    #[doc = "Bit 2 - Enables high current hysteresis buffer."]
    #[inline(always)]
    #[must_use]
    pub fn hyst_en(&mut self) -> HYST_EN_W<2> {
        HYST_EN_W::new(self)
    }
    #[doc = "Bit 3 - Enables higher 4x,2x current modes."]
    #[inline(always)]
    #[must_use]
    pub fn ibias_en(&mut self) -> IBIAS_EN_W<3> {
        IBIAS_EN_W::new(self)
    }
    #[doc = "Bit 4 - RTC Crystal Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<4> {
        BYPASS_W::new(self)
    }
    #[doc = "Bit 5 - RTC 32kHz Square Wave Output"]
    #[inline(always)]
    #[must_use]
    pub fn sqw_32k(&mut self) -> SQW_32K_W<5> {
        SQW_32K_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Oscillator Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscctrl](index.html) module"]
pub struct OSCCTRL_SPEC;
impl crate::RegisterSpec for OSCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oscctrl::R](R) reader structure"]
impl crate::Readable for OSCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oscctrl::W](W) writer structure"]
impl crate::Writable for OSCCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCCTRL to value 0"]
impl crate::Resettable for OSCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
