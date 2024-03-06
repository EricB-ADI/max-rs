#[doc = "Register `ZERO_CROSS_CAL_B` reader"]
pub struct R(crate::R<ZERO_CROSS_CAL_B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ZERO_CROSS_CAL_B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ZERO_CROSS_CAL_B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ZERO_CROSS_CAL_B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ZXCALB` reader - Zero Cross Calibrartion Value VREGO_B"]
pub type ZXCALB_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Zero Cross Calibrartion Value VREGO_B"]
    #[inline(always)]
    pub fn zxcalb(&self) -> ZXCALB_R {
        ZXCALB_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Zero Cross Calibration VERGO_B Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [zero_cross_cal_b](index.html) module"]
pub struct ZERO_CROSS_CAL_B_SPEC;
impl crate::RegisterSpec for ZERO_CROSS_CAL_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [zero_cross_cal_b::R](R) reader structure"]
impl crate::Readable for ZERO_CROSS_CAL_B_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ZERO_CROSS_CAL_B to value 0"]
impl crate::Resettable for ZERO_CROSS_CAL_B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
