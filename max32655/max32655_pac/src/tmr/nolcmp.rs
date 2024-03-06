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
#[doc = "Field `LO_A` reader - Non-Overlapping Low Compare value for Timer A controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
pub type LO_A_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LO_A` writer - Non-Overlapping Low Compare value for Timer A controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
pub type LO_A_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NOLCMP_SPEC, u8, u8, 8, O>;
#[doc = "Field `HI_A` reader - Non-Overlapping High Compare value for Timer A controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
pub type HI_A_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HI_A` writer - Non-Overlapping High Compare value for Timer A controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
pub type HI_A_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NOLCMP_SPEC, u8, u8, 8, O>;
#[doc = "Field `LO_B` reader - Non-Overlapping Low Compare value for Timer B controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
pub type LO_B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LO_B` writer - Non-Overlapping Low Compare value for Timer B controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
pub type LO_B_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NOLCMP_SPEC, u8, u8, 8, O>;
#[doc = "Field `HI_B` reader - Non-Overlapping High Compare value for Timer B controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
pub type HI_B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HI_B` writer - Non-Overlapping High Compare value for Timer B controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
pub type HI_B_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NOLCMP_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Non-Overlapping Low Compare value for Timer A controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
    #[inline(always)]
    pub fn lo_a(&self) -> LO_A_R {
        LO_A_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Non-Overlapping High Compare value for Timer A controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
    #[inline(always)]
    pub fn hi_a(&self) -> HI_A_R {
        HI_A_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Non-Overlapping Low Compare value for Timer B controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
    #[inline(always)]
    pub fn lo_b(&self) -> LO_B_R {
        LO_B_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Non-Overlapping High Compare value for Timer B controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
    #[inline(always)]
    pub fn hi_b(&self) -> HI_B_R {
        HI_B_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Non-Overlapping Low Compare value for Timer A controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
    #[inline(always)]
    #[must_use]
    pub fn lo_a(&mut self) -> LO_A_W<0> {
        LO_A_W::new(self)
    }
    #[doc = "Bits 8:15 - Non-Overlapping High Compare value for Timer A controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
    #[inline(always)]
    #[must_use]
    pub fn hi_a(&mut self) -> HI_A_W<8> {
        HI_A_W::new(self)
    }
    #[doc = "Bits 16:23 - Non-Overlapping Low Compare value for Timer B controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
    #[inline(always)]
    #[must_use]
    pub fn lo_b(&mut self) -> LO_B_W<16> {
        LO_B_W::new(self)
    }
    #[doc = "Bits 24:31 - Non-Overlapping High Compare value for Timer B controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
    #[inline(always)]
    #[must_use]
    pub fn hi_b(&mut self) -> HI_B_W<24> {
        HI_B_W::new(self)
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
