#[doc = "Register `WKFL` reader"]
pub struct R(crate::R<WKFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKFL` writer"]
pub struct W(crate::W<WKFL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKFL_SPEC>;
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
impl From<crate::W<WKFL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKFL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_NE` reader - Wake-Up Flag for RX FIFO Not Empty"]
pub type RX_NE_R = crate::BitReader<bool>;
#[doc = "Field `RX_NE` writer - Wake-Up Flag for RX FIFO Not Empty"]
pub type RX_NE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKFL_SPEC, bool, O>;
#[doc = "Field `RX_FULL` reader - Wake-Up Flag for RX FIFO Full"]
pub type RX_FULL_R = crate::BitReader<bool>;
#[doc = "Field `RX_FULL` writer - Wake-Up Flag for RX FIFO Full"]
pub type RX_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKFL_SPEC, bool, O>;
#[doc = "Field `RX_THD` reader - Wake-Up Flag for RX FIFO Threshold Met"]
pub type RX_THD_R = crate::BitReader<bool>;
#[doc = "Field `RX_THD` writer - Wake-Up Flag for RX FIFO Threshold Met"]
pub type RX_THD_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKFL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Wake-Up Flag for RX FIFO Not Empty"]
    #[inline(always)]
    pub fn rx_ne(&self) -> RX_NE_R {
        RX_NE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake-Up Flag for RX FIFO Full"]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-Up Flag for RX FIFO Threshold Met"]
    #[inline(always)]
    pub fn rx_thd(&self) -> RX_THD_R {
        RX_THD_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-Up Flag for RX FIFO Not Empty"]
    #[inline(always)]
    #[must_use]
    pub fn rx_ne(&mut self) -> RX_NE_W<0> {
        RX_NE_W::new(self)
    }
    #[doc = "Bit 1 - Wake-Up Flag for RX FIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn rx_full(&mut self) -> RX_FULL_W<1> {
        RX_FULL_W::new(self)
    }
    #[doc = "Bit 2 - Wake-Up Flag for RX FIFO Threshold Met"]
    #[inline(always)]
    #[must_use]
    pub fn rx_thd(&mut self) -> RX_THD_W<2> {
        RX_THD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake up Flags register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkfl](index.html) module"]
pub struct WKFL_SPEC;
impl crate::RegisterSpec for WKFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wkfl::R](R) reader structure"]
impl crate::Readable for WKFL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wkfl::W](W) writer structure"]
impl crate::Writable for WKFL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WKFL to value 0"]
impl crate::Resettable for WKFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
