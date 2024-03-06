#[doc = "Register `INTEN1` reader"]
pub struct R(crate::R<INTEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN1` writer"]
pub struct W(crate::W<INTEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN1_SPEC>;
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
impl From<crate::W<INTEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_OV` reader - Receiver Overflow Interrupt Enable."]
pub type RX_OV_R = crate::BitReader<RX_OV_A>;
#[doc = "Receiver Overflow Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_OV_A {
    #[doc = "0: No Interrupt is Pending."]
    DIS = 0,
    #[doc = "1: An interrupt is pending."]
    EN = 1,
}
impl From<RX_OV_A> for bool {
    #[inline(always)]
    fn from(variant: RX_OV_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_OV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_OV_A {
        match self.bits {
            false => RX_OV_A::DIS,
            true => RX_OV_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RX_OV_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RX_OV_A::EN
    }
}
#[doc = "Field `RX_OV` writer - Receiver Overflow Interrupt Enable."]
pub type RX_OV_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN1_SPEC, RX_OV_A, O>;
impl<'a, const O: u8> RX_OV_W<'a, O> {
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RX_OV_A::DIS)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RX_OV_A::EN)
    }
}
#[doc = "Field `TX_UN` reader - Transmit Underflow Interrupt Enable."]
pub type TX_UN_R = crate::BitReader<TX_UN_A>;
#[doc = "Transmit Underflow Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_UN_A {
    #[doc = "0: No Interrupt is Pending."]
    DIS = 0,
    #[doc = "1: An interrupt is pending."]
    EN = 1,
}
impl From<TX_UN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_UN_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_UN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_UN_A {
        match self.bits {
            false => TX_UN_A::DIS,
            true => TX_UN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TX_UN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TX_UN_A::EN
    }
}
#[doc = "Field `TX_UN` writer - Transmit Underflow Interrupt Enable."]
pub type TX_UN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN1_SPEC, TX_UN_A, O>;
impl<'a, const O: u8> TX_UN_W<'a, O> {
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_UN_A::DIS)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_UN_A::EN)
    }
}
#[doc = "Field `START` reader - START Condition Interrupt Enable."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - START Condition Interrupt Enable."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Receiver Overflow Interrupt Enable."]
    #[inline(always)]
    pub fn rx_ov(&self) -> RX_OV_R {
        RX_OV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Underflow Interrupt Enable."]
    #[inline(always)]
    pub fn tx_un(&self) -> TX_UN_R {
        TX_UN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - START Condition Interrupt Enable."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Overflow Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ov(&mut self) -> RX_OV_W<0> {
        RX_OV_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Underflow Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_un(&mut self) -> TX_UN_W<1> {
        TX_UN_W::new(self)
    }
    #[doc = "Bit 2 - START Condition Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<2> {
        START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Staus Register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten1](index.html) module"]
pub struct INTEN1_SPEC;
impl crate::RegisterSpec for INTEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten1::R](R) reader structure"]
impl crate::Readable for INTEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten1::W](W) writer structure"]
impl crate::Writable for INTEN1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN1 to value 0"]
impl crate::Resettable for INTEN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
