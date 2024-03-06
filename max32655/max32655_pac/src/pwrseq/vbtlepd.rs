#[doc = "Register `VBTLEPD` reader"]
pub struct R(crate::R<VBTLEPD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VBTLEPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VBTLEPD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VBTLEPD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VBTLEPD` writer"]
pub struct W(crate::W<VBTLEPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VBTLEPD_SPEC>;
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
impl From<crate::W<VBTLEPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VBTLEPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BTLE` reader - Power Down SIMO VREGO_D."]
pub type BTLE_R = crate::BitReader<bool>;
#[doc = "Field `BTLE` writer - Power Down SIMO VREGO_D."]
pub type BTLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VBTLEPD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Power Down SIMO VREGO_D."]
    #[inline(always)]
    pub fn btle(&self) -> BTLE_R {
        BTLE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Power Down SIMO VREGO_D."]
    #[inline(always)]
    #[must_use]
    pub fn btle(&mut self) -> BTLE_W<1> {
        BTLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low-Power VBTLE Power Down Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vbtlepd](index.html) module"]
pub struct VBTLEPD_SPEC;
impl crate::RegisterSpec for VBTLEPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vbtlepd::R](R) reader structure"]
impl crate::Readable for VBTLEPD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vbtlepd::W](W) writer structure"]
impl crate::Writable for VBTLEPD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTLEPD to value 0"]
impl crate::Resettable for VBTLEPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
