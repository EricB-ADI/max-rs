#[doc = "Register `IPOLO` reader"]
pub struct R(crate::R<IPOLO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPOLO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPOLO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPOLO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IPO_LIMITLO` reader - IPO Low Limit Trim."]
pub type IPO_LIMITLO_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - IPO Low Limit Trim."]
    #[inline(always)]
    pub fn ipo_limitlo(&self) -> IPO_LIMITLO_R {
        IPO_LIMITLO_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "IPO Low Trim System Initialization Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipolo](index.html) module"]
pub struct IPOLO_SPEC;
impl crate::RegisterSpec for IPOLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipolo::R](R) reader structure"]
impl crate::Readable for IPOLO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IPOLO to value 0"]
impl crate::Resettable for IPOLO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
