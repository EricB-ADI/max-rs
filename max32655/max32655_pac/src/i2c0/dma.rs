#[doc = "Register `DMA` reader"]
pub struct R(crate::R<DMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA` writer"]
pub struct W(crate::W<DMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_SPEC>;
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
impl From<crate::W<DMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_EN` reader - TX channel enable."]
pub type TX_EN_R = crate::BitReader<TX_EN_A>;
#[doc = "TX channel enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_EN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<TX_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_EN_A {
        match self.bits {
            false => TX_EN_A::DIS,
            true => TX_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TX_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TX_EN_A::EN
    }
}
#[doc = "Field `TX_EN` writer - TX channel enable."]
pub type TX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SPEC, TX_EN_A, O>;
impl<'a, const O: u8> TX_EN_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_EN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_EN_A::EN)
    }
}
#[doc = "Field `RX_EN` reader - RX channel enable."]
pub type RX_EN_R = crate::BitReader<RX_EN_A>;
#[doc = "RX channel enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_EN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<RX_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_EN_A {
        match self.bits {
            false => RX_EN_A::DIS,
            true => RX_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RX_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RX_EN_A::EN
    }
}
#[doc = "Field `RX_EN` writer - RX channel enable."]
pub type RX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_SPEC, RX_EN_A, O>;
impl<'a, const O: u8> RX_EN_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RX_EN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RX_EN_A::EN)
    }
}
impl R {
    #[doc = "Bit 0 - TX channel enable."]
    #[inline(always)]
    pub fn tx_en(&self) -> TX_EN_R {
        TX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX channel enable."]
    #[inline(always)]
    pub fn rx_en(&self) -> RX_EN_R {
        RX_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX channel enable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_en(&mut self) -> TX_EN_W<0> {
        TX_EN_W::new(self)
    }
    #[doc = "Bit 1 - RX channel enable."]
    #[inline(always)]
    #[must_use]
    pub fn rx_en(&mut self) -> RX_EN_W<1> {
        RX_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma](index.html) module"]
pub struct DMA_SPEC;
impl crate::RegisterSpec for DMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma::R](R) reader structure"]
impl crate::Readable for DMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma::W](W) writer structure"]
impl crate::Writable for DMA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA to value 0"]
impl crate::Resettable for DMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
