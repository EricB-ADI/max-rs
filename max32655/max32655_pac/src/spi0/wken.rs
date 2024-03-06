#[doc = "Register `WKEN` reader"]
pub struct R(crate::R<WKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKEN` writer"]
pub struct W(crate::W<WKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKEN_SPEC>;
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
impl From<crate::W<WKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_THD` reader - Wake on TX FIFO Threshold Crossed Enable."]
pub type TX_THD_R = crate::BitReader<TX_THD_A>;
#[doc = "Wake on TX FIFO Threshold Crossed Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_THD_A {
    #[doc = "0: Wakeup source disabled."]
    DIS = 0,
    #[doc = "1: Wakeup source enabled."]
    EN = 1,
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
    pub fn variant(&self) -> TX_THD_A {
        match self.bits {
            false => TX_THD_A::DIS,
            true => TX_THD_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TX_THD_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TX_THD_A::EN
    }
}
#[doc = "Field `TX_THD` writer - Wake on TX FIFO Threshold Crossed Enable."]
pub type TX_THD_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKEN_SPEC, TX_THD_A, O>;
impl<'a, const O: u8> TX_THD_W<'a, O> {
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_THD_A::DIS)
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_THD_A::EN)
    }
}
#[doc = "Field `TX_EM` reader - Wake on TX FIFO Empty Enable."]
pub type TX_EM_R = crate::BitReader<TX_EM_A>;
#[doc = "Wake on TX FIFO Empty Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_EM_A {
    #[doc = "0: Wakeup source disabled."]
    DIS = 0,
    #[doc = "1: Wakeup source enabled."]
    EN = 1,
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
    pub fn variant(&self) -> TX_EM_A {
        match self.bits {
            false => TX_EM_A::DIS,
            true => TX_EM_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TX_EM_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TX_EM_A::EN
    }
}
#[doc = "Field `TX_EM` writer - Wake on TX FIFO Empty Enable."]
pub type TX_EM_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKEN_SPEC, TX_EM_A, O>;
impl<'a, const O: u8> TX_EM_W<'a, O> {
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_EM_A::DIS)
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_EM_A::EN)
    }
}
#[doc = "Field `RX_THD` reader - Wake on RX FIFO Threshold Crossed Enable."]
pub type RX_THD_R = crate::BitReader<RX_THD_A>;
#[doc = "Wake on RX FIFO Threshold Crossed Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_THD_A {
    #[doc = "0: Wakeup source disabled."]
    DIS = 0,
    #[doc = "1: Wakeup source enabled."]
    EN = 1,
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
    pub fn variant(&self) -> RX_THD_A {
        match self.bits {
            false => RX_THD_A::DIS,
            true => RX_THD_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RX_THD_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RX_THD_A::EN
    }
}
#[doc = "Field `RX_THD` writer - Wake on RX FIFO Threshold Crossed Enable."]
pub type RX_THD_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKEN_SPEC, RX_THD_A, O>;
impl<'a, const O: u8> RX_THD_W<'a, O> {
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RX_THD_A::DIS)
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RX_THD_A::EN)
    }
}
#[doc = "Field `RX_FULL` reader - Wake on RX FIFO Full Enable."]
pub type RX_FULL_R = crate::BitReader<RX_FULL_A>;
#[doc = "Wake on RX FIFO Full Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FULL_A {
    #[doc = "0: Wakeup source disabled."]
    DIS = 0,
    #[doc = "1: Wakeup source enabled."]
    EN = 1,
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
    pub fn variant(&self) -> RX_FULL_A {
        match self.bits {
            false => RX_FULL_A::DIS,
            true => RX_FULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RX_FULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RX_FULL_A::EN
    }
}
#[doc = "Field `RX_FULL` writer - Wake on RX FIFO Full Enable."]
pub type RX_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKEN_SPEC, RX_FULL_A, O>;
impl<'a, const O: u8> RX_FULL_W<'a, O> {
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RX_FULL_A::DIS)
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RX_FULL_A::EN)
    }
}
impl R {
    #[doc = "Bit 0 - Wake on TX FIFO Threshold Crossed Enable."]
    #[inline(always)]
    pub fn tx_thd(&self) -> TX_THD_R {
        TX_THD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake on TX FIFO Empty Enable."]
    #[inline(always)]
    pub fn tx_em(&self) -> TX_EM_R {
        TX_EM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake on RX FIFO Threshold Crossed Enable."]
    #[inline(always)]
    pub fn rx_thd(&self) -> RX_THD_R {
        RX_THD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wake on RX FIFO Full Enable."]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake on TX FIFO Threshold Crossed Enable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_thd(&mut self) -> TX_THD_W<0> {
        TX_THD_W::new(self)
    }
    #[doc = "Bit 1 - Wake on TX FIFO Empty Enable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_em(&mut self) -> TX_EM_W<1> {
        TX_EM_W::new(self)
    }
    #[doc = "Bit 2 - Wake on RX FIFO Threshold Crossed Enable."]
    #[inline(always)]
    #[must_use]
    pub fn rx_thd(&mut self) -> RX_THD_W<2> {
        RX_THD_W::new(self)
    }
    #[doc = "Bit 3 - Wake on RX FIFO Full Enable."]
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
#[doc = "Register for wake up enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wken](index.html) module"]
pub struct WKEN_SPEC;
impl crate::RegisterSpec for WKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wken::R](R) reader structure"]
impl crate::Readable for WKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wken::W](W) writer structure"]
impl crate::Writable for WKEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WKEN to value 0"]
impl crate::Resettable for WKEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
