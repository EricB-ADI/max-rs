#[doc = "Register `PNR` reader"]
pub struct R(crate::R<PNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PNR` writer"]
pub struct W(crate::W<PNR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PNR_SPEC>;
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
impl From<crate::W<PNR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PNR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTS` reader - Current sampled value of CTS IO"]
pub type CTS_R = crate::BitReader<bool>;
#[doc = "Field `RTS` reader - This bit controls the value to apply on the RTS IO. If set to 1, the RTS IO is set to high level. If set to 0, the RTS IO is set to low level."]
pub type RTS_R = crate::BitReader<bool>;
#[doc = "Field `RTS` writer - This bit controls the value to apply on the RTS IO. If set to 1, the RTS IO is set to high level. If set to 0, the RTS IO is set to low level."]
pub type RTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PNR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Current sampled value of CTS IO"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit controls the value to apply on the RTS IO. If set to 1, the RTS IO is set to high level. If set to 0, the RTS IO is set to low level."]
    #[inline(always)]
    pub fn rts(&self) -> RTS_R {
        RTS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - This bit controls the value to apply on the RTS IO. If set to 1, the RTS IO is set to high level. If set to 0, the RTS IO is set to low level."]
    #[inline(always)]
    #[must_use]
    pub fn rts(&mut self) -> RTS_W<1> {
        RTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pnr](index.html) module"]
pub struct PNR_SPEC;
impl crate::RegisterSpec for PNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pnr::R](R) reader structure"]
impl crate::Readable for PNR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pnr::W](W) writer structure"]
impl crate::Writable for PNR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PNR to value 0"]
impl crate::Resettable for PNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
