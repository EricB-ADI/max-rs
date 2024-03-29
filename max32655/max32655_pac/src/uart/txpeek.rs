#[doc = "Register `TXPEEK` reader"]
pub struct R(crate::R<TXPEEK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXPEEK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXPEEK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXPEEK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXPEEK` writer"]
pub struct W(crate::W<TXPEEK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXPEEK_SPEC>;
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
impl From<crate::W<TXPEEK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXPEEK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Read TX FIFO next data. Reading from this field does not affect the contents of TX FIFO. Note that the parity bit is available from this field."]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA` writer - Read TX FIFO next data. Reading from this field does not affect the contents of TX FIFO. Note that the parity bit is available from this field."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXPEEK_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Read TX FIFO next data. Reading from this field does not affect the contents of TX FIFO. Note that the parity bit is available from this field."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read TX FIFO next data. Reading from this field does not affect the contents of TX FIFO. Note that the parity bit is available from this field."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX FIFO Output Peek register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txpeek](index.html) module"]
pub struct TXPEEK_SPEC;
impl crate::RegisterSpec for TXPEEK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txpeek::R](R) reader structure"]
impl crate::Readable for TXPEEK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txpeek::W](W) writer structure"]
impl crate::Writable for TXPEEK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXPEEK to value 0"]
impl crate::Resettable for TXPEEK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
