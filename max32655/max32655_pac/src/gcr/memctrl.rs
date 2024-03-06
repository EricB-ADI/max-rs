#[doc = "Register `MEMCTRL` reader"]
pub struct R(crate::R<MEMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEMCTRL` writer"]
pub struct W(crate::W<MEMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMCTRL_SPEC>;
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
impl From<crate::W<MEMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FWS` reader - Flash Wait State. These bits define the number of wait-state cycles per Flash data read access. Minimum wait state is 2."]
pub type FWS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FWS` writer - Flash Wait State. These bits define the number of wait-state cycles per Flash data read access. Minimum wait state is 2."]
pub type FWS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MEMCTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SYSRAM0ECC` reader - SYSRAM0 ECC Select."]
pub type SYSRAM0ECC_R = crate::BitReader<bool>;
#[doc = "Field `SYSRAM0ECC` writer - SYSRAM0 ECC Select."]
pub type SYSRAM0ECC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MEMCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Flash Wait State. These bits define the number of wait-state cycles per Flash data read access. Minimum wait state is 2."]
    #[inline(always)]
    pub fn fws(&self) -> FWS_R {
        FWS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 16 - SYSRAM0 ECC Select."]
    #[inline(always)]
    pub fn sysram0ecc(&self) -> SYSRAM0ECC_R {
        SYSRAM0ECC_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Flash Wait State. These bits define the number of wait-state cycles per Flash data read access. Minimum wait state is 2."]
    #[inline(always)]
    #[must_use]
    pub fn fws(&mut self) -> FWS_W<0> {
        FWS_W::new(self)
    }
    #[doc = "Bit 16 - SYSRAM0 ECC Select."]
    #[inline(always)]
    #[must_use]
    pub fn sysram0ecc(&mut self) -> SYSRAM0ECC_W<16> {
        SYSRAM0ECC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Clock Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memctrl](index.html) module"]
pub struct MEMCTRL_SPEC;
impl crate::RegisterSpec for MEMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [memctrl::R](R) reader structure"]
impl crate::Readable for MEMCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [memctrl::W](W) writer structure"]
impl crate::Writable for MEMCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEMCTRL to value 0"]
impl crate::Resettable for MEMCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
