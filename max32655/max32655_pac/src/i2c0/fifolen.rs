#[doc = "Register `FIFOLEN` reader"]
pub struct R(crate::R<FIFOLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOLEN` writer"]
pub struct W(crate::W<FIFOLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOLEN_SPEC>;
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
impl From<crate::W<FIFOLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_DEPTH` reader - Receive FIFO Length."]
pub type RX_DEPTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_DEPTH` reader - Transmit FIFO Length."]
pub type TX_DEPTH_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Receive FIFO Length."]
    #[inline(always)]
    pub fn rx_depth(&self) -> RX_DEPTH_R {
        RX_DEPTH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Transmit FIFO Length."]
    #[inline(always)]
    pub fn tx_depth(&self) -> TX_DEPTH_R {
        TX_DEPTH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Configuration Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifolen](index.html) module"]
pub struct FIFOLEN_SPEC;
impl crate::RegisterSpec for FIFOLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifolen::R](R) reader structure"]
impl crate::Readable for FIFOLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifolen::W](W) writer structure"]
impl crate::Writable for FIFOLEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFOLEN to value 0"]
impl crate::Resettable for FIFOLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
