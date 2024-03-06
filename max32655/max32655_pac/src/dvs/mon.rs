#[doc = "Register `MON` reader"]
pub struct R(crate::R<MON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MON` writer"]
pub struct W(crate::W<MON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MON_SPEC>;
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
impl From<crate::W<MON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLY` reader - Number of prescaled clocks between delay line samples"]
pub type DLY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DLY` writer - Number of prescaled clocks between delay line samples"]
pub type DLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MON_SPEC, u32, u32, 24, O>;
#[doc = "Field `PRE` reader - Number of clocks before DVS_MON_DLY is decremented"]
pub type PRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRE` writer - Number of clocks before DVS_MON_DLY is decremented"]
pub type PRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MON_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:23 - Number of prescaled clocks between delay line samples"]
    #[inline(always)]
    pub fn dly(&self) -> DLY_R {
        DLY_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - Number of clocks before DVS_MON_DLY is decremented"]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Number of prescaled clocks between delay line samples"]
    #[inline(always)]
    #[must_use]
    pub fn dly(&mut self) -> DLY_W<0> {
        DLY_W::new(self)
    }
    #[doc = "Bits 24:31 - Number of clocks before DVS_MON_DLY is decremented"]
    #[inline(always)]
    #[must_use]
    pub fn pre(&mut self) -> PRE_W<24> {
        PRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Monitor Delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mon](index.html) module"]
pub struct MON_SPEC;
impl crate::RegisterSpec for MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mon::R](R) reader structure"]
impl crate::Readable for MON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mon::W](W) writer structure"]
impl crate::Writable for MON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MON to value 0"]
impl crate::Resettable for MON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
