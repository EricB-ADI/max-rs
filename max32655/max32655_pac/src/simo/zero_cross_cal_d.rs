#[doc = "Register `ZERO_CROSS_CAL_D` reader"]
pub struct R(crate::R<ZERO_CROSS_CAL_D_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ZERO_CROSS_CAL_D_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ZERO_CROSS_CAL_D_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ZERO_CROSS_CAL_D_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ZXCALD` reader - Zero Cross Calibrartion Value VREGO_D"]
pub type ZXCALD_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Zero Cross Calibrartion Value VREGO_D"]
    #[inline(always)]
    pub fn zxcald(&self) -> ZXCALD_R {
        ZXCALD_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Zero Cross Calibration VERGO_D Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [zero_cross_cal_d](index.html) module"]
pub struct ZERO_CROSS_CAL_D_SPEC;
impl crate::RegisterSpec for ZERO_CROSS_CAL_D_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [zero_cross_cal_d::R](R) reader structure"]
impl crate::Readable for ZERO_CROSS_CAL_D_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ZERO_CROSS_CAL_D to value 0"]
impl crate::Resettable for ZERO_CROSS_CAL_D_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
