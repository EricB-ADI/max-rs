#[doc = "Register `FIFO` reader"]
pub struct R(crate::R<FIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO` writer"]
pub struct W(crate::W<FIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_SPEC>;
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
impl From<crate::W<FIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Load/unload location for TX and RX FIFO buffers."]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA` writer - Load/unload location for TX and RX FIFO buffers."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFO_SPEC, u8, u8, 8, O>;
#[doc = "Field `RX_PAR` reader - Parity error flag for next byte to be read from FIFO."]
pub type RX_PAR_R = crate::BitReader<bool>;
#[doc = "Field `RX_PAR` writer - Parity error flag for next byte to be read from FIFO."]
pub type RX_PAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFO_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Load/unload location for TX and RX FIFO buffers."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Parity error flag for next byte to be read from FIFO."]
    #[inline(always)]
    pub fn rx_par(&self) -> RX_PAR_R {
        RX_PAR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Load/unload location for TX and RX FIFO buffers."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Bit 8 - Parity error flag for next byte to be read from FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn rx_par(&mut self) -> RX_PAR_W<8> {
        RX_PAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Read/Write register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo](index.html) module"]
pub struct FIFO_SPEC;
impl crate::RegisterSpec for FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo::R](R) reader structure"]
impl crate::Readable for FIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo::W](W) writer structure"]
impl crate::Writable for FIFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFO to value 0"]
impl crate::Resettable for FIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
