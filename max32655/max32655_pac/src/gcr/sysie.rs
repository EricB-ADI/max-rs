#[doc = "Register `SYSIE` reader"]
pub struct R(crate::R<SYSIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSIE` writer"]
pub struct W(crate::W<SYSIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSIE_SPEC>;
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
impl From<crate::W<SYSIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICEUNLOCK` reader - ARM ICE Unlock Interrupt Enable."]
pub type ICEUNLOCK_R = crate::BitReader<ICEUNLOCK_A>;
#[doc = "ARM ICE Unlock Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICEUNLOCK_A {
    #[doc = "0: disabled."]
    DIS = 0,
    #[doc = "1: enabled."]
    EN = 1,
}
impl From<ICEUNLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: ICEUNLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl ICEUNLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICEUNLOCK_A {
        match self.bits {
            false => ICEUNLOCK_A::DIS,
            true => ICEUNLOCK_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ICEUNLOCK_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ICEUNLOCK_A::EN
    }
}
#[doc = "Field `ICEUNLOCK` writer - ARM ICE Unlock Interrupt Enable."]
pub type ICEUNLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSIE_SPEC, ICEUNLOCK_A, O>;
impl<'a, const O: u8> ICEUNLOCK_W<'a, O> {
    #[doc = "disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ICEUNLOCK_A::DIS)
    }
    #[doc = "enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ICEUNLOCK_A::EN)
    }
}
impl R {
    #[doc = "Bit 0 - ARM ICE Unlock Interrupt Enable."]
    #[inline(always)]
    pub fn iceunlock(&self) -> ICEUNLOCK_R {
        ICEUNLOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ARM ICE Unlock Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn iceunlock(&mut self) -> ICEUNLOCK_W<0> {
        ICEUNLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Status Interrupt Enable Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysie](index.html) module"]
pub struct SYSIE_SPEC;
impl crate::RegisterSpec for SYSIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysie::R](R) reader structure"]
impl crate::Readable for SYSIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysie::W](W) writer structure"]
impl crate::Writable for SYSIE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSIE to value 0"]
impl crate::Resettable for SYSIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
