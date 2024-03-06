#[doc = "Register `PCLKDIV` reader"]
pub struct R(crate::R<PCLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCLKDIV` writer"]
pub struct W(crate::W<PCLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCLKDIV_SPEC>;
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
impl From<crate::W<PCLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCFRQ` reader - ADC clock Frequency. These bits define the ADC clock frequency. fADC = fPCLK / (ADCFRQ)"]
pub type ADCFRQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCFRQ` writer - ADC clock Frequency. These bits define the ADC clock frequency. fADC = fPCLK / (ADCFRQ)"]
pub type ADCFRQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCLKDIV_SPEC, u8, u8, 4, O>;
#[doc = "Field `CNNCLKDIV` reader - CNN Clock Divider."]
pub type CNNCLKDIV_R = crate::FieldReader<u8, CNNCLKDIV_A>;
#[doc = "CNN Clock Divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CNNCLKDIV_A {
    #[doc = "0: `0`"]
    DIV2 = 0,
    #[doc = "1: `1`"]
    DIV4 = 1,
    #[doc = "2: `10`"]
    DIV8 = 2,
    #[doc = "3: `11`"]
    DIV16 = 3,
    #[doc = "4: `100`"]
    DIV1 = 4,
}
impl From<CNNCLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CNNCLKDIV_A) -> Self {
        variant as _
    }
}
impl CNNCLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CNNCLKDIV_A> {
        match self.bits {
            0 => Some(CNNCLKDIV_A::DIV2),
            1 => Some(CNNCLKDIV_A::DIV4),
            2 => Some(CNNCLKDIV_A::DIV8),
            3 => Some(CNNCLKDIV_A::DIV16),
            4 => Some(CNNCLKDIV_A::DIV1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CNNCLKDIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CNNCLKDIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CNNCLKDIV_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CNNCLKDIV_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CNNCLKDIV_A::DIV1
    }
}
#[doc = "Field `CNNCLKDIV` writer - CNN Clock Divider."]
pub type CNNCLKDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PCLKDIV_SPEC, u8, CNNCLKDIV_A, 3, O>;
impl<'a, const O: u8> CNNCLKDIV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CNNCLKDIV_A::DIV2)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CNNCLKDIV_A::DIV4)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(CNNCLKDIV_A::DIV8)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(CNNCLKDIV_A::DIV16)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CNNCLKDIV_A::DIV1)
    }
}
#[doc = "Field `CNNCLKSEL` reader - CNN Clock Select."]
pub type CNNCLKSEL_R = crate::BitReader<CNNCLKSEL_A>;
#[doc = "CNN Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNNCLKSEL_A {
    #[doc = "0: `0`"]
    SYSTEM = 0,
    #[doc = "1: `1`"]
    IBRO60 = 1,
}
impl From<CNNCLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CNNCLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CNNCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNNCLKSEL_A {
        match self.bits {
            false => CNNCLKSEL_A::SYSTEM,
            true => CNNCLKSEL_A::IBRO60,
        }
    }
    #[doc = "Checks if the value of the field is `SYSTEM`"]
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == CNNCLKSEL_A::SYSTEM
    }
    #[doc = "Checks if the value of the field is `IBRO60`"]
    #[inline(always)]
    pub fn is_ibro60(&self) -> bool {
        *self == CNNCLKSEL_A::IBRO60
    }
}
#[doc = "Field `CNNCLKSEL` writer - CNN Clock Select."]
pub type CNNCLKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCLKDIV_SPEC, CNNCLKSEL_A, O>;
impl<'a, const O: u8> CNNCLKSEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(CNNCLKSEL_A::SYSTEM)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ibro60(self) -> &'a mut W {
        self.variant(CNNCLKSEL_A::IBRO60)
    }
}
impl R {
    #[doc = "Bits 10:13 - ADC clock Frequency. These bits define the ADC clock frequency. fADC = fPCLK / (ADCFRQ)"]
    #[inline(always)]
    pub fn adcfrq(&self) -> ADCFRQ_R {
        ADCFRQ_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:16 - CNN Clock Divider."]
    #[inline(always)]
    pub fn cnnclkdiv(&self) -> CNNCLKDIV_R {
        CNNCLKDIV_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 17 - CNN Clock Select."]
    #[inline(always)]
    pub fn cnnclksel(&self) -> CNNCLKSEL_R {
        CNNCLKSEL_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 10:13 - ADC clock Frequency. These bits define the ADC clock frequency. fADC = fPCLK / (ADCFRQ)"]
    #[inline(always)]
    #[must_use]
    pub fn adcfrq(&mut self) -> ADCFRQ_W<10> {
        ADCFRQ_W::new(self)
    }
    #[doc = "Bits 14:16 - CNN Clock Divider."]
    #[inline(always)]
    #[must_use]
    pub fn cnnclkdiv(&mut self) -> CNNCLKDIV_W<14> {
        CNNCLKDIV_W::new(self)
    }
    #[doc = "Bit 17 - CNN Clock Select."]
    #[inline(always)]
    #[must_use]
    pub fn cnnclksel(&mut self) -> CNNCLKSEL_W<17> {
        CNNCLKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Clock Divider.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pclkdiv](index.html) module"]
pub struct PCLKDIV_SPEC;
impl crate::RegisterSpec for PCLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pclkdiv::R](R) reader structure"]
impl crate::Readable for PCLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pclkdiv::W](W) writer structure"]
impl crate::Writable for PCLKDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCLKDIV to value 0x01"]
impl crate::Resettable for PCLKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
