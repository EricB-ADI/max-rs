#[doc = "Register `INT_EN` reader"]
pub struct R(crate::R<INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_EN` writer"]
pub struct W(crate::W<INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_EN_SPEC>;
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
impl From<crate::W<INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_FERR` reader - Enable Interrupt For RX Frame Error"]
pub type RX_FERR_R = crate::BitReader<bool>;
#[doc = "Field `RX_FERR` writer - Enable Interrupt For RX Frame Error"]
pub type RX_FERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
#[doc = "Field `RX_PAR` reader - Enable Interrupt For RX Parity Error"]
pub type RX_PAR_R = crate::BitReader<bool>;
#[doc = "Field `RX_PAR` writer - Enable Interrupt For RX Parity Error"]
pub type RX_PAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
#[doc = "Field `CTS_EV` reader - Enable Interrupt For CTS signal change Error"]
pub type CTS_EV_R = crate::BitReader<bool>;
#[doc = "Field `CTS_EV` writer - Enable Interrupt For CTS signal change Error"]
pub type CTS_EV_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
#[doc = "Field `RX_OV` reader - Enable Interrupt For RX FIFO Overrun Error"]
pub type RX_OV_R = crate::BitReader<bool>;
#[doc = "Field `RX_OV` writer - Enable Interrupt For RX FIFO Overrun Error"]
pub type RX_OV_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
#[doc = "Field `RX_THD` reader - Enable Interrupt For RX FIFO reaches the number of bytes configured by RXTHD"]
pub type RX_THD_R = crate::BitReader<bool>;
#[doc = "Field `RX_THD` writer - Enable Interrupt For RX FIFO reaches the number of bytes configured by RXTHD"]
pub type RX_THD_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
#[doc = "Field `TX_OB` reader - Enable Interrupt For TX FIFO has one byte remaining"]
pub type TX_OB_R = crate::BitReader<bool>;
#[doc = "Field `TX_OB` writer - Enable Interrupt For TX FIFO has one byte remaining"]
pub type TX_OB_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
#[doc = "Field `TX_HE` reader - Enable Interrupt For TX FIFO has half empty"]
pub type TX_HE_R = crate::BitReader<bool>;
#[doc = "Field `TX_HE` writer - Enable Interrupt For TX FIFO has half empty"]
pub type TX_HE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable Interrupt For RX Frame Error"]
    #[inline(always)]
    pub fn rx_ferr(&self) -> RX_FERR_R {
        RX_FERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Interrupt For RX Parity Error"]
    #[inline(always)]
    pub fn rx_par(&self) -> RX_PAR_R {
        RX_PAR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Interrupt For CTS signal change Error"]
    #[inline(always)]
    pub fn cts_ev(&self) -> CTS_EV_R {
        CTS_EV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Interrupt For RX FIFO Overrun Error"]
    #[inline(always)]
    pub fn rx_ov(&self) -> RX_OV_R {
        RX_OV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Interrupt For RX FIFO reaches the number of bytes configured by RXTHD"]
    #[inline(always)]
    pub fn rx_thd(&self) -> RX_THD_R {
        RX_THD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Interrupt For TX FIFO has one byte remaining"]
    #[inline(always)]
    pub fn tx_ob(&self) -> TX_OB_R {
        TX_OB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Interrupt For TX FIFO has half empty"]
    #[inline(always)]
    pub fn tx_he(&self) -> TX_HE_R {
        TX_HE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Interrupt For RX Frame Error"]
    #[inline(always)]
    #[must_use]
    pub fn rx_ferr(&mut self) -> RX_FERR_W<0> {
        RX_FERR_W::new(self)
    }
    #[doc = "Bit 1 - Enable Interrupt For RX Parity Error"]
    #[inline(always)]
    #[must_use]
    pub fn rx_par(&mut self) -> RX_PAR_W<1> {
        RX_PAR_W::new(self)
    }
    #[doc = "Bit 2 - Enable Interrupt For CTS signal change Error"]
    #[inline(always)]
    #[must_use]
    pub fn cts_ev(&mut self) -> CTS_EV_W<2> {
        CTS_EV_W::new(self)
    }
    #[doc = "Bit 3 - Enable Interrupt For RX FIFO Overrun Error"]
    #[inline(always)]
    #[must_use]
    pub fn rx_ov(&mut self) -> RX_OV_W<3> {
        RX_OV_W::new(self)
    }
    #[doc = "Bit 4 - Enable Interrupt For RX FIFO reaches the number of bytes configured by RXTHD"]
    #[inline(always)]
    #[must_use]
    pub fn rx_thd(&mut self) -> RX_THD_W<4> {
        RX_THD_W::new(self)
    }
    #[doc = "Bit 5 - Enable Interrupt For TX FIFO has one byte remaining"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ob(&mut self) -> TX_OB_W<5> {
        TX_OB_W::new(self)
    }
    #[doc = "Bit 6 - Enable Interrupt For TX FIFO has half empty"]
    #[inline(always)]
    #[must_use]
    pub fn tx_he(&mut self) -> TX_HE_W<6> {
        TX_HE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_en](index.html) module"]
pub struct INT_EN_SPEC;
impl crate::RegisterSpec for INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_en::R](R) reader structure"]
impl crate::Readable for INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_en::W](W) writer structure"]
impl crate::Writable for INT_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_EN to value 0"]
impl crate::Resettable for INT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
