#[doc = "Register `ADJ_DWN` reader"]
pub struct R(crate::R<ADJ_DWN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADJ_DWN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADJ_DWN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADJ_DWN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADJ_DWN` writer"]
pub struct W(crate::W<ADJ_DWN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADJ_DWN_SPEC>;
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
impl From<crate::W<ADJ_DWN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADJ_DWN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLY` reader - Number of prescaled clocks between updates of the adjustment delay counter"]
pub type DLY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DLY` writer - Number of prescaled clocks between updates of the adjustment delay counter"]
pub type DLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADJ_DWN_SPEC, u16, u16, 16, O>;
#[doc = "Field `PRE` reader - Number of clocks before DVS_ADJ_DWN_DLY is decremented"]
pub type PRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRE` writer - Number of clocks before DVS_ADJ_DWN_DLY is decremented"]
pub type PRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADJ_DWN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:15 - Number of prescaled clocks between updates of the adjustment delay counter"]
    #[inline(always)]
    pub fn dly(&self) -> DLY_R {
        DLY_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Number of clocks before DVS_ADJ_DWN_DLY is decremented"]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of prescaled clocks between updates of the adjustment delay counter"]
    #[inline(always)]
    #[must_use]
    pub fn dly(&mut self) -> DLY_W<0> {
        DLY_W::new(self)
    }
    #[doc = "Bits 16:23 - Number of clocks before DVS_ADJ_DWN_DLY is decremented"]
    #[inline(always)]
    #[must_use]
    pub fn pre(&mut self) -> PRE_W<16> {
        PRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Down Delay Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adj_dwn](index.html) module"]
pub struct ADJ_DWN_SPEC;
impl crate::RegisterSpec for ADJ_DWN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adj_dwn::R](R) reader structure"]
impl crate::Readable for ADJ_DWN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adj_dwn::W](W) writer structure"]
impl crate::Writable for ADJ_DWN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADJ_DWN to value 0"]
impl crate::Resettable for ADJ_DWN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
