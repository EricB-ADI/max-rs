#[doc = "Register `DATA` reader"]
pub struct R(crate::R<DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Data. The content of this register is valid only when RNG_IS =1. When TNRG is disabled, read returns 0x0000 0000."]
pub type DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data. The content of this register is valid only when RNG_IS =1. When TNRG is disabled, read returns 0x0000 0000."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Data. The content of this register is valid only when RNG_IS = 1. When TRNG is disabled, read returns 0x0000 0000.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](index.html) module"]
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data::R](R) reader structure"]
impl crate::Readable for DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
