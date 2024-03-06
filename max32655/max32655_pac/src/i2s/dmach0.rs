#[doc = "Register `DMACH0` reader"]
pub struct R(crate::R<DMACH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACH0` writer"]
pub struct W(crate::W<DMACH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACH0_SPEC>;
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
impl From<crate::W<DMACH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_TX_THD_VAL` reader - TX FIFO Level DMA Trigger."]
pub type DMA_TX_THD_VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_TX_THD_VAL` writer - TX FIFO Level DMA Trigger."]
pub type DMA_TX_THD_VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACH0_SPEC, u8, u8, 7, O>;
#[doc = "Field `DMA_TX_EN` reader - TX DMA channel enable."]
pub type DMA_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA_TX_EN` writer - TX DMA channel enable."]
pub type DMA_TX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACH0_SPEC, bool, O>;
#[doc = "Field `DMA_RX_THD_VAL` reader - RX FIFO Level DMA Trigger."]
pub type DMA_RX_THD_VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_RX_THD_VAL` writer - RX FIFO Level DMA Trigger."]
pub type DMA_RX_THD_VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACH0_SPEC, u8, u8, 7, O>;
#[doc = "Field `DMA_RX_EN` reader - RX DMA channel enable."]
pub type DMA_RX_EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA_RX_EN` writer - RX DMA channel enable."]
pub type DMA_RX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACH0_SPEC, bool, O>;
#[doc = "Field `TX_LVL` reader - Number of data word in the TX FIFO."]
pub type TX_LVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_LVL` writer - Number of data word in the TX FIFO."]
pub type TX_LVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACH0_SPEC, u8, u8, 8, O>;
#[doc = "Field `RX_LVL` reader - Number of data word in the RX FIFO."]
pub type RX_LVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_LVL` writer - Number of data word in the RX FIFO."]
pub type RX_LVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACH0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:6 - TX FIFO Level DMA Trigger."]
    #[inline(always)]
    pub fn dma_tx_thd_val(&self) -> DMA_TX_THD_VAL_R {
        DMA_TX_THD_VAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - TX DMA channel enable."]
    #[inline(always)]
    pub fn dma_tx_en(&self) -> DMA_TX_EN_R {
        DMA_TX_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - RX FIFO Level DMA Trigger."]
    #[inline(always)]
    pub fn dma_rx_thd_val(&self) -> DMA_RX_THD_VAL_R {
        DMA_RX_THD_VAL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - RX DMA channel enable."]
    #[inline(always)]
    pub fn dma_rx_en(&self) -> DMA_RX_EN_R {
        DMA_RX_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Number of data word in the TX FIFO."]
    #[inline(always)]
    pub fn tx_lvl(&self) -> TX_LVL_R {
        TX_LVL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Number of data word in the RX FIFO."]
    #[inline(always)]
    pub fn rx_lvl(&self) -> RX_LVL_R {
        RX_LVL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - TX FIFO Level DMA Trigger."]
    #[inline(always)]
    #[must_use]
    pub fn dma_tx_thd_val(&mut self) -> DMA_TX_THD_VAL_W<0> {
        DMA_TX_THD_VAL_W::new(self)
    }
    #[doc = "Bit 7 - TX DMA channel enable."]
    #[inline(always)]
    #[must_use]
    pub fn dma_tx_en(&mut self) -> DMA_TX_EN_W<7> {
        DMA_TX_EN_W::new(self)
    }
    #[doc = "Bits 8:14 - RX FIFO Level DMA Trigger."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_thd_val(&mut self) -> DMA_RX_THD_VAL_W<8> {
        DMA_RX_THD_VAL_W::new(self)
    }
    #[doc = "Bit 15 - RX DMA channel enable."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_en(&mut self) -> DMA_RX_EN_W<15> {
        DMA_RX_EN_W::new(self)
    }
    #[doc = "Bits 16:23 - Number of data word in the TX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn tx_lvl(&mut self) -> TX_LVL_W<16> {
        TX_LVL_W::new(self)
    }
    #[doc = "Bits 24:31 - Number of data word in the RX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn rx_lvl(&mut self) -> RX_LVL_W<24> {
        RX_LVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmach0](index.html) module"]
pub struct DMACH0_SPEC;
impl crate::RegisterSpec for DMACH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmach0::R](R) reader structure"]
impl crate::Readable for DMACH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmach0::W](W) writer structure"]
impl crate::Writable for DMACH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACH0 to value 0"]
impl crate::Resettable for DMACH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
