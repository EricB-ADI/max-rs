#[doc = "Register `RTC` reader"]
pub struct R(crate::R<RTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC` writer"]
pub struct W(crate::W<RTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_SPEC>;
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
impl From<crate::W<RTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `X1TRIM` reader - RTC X1 Trim."]
pub type X1TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `X1TRIM` writer - RTC X1 Trim."]
pub type X1TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTC_SPEC, u8, u8, 5, O>;
#[doc = "Field `X2TRIM` reader - RTC X2 Trim."]
pub type X2TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `X2TRIM` writer - RTC X2 Trim."]
pub type X2TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTC_SPEC, u8, u8, 5, O>;
#[doc = "Field `LOCK` reader - Lock."]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - Lock."]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 16:20 - RTC X1 Trim."]
    #[inline(always)]
    pub fn x1trim(&self) -> X1TRIM_R {
        X1TRIM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - RTC X2 Trim."]
    #[inline(always)]
    pub fn x2trim(&self) -> X2TRIM_R {
        X2TRIM_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Lock."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:20 - RTC X1 Trim."]
    #[inline(always)]
    #[must_use]
    pub fn x1trim(&mut self) -> X1TRIM_W<16> {
        X1TRIM_W::new(self)
    }
    #[doc = "Bits 21:25 - RTC X2 Trim."]
    #[inline(always)]
    #[must_use]
    pub fn x2trim(&mut self) -> X2TRIM_W<21> {
        X2TRIM_W::new(self)
    }
    #[doc = "Bit 31 - Lock."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Trim System Initialization Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc](index.html) module"]
pub struct RTC_SPEC;
impl crate::RegisterSpec for RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc::R](R) reader structure"]
impl crate::Readable for RTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc::W](W) writer structure"]
impl crate::Writable for RTC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC to value 0"]
impl crate::Resettable for RTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
