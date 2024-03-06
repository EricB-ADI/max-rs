#[doc = "Register `ILOAD_C` reader"]
pub struct R(crate::R<ILOAD_C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ILOAD_C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ILOAD_C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ILOAD_C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ILOADC` reader - Number of buck cycles that occur within the cycle clock"]
pub type ILOADC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of buck cycles that occur within the cycle clock"]
    #[inline(always)]
    pub fn iloadc(&self) -> ILOADC_R {
        ILOADC_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Buck Cycle Count VREGO_C Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iload_c](index.html) module"]
pub struct ILOAD_C_SPEC;
impl crate::RegisterSpec for ILOAD_C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iload_c::R](R) reader structure"]
impl crate::Readable for ILOAD_C_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ILOAD_C to value 0"]
impl crate::Resettable for ILOAD_C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
