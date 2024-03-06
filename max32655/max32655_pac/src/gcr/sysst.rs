#[doc = "Register `SYSST` reader"]
pub struct R(crate::R<SYSST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSST` writer"]
pub struct W(crate::W<SYSST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSST_SPEC>;
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
impl From<crate::W<SYSST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICELOCK` reader - ARM ICE Lock Status."]
pub type ICELOCK_R = crate::BitReader<ICELOCK_A>;
#[doc = "ARM ICE Lock Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICELOCK_A {
    #[doc = "0: ICE is unlocked."]
    UNLOCKED = 0,
    #[doc = "1: ICE is locked."]
    LOCKED = 1,
}
impl From<ICELOCK_A> for bool {
    #[inline(always)]
    fn from(variant: ICELOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl ICELOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICELOCK_A {
        match self.bits {
            false => ICELOCK_A::UNLOCKED,
            true => ICELOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == ICELOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == ICELOCK_A::LOCKED
    }
}
#[doc = "Field `ICELOCK` writer - ARM ICE Lock Status."]
pub type ICELOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSST_SPEC, ICELOCK_A, O>;
impl<'a, const O: u8> ICELOCK_W<'a, O> {
    #[doc = "ICE is unlocked."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(ICELOCK_A::UNLOCKED)
    }
    #[doc = "ICE is locked."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(ICELOCK_A::LOCKED)
    }
}
impl R {
    #[doc = "Bit 0 - ARM ICE Lock Status."]
    #[inline(always)]
    pub fn icelock(&self) -> ICELOCK_R {
        ICELOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ARM ICE Lock Status."]
    #[inline(always)]
    #[must_use]
    pub fn icelock(&mut self) -> ICELOCK_W<0> {
        ICELOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysst](index.html) module"]
pub struct SYSST_SPEC;
impl crate::RegisterSpec for SYSST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysst::R](R) reader structure"]
impl crate::Readable for SYSST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysst::W](W) writer structure"]
impl crate::Writable for SYSST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSST to value 0"]
impl crate::Resettable for SYSST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
