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
#[doc = "Field `EN` reader - CRC Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - CRC Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DMA_EN` reader - DMA Request Enable"]
pub type DMA_EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA_EN` writer - DMA Request Enable"]
pub type DMA_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `MSB` reader - MSB Select"]
pub type MSB_R = crate::BitReader<bool>;
#[doc = "Field `MSB` writer - MSB Select"]
pub type MSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BYTE_SWAP_IN` reader - Byte Swap CRC Data Input"]
pub type BYTE_SWAP_IN_R = crate::BitReader<bool>;
#[doc = "Field `BYTE_SWAP_IN` writer - Byte Swap CRC Data Input"]
pub type BYTE_SWAP_IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BYTE_SWAP_OUT` reader - Byte Swap CRC Value Output"]
pub type BYTE_SWAP_OUT_R = crate::BitReader<bool>;
#[doc = "Field `BYTE_SWAP_OUT` writer - Byte Swap CRC Value Output"]
pub type BYTE_SWAP_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BUSY` reader - CRC Busy"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - CRC Busy"]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CRC Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Request Enable"]
    #[inline(always)]
    pub fn dma_en(&self) -> DMA_EN_R {
        DMA_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSB Select"]
    #[inline(always)]
    pub fn msb(&self) -> MSB_R {
        MSB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Byte Swap CRC Data Input"]
    #[inline(always)]
    pub fn byte_swap_in(&self) -> BYTE_SWAP_IN_R {
        BYTE_SWAP_IN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Byte Swap CRC Value Output"]
    #[inline(always)]
    pub fn byte_swap_out(&self) -> BYTE_SWAP_OUT_R {
        BYTE_SWAP_OUT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - CRC Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - DMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_en(&mut self) -> DMA_EN_W<1> {
        DMA_EN_W::new(self)
    }
    #[doc = "Bit 2 - MSB Select"]
    #[inline(always)]
    #[must_use]
    pub fn msb(&mut self) -> MSB_W<2> {
        MSB_W::new(self)
    }
    #[doc = "Bit 3 - Byte Swap CRC Data Input"]
    #[inline(always)]
    #[must_use]
    pub fn byte_swap_in(&mut self) -> BYTE_SWAP_IN_W<3> {
        BYTE_SWAP_IN_W::new(self)
    }
    #[doc = "Bit 4 - Byte Swap CRC Value Output"]
    #[inline(always)]
    #[must_use]
    pub fn byte_swap_out(&mut self) -> BYTE_SWAP_OUT_W<4> {
        BYTE_SWAP_OUT_W::new(self)
    }
    #[doc = "Bit 16 - CRC Busy"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<16> {
        BUSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
