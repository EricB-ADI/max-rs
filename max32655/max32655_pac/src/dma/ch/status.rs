#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATUS` reader - Channel Status. This bit is used to indicate to the programmer when it is safe to change the configuration, address, and count registers for the channel. Whenever this bit is cleared by hardware, the DMA_CFG.CHEN bit is also cleared (if not cleared already)."]
pub type STATUS_R = crate::BitReader<STATUS_A>;
#[doc = "Channel Status. This bit is used to indicate to the programmer when it is safe to change the configuration, address, and count registers for the channel. Whenever this bit is cleared by hardware, the DMA_CFG.CHEN bit is also cleared (if not cleared already).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STATUS_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATUS_A {
        match self.bits {
            false => STATUS_A::DIS,
            true => STATUS_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == STATUS_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == STATUS_A::EN
    }
}
#[doc = "Field `IPEND` reader - Channel Interrupt."]
pub type IPEND_R = crate::BitReader<IPEND_A>;
#[doc = "Channel Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPEND_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<IPEND_A> for bool {
    #[inline(always)]
    fn from(variant: IPEND_A) -> Self {
        variant as u8 != 0
    }
}
impl IPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPEND_A {
        match self.bits {
            false => IPEND_A::INACTIVE,
            true => IPEND_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == IPEND_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == IPEND_A::PENDING
    }
}
#[doc = "Field `CTZ_IF` reader - Count-to-Zero (CTZ) Interrupt Flag"]
pub type CTZ_IF_R = crate::BitReader<bool>;
#[doc = "Field `CTZ_IF` writer - Count-to-Zero (CTZ) Interrupt Flag"]
pub type CTZ_IF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `RLD_IF` reader - Reload Event Interrupt Flag."]
pub type RLD_IF_R = crate::BitReader<bool>;
#[doc = "Field `RLD_IF` writer - Reload Event Interrupt Flag."]
pub type RLD_IF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `BUS_ERR` reader - Bus Error. Indicates that an AHB abort was received and the channel has been disabled."]
pub type BUS_ERR_R = crate::BitReader<bool>;
#[doc = "Field `BUS_ERR` writer - Bus Error. Indicates that an AHB abort was received and the channel has been disabled."]
pub type BUS_ERR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `TO_IF` reader - Time-Out Event Interrupt Flag."]
pub type TO_IF_R = crate::BitReader<bool>;
#[doc = "Field `TO_IF` writer - Time-Out Event Interrupt Flag."]
pub type TO_IF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Channel Status. This bit is used to indicate to the programmer when it is safe to change the configuration, address, and count registers for the channel. Whenever this bit is cleared by hardware, the DMA_CFG.CHEN bit is also cleared (if not cleared already)."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Interrupt."]
    #[inline(always)]
    pub fn ipend(&self) -> IPEND_R {
        IPEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Count-to-Zero (CTZ) Interrupt Flag"]
    #[inline(always)]
    pub fn ctz_if(&self) -> CTZ_IF_R {
        CTZ_IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reload Event Interrupt Flag."]
    #[inline(always)]
    pub fn rld_if(&self) -> RLD_IF_R {
        RLD_IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bus Error. Indicates that an AHB abort was received and the channel has been disabled."]
    #[inline(always)]
    pub fn bus_err(&self) -> BUS_ERR_R {
        BUS_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Time-Out Event Interrupt Flag."]
    #[inline(always)]
    pub fn to_if(&self) -> TO_IF_R {
        TO_IF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Count-to-Zero (CTZ) Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctz_if(&mut self) -> CTZ_IF_W<2> {
        CTZ_IF_W::new(self)
    }
    #[doc = "Bit 3 - Reload Event Interrupt Flag."]
    #[inline(always)]
    #[must_use]
    pub fn rld_if(&mut self) -> RLD_IF_W<3> {
        RLD_IF_W::new(self)
    }
    #[doc = "Bit 4 - Bus Error. Indicates that an AHB abort was received and the channel has been disabled."]
    #[inline(always)]
    #[must_use]
    pub fn bus_err(&mut self) -> BUS_ERR_W<4> {
        BUS_ERR_W::new(self)
    }
    #[doc = "Bit 6 - Time-Out Event Interrupt Flag."]
    #[inline(always)]
    #[must_use]
    pub fn to_if(&mut self) -> TO_IF_W<6> {
        TO_IF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x5c;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
