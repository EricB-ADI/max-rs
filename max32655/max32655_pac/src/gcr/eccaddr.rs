#[doc = "Register `ECCADDR` reader"]
pub struct R(crate::R<ECCADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECCADDR` writer"]
pub struct W(crate::W<ECCADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECCADDR_SPEC>;
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
impl From<crate::W<ECCADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECCADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECCERRAD` reader - ECC Error Address."]
pub type ECCERRAD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ECCERRAD` writer - ECC Error Address."]
pub type ECCERRAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECCADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - ECC Error Address."]
    #[inline(always)]
    pub fn eccerrad(&self) -> ECCERRAD_R {
        ECCERRAD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ECC Error Address."]
    #[inline(always)]
    #[must_use]
    pub fn eccerrad(&mut self) -> ECCERRAD_W<0> {
        ECCERRAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECC Error Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccaddr](index.html) module"]
pub struct ECCADDR_SPEC;
impl crate::RegisterSpec for ECCADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eccaddr::R](R) reader structure"]
impl crate::Readable for ECCADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eccaddr::W](W) writer structure"]
impl crate::Writable for ECCADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECCADDR to value 0"]
impl crate::Resettable for ECCADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
