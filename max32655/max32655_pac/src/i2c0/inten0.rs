#[doc = "Register `INTEN0` reader"]
pub struct R(crate::R<INTEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN0` writer"]
pub struct W(crate::W<INTEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN0_SPEC>;
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
impl From<crate::W<INTEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DONE` reader - Transfer Done Interrupt Enable."]
pub type DONE_R = crate::BitReader<DONE_A>;
#[doc = "Transfer Done Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DONE_A {
    #[doc = "0: Interrupt disabled."]
    DIS = 0,
    #[doc = "1: Interrupt enabled when DONE = 1."]
    EN = 1,
}
impl From<DONE_A> for bool {
    #[inline(always)]
    fn from(variant: DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONE_A {
        match self.bits {
            false => DONE_A::DIS,
            true => DONE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DONE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DONE_A::EN
    }
}
#[doc = "Field `DONE` writer - Transfer Done Interrupt Enable."]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN0_SPEC, DONE_A, O>;
impl<'a, const O: u8> DONE_W<'a, O> {
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DONE_A::DIS)
    }
    #[doc = "Interrupt enabled when DONE = 1."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DONE_A::EN)
    }
}
#[doc = "Field `IRXM` reader - Description not available."]
pub type IRXM_R = crate::BitReader<IRXM_A>;
#[doc = "Description not available.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRXM_A {
    #[doc = "0: Interrupt disabled."]
    DIS = 0,
    #[doc = "1: Interrupt enabled when RX_MODE = 1."]
    EN = 1,
}
impl From<IRXM_A> for bool {
    #[inline(always)]
    fn from(variant: IRXM_A) -> Self {
        variant as u8 != 0
    }
}
impl IRXM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRXM_A {
        match self.bits {
            false => IRXM_A::DIS,
            true => IRXM_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == IRXM_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == IRXM_A::EN
    }
}
#[doc = "Field `IRXM` writer - Description not available."]
pub type IRXM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN0_SPEC, IRXM_A, O>;
impl<'a, const O: u8> IRXM_W<'a, O> {
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(IRXM_A::DIS)
    }
    #[doc = "Interrupt enabled when RX_MODE = 1."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(IRXM_A::EN)
    }
}
#[doc = "Field `GC_ADDR_MATCH` reader - Slave mode general call address match received input enable."]
pub type GC_ADDR_MATCH_R = crate::BitReader<GC_ADDR_MATCH_A>;
#[doc = "Slave mode general call address match received input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GC_ADDR_MATCH_A {
    #[doc = "0: Interrupt disabled."]
    DIS = 0,
    #[doc = "1: Interrupt enabled when GEN_CTRL_ADDR = 1."]
    EN = 1,
}
impl From<GC_ADDR_MATCH_A> for bool {
    #[inline(always)]
    fn from(variant: GC_ADDR_MATCH_A) -> Self {
        variant as u8 != 0
    }
}
impl GC_ADDR_MATCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GC_ADDR_MATCH_A {
        match self.bits {
            false => GC_ADDR_MATCH_A::DIS,
            true => GC_ADDR_MATCH_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GC_ADDR_MATCH_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GC_ADDR_MATCH_A::EN
    }
}
#[doc = "Field `GC_ADDR_MATCH` writer - Slave mode general call address match received input enable."]
pub type GC_ADDR_MATCH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTEN0_SPEC, GC_ADDR_MATCH_A, O>;
impl<'a, const O: u8> GC_ADDR_MATCH_W<'a, O> {
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GC_ADDR_MATCH_A::DIS)
    }
    #[doc = "Interrupt enabled when GEN_CTRL_ADDR = 1."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(GC_ADDR_MATCH_A::EN)
    }
}
#[doc = "Field `ADDR_MATCH` reader - Slave mode incoming address match interrupt."]
pub type ADDR_MATCH_R = crate::BitReader<ADDR_MATCH_A>;
#[doc = "Slave mode incoming address match interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR_MATCH_A {
    #[doc = "0: Interrupt disabled."]
    DIS = 0,
    #[doc = "1: Interrupt enabled when ADDR_MATCH = 1."]
    EN = 1,
}
impl From<ADDR_MATCH_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR_MATCH_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDR_MATCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR_MATCH_A {
        match self.bits {
            false => ADDR_MATCH_A::DIS,
            true => ADDR_MATCH_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ADDR_MATCH_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ADDR_MATCH_A::EN
    }
}
#[doc = "Field `ADDR_MATCH` writer - Slave mode incoming address match interrupt."]
pub type ADDR_MATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN0_SPEC, ADDR_MATCH_A, O>;
impl<'a, const O: u8> ADDR_MATCH_W<'a, O> {
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ADDR_MATCH_A::DIS)
    }
    #[doc = "Interrupt enabled when ADDR_MATCH = 1."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ADDR_MATCH_A::EN)
    }
}
#[doc = "Field `RX_THD` reader - RX FIFO Above Treshold Level Interrupt Enable."]
pub type RX_THD_R = crate::BitReader<RX_THD_A>;
#[doc = "RX FIFO Above Treshold Level Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_THD_A {
    #[doc = "0: Interrupt disabled."]
    DIS = 0,
    #[doc = "1: Interrupt enabled."]
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
#[doc = "Field `RX_THD` writer - RX FIFO Above Treshold Level Interrupt Enable."]
pub type RX_THD_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN0_SPEC, RX_THD_A, O>;
impl<'a, const O: u8> RX_THD_W<'a, O> {
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RX_THD_A::DIS)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RX_THD_A::EN)
    }
}
#[doc = "Field `TX_THD` reader - TX FIFO Below Treshold Level Interrupt Enable."]
pub type TX_THD_R = crate::BitReader<TX_THD_A>;
#[doc = "TX FIFO Below Treshold Level Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_THD_A {
    #[doc = "0: Interrupt disabled."]
    DIS = 0,
    #[doc = "1: Interrupt enabled."]
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
#[doc = "Field `TX_THD` writer - TX FIFO Below Treshold Level Interrupt Enable."]
pub type TX_THD_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN0_SPEC, TX_THD_A, O>;
impl<'a, const O: u8> TX_THD_W<'a, O> {
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_THD_A::DIS)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_THD_A::EN)
    }
}
#[doc = "Field `STOP` reader - Stop Interrupt Enable"]
pub type STOP_R = crate::BitReader<STOP_A>;
#[doc = "Stop Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_A {
    #[doc = "0: Interrupt disabled."]
    DIS = 0,
    #[doc = "1: Interrupt enabled when STOP = 1."]
    EN = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::DIS,
            true => STOP_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == STOP_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == STOP_A::EN
    }
}
#[doc = "Field `STOP` writer - Stop Interrupt Enable"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN0_SPEC, STOP_A, O>;
impl<'a, const O: u8> STOP_W<'a, O> {
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(STOP_A::DIS)
    }
    #[doc = "Interrupt enabled when STOP = 1."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(STOP_A::EN)
    }
}
#[doc = "Field `ADDR_ACK` reader - Received Address ACK from Slave Interrupt."]
pub type ADDR_ACK_R = crate::BitReader<ADDR_ACK_A>;
#[doc = "Received Address ACK from Slave Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR_ACK_A {
    #[doc = "0: Interrupt disabled."]
    DIS = 0,
    #[doc = "1: Interrupt enabled."]
    EN = 1,
}
impl From<ADDR_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDR_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR_ACK_A {
        match self.bits {
            false => ADDR_ACK_A::DIS,
            true => ADDR_ACK_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ADDR_ACK_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ADDR_ACK_A::EN
    }
}
#[doc = "Field `ADDR_ACK` writer - Received Address ACK from Slave Interrupt."]
pub type ADDR_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN0_SPEC, ADDR_ACK_A, O>;
impl<'a, const O: u8> ADDR_ACK_W<'a, O> {
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ADDR_ACK_A::DIS)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ADDR_ACK_A::EN)
    }
}
#[doc = "Field `ARB_ERR` reader - Master Mode Arbitration Lost Interrupt."]
pub type ARB_ERR_R = crate::BitReader<ARB_ERR_A>;
#[doc = "Master Mode Arbitration Lost Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARB_ERR_A {
    #[doc = "0: Interrupt disabled."]
    DIS = 0,
    #[doc = "1: Interrupt enabled."]
    EN = 1,
}
impl From<ARB_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ARB_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl ARB_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARB_ERR_A {
        match self.bits {
            false => ARB_ERR_A::DIS,
            true => ARB_ERR_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ARB_ERR_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ARB_ERR_A::EN
    }
}
#[doc = "Field `ARB_ERR` writer - Master Mode Arbitration Lost Interrupt."]
pub type ARB_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN0_SPEC, ARB_ERR_A, O>;
impl<'a, const O: u8> ARB_ERR_W<'a, O> {
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ARB_ERR_A::DIS)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ARB_ERR_A::EN)
    }
}
#[doc = "Field `TO_ERR` reader - Timeout Error Interrupt Enable."]
pub type TO_ERR_R = crate::BitReader<TO_ERR_A>;
#[doc = "Timeout Error Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TO_ERR_A {
    #[doc = "0: Interrupt disabled."]
    DIS = 0,
    #[doc = "1: Interrupt enabled."]
    EN = 1,
}
impl From<TO_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: TO_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl TO_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TO_ERR_A {
        match self.bits {
            false => TO_ERR_A::DIS,
            true => TO_ERR_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TO_ERR_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TO_ERR_A::EN
    }
}
#[doc = "Field `TO_ERR` writer - Timeout Error Interrupt Enable."]
pub type TO_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN0_SPEC, TO_ERR_A, O>;
impl<'a, const O: u8> TO_ERR_W<'a, O> {
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TO_ERR_A::DIS)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TO_ERR_A::EN)
    }
}
#[doc = "Field `ADDR_NACK_ERR` reader - Master Mode Address NACK Received Interrupt."]
pub type ADDR_NACK_ERR_R = crate::BitReader<ADDR_NACK_ERR_A>;
#[doc = "Master Mode Address NACK Received Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR_NACK_ERR_A {
    #[doc = "0: Interrupt disabled."]
    DIS = 0,
    #[doc = "1: Interrupt enabled."]
    EN = 1,
}
impl From<ADDR_NACK_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR_NACK_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDR_NACK_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR_NACK_ERR_A {
        match self.bits {
            false => ADDR_NACK_ERR_A::DIS,
            true => ADDR_NACK_ERR_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ADDR_NACK_ERR_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ADDR_NACK_ERR_A::EN
    }
}
#[doc = "Field `ADDR_NACK_ERR` writer - Master Mode Address NACK Received Interrupt."]
pub type ADDR_NACK_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTEN0_SPEC, ADDR_NACK_ERR_A, O>;
impl<'a, const O: u8> ADDR_NACK_ERR_W<'a, O> {
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ADDR_NACK_ERR_A::DIS)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ADDR_NACK_ERR_A::EN)
    }
}
#[doc = "Field `DATA_ERR` reader - Master Mode Data NACK Received Interrupt."]
pub type DATA_ERR_R = crate::BitReader<DATA_ERR_A>;
#[doc = "Master Mode Data NACK Received Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATA_ERR_A {
    #[doc = "0: Interrupt disabled."]
    DIS = 0,
    #[doc = "1: Interrupt enabled."]
    EN = 1,
}
impl From<DATA_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl DATA_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_ERR_A {
        match self.bits {
            false => DATA_ERR_A::DIS,
            true => DATA_ERR_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DATA_ERR_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DATA_ERR_A::EN
    }
}
#[doc = "Field `DATA_ERR` writer - Master Mode Data NACK Received Interrupt."]
pub type DATA_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN0_SPEC, DATA_ERR_A, O>;
impl<'a, const O: u8> DATA_ERR_W<'a, O> {
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DATA_ERR_A::DIS)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DATA_ERR_A::EN)
    }
}
#[doc = "Field `DNR_ERR` reader - Slave Mode Do Not Respond Interrupt."]
pub type DNR_ERR_R = crate::BitReader<DNR_ERR_A>;
#[doc = "Slave Mode Do Not Respond Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DNR_ERR_A {
    #[doc = "0: Interrupt disabled."]
    DIS = 0,
    #[doc = "1: Interrupt enabled."]
    EN = 1,
}
impl From<DNR_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: DNR_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl DNR_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DNR_ERR_A {
        match self.bits {
            false => DNR_ERR_A::DIS,
            true => DNR_ERR_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DNR_ERR_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DNR_ERR_A::EN
    }
}
#[doc = "Field `DNR_ERR` writer - Slave Mode Do Not Respond Interrupt."]
pub type DNR_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN0_SPEC, DNR_ERR_A, O>;
impl<'a, const O: u8> DNR_ERR_W<'a, O> {
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DNR_ERR_A::DIS)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DNR_ERR_A::EN)
    }
}
#[doc = "Field `START_ERR` reader - Out of Sequence START condition detected interrupt."]
pub type START_ERR_R = crate::BitReader<START_ERR_A>;
#[doc = "Out of Sequence START condition detected interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_ERR_A {
    #[doc = "0: Interrupt disabled."]
    DIS = 0,
    #[doc = "1: Interrupt enabled."]
    EN = 1,
}
impl From<START_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: START_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl START_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_ERR_A {
        match self.bits {
            false => START_ERR_A::DIS,
            true => START_ERR_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == START_ERR_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == START_ERR_A::EN
    }
}
#[doc = "Field `START_ERR` writer - Out of Sequence START condition detected interrupt."]
pub type START_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN0_SPEC, START_ERR_A, O>;
impl<'a, const O: u8> START_ERR_W<'a, O> {
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(START_ERR_A::DIS)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(START_ERR_A::EN)
    }
}
#[doc = "Field `STOP_ERR` reader - Out of Sequence STOP condition detected interrupt."]
pub type STOP_ERR_R = crate::BitReader<STOP_ERR_A>;
#[doc = "Out of Sequence STOP condition detected interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_ERR_A {
    #[doc = "0: Interrupt disabled."]
    DIS = 0,
    #[doc = "1: Interrupt enabled."]
    EN = 1,
}
impl From<STOP_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl STOP_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_ERR_A {
        match self.bits {
            false => STOP_ERR_A::DIS,
            true => STOP_ERR_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == STOP_ERR_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == STOP_ERR_A::EN
    }
}
#[doc = "Field `STOP_ERR` writer - Out of Sequence STOP condition detected interrupt."]
pub type STOP_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN0_SPEC, STOP_ERR_A, O>;
impl<'a, const O: u8> STOP_ERR_W<'a, O> {
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(STOP_ERR_A::DIS)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(STOP_ERR_A::EN)
    }
}
#[doc = "Field `TX_LOCKOUT` reader - TX FIFO Locked Out Interrupt."]
pub type TX_LOCKOUT_R = crate::BitReader<bool>;
#[doc = "Field `TX_LOCKOUT` writer - TX FIFO Locked Out Interrupt."]
pub type TX_LOCKOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN0_SPEC, bool, O>;
#[doc = "Field `MAMI` reader - Multiple Address Match Interrupt"]
pub type MAMI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAMI` writer - Multiple Address Match Interrupt"]
pub type MAMI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTEN0_SPEC, u8, u8, 6, O>;
#[doc = "Field `RD_ADDR_MATCH` reader - Slave Read Address Match Interrupt"]
pub type RD_ADDR_MATCH_R = crate::BitReader<bool>;
#[doc = "Field `RD_ADDR_MATCH` writer - Slave Read Address Match Interrupt"]
pub type RD_ADDR_MATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN0_SPEC, bool, O>;
#[doc = "Field `WR_ADDR_MATCH` reader - Slave Write Address Match Interrupt"]
pub type WR_ADDR_MATCH_R = crate::BitReader<bool>;
#[doc = "Field `WR_ADDR_MATCH` writer - Slave Write Address Match Interrupt"]
pub type WR_ADDR_MATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transfer Done Interrupt Enable."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Description not available."]
    #[inline(always)]
    pub fn irxm(&self) -> IRXM_R {
        IRXM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave mode general call address match received input enable."]
    #[inline(always)]
    pub fn gc_addr_match(&self) -> GC_ADDR_MATCH_R {
        GC_ADDR_MATCH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave mode incoming address match interrupt."]
    #[inline(always)]
    pub fn addr_match(&self) -> ADDR_MATCH_R {
        ADDR_MATCH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX FIFO Above Treshold Level Interrupt Enable."]
    #[inline(always)]
    pub fn rx_thd(&self) -> RX_THD_R {
        RX_THD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX FIFO Below Treshold Level Interrupt Enable."]
    #[inline(always)]
    pub fn tx_thd(&self) -> TX_THD_R {
        TX_THD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stop Interrupt Enable"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Received Address ACK from Slave Interrupt."]
    #[inline(always)]
    pub fn addr_ack(&self) -> ADDR_ACK_R {
        ADDR_ACK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Master Mode Arbitration Lost Interrupt."]
    #[inline(always)]
    pub fn arb_err(&self) -> ARB_ERR_R {
        ARB_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timeout Error Interrupt Enable."]
    #[inline(always)]
    pub fn to_err(&self) -> TO_ERR_R {
        TO_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Master Mode Address NACK Received Interrupt."]
    #[inline(always)]
    pub fn addr_nack_err(&self) -> ADDR_NACK_ERR_R {
        ADDR_NACK_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Master Mode Data NACK Received Interrupt."]
    #[inline(always)]
    pub fn data_err(&self) -> DATA_ERR_R {
        DATA_ERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Slave Mode Do Not Respond Interrupt."]
    #[inline(always)]
    pub fn dnr_err(&self) -> DNR_ERR_R {
        DNR_ERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Out of Sequence START condition detected interrupt."]
    #[inline(always)]
    pub fn start_err(&self) -> START_ERR_R {
        START_ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Out of Sequence STOP condition detected interrupt."]
    #[inline(always)]
    pub fn stop_err(&self) -> STOP_ERR_R {
        STOP_ERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TX FIFO Locked Out Interrupt."]
    #[inline(always)]
    pub fn tx_lockout(&self) -> TX_LOCKOUT_R {
        TX_LOCKOUT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Multiple Address Match Interrupt"]
    #[inline(always)]
    pub fn mami(&self) -> MAMI_R {
        MAMI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Slave Read Address Match Interrupt"]
    #[inline(always)]
    pub fn rd_addr_match(&self) -> RD_ADDR_MATCH_R {
        RD_ADDR_MATCH_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Slave Write Address Match Interrupt"]
    #[inline(always)]
    pub fn wr_addr_match(&self) -> WR_ADDR_MATCH_R {
        WR_ADDR_MATCH_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Done Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<0> {
        DONE_W::new(self)
    }
    #[doc = "Bit 1 - Description not available."]
    #[inline(always)]
    #[must_use]
    pub fn irxm(&mut self) -> IRXM_W<1> {
        IRXM_W::new(self)
    }
    #[doc = "Bit 2 - Slave mode general call address match received input enable."]
    #[inline(always)]
    #[must_use]
    pub fn gc_addr_match(&mut self) -> GC_ADDR_MATCH_W<2> {
        GC_ADDR_MATCH_W::new(self)
    }
    #[doc = "Bit 3 - Slave mode incoming address match interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn addr_match(&mut self) -> ADDR_MATCH_W<3> {
        ADDR_MATCH_W::new(self)
    }
    #[doc = "Bit 4 - RX FIFO Above Treshold Level Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn rx_thd(&mut self) -> RX_THD_W<4> {
        RX_THD_W::new(self)
    }
    #[doc = "Bit 5 - TX FIFO Below Treshold Level Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_thd(&mut self) -> TX_THD_W<5> {
        TX_THD_W::new(self)
    }
    #[doc = "Bit 6 - Stop Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<6> {
        STOP_W::new(self)
    }
    #[doc = "Bit 7 - Received Address ACK from Slave Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn addr_ack(&mut self) -> ADDR_ACK_W<7> {
        ADDR_ACK_W::new(self)
    }
    #[doc = "Bit 8 - Master Mode Arbitration Lost Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn arb_err(&mut self) -> ARB_ERR_W<8> {
        ARB_ERR_W::new(self)
    }
    #[doc = "Bit 9 - Timeout Error Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn to_err(&mut self) -> TO_ERR_W<9> {
        TO_ERR_W::new(self)
    }
    #[doc = "Bit 10 - Master Mode Address NACK Received Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn addr_nack_err(&mut self) -> ADDR_NACK_ERR_W<10> {
        ADDR_NACK_ERR_W::new(self)
    }
    #[doc = "Bit 11 - Master Mode Data NACK Received Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn data_err(&mut self) -> DATA_ERR_W<11> {
        DATA_ERR_W::new(self)
    }
    #[doc = "Bit 12 - Slave Mode Do Not Respond Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dnr_err(&mut self) -> DNR_ERR_W<12> {
        DNR_ERR_W::new(self)
    }
    #[doc = "Bit 13 - Out of Sequence START condition detected interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn start_err(&mut self) -> START_ERR_W<13> {
        START_ERR_W::new(self)
    }
    #[doc = "Bit 14 - Out of Sequence STOP condition detected interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn stop_err(&mut self) -> STOP_ERR_W<14> {
        STOP_ERR_W::new(self)
    }
    #[doc = "Bit 15 - TX FIFO Locked Out Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_lockout(&mut self) -> TX_LOCKOUT_W<15> {
        TX_LOCKOUT_W::new(self)
    }
    #[doc = "Bits 16:21 - Multiple Address Match Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mami(&mut self) -> MAMI_W<16> {
        MAMI_W::new(self)
    }
    #[doc = "Bit 22 - Slave Read Address Match Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rd_addr_match(&mut self) -> RD_ADDR_MATCH_W<22> {
        RD_ADDR_MATCH_W::new(self)
    }
    #[doc = "Bit 23 - Slave Write Address Match Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wr_addr_match(&mut self) -> WR_ADDR_MATCH_W<23> {
        WR_ADDR_MATCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten0](index.html) module"]
pub struct INTEN0_SPEC;
impl crate::RegisterSpec for INTEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten0::R](R) reader structure"]
impl crate::Readable for INTEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten0::W](W) writer structure"]
impl crate::Writable for INTEN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN0 to value 0"]
impl crate::Resettable for INTEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
