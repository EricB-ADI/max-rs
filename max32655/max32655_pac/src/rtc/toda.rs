#[doc = "Register `TODA` reader"]
pub struct R(crate::R<TODA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TODA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TODA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TODA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TODA` writer"]
pub struct W(crate::W<TODA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TODA_SPEC>;
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
impl From<crate::W<TODA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TODA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOD_ALARM` reader - Time-of-day Alarm."]
pub type TOD_ALARM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TOD_ALARM` writer - Time-of-day Alarm."]
pub type TOD_ALARM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TODA_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - Time-of-day Alarm."]
    #[inline(always)]
    pub fn tod_alarm(&self) -> TOD_ALARM_R {
        TOD_ALARM_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Time-of-day Alarm."]
    #[inline(always)]
    #[must_use]
    pub fn tod_alarm(&mut self) -> TOD_ALARM_W<0> {
        TOD_ALARM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time-of-day Alarm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [toda](index.html) module"]
pub struct TODA_SPEC;
impl crate::RegisterSpec for TODA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [toda::R](R) reader structure"]
impl crate::Readable for TODA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [toda::W](W) writer structure"]
impl crate::Writable for TODA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TODA to value 0"]
impl crate::Resettable for TODA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
