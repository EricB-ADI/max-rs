#[doc = "Register `THRES_CMP` reader"]
pub struct R(crate::R<THRES_CMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THRES_CMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THRES_CMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THRES_CMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `THRES_CMP` writer"]
pub struct W(crate::W<THRES_CMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THRES_CMP_SPEC>;
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
impl From<crate::W<THRES_CMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THRES_CMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VCNTR_THRES_CNT` reader - Value used to determine 'low voltage' range"]
pub type VCNTR_THRES_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VCNTR_THRES_CNT` writer - Value used to determine 'low voltage' range"]
pub type VCNTR_THRES_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, THRES_CMP_SPEC, u8, u8, 7, O>;
#[doc = "Field `VCNTR_THRES_MASK` reader - Mask applied to threshold and vcount to determine if the device is in a low voltage range"]
pub type VCNTR_THRES_MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VCNTR_THRES_MASK` writer - Mask applied to threshold and vcount to determine if the device is in a low voltage range"]
pub type VCNTR_THRES_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, THRES_CMP_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Value used to determine 'low voltage' range"]
    #[inline(always)]
    pub fn vcntr_thres_cnt(&self) -> VCNTR_THRES_CNT_R {
        VCNTR_THRES_CNT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Mask applied to threshold and vcount to determine if the device is in a low voltage range"]
    #[inline(always)]
    pub fn vcntr_thres_mask(&self) -> VCNTR_THRES_MASK_R {
        VCNTR_THRES_MASK_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Value used to determine 'low voltage' range"]
    #[inline(always)]
    #[must_use]
    pub fn vcntr_thres_cnt(&mut self) -> VCNTR_THRES_CNT_W<0> {
        VCNTR_THRES_CNT_W::new(self)
    }
    #[doc = "Bits 8:14 - Mask applied to threshold and vcount to determine if the device is in a low voltage range"]
    #[inline(always)]
    #[must_use]
    pub fn vcntr_thres_mask(&mut self) -> VCNTR_THRES_MASK_W<8> {
        VCNTR_THRES_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Up Delay Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thres_cmp](index.html) module"]
pub struct THRES_CMP_SPEC;
impl crate::RegisterSpec for THRES_CMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [thres_cmp::R](R) reader structure"]
impl crate::Readable for THRES_CMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [thres_cmp::W](W) writer structure"]
impl crate::Writable for THRES_CMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets THRES_CMP to value 0"]
impl crate::Resettable for THRES_CMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
