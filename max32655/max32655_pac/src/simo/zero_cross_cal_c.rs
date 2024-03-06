#[doc = "Register `ZERO_CROSS_CAL_C` reader"]
pub struct R(crate::R<ZERO_CROSS_CAL_C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ZERO_CROSS_CAL_C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ZERO_CROSS_CAL_C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ZERO_CROSS_CAL_C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ZXCALC` reader - Zero Cross Calibrartion Value VREGO_C"]
pub type ZXCALC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Zero Cross Calibrartion Value VREGO_C"]
    #[inline(always)]
    pub fn zxcalc(&self) -> ZXCALC_R {
        ZXCALC_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Zero Cross Calibration VERGO_C Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [zero_cross_cal_c](index.html) module"]
pub struct ZERO_CROSS_CAL_C_SPEC;
impl crate::RegisterSpec for ZERO_CROSS_CAL_C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [zero_cross_cal_c::R](R) reader structure"]
impl crate::Readable for ZERO_CROSS_CAL_C_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ZERO_CROSS_CAL_C to value 0"]
impl crate::Resettable for ZERO_CROSS_CAL_C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
