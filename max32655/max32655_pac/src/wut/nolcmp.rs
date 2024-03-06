#[doc = "Register `NOLCMP` reader"]
pub struct R(crate::R<NOLCMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NOLCMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NOLCMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NOLCMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NOLCMP` writer"]
pub struct W(crate::W<NOLCMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NOLCMP_SPEC>;
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
impl From<crate::W<NOLCMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NOLCMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NOLLCMP` reader - Non-overlapping Low Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A and next rising edge of PWM output 0A'."]
pub type NOLLCMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NOLLCMP` writer - Non-overlapping Low Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A and next rising edge of PWM output 0A'."]
pub type NOLLCMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NOLCMP_SPEC, u8, u8, 8, O>;
#[doc = "Field `NOLHCMP` reader - Non-overlapping High Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A' and next rising edge of PWM output 0A."]
pub type NOLHCMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NOLHCMP` writer - Non-overlapping High Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A' and next rising edge of PWM output 0A."]
pub type NOLHCMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NOLCMP_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Non-overlapping Low Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A and next rising edge of PWM output 0A'."]
    #[inline(always)]
    pub fn nollcmp(&self) -> NOLLCMP_R {
        NOLLCMP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Non-overlapping High Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A' and next rising edge of PWM output 0A."]
    #[inline(always)]
    pub fn nolhcmp(&self) -> NOLHCMP_R {
        NOLHCMP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Non-overlapping Low Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A and next rising edge of PWM output 0A'."]
    #[inline(always)]
    #[must_use]
    pub fn nollcmp(&mut self) -> NOLLCMP_W<0> {
        NOLLCMP_W::new(self)
    }
    #[doc = "Bits 8:15 - Non-overlapping High Compare. The 8-bit timer count value of non-overlapping time between falling edge of PWM output 0A' and next rising edge of PWM output 0A."]
    #[inline(always)]
    #[must_use]
    pub fn nolhcmp(&mut self) -> NOLHCMP_W<8> {
        NOLHCMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Non-Overlapping Compare Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nolcmp](index.html) module"]
pub struct NOLCMP_SPEC;
impl crate::RegisterSpec for NOLCMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nolcmp::R](R) reader structure"]
impl crate::Readable for NOLCMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nolcmp::W](W) writer structure"]
impl crate::Writable for NOLCMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NOLCMP to value 0"]
impl crate::Resettable for NOLCMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
