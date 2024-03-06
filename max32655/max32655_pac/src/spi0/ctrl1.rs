#[doc = "Register `CTRL1` reader"]
pub struct R(crate::R<CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL1` writer"]
pub struct W(crate::W<CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL1_SPEC>;
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
impl From<crate::W<CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_NUM_CHAR` reader - Nubmer of Characters to transmit."]
pub type TX_NUM_CHAR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TX_NUM_CHAR` writer - Nubmer of Characters to transmit."]
pub type TX_NUM_CHAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL1_SPEC, u16, u16, 16, O>;
#[doc = "Field `RX_NUM_CHAR` reader - Nubmer of Characters to receive."]
pub type RX_NUM_CHAR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RX_NUM_CHAR` writer - Nubmer of Characters to receive."]
pub type RX_NUM_CHAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Nubmer of Characters to transmit."]
    #[inline(always)]
    pub fn tx_num_char(&self) -> TX_NUM_CHAR_R {
        TX_NUM_CHAR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Nubmer of Characters to receive."]
    #[inline(always)]
    pub fn rx_num_char(&self) -> RX_NUM_CHAR_R {
        RX_NUM_CHAR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Nubmer of Characters to transmit."]
    #[inline(always)]
    #[must_use]
    pub fn tx_num_char(&mut self) -> TX_NUM_CHAR_W<0> {
        TX_NUM_CHAR_W::new(self)
    }
    #[doc = "Bits 16:31 - Nubmer of Characters to receive."]
    #[inline(always)]
    #[must_use]
    pub fn rx_num_char(&mut self) -> RX_NUM_CHAR_W<16> {
        RX_NUM_CHAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for controlling SPI peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](index.html) module"]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl1::R](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
