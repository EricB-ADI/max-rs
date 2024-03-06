#[doc = "Register `RXCTRL1` reader"]
pub struct R(crate::R<RXCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXCTRL1` writer"]
pub struct W(crate::W<RXCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXCTRL1_SPEC>;
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
impl From<crate::W<RXCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - Receive Count Bits. These bits define the number of bytes to be received in a transaction, except for the case RXCNT = 0. RXCNT = 0 means 256 bytes to be received in a transaction."]
pub type CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNT` writer - Receive Count Bits. These bits define the number of bytes to be received in a transaction, except for the case RXCNT = 0. RXCNT = 0 means 256 bytes to be received in a transaction."]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXCTRL1_SPEC, u8, u8, 8, O>;
#[doc = "Field `LVL` reader - Receive FIFO Count. These bits reflect the number of byte in the RX_FIFO. These bits are flushed when I2CEN = 0."]
pub type LVL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Receive Count Bits. These bits define the number of bytes to be received in a transaction, except for the case RXCNT = 0. RXCNT = 0 means 256 bytes to be received in a transaction."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Receive FIFO Count. These bits reflect the number of byte in the RX_FIFO. These bits are flushed when I2CEN = 0."]
    #[inline(always)]
    pub fn lvl(&self) -> LVL_R {
        LVL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Count Bits. These bits define the number of bytes to be received in a transaction, except for the case RXCNT = 0. RXCNT = 0 means 256 bytes to be received in a transaction."]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<0> {
        CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Control Register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxctrl1](index.html) module"]
pub struct RXCTRL1_SPEC;
impl crate::RegisterSpec for RXCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxctrl1::R](R) reader structure"]
impl crate::Readable for RXCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxctrl1::W](W) writer structure"]
impl crate::Writable for RXCTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXCTRL1 to value 0"]
impl crate::Resettable for RXCTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
