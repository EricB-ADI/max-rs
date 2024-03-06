#[doc = "Register `EVENTEN` reader"]
pub struct R(crate::R<EVENTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTEN` writer"]
pub struct W(crate::W<EVENTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTEN_SPEC>;
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
impl From<crate::W<EVENTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA` reader - Enable DMA event. When this bit is set, a DMA event will cause an RXEV event to wake the CPU from WFE sleep mode."]
pub type DMA_R = crate::BitReader<bool>;
#[doc = "Field `DMA` writer - Enable DMA event. When this bit is set, a DMA event will cause an RXEV event to wake the CPU from WFE sleep mode."]
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENTEN_SPEC, bool, O>;
#[doc = "Field `RX` reader - Enable RXEV pin event. When this bit is set, a logic high of GPIO1.8 will cause an RXEV event to wake the CPU from WFE sleep mode."]
pub type RX_R = crate::BitReader<bool>;
#[doc = "Field `RX` writer - Enable RXEV pin event. When this bit is set, a logic high of GPIO1.8 will cause an RXEV event to wake the CPU from WFE sleep mode."]
pub type RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENTEN_SPEC, bool, O>;
#[doc = "Field `TX` reader - Enable TXEV pin event. When this bit is set, TXEV event from the CPU is output to GPIO1.9."]
pub type TX_R = crate::BitReader<bool>;
#[doc = "Field `TX` writer - Enable TXEV pin event. When this bit is set, TXEV event from the CPU is output to GPIO1.9."]
pub type TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENTEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable DMA event. When this bit is set, a DMA event will cause an RXEV event to wake the CPU from WFE sleep mode."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable RXEV pin event. When this bit is set, a logic high of GPIO1.8 will cause an RXEV event to wake the CPU from WFE sleep mode."]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable TXEV pin event. When this bit is set, TXEV event from the CPU is output to GPIO1.9."]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DMA event. When this bit is set, a DMA event will cause an RXEV event to wake the CPU from WFE sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<0> {
        DMA_W::new(self)
    }
    #[doc = "Bit 1 - Enable RXEV pin event. When this bit is set, a logic high of GPIO1.8 will cause an RXEV event to wake the CPU from WFE sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RX_W<1> {
        RX_W::new(self)
    }
    #[doc = "Bit 2 - Enable TXEV pin event. When this bit is set, TXEV event from the CPU is output to GPIO1.9."]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<2> {
        TX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Enable Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eventen](index.html) module"]
pub struct EVENTEN_SPEC;
impl crate::RegisterSpec for EVENTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eventen::R](R) reader structure"]
impl crate::Readable for EVENTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eventen::W](W) writer structure"]
impl crate::Writable for EVENTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVENTEN to value 0"]
impl crate::Resettable for EVENTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
