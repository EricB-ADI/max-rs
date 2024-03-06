#[doc = "Register `MSTCTRL` reader"]
pub struct R(crate::R<MSTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSTCTRL` writer"]
pub struct W(crate::W<MSTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTCTRL_SPEC>;
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
impl From<crate::W<MSTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - Setting this bit to 1 will start a master transfer."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Setting this bit to 1 will start a master transfer."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTCTRL_SPEC, bool, O>;
#[doc = "Field `RESTART` reader - Setting this bit to 1 will generate a repeated START."]
pub type RESTART_R = crate::BitReader<bool>;
#[doc = "Field `RESTART` writer - Setting this bit to 1 will generate a repeated START."]
pub type RESTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTCTRL_SPEC, bool, O>;
#[doc = "Field `STOP` reader - Setting this bit to 1 will generate a STOP condition."]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - Setting this bit to 1 will generate a STOP condition."]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTCTRL_SPEC, bool, O>;
#[doc = "Field `EX_ADDR_EN` reader - Slave Extend Address Select."]
pub type EX_ADDR_EN_R = crate::BitReader<EX_ADDR_EN_A>;
#[doc = "Slave Extend Address Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EX_ADDR_EN_A {
    #[doc = "0: 7-bit address."]
    _7_BITS_ADDRESS = 0,
    #[doc = "1: 10-bit address."]
    _10_BITS_ADDRESS = 1,
}
impl From<EX_ADDR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EX_ADDR_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EX_ADDR_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EX_ADDR_EN_A {
        match self.bits {
            false => EX_ADDR_EN_A::_7_BITS_ADDRESS,
            true => EX_ADDR_EN_A::_10_BITS_ADDRESS,
        }
    }
    #[doc = "Checks if the value of the field is `_7_BITS_ADDRESS`"]
    #[inline(always)]
    pub fn is_7_bits_address(&self) -> bool {
        *self == EX_ADDR_EN_A::_7_BITS_ADDRESS
    }
    #[doc = "Checks if the value of the field is `_10_BITS_ADDRESS`"]
    #[inline(always)]
    pub fn is_10_bits_address(&self) -> bool {
        *self == EX_ADDR_EN_A::_10_BITS_ADDRESS
    }
}
#[doc = "Field `EX_ADDR_EN` writer - Slave Extend Address Select."]
pub type EX_ADDR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTCTRL_SPEC, EX_ADDR_EN_A, O>;
impl<'a, const O: u8> EX_ADDR_EN_W<'a, O> {
    #[doc = "7-bit address."]
    #[inline(always)]
    pub fn _7_bits_address(self) -> &'a mut W {
        self.variant(EX_ADDR_EN_A::_7_BITS_ADDRESS)
    }
    #[doc = "10-bit address."]
    #[inline(always)]
    pub fn _10_bits_address(self) -> &'a mut W {
        self.variant(EX_ADDR_EN_A::_10_BITS_ADDRESS)
    }
}
impl R {
    #[doc = "Bit 0 - Setting this bit to 1 will start a master transfer."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setting this bit to 1 will generate a repeated START."]
    #[inline(always)]
    pub fn restart(&self) -> RESTART_R {
        RESTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Setting this bit to 1 will generate a STOP condition."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Slave Extend Address Select."]
    #[inline(always)]
    pub fn ex_addr_en(&self) -> EX_ADDR_EN_R {
        EX_ADDR_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit to 1 will start a master transfer."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Setting this bit to 1 will generate a repeated START."]
    #[inline(always)]
    #[must_use]
    pub fn restart(&mut self) -> RESTART_W<1> {
        RESTART_W::new(self)
    }
    #[doc = "Bit 2 - Setting this bit to 1 will generate a STOP condition."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<2> {
        STOP_W::new(self)
    }
    #[doc = "Bit 7 - Slave Extend Address Select."]
    #[inline(always)]
    #[must_use]
    pub fn ex_addr_en(&mut self) -> EX_ADDR_EN_W<7> {
        EX_ADDR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstctrl](index.html) module"]
pub struct MSTCTRL_SPEC;
impl crate::RegisterSpec for MSTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mstctrl::R](R) reader structure"]
impl crate::Readable for MSTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mstctrl::W](W) writer structure"]
impl crate::Writable for MSTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSTCTRL to value 0"]
impl crate::Resettable for MSTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
