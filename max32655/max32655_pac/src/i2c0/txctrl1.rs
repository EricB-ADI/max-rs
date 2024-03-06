#[doc = "Register `TXCTRL1` reader"]
pub struct R(crate::R<TXCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXCTRL1` writer"]
pub struct W(crate::W<TXCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXCTRL1_SPEC>;
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
impl From<crate::W<TXCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRELOAD_RDY` reader - Transmit FIFO Preload Ready."]
pub type PRELOAD_RDY_R = crate::BitReader<bool>;
#[doc = "Field `PRELOAD_RDY` writer - Transmit FIFO Preload Ready."]
pub type PRELOAD_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXCTRL1_SPEC, bool, O>;
#[doc = "Field `LVL` reader - Transmit FIFO Count. These bits reflect the number of bytes in the TX_FIFO."]
pub type LVL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Transmit FIFO Preload Ready."]
    #[inline(always)]
    pub fn preload_rdy(&self) -> PRELOAD_RDY_R {
        PRELOAD_RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Transmit FIFO Count. These bits reflect the number of bytes in the TX_FIFO."]
    #[inline(always)]
    pub fn lvl(&self) -> LVL_R {
        LVL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO Preload Ready."]
    #[inline(always)]
    #[must_use]
    pub fn preload_rdy(&mut self) -> PRELOAD_RDY_W<0> {
        PRELOAD_RDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Control Register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txctrl1](index.html) module"]
pub struct TXCTRL1_SPEC;
impl crate::RegisterSpec for TXCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txctrl1::R](R) reader structure"]
impl crate::Readable for TXCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txctrl1::W](W) writer structure"]
impl crate::Writable for TXCTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXCTRL1 to value 0"]
impl crate::Resettable for TXCTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
