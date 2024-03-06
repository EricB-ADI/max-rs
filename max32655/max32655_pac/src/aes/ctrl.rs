#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - AES Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - AES Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DMA_RX_EN` reader - DMA Request To Read Data Output FIFO"]
pub type DMA_RX_EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA_RX_EN` writer - DMA Request To Read Data Output FIFO"]
pub type DMA_RX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DMA_TX_EN` reader - DMA Request To Write Data Input FIFO"]
pub type DMA_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA_TX_EN` writer - DMA Request To Write Data Input FIFO"]
pub type DMA_TX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `START` reader - Start AES Calculation"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Start AES Calculation"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `INPUT_FLUSH` reader - Flush the data input FIFO"]
pub type INPUT_FLUSH_R = crate::BitReader<bool>;
#[doc = "Field `INPUT_FLUSH` writer - Flush the data input FIFO"]
pub type INPUT_FLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `OUTPUT_FLUSH` reader - Flush the data output FIFO"]
pub type OUTPUT_FLUSH_R = crate::BitReader<bool>;
#[doc = "Field `OUTPUT_FLUSH` writer - Flush the data output FIFO"]
pub type OUTPUT_FLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `KEY_SIZE` reader - Encryption Key Size"]
pub type KEY_SIZE_R = crate::FieldReader<u8, KEY_SIZE_A>;
#[doc = "Encryption Key Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_SIZE_A {
    #[doc = "0: 128 Bits."]
    AES128 = 0,
    #[doc = "1: 192 Bits."]
    AES192 = 1,
    #[doc = "2: 256 Bits."]
    AES256 = 2,
}
impl From<KEY_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_SIZE_A) -> Self {
        variant as _
    }
}
impl KEY_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEY_SIZE_A> {
        match self.bits {
            0 => Some(KEY_SIZE_A::AES128),
            1 => Some(KEY_SIZE_A::AES192),
            2 => Some(KEY_SIZE_A::AES256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AES128`"]
    #[inline(always)]
    pub fn is_aes128(&self) -> bool {
        *self == KEY_SIZE_A::AES128
    }
    #[doc = "Checks if the value of the field is `AES192`"]
    #[inline(always)]
    pub fn is_aes192(&self) -> bool {
        *self == KEY_SIZE_A::AES192
    }
    #[doc = "Checks if the value of the field is `AES256`"]
    #[inline(always)]
    pub fn is_aes256(&self) -> bool {
        *self == KEY_SIZE_A::AES256
    }
}
#[doc = "Field `KEY_SIZE` writer - Encryption Key Size"]
pub type KEY_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, KEY_SIZE_A, 2, O>;
impl<'a, const O: u8> KEY_SIZE_W<'a, O> {
    #[doc = "128 Bits."]
    #[inline(always)]
    pub fn aes128(self) -> &'a mut W {
        self.variant(KEY_SIZE_A::AES128)
    }
    #[doc = "192 Bits."]
    #[inline(always)]
    pub fn aes192(self) -> &'a mut W {
        self.variant(KEY_SIZE_A::AES192)
    }
    #[doc = "256 Bits."]
    #[inline(always)]
    pub fn aes256(self) -> &'a mut W {
        self.variant(KEY_SIZE_A::AES256)
    }
}
#[doc = "Field `TYPE` reader - Encryption Type Selection"]
pub type TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TYPE` writer - Encryption Type Selection"]
pub type TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - AES Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Request To Read Data Output FIFO"]
    #[inline(always)]
    pub fn dma_rx_en(&self) -> DMA_RX_EN_R {
        DMA_RX_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Request To Write Data Input FIFO"]
    #[inline(always)]
    pub fn dma_tx_en(&self) -> DMA_TX_EN_R {
        DMA_TX_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start AES Calculation"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flush the data input FIFO"]
    #[inline(always)]
    pub fn input_flush(&self) -> INPUT_FLUSH_R {
        INPUT_FLUSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flush the data output FIFO"]
    #[inline(always)]
    pub fn output_flush(&self) -> OUTPUT_FLUSH_R {
        OUTPUT_FLUSH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Encryption Key Size"]
    #[inline(always)]
    pub fn key_size(&self) -> KEY_SIZE_R {
        KEY_SIZE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Encryption Type Selection"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AES Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - DMA Request To Read Data Output FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_en(&mut self) -> DMA_RX_EN_W<1> {
        DMA_RX_EN_W::new(self)
    }
    #[doc = "Bit 2 - DMA Request To Write Data Input FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn dma_tx_en(&mut self) -> DMA_TX_EN_W<2> {
        DMA_TX_EN_W::new(self)
    }
    #[doc = "Bit 3 - Start AES Calculation"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<3> {
        START_W::new(self)
    }
    #[doc = "Bit 4 - Flush the data input FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn input_flush(&mut self) -> INPUT_FLUSH_W<4> {
        INPUT_FLUSH_W::new(self)
    }
    #[doc = "Bit 5 - Flush the data output FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn output_flush(&mut self) -> OUTPUT_FLUSH_W<5> {
        OUTPUT_FLUSH_W::new(self)
    }
    #[doc = "Bits 6:7 - Encryption Key Size"]
    #[inline(always)]
    #[must_use]
    pub fn key_size(&mut self) -> KEY_SIZE_W<6> {
        KEY_SIZE_W::new(self)
    }
    #[doc = "Bits 8:9 - Encryption Type Selection"]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TYPE_W<8> {
        TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
