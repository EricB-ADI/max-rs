#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_OV_CH0` reader - Enable for RX FIFO Overrun interrupt."]
pub type RX_OV_CH0_R = crate::BitReader<bool>;
#[doc = "Field `RX_OV_CH0` writer - Enable for RX FIFO Overrun interrupt."]
pub type RX_OV_CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `RX_THD_CH0` reader - Enable for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
pub type RX_THD_CH0_R = crate::BitReader<bool>;
#[doc = "Field `RX_THD_CH0` writer - Enable for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
pub type RX_THD_CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `TX_OB_CH0` reader - Enable for interrupt when TX FIFO has only one byte remaining."]
pub type TX_OB_CH0_R = crate::BitReader<bool>;
#[doc = "Field `TX_OB_CH0` writer - Enable for interrupt when TX FIFO has only one byte remaining."]
pub type TX_OB_CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `TX_HE_CH0` reader - Enable for interrupt when TX FIFO is half empty."]
pub type TX_HE_CH0_R = crate::BitReader<bool>;
#[doc = "Field `TX_HE_CH0` writer - Enable for interrupt when TX FIFO is half empty."]
pub type TX_HE_CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable for RX FIFO Overrun interrupt."]
    #[inline(always)]
    pub fn rx_ov_ch0(&self) -> RX_OV_CH0_R {
        RX_OV_CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
    #[inline(always)]
    pub fn rx_thd_ch0(&self) -> RX_THD_CH0_R {
        RX_THD_CH0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable for interrupt when TX FIFO has only one byte remaining."]
    #[inline(always)]
    pub fn tx_ob_ch0(&self) -> TX_OB_CH0_R {
        TX_OB_CH0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable for interrupt when TX FIFO is half empty."]
    #[inline(always)]
    pub fn tx_he_ch0(&self) -> TX_HE_CH0_R {
        TX_HE_CH0_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable for RX FIFO Overrun interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ov_ch0(&mut self) -> RX_OV_CH0_W<0> {
        RX_OV_CH0_W::new(self)
    }
    #[doc = "Bit 1 - Enable for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
    #[inline(always)]
    #[must_use]
    pub fn rx_thd_ch0(&mut self) -> RX_THD_CH0_W<1> {
        RX_THD_CH0_W::new(self)
    }
    #[doc = "Bit 2 - Enable for interrupt when TX FIFO has only one byte remaining."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ob_ch0(&mut self) -> TX_OB_CH0_W<2> {
        TX_OB_CH0_W::new(self)
    }
    #[doc = "Bit 3 - Enable for interrupt when TX FIFO is half empty."]
    #[inline(always)]
    #[must_use]
    pub fn tx_he_ch0(&mut self) -> TX_HE_CH0_W<3> {
        TX_HE_CH0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
