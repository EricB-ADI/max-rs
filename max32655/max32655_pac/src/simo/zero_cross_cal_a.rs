#[doc = "Register `ZERO_CROSS_CAL_A` reader"]
pub struct R(crate::R<ZERO_CROSS_CAL_A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ZERO_CROSS_CAL_A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ZERO_CROSS_CAL_A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ZERO_CROSS_CAL_A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ZXCALA` reader - Zero Cross Calibrartion Value VREGO_A"]
pub type ZXCALA_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Zero Cross Calibrartion Value VREGO_A"]
    #[inline(always)]
    pub fn zxcala(&self) -> ZXCALA_R {
        ZXCALA_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Zero Cross Calibration VERGO_A Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [zero_cross_cal_a](index.html) module"]
pub struct ZERO_CROSS_CAL_A_SPEC;
impl crate::RegisterSpec for ZERO_CROSS_CAL_A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [zero_cross_cal_a::R](R) reader structure"]
impl crate::Readable for ZERO_CROSS_CAL_A_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ZERO_CROSS_CAL_A to value 0"]
impl crate::Resettable for ZERO_CROSS_CAL_A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
