#[doc = "Register `CTRL0CH0` reader"]
pub struct R(crate::R<CTRL0CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL0CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL0CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL0CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL0CH0` writer"]
pub struct W(crate::W<CTRL0CH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL0CH0_SPEC>;
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
impl From<crate::W<CTRL0CH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL0CH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSB_FIRST` reader - LSB Transmit Receive First."]
pub type LSB_FIRST_R = crate::BitReader<bool>;
#[doc = "Field `LSB_FIRST` writer - LSB Transmit Receive First."]
pub type LSB_FIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL0CH0_SPEC, bool, O>;
#[doc = "Field `PDM_FILT` reader - PDM Filter."]
pub type PDM_FILT_R = crate::BitReader<bool>;
#[doc = "Field `PDM_FILT` writer - PDM Filter."]
pub type PDM_FILT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL0CH0_SPEC, bool, O>;
#[doc = "Field `PDM_EN` reader - PDM Enable."]
pub type PDM_EN_R = crate::BitReader<bool>;
#[doc = "Field `PDM_EN` writer - PDM Enable."]
pub type PDM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL0CH0_SPEC, bool, O>;
#[doc = "Field `USEDDR` reader - DDR."]
pub type USEDDR_R = crate::BitReader<bool>;
#[doc = "Field `USEDDR` writer - DDR."]
pub type USEDDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL0CH0_SPEC, bool, O>;
#[doc = "Field `PDM_INV` reader - Invert PDM."]
pub type PDM_INV_R = crate::BitReader<bool>;
#[doc = "Field `PDM_INV` writer - Invert PDM."]
pub type PDM_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL0CH0_SPEC, bool, O>;
#[doc = "Field `CH_MODE` reader - SCK Select."]
pub type CH_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH_MODE` writer - SCK Select."]
pub type CH_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL0CH0_SPEC, u8, u8, 2, O>;
#[doc = "Field `WS_POL` reader - WS polarity select."]
pub type WS_POL_R = crate::BitReader<bool>;
#[doc = "Field `WS_POL` writer - WS polarity select."]
pub type WS_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL0CH0_SPEC, bool, O>;
#[doc = "Field `MSB_LOC` reader - MSB location."]
pub type MSB_LOC_R = crate::BitReader<bool>;
#[doc = "Field `ALIGN` reader - Align to MSB or LSB."]
pub type ALIGN_R = crate::BitReader<bool>;
#[doc = "Field `EXT_SEL` reader - External SCK/WS selection."]
pub type EXT_SEL_R = crate::BitReader<bool>;
#[doc = "Field `EXT_SEL` writer - External SCK/WS selection."]
pub type EXT_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL0CH0_SPEC, bool, O>;
#[doc = "Field `STEREO` reader - Stereo mode of I2S."]
pub type STEREO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WSIZE` reader - Data size when write to FIFO."]
pub type WSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WSIZE` writer - Data size when write to FIFO."]
pub type WSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL0CH0_SPEC, u8, u8, 2, O>;
#[doc = "Field `TX_EN` reader - TX channel enable."]
pub type TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_EN` writer - TX channel enable."]
pub type TX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL0CH0_SPEC, bool, O>;
#[doc = "Field `RX_EN` reader - RX channel enable."]
pub type RX_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_EN` writer - RX channel enable."]
pub type RX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL0CH0_SPEC, bool, O>;
#[doc = "Field `FLUSH` reader - Flushes the TX/RX FIFO buffer."]
pub type FLUSH_R = crate::BitReader<bool>;
#[doc = "Field `FLUSH` writer - Flushes the TX/RX FIFO buffer."]
pub type FLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL0CH0_SPEC, bool, O>;
#[doc = "Field `RST` reader - Write 1 to reset channel."]
pub type RST_R = crate::BitReader<bool>;
#[doc = "Field `RST` writer - Write 1 to reset channel."]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL0CH0_SPEC, bool, O>;
#[doc = "Field `FIFO_LSB` reader - Bit Field Control."]
pub type FIFO_LSB_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_LSB` writer - Bit Field Control."]
pub type FIFO_LSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL0CH0_SPEC, bool, O>;
#[doc = "Field `RX_THD_VAL` reader - depth of receive FIFO for threshold interrupt generation."]
pub type RX_THD_VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_THD_VAL` writer - depth of receive FIFO for threshold interrupt generation."]
pub type RX_THD_VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL0CH0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 1 - LSB Transmit Receive First."]
    #[inline(always)]
    pub fn lsb_first(&self) -> LSB_FIRST_R {
        LSB_FIRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDM Filter."]
    #[inline(always)]
    pub fn pdm_filt(&self) -> PDM_FILT_R {
        PDM_FILT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDM Enable."]
    #[inline(always)]
    pub fn pdm_en(&self) -> PDM_EN_R {
        PDM_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DDR."]
    #[inline(always)]
    pub fn useddr(&self) -> USEDDR_R {
        USEDDR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Invert PDM."]
    #[inline(always)]
    pub fn pdm_inv(&self) -> PDM_INV_R {
        PDM_INV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - SCK Select."]
    #[inline(always)]
    pub fn ch_mode(&self) -> CH_MODE_R {
        CH_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - WS polarity select."]
    #[inline(always)]
    pub fn ws_pol(&self) -> WS_POL_R {
        WS_POL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MSB location."]
    #[inline(always)]
    pub fn msb_loc(&self) -> MSB_LOC_R {
        MSB_LOC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Align to MSB or LSB."]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External SCK/WS selection."]
    #[inline(always)]
    pub fn ext_sel(&self) -> EXT_SEL_R {
        EXT_SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Stereo mode of I2S."]
    #[inline(always)]
    pub fn stereo(&self) -> STEREO_R {
        STEREO_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Data size when write to FIFO."]
    #[inline(always)]
    pub fn wsize(&self) -> WSIZE_R {
        WSIZE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - TX channel enable."]
    #[inline(always)]
    pub fn tx_en(&self) -> TX_EN_R {
        TX_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RX channel enable."]
    #[inline(always)]
    pub fn rx_en(&self) -> RX_EN_R {
        RX_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Flushes the TX/RX FIFO buffer."]
    #[inline(always)]
    pub fn flush(&self) -> FLUSH_R {
        FLUSH_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write 1 to reset channel."]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Bit Field Control."]
    #[inline(always)]
    pub fn fifo_lsb(&self) -> FIFO_LSB_R {
        FIFO_LSB_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:31 - depth of receive FIFO for threshold interrupt generation."]
    #[inline(always)]
    pub fn rx_thd_val(&self) -> RX_THD_VAL_R {
        RX_THD_VAL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - LSB Transmit Receive First."]
    #[inline(always)]
    #[must_use]
    pub fn lsb_first(&mut self) -> LSB_FIRST_W<1> {
        LSB_FIRST_W::new(self)
    }
    #[doc = "Bit 2 - PDM Filter."]
    #[inline(always)]
    #[must_use]
    pub fn pdm_filt(&mut self) -> PDM_FILT_W<2> {
        PDM_FILT_W::new(self)
    }
    #[doc = "Bit 3 - PDM Enable."]
    #[inline(always)]
    #[must_use]
    pub fn pdm_en(&mut self) -> PDM_EN_W<3> {
        PDM_EN_W::new(self)
    }
    #[doc = "Bit 4 - DDR."]
    #[inline(always)]
    #[must_use]
    pub fn useddr(&mut self) -> USEDDR_W<4> {
        USEDDR_W::new(self)
    }
    #[doc = "Bit 5 - Invert PDM."]
    #[inline(always)]
    #[must_use]
    pub fn pdm_inv(&mut self) -> PDM_INV_W<5> {
        PDM_INV_W::new(self)
    }
    #[doc = "Bits 6:7 - SCK Select."]
    #[inline(always)]
    #[must_use]
    pub fn ch_mode(&mut self) -> CH_MODE_W<6> {
        CH_MODE_W::new(self)
    }
    #[doc = "Bit 8 - WS polarity select."]
    #[inline(always)]
    #[must_use]
    pub fn ws_pol(&mut self) -> WS_POL_W<8> {
        WS_POL_W::new(self)
    }
    #[doc = "Bit 11 - External SCK/WS selection."]
    #[inline(always)]
    #[must_use]
    pub fn ext_sel(&mut self) -> EXT_SEL_W<11> {
        EXT_SEL_W::new(self)
    }
    #[doc = "Bits 14:15 - Data size when write to FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn wsize(&mut self) -> WSIZE_W<14> {
        WSIZE_W::new(self)
    }
    #[doc = "Bit 16 - TX channel enable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_en(&mut self) -> TX_EN_W<16> {
        TX_EN_W::new(self)
    }
    #[doc = "Bit 17 - RX channel enable."]
    #[inline(always)]
    #[must_use]
    pub fn rx_en(&mut self) -> RX_EN_W<17> {
        RX_EN_W::new(self)
    }
    #[doc = "Bit 18 - Flushes the TX/RX FIFO buffer."]
    #[inline(always)]
    #[must_use]
    pub fn flush(&mut self) -> FLUSH_W<18> {
        FLUSH_W::new(self)
    }
    #[doc = "Bit 19 - Write 1 to reset channel."]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<19> {
        RST_W::new(self)
    }
    #[doc = "Bit 20 - Bit Field Control."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_lsb(&mut self) -> FIFO_LSB_W<20> {
        FIFO_LSB_W::new(self)
    }
    #[doc = "Bits 24:31 - depth of receive FIFO for threshold interrupt generation."]
    #[inline(always)]
    #[must_use]
    pub fn rx_thd_val(&mut self) -> RX_THD_VAL_W<24> {
        RX_THD_VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global mode channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl0ch0](index.html) module"]
pub struct CTRL0CH0_SPEC;
impl crate::RegisterSpec for CTRL0CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl0ch0::R](R) reader structure"]
impl crate::Readable for CTRL0CH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl0ch0::W](W) writer structure"]
impl crate::Writable for CTRL0CH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL0CH0 to value 0"]
impl crate::Resettable for CTRL0CH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
