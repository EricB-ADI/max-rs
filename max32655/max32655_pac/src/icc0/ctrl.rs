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
#[doc = "Field `EN` reader - Cache Enable. Controls whether the cache is bypassed or is in use. Changing the state of this bit will cause the instruction cache to be flushed and its contents invalidated."]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "Cache Enable. Controls whether the cache is bypassed or is in use. Changing the state of this bit will cause the instruction cache to be flushed and its contents invalidated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: Cache Bypassed. Instruction data is stored in the line fill buffer but is not written to main cache memory array."]
    DIS = 0,
    #[doc = "1: Cache Enabled."]
    EN = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::DIS,
            true => EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN_A::EN
    }
}
#[doc = "Field `EN` writer - Cache Enable. Controls whether the cache is bypassed or is in use. Changing the state of this bit will cause the instruction cache to be flushed and its contents invalidated."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Cache Bypassed. Instruction data is stored in the line fill buffer but is not written to main cache memory array."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN_A::DIS)
    }
    #[doc = "Cache Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN_A::EN)
    }
}
#[doc = "Field `RDY` reader - Cache Ready flag. Cleared by hardware when at any time the cache as a whole is invalidated (including a system reset). When this bit is 0, the cache is effectively in bypass mode (instruction fetches will come from main memory or from the line fill buffer). Set by hardware when the invalidate operation is complete and the cache is ready."]
pub type RDY_R = crate::BitReader<RDY_A>;
#[doc = "Cache Ready flag. Cleared by hardware when at any time the cache as a whole is invalidated (including a system reset). When this bit is 0, the cache is effectively in bypass mode (instruction fetches will come from main memory or from the line fill buffer). Set by hardware when the invalidate operation is complete and the cache is ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDY_A {
    #[doc = "0: Not Ready."]
    NOT_READY = 0,
    #[doc = "1: Ready."]
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
            false => RDY_A::NOT_READY,
            true => RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_READY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == RDY_A::NOT_READY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == RDY_A::READY
    }
}
impl R {
    #[doc = "Bit 0 - Cache Enable. Controls whether the cache is bypassed or is in use. Changing the state of this bit will cause the instruction cache to be flushed and its contents invalidated."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Cache Ready flag. Cleared by hardware when at any time the cache as a whole is invalidated (including a system reset). When this bit is 0, the cache is effectively in bypass mode (instruction fetches will come from main memory or from the line fill buffer). Set by hardware when the invalidate operation is complete and the cache is ready."]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache Enable. Controls whether the cache is bypassed or is in use. Changing the state of this bit will cause the instruction cache to be flushed and its contents invalidated."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Control and Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
