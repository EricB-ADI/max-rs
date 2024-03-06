#[doc = "Register `SLAVE_MULTI[%s]` reader"]
pub struct R(crate::R<SLAVE_MULTI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLAVE_MULTI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLAVE_MULTI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLAVE_MULTI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLAVE_MULTI[%s]` writer"]
pub struct W(crate::W<SLAVE_MULTI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLAVE_MULTI_SPEC>;
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
impl From<crate::W<SLAVE_MULTI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLAVE_MULTI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Slave Address."]
pub type ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDR` writer - Slave Address."]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SLAVE_MULTI_SPEC, u16, u16, 10, O>;
#[doc = "Field `DIS` reader - Slave Disable."]
pub type DIS_R = crate::BitReader<bool>;
#[doc = "Field `DIS` writer - Slave Disable."]
pub type DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLAVE_MULTI_SPEC, bool, O>;
#[doc = "Field `EXT_ADDR_EN` reader - Extended Address Select."]
pub type EXT_ADDR_EN_R = crate::BitReader<EXT_ADDR_EN_A>;
#[doc = "Extended Address Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXT_ADDR_EN_A {
    #[doc = "0: 7-bit address."]
    _7_BITS_ADDRESS = 0,
    #[doc = "1: 10-bit address."]
    _10_BITS_ADDRESS = 1,
}
impl From<EXT_ADDR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EXT_ADDR_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EXT_ADDR_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXT_ADDR_EN_A {
        match self.bits {
            false => EXT_ADDR_EN_A::_7_BITS_ADDRESS,
            true => EXT_ADDR_EN_A::_10_BITS_ADDRESS,
        }
    }
    #[doc = "Checks if the value of the field is `_7_BITS_ADDRESS`"]
    #[inline(always)]
    pub fn is_7_bits_address(&self) -> bool {
        *self == EXT_ADDR_EN_A::_7_BITS_ADDRESS
    }
    #[doc = "Checks if the value of the field is `_10_BITS_ADDRESS`"]
    #[inline(always)]
    pub fn is_10_bits_address(&self) -> bool {
        *self == EXT_ADDR_EN_A::_10_BITS_ADDRESS
    }
}
#[doc = "Field `EXT_ADDR_EN` writer - Extended Address Select."]
pub type EXT_ADDR_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLAVE_MULTI_SPEC, EXT_ADDR_EN_A, O>;
impl<'a, const O: u8> EXT_ADDR_EN_W<'a, O> {
    #[doc = "7-bit address."]
    #[inline(always)]
    pub fn _7_bits_address(self) -> &'a mut W {
        self.variant(EXT_ADDR_EN_A::_7_BITS_ADDRESS)
    }
    #[doc = "10-bit address."]
    #[inline(always)]
    pub fn _10_bits_address(self) -> &'a mut W {
        self.variant(EXT_ADDR_EN_A::_10_BITS_ADDRESS)
    }
}
impl R {
    #[doc = "Bits 0:9 - Slave Address."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Slave Disable."]
    #[inline(always)]
    pub fn dis(&self) -> DIS_R {
        DIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Extended Address Select."]
    #[inline(always)]
    pub fn ext_addr_en(&self) -> EXT_ADDR_EN_R {
        EXT_ADDR_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave Address."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Bit 10 - Slave Disable."]
    #[inline(always)]
    #[must_use]
    pub fn dis(&mut self) -> DIS_W<10> {
        DIS_W::new(self)
    }
    #[doc = "Bit 15 - Extended Address Select."]
    #[inline(always)]
    #[must_use]
    pub fn ext_addr_en(&mut self) -> EXT_ADDR_EN_W<15> {
        EXT_ADDR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Address Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave_multi](index.html) module"]
pub struct SLAVE_MULTI_SPEC;
impl crate::RegisterSpec for SLAVE_MULTI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slave_multi::R](R) reader structure"]
impl crate::Readable for SLAVE_MULTI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slave_multi::W](W) writer structure"]
impl crate::Writable for SLAVE_MULTI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLAVE_MULTI[%s]
to value 0"]
impl crate::Resettable for SLAVE_MULTI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
