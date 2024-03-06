#[doc = "Register `BUCK_ALERT_THR_D` reader"]
pub struct R(crate::R<BUCK_ALERT_THR_D_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUCK_ALERT_THR_D_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUCK_ALERT_THR_D_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUCK_ALERT_THR_D_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUCK_ALERT_THR_D` writer"]
pub struct W(crate::W<BUCK_ALERT_THR_D_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUCK_ALERT_THR_D_SPEC>;
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
impl From<crate::W<BUCK_ALERT_THR_D_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUCK_ALERT_THR_D_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUCKTHRD` reader - Threshold for ILOADD to generate the BUCK_ALERT"]
pub type BUCKTHRD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUCKTHRD` writer - Threshold for ILOADD to generate the BUCK_ALERT"]
pub type BUCKTHRD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BUCK_ALERT_THR_D_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Threshold for ILOADD to generate the BUCK_ALERT"]
    #[inline(always)]
    pub fn buckthrd(&self) -> BUCKTHRD_R {
        BUCKTHRD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Threshold for ILOADD to generate the BUCK_ALERT"]
    #[inline(always)]
    #[must_use]
    pub fn buckthrd(&mut self) -> BUCKTHRD_W<0> {
        BUCKTHRD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buck Cycle Count Alert VERGO_D Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buck_alert_thr_d](index.html) module"]
pub struct BUCK_ALERT_THR_D_SPEC;
impl crate::RegisterSpec for BUCK_ALERT_THR_D_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buck_alert_thr_d::R](R) reader structure"]
impl crate::Readable for BUCK_ALERT_THR_D_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buck_alert_thr_d::W](W) writer structure"]
impl crate::Writable for BUCK_ALERT_THR_D_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUCK_ALERT_THR_D to value 0"]
impl crate::Resettable for BUCK_ALERT_THR_D_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
