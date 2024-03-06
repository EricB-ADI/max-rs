#[doc = "Register `ECCERR` reader"]
pub struct R(crate::R<ECCERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECCERR` writer"]
pub struct W(crate::W<ECCERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECCERR_SPEC>;
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
impl From<crate::W<ECCERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECCERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAM` reader - ECC System RAM0 Error Flag. Write 1 to clear."]
pub type RAM_R = crate::BitReader<bool>;
#[doc = "Field `RAM` writer - ECC System RAM0 Error Flag. Write 1 to clear."]
pub type RAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECCERR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ECC System RAM0 Error Flag. Write 1 to clear."]
    #[inline(always)]
    pub fn ram(&self) -> RAM_R {
        RAM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC System RAM0 Error Flag. Write 1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn ram(&mut self) -> RAM_W<0> {
        RAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECC Error Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccerr](index.html) module"]
pub struct ECCERR_SPEC;
impl crate::RegisterSpec for ECCERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eccerr::R](R) reader structure"]
impl crate::Readable for ECCERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eccerr::W](W) writer structure"]
impl crate::Writable for ECCERR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECCERR to value 0"]
impl crate::Resettable for ECCERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
