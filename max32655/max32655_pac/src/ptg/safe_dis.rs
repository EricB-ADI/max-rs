#[doc = "Register `SAFE_DIS` writer"]
pub struct W(crate::W<SAFE_DIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAFE_DIS_SPEC>;
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
impl From<crate::W<SAFE_DIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAFE_DIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PT0` writer - "]
pub type PT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAFE_DIS_SPEC, bool, O>;
#[doc = "Field `PT1` writer - "]
pub type PT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAFE_DIS_SPEC, bool, O>;
#[doc = "Field `PT2` writer - "]
pub type PT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAFE_DIS_SPEC, bool, O>;
#[doc = "Field `PT3` writer - "]
pub type PT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAFE_DIS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pt0(&mut self) -> PT0_W<0> {
        PT0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pt1(&mut self) -> PT1_W<1> {
        PT1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pt2(&mut self) -> PT2_W<2> {
        PT2_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pt3(&mut self) -> PT3_W<3> {
        PT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pulse Train Global Safe Disable.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [safe_dis](index.html) module"]
pub struct SAFE_DIS_SPEC;
impl crate::RegisterSpec for SAFE_DIS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [safe_dis::W](W) writer structure"]
impl crate::Writable for SAFE_DIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAFE_DIS to value 0"]
impl crate::Resettable for SAFE_DIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
