#[doc = "Register `SSTIME` reader"]
pub struct R(crate::R<SSTIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSTIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSTIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSTIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSTIME` writer"]
pub struct W(crate::W<SSTIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSTIME_SPEC>;
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
impl From<crate::W<SSTIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSTIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRE` reader - Slave Select Pre delay 1."]
pub type PRE_R = crate::FieldReader<u8, PRE_A>;
#[doc = "Slave Select Pre delay 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRE_A {
    #[doc = "0: 256 system clocks between SS active and first serial clock edge."]
    _256 = 0,
}
impl From<PRE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRE_A) -> Self {
        variant as _
    }
}
impl PRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRE_A> {
        match self.bits {
            0 => Some(PRE_A::_256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == PRE_A::_256
    }
}
#[doc = "Field `PRE` writer - Slave Select Pre delay 1."]
pub type PRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSTIME_SPEC, u8, PRE_A, 8, O>;
impl<'a, const O: u8> PRE_W<'a, O> {
    #[doc = "256 system clocks between SS active and first serial clock edge."]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(PRE_A::_256)
    }
}
#[doc = "Field `POST` reader - Slave Select Post delay 2."]
pub type POST_R = crate::FieldReader<u8, POST_A>;
#[doc = "Slave Select Post delay 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POST_A {
    #[doc = "0: 256 system clocks between last serial clock edge and SS inactive."]
    _256 = 0,
}
impl From<POST_A> for u8 {
    #[inline(always)]
    fn from(variant: POST_A) -> Self {
        variant as _
    }
}
impl POST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<POST_A> {
        match self.bits {
            0 => Some(POST_A::_256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == POST_A::_256
    }
}
#[doc = "Field `POST` writer - Slave Select Post delay 2."]
pub type POST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSTIME_SPEC, u8, POST_A, 8, O>;
impl<'a, const O: u8> POST_W<'a, O> {
    #[doc = "256 system clocks between last serial clock edge and SS inactive."]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(POST_A::_256)
    }
}
#[doc = "Field `INACT` reader - Slave Select Inactive delay."]
pub type INACT_R = crate::FieldReader<u8, INACT_A>;
#[doc = "Slave Select Inactive delay.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INACT_A {
    #[doc = "0: 256 system clocks between transactions."]
    _256 = 0,
}
impl From<INACT_A> for u8 {
    #[inline(always)]
    fn from(variant: INACT_A) -> Self {
        variant as _
    }
}
impl INACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INACT_A> {
        match self.bits {
            0 => Some(INACT_A::_256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == INACT_A::_256
    }
}
#[doc = "Field `INACT` writer - Slave Select Inactive delay."]
pub type INACT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSTIME_SPEC, u8, INACT_A, 8, O>;
impl<'a, const O: u8> INACT_W<'a, O> {
    #[doc = "256 system clocks between transactions."]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(INACT_A::_256)
    }
}
impl R {
    #[doc = "Bits 0:7 - Slave Select Pre delay 1."]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Slave Select Post delay 2."]
    #[inline(always)]
    pub fn post(&self) -> POST_R {
        POST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Slave Select Inactive delay."]
    #[inline(always)]
    pub fn inact(&self) -> INACT_R {
        INACT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slave Select Pre delay 1."]
    #[inline(always)]
    #[must_use]
    pub fn pre(&mut self) -> PRE_W<0> {
        PRE_W::new(self)
    }
    #[doc = "Bits 8:15 - Slave Select Post delay 2."]
    #[inline(always)]
    #[must_use]
    pub fn post(&mut self) -> POST_W<8> {
        POST_W::new(self)
    }
    #[doc = "Bits 16:23 - Slave Select Inactive delay."]
    #[inline(always)]
    #[must_use]
    pub fn inact(&mut self) -> INACT_W<16> {
        INACT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for controlling SPI peripheral/Slave Select Timing.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sstime](index.html) module"]
pub struct SSTIME_SPEC;
impl crate::RegisterSpec for SSTIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sstime::R](R) reader structure"]
impl crate::Readable for SSTIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sstime::W](W) writer structure"]
impl crate::Writable for SSTIME_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSTIME to value 0"]
impl crate::Resettable for SSTIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
