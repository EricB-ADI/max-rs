#[doc = "Register `AUTOCAL0` reader"]
pub struct R(crate::R<AUTOCAL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTOCAL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTOCAL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTOCAL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUTOCAL0` writer"]
pub struct W(crate::W<AUTOCAL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUTOCAL0_SPEC>;
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
impl From<crate::W<AUTOCAL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUTOCAL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACEN` reader - Auto-calibration Enable."]
pub type ACEN_R = crate::BitReader<ACEN_A>;
#[doc = "Auto-calibration Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACEN_A {
    #[doc = "0: Disabled."]
    DIS = 0,
    #[doc = "1: Enabled."]
    EN = 1,
}
impl From<ACEN_A> for bool {
    #[inline(always)]
    fn from(variant: ACEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ACEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACEN_A {
        match self.bits {
            false => ACEN_A::DIS,
            true => ACEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ACEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ACEN_A::EN
    }
}
#[doc = "Field `ACEN` writer - Auto-calibration Enable."]
pub type ACEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTOCAL0_SPEC, ACEN_A, O>;
impl<'a, const O: u8> ACEN_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ACEN_A::DIS)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ACEN_A::EN)
    }
}
#[doc = "Field `ACRUN` reader - Autocalibration Run."]
pub type ACRUN_R = crate::BitReader<ACRUN_A>;
#[doc = "Autocalibration Run.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACRUN_A {
    #[doc = "0: Not Running."]
    NOT = 0,
    #[doc = "1: Running."]
    RUN = 1,
}
impl From<ACRUN_A> for bool {
    #[inline(always)]
    fn from(variant: ACRUN_A) -> Self {
        variant as u8 != 0
    }
}
impl ACRUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACRUN_A {
        match self.bits {
            false => ACRUN_A::NOT,
            true => ACRUN_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == ACRUN_A::NOT
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == ACRUN_A::RUN
    }
}
#[doc = "Field `ACRUN` writer - Autocalibration Run."]
pub type ACRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTOCAL0_SPEC, ACRUN_A, O>;
impl<'a, const O: u8> ACRUN_W<'a, O> {
    #[doc = "Not Running."]
    #[inline(always)]
    pub fn not(self) -> &'a mut W {
        self.variant(ACRUN_A::NOT)
    }
    #[doc = "Running."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(ACRUN_A::RUN)
    }
}
#[doc = "Field `LDTRM` reader - Load Trim."]
pub type LDTRM_R = crate::BitReader<bool>;
#[doc = "Field `LDTRM` writer - Load Trim."]
pub type LDTRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTOCAL0_SPEC, bool, O>;
#[doc = "Field `GAININV` reader - Invert Gain."]
pub type GAININV_R = crate::BitReader<GAININV_A>;
#[doc = "Invert Gain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GAININV_A {
    #[doc = "0: Not Running."]
    NOT = 0,
    #[doc = "1: Running."]
    RUN = 1,
}
impl From<GAININV_A> for bool {
    #[inline(always)]
    fn from(variant: GAININV_A) -> Self {
        variant as u8 != 0
    }
}
impl GAININV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAININV_A {
        match self.bits {
            false => GAININV_A::NOT,
            true => GAININV_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == GAININV_A::NOT
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == GAININV_A::RUN
    }
}
#[doc = "Field `GAININV` writer - Invert Gain."]
pub type GAININV_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTOCAL0_SPEC, GAININV_A, O>;
impl<'a, const O: u8> GAININV_W<'a, O> {
    #[doc = "Not Running."]
    #[inline(always)]
    pub fn not(self) -> &'a mut W {
        self.variant(GAININV_A::NOT)
    }
    #[doc = "Running."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(GAININV_A::RUN)
    }
}
#[doc = "Field `ATOMIC` reader - Atomic mode."]
pub type ATOMIC_R = crate::BitReader<ATOMIC_A>;
#[doc = "Atomic mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATOMIC_A {
    #[doc = "0: Not Running."]
    NOT = 0,
    #[doc = "1: Running."]
    RUN = 1,
}
impl From<ATOMIC_A> for bool {
    #[inline(always)]
    fn from(variant: ATOMIC_A) -> Self {
        variant as u8 != 0
    }
}
impl ATOMIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATOMIC_A {
        match self.bits {
            false => ATOMIC_A::NOT,
            true => ATOMIC_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == ATOMIC_A::NOT
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == ATOMIC_A::RUN
    }
}
#[doc = "Field `ATOMIC` writer - Atomic mode."]
pub type ATOMIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTOCAL0_SPEC, ATOMIC_A, O>;
impl<'a, const O: u8> ATOMIC_W<'a, O> {
    #[doc = "Not Running."]
    #[inline(always)]
    pub fn not(self) -> &'a mut W {
        self.variant(ATOMIC_A::NOT)
    }
    #[doc = "Running."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(ATOMIC_A::RUN)
    }
}
#[doc = "Field `MU` reader - MU value."]
pub type MU_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MU` writer - MU value."]
pub type MU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUTOCAL0_SPEC, u16, u16, 12, O>;
#[doc = "Field `HIRC96MACTMROUT` reader - HIRC96M Trim Value."]
pub type HIRC96MACTMROUT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HIRC96MACTMROUT` writer - HIRC96M Trim Value."]
pub type HIRC96MACTMROUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AUTOCAL0_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bit 0 - Auto-calibration Enable."]
    #[inline(always)]
    pub fn acen(&self) -> ACEN_R {
        ACEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Autocalibration Run."]
    #[inline(always)]
    pub fn acrun(&self) -> ACRUN_R {
        ACRUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Load Trim."]
    #[inline(always)]
    pub fn ldtrm(&self) -> LDTRM_R {
        LDTRM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Invert Gain."]
    #[inline(always)]
    pub fn gaininv(&self) -> GAININV_R {
        GAININV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Atomic mode."]
    #[inline(always)]
    pub fn atomic(&self) -> ATOMIC_R {
        ATOMIC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:19 - MU value."]
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 23:31 - HIRC96M Trim Value."]
    #[inline(always)]
    pub fn hirc96mactmrout(&self) -> HIRC96MACTMROUT_R {
        HIRC96MACTMROUT_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Auto-calibration Enable."]
    #[inline(always)]
    #[must_use]
    pub fn acen(&mut self) -> ACEN_W<0> {
        ACEN_W::new(self)
    }
    #[doc = "Bit 1 - Autocalibration Run."]
    #[inline(always)]
    #[must_use]
    pub fn acrun(&mut self) -> ACRUN_W<1> {
        ACRUN_W::new(self)
    }
    #[doc = "Bit 2 - Load Trim."]
    #[inline(always)]
    #[must_use]
    pub fn ldtrm(&mut self) -> LDTRM_W<2> {
        LDTRM_W::new(self)
    }
    #[doc = "Bit 3 - Invert Gain."]
    #[inline(always)]
    #[must_use]
    pub fn gaininv(&mut self) -> GAININV_W<3> {
        GAININV_W::new(self)
    }
    #[doc = "Bit 4 - Atomic mode."]
    #[inline(always)]
    #[must_use]
    pub fn atomic(&mut self) -> ATOMIC_W<4> {
        ATOMIC_W::new(self)
    }
    #[doc = "Bits 8:19 - MU value."]
    #[inline(always)]
    #[must_use]
    pub fn mu(&mut self) -> MU_W<8> {
        MU_W::new(self)
    }
    #[doc = "Bits 23:31 - HIRC96M Trim Value."]
    #[inline(always)]
    #[must_use]
    pub fn hirc96mactmrout(&mut self) -> HIRC96MACTMROUT_W<23> {
        HIRC96MACTMROUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Automatic Calibration 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autocal0](index.html) module"]
pub struct AUTOCAL0_SPEC;
impl crate::RegisterSpec for AUTOCAL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [autocal0::R](R) reader structure"]
impl crate::Readable for AUTOCAL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [autocal0::W](W) writer structure"]
impl crate::Writable for AUTOCAL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUTOCAL0 to value 0"]
impl crate::Resettable for AUTOCAL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
