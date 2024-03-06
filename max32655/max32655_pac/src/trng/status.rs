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
#[doc = "Field `RDY` reader - 32-bit random data is ready to read from TRNG_DATA register. Reading TRNG_DATA when RND_RDY=0 will return all 0's. IRQ is generated when RND_RDY=1 if TRNG_CN.RND_IRQ_EN=1."]
pub type RDY_R = crate::BitReader<RDY_A>;
#[doc = "32-bit random data is ready to read from TRNG_DATA register. Reading TRNG_DATA when RND_RDY=0 will return all 0's. IRQ is generated when RND_RDY=1 if TRNG_CN.RND_IRQ_EN=1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDY_A {
    #[doc = "0: TRNG Busy"]
    BUSY = 0,
    #[doc = "1: 32 bit random data is ready"]
    READY = 1,
}
impl From<RDY_A> for bool {
    #[inline(always)]
    fn from(variant: RDY_A) -> Self {
        variant as u8 != 0
    }
}
impl RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDY_A {
        match self.bits {
            false => RDY_A::BUSY,
            true => RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RDY_A::BUSY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == RDY_A::READY
    }
}
#[doc = "Field `RDY` writer - 32-bit random data is ready to read from TRNG_DATA register. Reading TRNG_DATA when RND_RDY=0 will return all 0's. IRQ is generated when RND_RDY=1 if TRNG_CN.RND_IRQ_EN=1."]
pub type RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, RDY_A, O>;
impl<'a, const O: u8> RDY_W<'a, O> {
    #[doc = "TRNG Busy"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(RDY_A::BUSY)
    }
    #[doc = "32 bit random data is ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(RDY_A::READY)
    }
}
#[doc = "Field `ODHT` reader - On-Demand health test status."]
pub type ODHT_R = crate::BitReader<bool>;
#[doc = "Field `ODHT` writer - On-Demand health test status."]
pub type ODHT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `HT` reader - Health test status."]
pub type HT_R = crate::BitReader<bool>;
#[doc = "Field `HT` writer - Health test status."]
pub type HT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `SRCFAIL` reader - Entropy source has failed."]
pub type SRCFAIL_R = crate::BitReader<bool>;
#[doc = "Field `SRCFAIL` writer - Entropy source has failed."]
pub type SRCFAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `AESKGD` reader - AESKGD."]
pub type AESKGD_R = crate::BitReader<bool>;
#[doc = "Field `AESKGD` writer - AESKGD."]
pub type AESKGD_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `LD_CNT` reader - LD_CNT."]
pub type LD_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LD_CNT` writer - LD_CNT."]
pub type LD_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STATUS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - 32-bit random data is ready to read from TRNG_DATA register. Reading TRNG_DATA when RND_RDY=0 will return all 0's. IRQ is generated when RND_RDY=1 if TRNG_CN.RND_IRQ_EN=1."]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - On-Demand health test status."]
    #[inline(always)]
    pub fn odht(&self) -> ODHT_R {
        ODHT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Health test status."]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Entropy source has failed."]
    #[inline(always)]
    pub fn srcfail(&self) -> SRCFAIL_R {
        SRCFAIL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AESKGD."]
    #[inline(always)]
    pub fn aeskgd(&self) -> AESKGD_R {
        AESKGD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 24:31 - LD_CNT."]
    #[inline(always)]
    pub fn ld_cnt(&self) -> LD_CNT_R {
        LD_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 32-bit random data is ready to read from TRNG_DATA register. Reading TRNG_DATA when RND_RDY=0 will return all 0's. IRQ is generated when RND_RDY=1 if TRNG_CN.RND_IRQ_EN=1."]
    #[inline(always)]
    #[must_use]
    pub fn rdy(&mut self) -> RDY_W<0> {
        RDY_W::new(self)
    }
    #[doc = "Bit 1 - On-Demand health test status."]
    #[inline(always)]
    #[must_use]
    pub fn odht(&mut self) -> ODHT_W<1> {
        ODHT_W::new(self)
    }
    #[doc = "Bit 2 - Health test status."]
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HT_W<2> {
        HT_W::new(self)
    }
    #[doc = "Bit 3 - Entropy source has failed."]
    #[inline(always)]
    #[must_use]
    pub fn srcfail(&mut self) -> SRCFAIL_W<3> {
        SRCFAIL_W::new(self)
    }
    #[doc = "Bit 4 - AESKGD."]
    #[inline(always)]
    #[must_use]
    pub fn aeskgd(&mut self) -> AESKGD_W<4> {
        AESKGD_W::new(self)
    }
    #[doc = "Bits 24:31 - LD_CNT."]
    #[inline(always)]
    #[must_use]
    pub fn ld_cnt(&mut self) -> LD_CNT_W<24> {
        LD_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data. The content of this register is valid only when RNG_IS = 1. When TRNG is disabled, read returns 0x0000 0000.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
