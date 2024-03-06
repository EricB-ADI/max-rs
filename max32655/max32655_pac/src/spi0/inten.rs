#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_THD` reader - TX FIFO Threshold interrupt enable."]
pub type TX_THD_R = crate::BitReader<TX_THD_A>;
#[doc = "TX FIFO Threshold interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_THD_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
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
#[doc = "Field `TX_THD` writer - TX FIFO Threshold interrupt enable."]
pub type TX_THD_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, TX_THD_A, O>;
impl<'a, const O: u8> TX_THD_W<'a, O> {
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_THD_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_THD_A::EN)
    }
}
#[doc = "Field `TX_EM` reader - TX FIFO Empty interrupt enable."]
pub type TX_EM_R = crate::BitReader<TX_EM_A>;
#[doc = "TX FIFO Empty interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_EM_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
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
#[doc = "Field `TX_EM` writer - TX FIFO Empty interrupt enable."]
pub type TX_EM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, TX_EM_A, O>;
impl<'a, const O: u8> TX_EM_W<'a, O> {
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_EM_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_EM_A::EN)
    }
}
#[doc = "Field `RX_THD` reader - RX FIFO Threshold Crossed interrupt enable."]
pub type RX_THD_R = crate::BitReader<RX_THD_A>;
#[doc = "RX FIFO Threshold Crossed interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_THD_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
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
#[doc = "Field `RX_THD` writer - RX FIFO Threshold Crossed interrupt enable."]
pub type RX_THD_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RX_THD_A, O>;
impl<'a, const O: u8> RX_THD_W<'a, O> {
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RX_THD_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RX_THD_A::EN)
    }
}
#[doc = "Field `RX_FULL` reader - RX FIFO FULL interrupt enable."]
pub type RX_FULL_R = crate::BitReader<RX_FULL_A>;
#[doc = "RX FIFO FULL interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FULL_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
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
#[doc = "Field `RX_FULL` writer - RX FIFO FULL interrupt enable."]
pub type RX_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RX_FULL_A, O>;
impl<'a, const O: u8> RX_FULL_W<'a, O> {
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RX_FULL_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RX_FULL_A::EN)
    }
}
#[doc = "Field `SSA` reader - Slave Select Asserted interrupt enable."]
pub type SSA_R = crate::BitReader<SSA_A>;
#[doc = "Slave Select Asserted interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSA_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
    EN = 1,
}
impl From<SSA_A> for bool {
    #[inline(always)]
    fn from(variant: SSA_A) -> Self {
        variant as u8 != 0
    }
}
impl SSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSA_A {
        match self.bits {
            false => SSA_A::DIS,
            true => SSA_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SSA_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SSA_A::EN
    }
}
#[doc = "Field `SSA` writer - Slave Select Asserted interrupt enable."]
pub type SSA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, SSA_A, O>;
impl<'a, const O: u8> SSA_W<'a, O> {
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SSA_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SSA_A::EN)
    }
}
#[doc = "Field `SSD` reader - Slave Select Deasserted interrupt enable."]
pub type SSD_R = crate::BitReader<SSD_A>;
#[doc = "Slave Select Deasserted interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSD_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
    EN = 1,
}
impl From<SSD_A> for bool {
    #[inline(always)]
    fn from(variant: SSD_A) -> Self {
        variant as u8 != 0
    }
}
impl SSD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSD_A {
        match self.bits {
            false => SSD_A::DIS,
            true => SSD_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SSD_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SSD_A::EN
    }
}
#[doc = "Field `SSD` writer - Slave Select Deasserted interrupt enable."]
pub type SSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, SSD_A, O>;
impl<'a, const O: u8> SSD_W<'a, O> {
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SSD_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SSD_A::EN)
    }
}
#[doc = "Field `FAULT` reader - Multi-Master Mode Fault interrupt enable."]
pub type FAULT_R = crate::BitReader<FAULT_A>;
#[doc = "Multi-Master Mode Fault interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAULT_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
    EN = 1,
}
impl From<FAULT_A> for bool {
    #[inline(always)]
    fn from(variant: FAULT_A) -> Self {
        variant as u8 != 0
    }
}
impl FAULT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULT_A {
        match self.bits {
            false => FAULT_A::DIS,
            true => FAULT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FAULT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == FAULT_A::EN
    }
}
#[doc = "Field `FAULT` writer - Multi-Master Mode Fault interrupt enable."]
pub type FAULT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, FAULT_A, O>;
impl<'a, const O: u8> FAULT_W<'a, O> {
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FAULT_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FAULT_A::EN)
    }
}
#[doc = "Field `ABORT` reader - Slave Abort Detected interrupt enable."]
pub type ABORT_R = crate::BitReader<ABORT_A>;
#[doc = "Slave Abort Detected interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
    EN = 1,
}
impl From<ABORT_A> for bool {
    #[inline(always)]
    fn from(variant: ABORT_A) -> Self {
        variant as u8 != 0
    }
}
impl ABORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABORT_A {
        match self.bits {
            false => ABORT_A::DIS,
            true => ABORT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ABORT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ABORT_A::EN
    }
}
#[doc = "Field `ABORT` writer - Slave Abort Detected interrupt enable."]
pub type ABORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, ABORT_A, O>;
impl<'a, const O: u8> ABORT_W<'a, O> {
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ABORT_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ABORT_A::EN)
    }
}
#[doc = "Field `MST_DONE` reader - Master Done interrupt enable."]
pub type MST_DONE_R = crate::BitReader<MST_DONE_A>;
#[doc = "Master Done interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MST_DONE_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
    EN = 1,
}
impl From<MST_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: MST_DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl MST_DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MST_DONE_A {
        match self.bits {
            false => MST_DONE_A::DIS,
            true => MST_DONE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == MST_DONE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == MST_DONE_A::EN
    }
}
#[doc = "Field `MST_DONE` writer - Master Done interrupt enable."]
pub type MST_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, MST_DONE_A, O>;
impl<'a, const O: u8> MST_DONE_W<'a, O> {
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(MST_DONE_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(MST_DONE_A::EN)
    }
}
#[doc = "Field `TX_OV` reader - Transmit FIFO Overrun interrupt enable."]
pub type TX_OV_R = crate::BitReader<TX_OV_A>;
#[doc = "Transmit FIFO Overrun interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_OV_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
    EN = 1,
}
impl From<TX_OV_A> for bool {
    #[inline(always)]
    fn from(variant: TX_OV_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_OV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_OV_A {
        match self.bits {
            false => TX_OV_A::DIS,
            true => TX_OV_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TX_OV_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TX_OV_A::EN
    }
}
#[doc = "Field `TX_OV` writer - Transmit FIFO Overrun interrupt enable."]
pub type TX_OV_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, TX_OV_A, O>;
impl<'a, const O: u8> TX_OV_W<'a, O> {
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_OV_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_OV_A::EN)
    }
}
#[doc = "Field `TX_UN` reader - Transmit FIFO Underrun interrupt enable."]
pub type TX_UN_R = crate::BitReader<TX_UN_A>;
#[doc = "Transmit FIFO Underrun interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_UN_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
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
#[doc = "Field `TX_UN` writer - Transmit FIFO Underrun interrupt enable."]
pub type TX_UN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, TX_UN_A, O>;
impl<'a, const O: u8> TX_UN_W<'a, O> {
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_UN_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_UN_A::EN)
    }
}
#[doc = "Field `RX_OV` reader - Receive FIFO Overrun interrupt enable."]
pub type RX_OV_R = crate::BitReader<RX_OV_A>;
#[doc = "Receive FIFO Overrun interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_OV_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
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
#[doc = "Field `RX_OV` writer - Receive FIFO Overrun interrupt enable."]
pub type RX_OV_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RX_OV_A, O>;
impl<'a, const O: u8> RX_OV_W<'a, O> {
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RX_OV_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RX_OV_A::EN)
    }
}
#[doc = "Field `RX_UN` reader - Receive FIFO Underrun interrupt enable."]
pub type RX_UN_R = crate::BitReader<RX_UN_A>;
#[doc = "Receive FIFO Underrun interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_UN_A {
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
    #[doc = "1: Interrupt is enabled."]
    EN = 1,
}
impl From<RX_UN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_UN_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_UN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_UN_A {
        match self.bits {
            false => RX_UN_A::DIS,
            true => RX_UN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RX_UN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RX_UN_A::EN
    }
}
#[doc = "Field `RX_UN` writer - Receive FIFO Underrun interrupt enable."]
pub type RX_UN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RX_UN_A, O>;
impl<'a, const O: u8> RX_UN_W<'a, O> {
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RX_UN_A::DIS)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RX_UN_A::EN)
    }
}
impl R {
    #[doc = "Bit 0 - TX FIFO Threshold interrupt enable."]
    #[inline(always)]
    pub fn tx_thd(&self) -> TX_THD_R {
        TX_THD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Empty interrupt enable."]
    #[inline(always)]
    pub fn tx_em(&self) -> TX_EM_R {
        TX_EM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO Threshold Crossed interrupt enable."]
    #[inline(always)]
    pub fn rx_thd(&self) -> RX_THD_R {
        RX_THD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO FULL interrupt enable."]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Select Asserted interrupt enable."]
    #[inline(always)]
    pub fn ssa(&self) -> SSA_R {
        SSA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Slave Select Deasserted interrupt enable."]
    #[inline(always)]
    pub fn ssd(&self) -> SSD_R {
        SSD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Multi-Master Mode Fault interrupt enable."]
    #[inline(always)]
    pub fn fault(&self) -> FAULT_R {
        FAULT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Slave Abort Detected interrupt enable."]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Master Done interrupt enable."]
    #[inline(always)]
    pub fn mst_done(&self) -> MST_DONE_R {
        MST_DONE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit FIFO Overrun interrupt enable."]
    #[inline(always)]
    pub fn tx_ov(&self) -> TX_OV_R {
        TX_OV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit FIFO Underrun interrupt enable."]
    #[inline(always)]
    pub fn tx_un(&self) -> TX_UN_R {
        TX_UN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive FIFO Overrun interrupt enable."]
    #[inline(always)]
    pub fn rx_ov(&self) -> RX_OV_R {
        RX_OV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO Underrun interrupt enable."]
    #[inline(always)]
    pub fn rx_un(&self) -> RX_UN_R {
        RX_UN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX FIFO Threshold interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_thd(&mut self) -> TX_THD_W<0> {
        TX_THD_W::new(self)
    }
    #[doc = "Bit 1 - TX FIFO Empty interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_em(&mut self) -> TX_EM_W<1> {
        TX_EM_W::new(self)
    }
    #[doc = "Bit 2 - RX FIFO Threshold Crossed interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn rx_thd(&mut self) -> RX_THD_W<2> {
        RX_THD_W::new(self)
    }
    #[doc = "Bit 3 - RX FIFO FULL interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn rx_full(&mut self) -> RX_FULL_W<3> {
        RX_FULL_W::new(self)
    }
    #[doc = "Bit 4 - Slave Select Asserted interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ssa(&mut self) -> SSA_W<4> {
        SSA_W::new(self)
    }
    #[doc = "Bit 5 - Slave Select Deasserted interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ssd(&mut self) -> SSD_W<5> {
        SSD_W::new(self)
    }
    #[doc = "Bit 8 - Multi-Master Mode Fault interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn fault(&mut self) -> FAULT_W<8> {
        FAULT_W::new(self)
    }
    #[doc = "Bit 9 - Slave Abort Detected interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<9> {
        ABORT_W::new(self)
    }
    #[doc = "Bit 11 - Master Done interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn mst_done(&mut self) -> MST_DONE_W<11> {
        MST_DONE_W::new(self)
    }
    #[doc = "Bit 12 - Transmit FIFO Overrun interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ov(&mut self) -> TX_OV_W<12> {
        TX_OV_W::new(self)
    }
    #[doc = "Bit 13 - Transmit FIFO Underrun interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_un(&mut self) -> TX_UN_W<13> {
        TX_UN_W::new(self)
    }
    #[doc = "Bit 14 - Receive FIFO Overrun interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ov(&mut self) -> RX_OV_W<14> {
        RX_OV_W::new(self)
    }
    #[doc = "Bit 15 - Receive FIFO Underrun interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn rx_un(&mut self) -> RX_UN_W<15> {
        RX_UN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for enabling interrupts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
