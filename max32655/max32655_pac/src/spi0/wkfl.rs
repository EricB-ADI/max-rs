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
#[doc = "Field `TX_THD` reader - Wake on TX FIFO Threshold Crossed."]
pub type TX_THD_R = crate::BitReader<TX_THD_A>;
#[doc = "Wake on TX FIFO Threshold Crossed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_THD_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<TX_THD_A> for bool {
    #[inline(always)]
    fn from(variant: TX_THD_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_THD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TX_THD_A> {
        match self.bits {
            true => Some(TX_THD_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TX_THD_A::CLEAR
    }
}
#[doc = "Field `TX_THD` writer - Wake on TX FIFO Threshold Crossed."]
pub type TX_THD_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKFL_SPEC, TX_THD_A, O>;
impl<'a, const O: u8> TX_THD_W<'a, O> {
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TX_THD_A::CLEAR)
    }
}
#[doc = "Field `TX_EM` reader - Wake on TX FIFO Empty."]
pub type TX_EM_R = crate::BitReader<TX_EM_A>;
#[doc = "Wake on TX FIFO Empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_EM_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<TX_EM_A> for bool {
    #[inline(always)]
    fn from(variant: TX_EM_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_EM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TX_EM_A> {
        match self.bits {
            true => Some(TX_EM_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TX_EM_A::CLEAR
    }
}
#[doc = "Field `TX_EM` writer - Wake on TX FIFO Empty."]
pub type TX_EM_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKFL_SPEC, TX_EM_A, O>;
impl<'a, const O: u8> TX_EM_W<'a, O> {
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TX_EM_A::CLEAR)
    }
}
#[doc = "Field `RX_THD` reader - Wake on RX FIFO Threshold Crossed."]
pub type RX_THD_R = crate::BitReader<RX_THD_A>;
#[doc = "Wake on RX FIFO Threshold Crossed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_THD_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<RX_THD_A> for bool {
    #[inline(always)]
    fn from(variant: RX_THD_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_THD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RX_THD_A> {
        match self.bits {
            true => Some(RX_THD_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RX_THD_A::CLEAR
    }
}
#[doc = "Field `RX_THD` writer - Wake on RX FIFO Threshold Crossed."]
pub type RX_THD_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKFL_SPEC, RX_THD_A, O>;
impl<'a, const O: u8> RX_THD_W<'a, O> {
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RX_THD_A::CLEAR)
    }
}
#[doc = "Field `RX_FULL` reader - Wake on RX FIFO Full."]
pub type RX_FULL_R = crate::BitReader<RX_FULL_A>;
#[doc = "Wake on RX FIFO Full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FULL_A {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    CLEAR = 1,
}
impl From<RX_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FULL_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_FULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RX_FULL_A> {
        match self.bits {
            true => Some(RX_FULL_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RX_FULL_A::CLEAR
    }
}
#[doc = "Field `RX_FULL` writer - Wake on RX FIFO Full."]
pub type RX_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKFL_SPEC, RX_FULL_A, O>;
impl<'a, const O: u8> RX_FULL_W<'a, O> {
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RX_FULL_A::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Wake on TX FIFO Threshold Crossed."]
    #[inline(always)]
    pub fn tx_thd(&self) -> TX_THD_R {
        TX_THD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake on TX FIFO Empty."]
    #[inline(always)]
    pub fn tx_em(&self) -> TX_EM_R {
        TX_EM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake on RX FIFO Threshold Crossed."]
    #[inline(always)]
    pub fn rx_thd(&self) -> RX_THD_R {
        RX_THD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wake on RX FIFO Full."]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake on TX FIFO Threshold Crossed."]
    #[inline(always)]
    #[must_use]
    pub fn tx_thd(&mut self) -> TX_THD_W<0> {
        TX_THD_W::new(self)
    }
    #[doc = "Bit 1 - Wake on TX FIFO Empty."]
    #[inline(always)]
    #[must_use]
    pub fn tx_em(&mut self) -> TX_EM_W<1> {
        TX_EM_W::new(self)
    }
    #[doc = "Bit 2 - Wake on RX FIFO Threshold Crossed."]
    #[inline(always)]
    #[must_use]
    pub fn rx_thd(&mut self) -> RX_THD_W<2> {
        RX_THD_W::new(self)
    }
    #[doc = "Bit 3 - Wake on RX FIFO Full."]
    #[inline(always)]
    #[must_use]
    pub fn rx_full(&mut self) -> RX_FULL_W<3> {
        RX_FULL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for wake up flags. All bits in this register are write 1 to clear.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkfl](index.html) module"]
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
