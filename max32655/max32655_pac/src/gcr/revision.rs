#[doc = "Register `REVISION` reader"]
pub struct R(crate::R<REVISION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REVISION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REVISION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REVISION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REVISION` reader - Manufacturer Chip Revision."]
pub type REVISION_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Manufacturer Chip Revision."]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Revision Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [revision](index.html) module"]
pub struct REVISION_SPEC;
impl crate::RegisterSpec for REVISION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [revision::R](R) reader structure"]
impl crate::Readable for REVISION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets REVISION to value 0"]
impl crate::Resettable for REVISION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
