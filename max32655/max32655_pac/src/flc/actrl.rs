#[doc = "Register `ACTRL` writer"]
pub struct W(crate::W<ACTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACTRL_SPEC>;
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
impl From<crate::W<ACTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACTRL` writer - Access control."]
pub type ACTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACTRL_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    #[must_use]
    pub fn actrl(&mut self) -> ACTRL_W<0> {
        ACTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access Control Register. Writing the ACTRL register with the following values in the order shown, allows read and write access to the system and user Information block: pflc-actrl = 0x3a7f5ca3; pflc-actrl = 0xa1e34f20; pflc-actrl = 0x9608b2c1. When unlocked, a write of any word will disable access to system and user information block. Readback of this register is always zero.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [actrl](index.html) module"]
pub struct ACTRL_SPEC;
impl crate::RegisterSpec for ACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [actrl::W](W) writer structure"]
impl crate::Writable for ACTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACTRL to value 0"]
impl crate::Resettable for ACTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
