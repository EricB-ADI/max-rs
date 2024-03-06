#[doc = "Register `PWM` reader"]
pub struct R(crate::R<PWM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM` writer"]
pub struct W(crate::W<PWM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_SPEC>;
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
impl From<crate::W<PWM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM` reader - Timer PWM Match: In PWM Mode, this field sets the count value for the first transition period of the PWM cycle. At the end of the cycle where CNT equals PWM, the PWM output transitions to the second period of the PWM cycle. The second PWM period count is stored in the CMP register. The value set for PWM must me less than the value set in CMP for PWM mode operation. Timer Capture Value: In Capture, Compare, and Capture/Compare modes, this field is used to store the CNT value when a Capture, Compare, or Capture/Compare event occurs."]
pub type PWM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PWM` writer - Timer PWM Match: In PWM Mode, this field sets the count value for the first transition period of the PWM cycle. At the end of the cycle where CNT equals PWM, the PWM output transitions to the second period of the PWM cycle. The second PWM period count is stored in the CMP register. The value set for PWM must me less than the value set in CMP for PWM mode operation. Timer Capture Value: In Capture, Compare, and Capture/Compare modes, this field is used to store the CNT value when a Capture, Compare, or Capture/Compare event occurs."]
pub type PWM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWM_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Timer PWM Match: In PWM Mode, this field sets the count value for the first transition period of the PWM cycle. At the end of the cycle where CNT equals PWM, the PWM output transitions to the second period of the PWM cycle. The second PWM period count is stored in the CMP register. The value set for PWM must me less than the value set in CMP for PWM mode operation. Timer Capture Value: In Capture, Compare, and Capture/Compare modes, this field is used to store the CNT value when a Capture, Compare, or Capture/Compare event occurs."]
    #[inline(always)]
    pub fn pwm(&self) -> PWM_R {
        PWM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer PWM Match: In PWM Mode, this field sets the count value for the first transition period of the PWM cycle. At the end of the cycle where CNT equals PWM, the PWM output transitions to the second period of the PWM cycle. The second PWM period count is stored in the CMP register. The value set for PWM must me less than the value set in CMP for PWM mode operation. Timer Capture Value: In Capture, Compare, and Capture/Compare modes, this field is used to store the CNT value when a Capture, Compare, or Capture/Compare event occurs."]
    #[inline(always)]
    #[must_use]
    pub fn pwm(&mut self) -> PWM_W<0> {
        PWM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer PWM Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm](index.html) module"]
pub struct PWM_SPEC;
impl crate::RegisterSpec for PWM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm::R](R) reader structure"]
impl crate::Readable for PWM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm::W](W) writer structure"]
impl crate::Writable for PWM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWM to value 0"]
impl crate::Resettable for PWM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
