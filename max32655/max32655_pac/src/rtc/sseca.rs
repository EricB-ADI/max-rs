#[doc = "Register `SSECA` reader"]
pub struct R(crate::R<SSECA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSECA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSECA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSECA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSECA` writer"]
pub struct W(crate::W<SSECA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSECA_SPEC>;
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
impl From<crate::W<SSECA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSECA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSEC_ALARM` reader - This register contains the reload value for the sub-second alarm."]
pub type SSEC_ALARM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SSEC_ALARM` writer - This register contains the reload value for the sub-second alarm."]
pub type SSEC_ALARM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSECA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This register contains the reload value for the sub-second alarm."]
    #[inline(always)]
    pub fn ssec_alarm(&self) -> SSEC_ALARM_R {
        SSEC_ALARM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register contains the reload value for the sub-second alarm."]
    #[inline(always)]
    #[must_use]
    pub fn ssec_alarm(&mut self) -> SSEC_ALARM_W<0> {
        SSEC_ALARM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC sub-second alarm. This register contains the reload value for the sub-second alarm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sseca](index.html) module"]
pub struct SSECA_SPEC;
impl crate::RegisterSpec for SSECA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sseca::R](R) reader structure"]
impl crate::Readable for SSECA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sseca::W](W) writer structure"]
impl crate::Writable for SSECA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSECA to value 0"]
impl crate::Resettable for SSECA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
