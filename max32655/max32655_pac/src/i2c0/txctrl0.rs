#[doc = "Register `TXCTRL0` reader"]
pub struct R(crate::R<TXCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXCTRL0` writer"]
pub struct W(crate::W<TXCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXCTRL0_SPEC>;
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
impl From<crate::W<TXCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRELOAD_MODE` reader - Transmit FIFO Preaload Mode. Setting this bit will allow for high speed application to preload the transmit FIFO prior to Slave Address Match."]
pub type PRELOAD_MODE_R = crate::BitReader<bool>;
#[doc = "Field `PRELOAD_MODE` writer - Transmit FIFO Preaload Mode. Setting this bit will allow for high speed application to preload the transmit FIFO prior to Slave Address Match."]
pub type PRELOAD_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXCTRL0_SPEC, bool, O>;
#[doc = "Field `TX_READY_MODE` reader - Transmit FIFO Ready Manual Mode."]
pub type TX_READY_MODE_R = crate::BitReader<TX_READY_MODE_A>;
#[doc = "Transmit FIFO Ready Manual Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_READY_MODE_A {
    #[doc = "0: HW control of I2CTXRDY enabled."]
    EN = 0,
    #[doc = "1: HW control of I2CTXRDY disabled."]
    DIS = 1,
}
impl From<TX_READY_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: TX_READY_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_READY_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_READY_MODE_A {
        match self.bits {
            false => TX_READY_MODE_A::EN,
            true => TX_READY_MODE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TX_READY_MODE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TX_READY_MODE_A::DIS
    }
}
#[doc = "Field `TX_READY_MODE` writer - Transmit FIFO Ready Manual Mode."]
pub type TX_READY_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TXCTRL0_SPEC, TX_READY_MODE_A, O>;
impl<'a, const O: u8> TX_READY_MODE_W<'a, O> {
    #[doc = "HW control of I2CTXRDY enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_READY_MODE_A::EN)
    }
    #[doc = "HW control of I2CTXRDY disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_READY_MODE_A::DIS)
    }
}
#[doc = "Field `GC_ADDR_FLUSH_DIS` reader - TX FIFO General Call Address Match Auto Flush Disable."]
pub type GC_ADDR_FLUSH_DIS_R = crate::BitReader<GC_ADDR_FLUSH_DIS_A>;
#[doc = "TX FIFO General Call Address Match Auto Flush Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GC_ADDR_FLUSH_DIS_A {
    #[doc = "0: Enabled."]
    EN = 0,
    #[doc = "1: Disabled."]
    DIS = 1,
}
impl From<GC_ADDR_FLUSH_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: GC_ADDR_FLUSH_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl GC_ADDR_FLUSH_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GC_ADDR_FLUSH_DIS_A {
        match self.bits {
            false => GC_ADDR_FLUSH_DIS_A::EN,
            true => GC_ADDR_FLUSH_DIS_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GC_ADDR_FLUSH_DIS_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GC_ADDR_FLUSH_DIS_A::DIS
    }
}
#[doc = "Field `GC_ADDR_FLUSH_DIS` writer - TX FIFO General Call Address Match Auto Flush Disable."]
pub type GC_ADDR_FLUSH_DIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TXCTRL0_SPEC, GC_ADDR_FLUSH_DIS_A, O>;
impl<'a, const O: u8> GC_ADDR_FLUSH_DIS_W<'a, O> {
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(GC_ADDR_FLUSH_DIS_A::EN)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GC_ADDR_FLUSH_DIS_A::DIS)
    }
}
#[doc = "Field `WR_ADDR_FLUSH_DIS` reader - TX FIFO Slave Address Match Write Auto Flush Disable."]
pub type WR_ADDR_FLUSH_DIS_R = crate::BitReader<WR_ADDR_FLUSH_DIS_A>;
#[doc = "TX FIFO Slave Address Match Write Auto Flush Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WR_ADDR_FLUSH_DIS_A {
    #[doc = "0: Enabled."]
    EN = 0,
    #[doc = "1: Disabled."]
    DIS = 1,
}
impl From<WR_ADDR_FLUSH_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: WR_ADDR_FLUSH_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl WR_ADDR_FLUSH_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WR_ADDR_FLUSH_DIS_A {
        match self.bits {
            false => WR_ADDR_FLUSH_DIS_A::EN,
            true => WR_ADDR_FLUSH_DIS_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WR_ADDR_FLUSH_DIS_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WR_ADDR_FLUSH_DIS_A::DIS
    }
}
#[doc = "Field `WR_ADDR_FLUSH_DIS` writer - TX FIFO Slave Address Match Write Auto Flush Disable."]
pub type WR_ADDR_FLUSH_DIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TXCTRL0_SPEC, WR_ADDR_FLUSH_DIS_A, O>;
impl<'a, const O: u8> WR_ADDR_FLUSH_DIS_W<'a, O> {
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(WR_ADDR_FLUSH_DIS_A::EN)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(WR_ADDR_FLUSH_DIS_A::DIS)
    }
}
#[doc = "Field `RD_ADDR_FLUSH_DIS` reader - TX FIFO Slave Address Match Read Auto Flush Disable."]
pub type RD_ADDR_FLUSH_DIS_R = crate::BitReader<RD_ADDR_FLUSH_DIS_A>;
#[doc = "TX FIFO Slave Address Match Read Auto Flush Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RD_ADDR_FLUSH_DIS_A {
    #[doc = "0: Enabled."]
    EN = 0,
    #[doc = "1: Disabled."]
    DIS = 1,
}
impl From<RD_ADDR_FLUSH_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: RD_ADDR_FLUSH_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl RD_ADDR_FLUSH_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RD_ADDR_FLUSH_DIS_A {
        match self.bits {
            false => RD_ADDR_FLUSH_DIS_A::EN,
            true => RD_ADDR_FLUSH_DIS_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RD_ADDR_FLUSH_DIS_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RD_ADDR_FLUSH_DIS_A::DIS
    }
}
#[doc = "Field `RD_ADDR_FLUSH_DIS` writer - TX FIFO Slave Address Match Read Auto Flush Disable."]
pub type RD_ADDR_FLUSH_DIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TXCTRL0_SPEC, RD_ADDR_FLUSH_DIS_A, O>;
impl<'a, const O: u8> RD_ADDR_FLUSH_DIS_W<'a, O> {
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RD_ADDR_FLUSH_DIS_A::EN)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RD_ADDR_FLUSH_DIS_A::DIS)
    }
}
#[doc = "Field `NACK_FLUSH_DIS` reader - TX FIFO received NACK Auto Flush Disable."]
pub type NACK_FLUSH_DIS_R = crate::BitReader<NACK_FLUSH_DIS_A>;
#[doc = "TX FIFO received NACK Auto Flush Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACK_FLUSH_DIS_A {
    #[doc = "0: Enabled."]
    EN = 0,
    #[doc = "1: Disabled."]
    DIS = 1,
}
impl From<NACK_FLUSH_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: NACK_FLUSH_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl NACK_FLUSH_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACK_FLUSH_DIS_A {
        match self.bits {
            false => NACK_FLUSH_DIS_A::EN,
            true => NACK_FLUSH_DIS_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == NACK_FLUSH_DIS_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == NACK_FLUSH_DIS_A::DIS
    }
}
#[doc = "Field `NACK_FLUSH_DIS` writer - TX FIFO received NACK Auto Flush Disable."]
pub type NACK_FLUSH_DIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TXCTRL0_SPEC, NACK_FLUSH_DIS_A, O>;
impl<'a, const O: u8> NACK_FLUSH_DIS_W<'a, O> {
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(NACK_FLUSH_DIS_A::EN)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(NACK_FLUSH_DIS_A::DIS)
    }
}
#[doc = "Field `FLUSH` reader - Transmit FIFO Flush. This bit is automatically cleared to 0 after the operation."]
pub type FLUSH_R = crate::BitReader<FLUSH_A>;
#[doc = "Transmit FIFO Flush. This bit is automatically cleared to 0 after the operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLUSH_A {
    #[doc = "0: FIFO not flushed."]
    NOT_FLUSHED = 0,
    #[doc = "1: Flush TX_FIFO."]
    FLUSH = 1,
}
impl From<FLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: FLUSH_A) -> Self {
        variant as u8 != 0
    }
}
impl FLUSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLUSH_A {
        match self.bits {
            false => FLUSH_A::NOT_FLUSHED,
            true => FLUSH_A::FLUSH,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FLUSHED`"]
    #[inline(always)]
    pub fn is_not_flushed(&self) -> bool {
        *self == FLUSH_A::NOT_FLUSHED
    }
    #[doc = "Checks if the value of the field is `FLUSH`"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == FLUSH_A::FLUSH
    }
}
#[doc = "Field `FLUSH` writer - Transmit FIFO Flush. This bit is automatically cleared to 0 after the operation."]
pub type FLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXCTRL0_SPEC, FLUSH_A, O>;
impl<'a, const O: u8> FLUSH_W<'a, O> {
    #[doc = "FIFO not flushed."]
    #[inline(always)]
    pub fn not_flushed(self) -> &'a mut W {
        self.variant(FLUSH_A::NOT_FLUSHED)
    }
    #[doc = "Flush TX_FIFO."]
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(FLUSH_A::FLUSH)
    }
}
#[doc = "Field `THD_VAL` reader - Transmit FIFO Threshold. These bits define the TX_FIFO interrupt threshold."]
pub type THD_VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THD_VAL` writer - Transmit FIFO Threshold. These bits define the TX_FIFO interrupt threshold."]
pub type THD_VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXCTRL0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Transmit FIFO Preaload Mode. Setting this bit will allow for high speed application to preload the transmit FIFO prior to Slave Address Match."]
    #[inline(always)]
    pub fn preload_mode(&self) -> PRELOAD_MODE_R {
        PRELOAD_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Ready Manual Mode."]
    #[inline(always)]
    pub fn tx_ready_mode(&self) -> TX_READY_MODE_R {
        TX_READY_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX FIFO General Call Address Match Auto Flush Disable."]
    #[inline(always)]
    pub fn gc_addr_flush_dis(&self) -> GC_ADDR_FLUSH_DIS_R {
        GC_ADDR_FLUSH_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX FIFO Slave Address Match Write Auto Flush Disable."]
    #[inline(always)]
    pub fn wr_addr_flush_dis(&self) -> WR_ADDR_FLUSH_DIS_R {
        WR_ADDR_FLUSH_DIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX FIFO Slave Address Match Read Auto Flush Disable."]
    #[inline(always)]
    pub fn rd_addr_flush_dis(&self) -> RD_ADDR_FLUSH_DIS_R {
        RD_ADDR_FLUSH_DIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX FIFO received NACK Auto Flush Disable."]
    #[inline(always)]
    pub fn nack_flush_dis(&self) -> NACK_FLUSH_DIS_R {
        NACK_FLUSH_DIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Flush. This bit is automatically cleared to 0 after the operation."]
    #[inline(always)]
    pub fn flush(&self) -> FLUSH_R {
        FLUSH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Transmit FIFO Threshold. These bits define the TX_FIFO interrupt threshold."]
    #[inline(always)]
    pub fn thd_val(&self) -> THD_VAL_R {
        THD_VAL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO Preaload Mode. Setting this bit will allow for high speed application to preload the transmit FIFO prior to Slave Address Match."]
    #[inline(always)]
    #[must_use]
    pub fn preload_mode(&mut self) -> PRELOAD_MODE_W<0> {
        PRELOAD_MODE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit FIFO Ready Manual Mode."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ready_mode(&mut self) -> TX_READY_MODE_W<1> {
        TX_READY_MODE_W::new(self)
    }
    #[doc = "Bit 2 - TX FIFO General Call Address Match Auto Flush Disable."]
    #[inline(always)]
    #[must_use]
    pub fn gc_addr_flush_dis(&mut self) -> GC_ADDR_FLUSH_DIS_W<2> {
        GC_ADDR_FLUSH_DIS_W::new(self)
    }
    #[doc = "Bit 3 - TX FIFO Slave Address Match Write Auto Flush Disable."]
    #[inline(always)]
    #[must_use]
    pub fn wr_addr_flush_dis(&mut self) -> WR_ADDR_FLUSH_DIS_W<3> {
        WR_ADDR_FLUSH_DIS_W::new(self)
    }
    #[doc = "Bit 4 - TX FIFO Slave Address Match Read Auto Flush Disable."]
    #[inline(always)]
    #[must_use]
    pub fn rd_addr_flush_dis(&mut self) -> RD_ADDR_FLUSH_DIS_W<4> {
        RD_ADDR_FLUSH_DIS_W::new(self)
    }
    #[doc = "Bit 5 - TX FIFO received NACK Auto Flush Disable."]
    #[inline(always)]
    #[must_use]
    pub fn nack_flush_dis(&mut self) -> NACK_FLUSH_DIS_W<5> {
        NACK_FLUSH_DIS_W::new(self)
    }
    #[doc = "Bit 7 - Transmit FIFO Flush. This bit is automatically cleared to 0 after the operation."]
    #[inline(always)]
    #[must_use]
    pub fn flush(&mut self) -> FLUSH_W<7> {
        FLUSH_W::new(self)
    }
    #[doc = "Bits 8:11 - Transmit FIFO Threshold. These bits define the TX_FIFO interrupt threshold."]
    #[inline(always)]
    #[must_use]
    pub fn thd_val(&mut self) -> THD_VAL_W<8> {
        THD_VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Control Register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txctrl0](index.html) module"]
pub struct TXCTRL0_SPEC;
impl crate::RegisterSpec for TXCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txctrl0::R](R) reader structure"]
impl crate::Readable for TXCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txctrl0::W](W) writer structure"]
impl crate::Writable for TXCTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXCTRL0 to value 0"]
impl crate::Resettable for TXCTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
