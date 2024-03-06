#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WR` reader - Write. This bit is automatically cleared after the operation."]
pub type WR_R = crate::BitReader<WR_A>;
#[doc = "Write. This bit is automatically cleared after the operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WR_A {
    #[doc = "0: No operation/complete."]
    COMPLETE = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<WR_A> for bool {
    #[inline(always)]
    fn from(variant: WR_A) -> Self {
        variant as u8 != 0
    }
}
impl WR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WR_A {
        match self.bits {
            false => WR_A::COMPLETE,
            true => WR_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == WR_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == WR_A::START
    }
}
#[doc = "Field `WR` writer - Write. This bit is automatically cleared after the operation."]
pub type WR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, WR_A, O>;
impl<'a, const O: u8> WR_W<'a, O> {
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(WR_A::COMPLETE)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(WR_A::START)
    }
}
#[doc = "Field `ME` reader - Mass Erase. This bit is automatically cleared after the operation."]
pub type ME_R = crate::BitReader<ME_A>;
#[doc = "Mass Erase. This bit is automatically cleared after the operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ME_A {
    #[doc = "0: No operation/complete."]
    COMPLETE = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<ME_A> for bool {
    #[inline(always)]
    fn from(variant: ME_A) -> Self {
        variant as u8 != 0
    }
}
impl ME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ME_A {
        match self.bits {
            false => ME_A::COMPLETE,
            true => ME_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == ME_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == ME_A::START
    }
}
#[doc = "Field `ME` writer - Mass Erase. This bit is automatically cleared after the operation."]
pub type ME_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, ME_A, O>;
impl<'a, const O: u8> ME_W<'a, O> {
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(ME_A::COMPLETE)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(ME_A::START)
    }
}
#[doc = "Field `PGE` reader - Page Erase. This bit is automatically cleared after the operation."]
pub type PGE_R = crate::BitReader<PGE_A>;
#[doc = "Page Erase. This bit is automatically cleared after the operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGE_A {
    #[doc = "0: No operation/complete."]
    COMPLETE = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<PGE_A> for bool {
    #[inline(always)]
    fn from(variant: PGE_A) -> Self {
        variant as u8 != 0
    }
}
impl PGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGE_A {
        match self.bits {
            false => PGE_A::COMPLETE,
            true => PGE_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == PGE_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == PGE_A::START
    }
}
#[doc = "Field `PGE` writer - Page Erase. This bit is automatically cleared after the operation."]
pub type PGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, PGE_A, O>;
impl<'a, const O: u8> PGE_W<'a, O> {
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(PGE_A::COMPLETE)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(PGE_A::START)
    }
}
#[doc = "Field `ERASE_CODE` reader - Erase Code. The ERASE_CODE must be set up property before erase operation can be initiated. These bits are automatically cleared after the operation is complete."]
pub type ERASE_CODE_R = crate::FieldReader<u8, ERASE_CODE_A>;
#[doc = "Erase Code. The ERASE_CODE must be set up property before erase operation can be initiated. These bits are automatically cleared after the operation is complete.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ERASE_CODE_A {
    #[doc = "0: No operation."]
    NOP = 0,
    #[doc = "85: Enable Page Erase."]
    ERASE_PAGE = 85,
    #[doc = "170: Enable Mass Erase. The debug port must be enabled."]
    ERASE_ALL = 170,
}
impl From<ERASE_CODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ERASE_CODE_A) -> Self {
        variant as _
    }
}
impl ERASE_CODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ERASE_CODE_A> {
        match self.bits {
            0 => Some(ERASE_CODE_A::NOP),
            85 => Some(ERASE_CODE_A::ERASE_PAGE),
            170 => Some(ERASE_CODE_A::ERASE_ALL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == ERASE_CODE_A::NOP
    }
    #[doc = "Checks if the value of the field is `ERASE_PAGE`"]
    #[inline(always)]
    pub fn is_erase_page(&self) -> bool {
        *self == ERASE_CODE_A::ERASE_PAGE
    }
    #[doc = "Checks if the value of the field is `ERASE_ALL`"]
    #[inline(always)]
    pub fn is_erase_all(&self) -> bool {
        *self == ERASE_CODE_A::ERASE_ALL
    }
}
#[doc = "Field `ERASE_CODE` writer - Erase Code. The ERASE_CODE must be set up property before erase operation can be initiated. These bits are automatically cleared after the operation is complete."]
pub type ERASE_CODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_SPEC, u8, ERASE_CODE_A, 8, O>;
impl<'a, const O: u8> ERASE_CODE_W<'a, O> {
    #[doc = "No operation."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(ERASE_CODE_A::NOP)
    }
    #[doc = "Enable Page Erase."]
    #[inline(always)]
    pub fn erase_page(self) -> &'a mut W {
        self.variant(ERASE_CODE_A::ERASE_PAGE)
    }
    #[doc = "Enable Mass Erase. The debug port must be enabled."]
    #[inline(always)]
    pub fn erase_all(self) -> &'a mut W {
        self.variant(ERASE_CODE_A::ERASE_ALL)
    }
}
#[doc = "Field `PEND` reader - Flash Pending. When Flash operation is in progress (busy), Flash reads and writes will fail. When PEND is set, write to all Flash registers, with exception of the Flash interrupt register, are ignored."]
pub type PEND_R = crate::BitReader<PEND_A>;
#[doc = "Flash Pending. When Flash operation is in progress (busy), Flash reads and writes will fail. When PEND is set, write to all Flash registers, with exception of the Flash interrupt register, are ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEND_A {
    #[doc = "0: Idle."]
    IDLE = 0,
    #[doc = "1: Busy."]
    BUSY = 1,
}
impl From<PEND_A> for bool {
    #[inline(always)]
    fn from(variant: PEND_A) -> Self {
        variant as u8 != 0
    }
}
impl PEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEND_A {
        match self.bits {
            false => PEND_A::IDLE,
            true => PEND_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == PEND_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == PEND_A::BUSY
    }
}
#[doc = "Field `LVE` reader - Low Voltage enable."]
pub type LVE_R = crate::BitReader<bool>;
#[doc = "Field `LVE` writer - Low Voltage enable."]
pub type LVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `UNLOCK` reader - Flash Unlock. The correct unlock code must be written to these four bits before any Flash write or erase operation is allowed."]
pub type UNLOCK_R = crate::FieldReader<u8, UNLOCK_A>;
#[doc = "Flash Unlock. The correct unlock code must be written to these four bits before any Flash write or erase operation is allowed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UNLOCK_A {
    #[doc = "2: Flash Unlocked."]
    UNLOCKED = 2,
    #[doc = "3: Flash Locked."]
    LOCKED = 3,
}
impl From<UNLOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: UNLOCK_A) -> Self {
        variant as _
    }
}
impl UNLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UNLOCK_A> {
        match self.bits {
            2 => Some(UNLOCK_A::UNLOCKED),
            3 => Some(UNLOCK_A::LOCKED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == UNLOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == UNLOCK_A::LOCKED
    }
}
#[doc = "Field `UNLOCK` writer - Flash Unlock. The correct unlock code must be written to these four bits before any Flash write or erase operation is allowed."]
pub type UNLOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, UNLOCK_A, 4, O>;
impl<'a, const O: u8> UNLOCK_W<'a, O> {
    #[doc = "Flash Unlocked."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(UNLOCK_A::UNLOCKED)
    }
    #[doc = "Flash Locked."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(UNLOCK_A::LOCKED)
    }
}
impl R {
    #[doc = "Bit 0 - Write. This bit is automatically cleared after the operation."]
    #[inline(always)]
    pub fn wr(&self) -> WR_R {
        WR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mass Erase. This bit is automatically cleared after the operation."]
    #[inline(always)]
    pub fn me(&self) -> ME_R {
        ME_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Page Erase. This bit is automatically cleared after the operation."]
    #[inline(always)]
    pub fn pge(&self) -> PGE_R {
        PGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Erase Code. The ERASE_CODE must be set up property before erase operation can be initiated. These bits are automatically cleared after the operation is complete."]
    #[inline(always)]
    pub fn erase_code(&self) -> ERASE_CODE_R {
        ERASE_CODE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Flash Pending. When Flash operation is in progress (busy), Flash reads and writes will fail. When PEND is set, write to all Flash registers, with exception of the Flash interrupt register, are ignored."]
    #[inline(always)]
    pub fn pend(&self) -> PEND_R {
        PEND_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Low Voltage enable."]
    #[inline(always)]
    pub fn lve(&self) -> LVE_R {
        LVE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Flash Unlock. The correct unlock code must be written to these four bits before any Flash write or erase operation is allowed."]
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Write. This bit is automatically cleared after the operation."]
    #[inline(always)]
    #[must_use]
    pub fn wr(&mut self) -> WR_W<0> {
        WR_W::new(self)
    }
    #[doc = "Bit 1 - Mass Erase. This bit is automatically cleared after the operation."]
    #[inline(always)]
    #[must_use]
    pub fn me(&mut self) -> ME_W<1> {
        ME_W::new(self)
    }
    #[doc = "Bit 2 - Page Erase. This bit is automatically cleared after the operation."]
    #[inline(always)]
    #[must_use]
    pub fn pge(&mut self) -> PGE_W<2> {
        PGE_W::new(self)
    }
    #[doc = "Bits 8:15 - Erase Code. The ERASE_CODE must be set up property before erase operation can be initiated. These bits are automatically cleared after the operation is complete."]
    #[inline(always)]
    #[must_use]
    pub fn erase_code(&mut self) -> ERASE_CODE_W<8> {
        ERASE_CODE_W::new(self)
    }
    #[doc = "Bit 25 - Low Voltage enable."]
    #[inline(always)]
    #[must_use]
    pub fn lve(&mut self) -> LVE_W<25> {
        LVE_W::new(self)
    }
    #[doc = "Bits 28:31 - Flash Unlock. The correct unlock code must be written to these four bits before any Flash write or erase operation is allowed."]
    #[inline(always)]
    #[must_use]
    pub fn unlock(&mut self) -> UNLOCK_W<28> {
        UNLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
