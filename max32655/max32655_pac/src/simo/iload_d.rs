#[doc = "Register `ILOAD_D` reader"]
pub struct R(crate::R<ILOAD_D_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ILOAD_D_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ILOAD_D_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ILOAD_D_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ILOADD` reader - Number of buck cycles that occur within the cycle clock"]
pub type ILOADD_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of buck cycles that occur within the cycle clock"]
    #[inline(always)]
    pub fn iloadd(&self) -> ILOADD_R {
        ILOADD_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Buck Cycle Count VREGO_D Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iload_d](index.html) module"]
pub struct ILOAD_D_SPEC;
impl crate::RegisterSpec for ILOAD_D_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iload_d::R](R) reader structure"]
impl crate::Readable for ILOAD_D_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ILOAD_D to value 0"]
impl crate::Resettable for ILOAD_D_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
