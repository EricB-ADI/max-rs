#[doc = "Register `FCTRL0` reader"]
pub struct R(crate::R<FCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCTRL0` writer"]
pub struct W(crate::W<FCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCTRL0_SPEC>;
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
impl From<crate::W<FCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C0DGEN0` reader - I2C0 SDA Pad Deglitcher enable."]
pub type I2C0DGEN0_R = crate::BitReader<I2C0DGEN0_A>;
#[doc = "I2C0 SDA Pad Deglitcher enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C0DGEN0_A {
    #[doc = "0: Deglitcher disabled."]
    DIS = 0,
    #[doc = "1: Deglitcher enabled."]
    EN = 1,
}
impl From<I2C0DGEN0_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0DGEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C0DGEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0DGEN0_A {
        match self.bits {
            false => I2C0DGEN0_A::DIS,
            true => I2C0DGEN0_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2C0DGEN0_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2C0DGEN0_A::EN
    }
}
#[doc = "Field `I2C0DGEN0` writer - I2C0 SDA Pad Deglitcher enable."]
pub type I2C0DGEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCTRL0_SPEC, I2C0DGEN0_A, O>;
impl<'a, const O: u8> I2C0DGEN0_W<'a, O> {
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2C0DGEN0_A::DIS)
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2C0DGEN0_A::EN)
    }
}
#[doc = "Field `I2C0DGEN1` reader - I2C0 SCL Pad Deglitcher enable."]
pub type I2C0DGEN1_R = crate::BitReader<I2C0DGEN1_A>;
#[doc = "I2C0 SCL Pad Deglitcher enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C0DGEN1_A {
    #[doc = "0: Deglitcher disabled."]
    DIS = 0,
    #[doc = "1: Deglitcher enabled."]
    EN = 1,
}
impl From<I2C0DGEN1_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0DGEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C0DGEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0DGEN1_A {
        match self.bits {
            false => I2C0DGEN1_A::DIS,
            true => I2C0DGEN1_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2C0DGEN1_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2C0DGEN1_A::EN
    }
}
#[doc = "Field `I2C0DGEN1` writer - I2C0 SCL Pad Deglitcher enable."]
pub type I2C0DGEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCTRL0_SPEC, I2C0DGEN1_A, O>;
impl<'a, const O: u8> I2C0DGEN1_W<'a, O> {
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2C0DGEN1_A::DIS)
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2C0DGEN1_A::EN)
    }
}
#[doc = "Field `I2C1DGEN0` reader - I2C1 SDA Pad Deglitcher enable."]
pub type I2C1DGEN0_R = crate::BitReader<I2C1DGEN0_A>;
#[doc = "I2C1 SDA Pad Deglitcher enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1DGEN0_A {
    #[doc = "0: Deglitcher disabled."]
    DIS = 0,
    #[doc = "1: Deglitcher enabled."]
    EN = 1,
}
impl From<I2C1DGEN0_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1DGEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1DGEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1DGEN0_A {
        match self.bits {
            false => I2C1DGEN0_A::DIS,
            true => I2C1DGEN0_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2C1DGEN0_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2C1DGEN0_A::EN
    }
}
#[doc = "Field `I2C1DGEN0` writer - I2C1 SDA Pad Deglitcher enable."]
pub type I2C1DGEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCTRL0_SPEC, I2C1DGEN0_A, O>;
impl<'a, const O: u8> I2C1DGEN0_W<'a, O> {
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2C1DGEN0_A::DIS)
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2C1DGEN0_A::EN)
    }
}
#[doc = "Field `I2C1DGEN1` reader - I2C1 SCL Pad Deglitcher enable."]
pub type I2C1DGEN1_R = crate::BitReader<I2C1DGEN1_A>;
#[doc = "I2C1 SCL Pad Deglitcher enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1DGEN1_A {
    #[doc = "0: Deglitcher disabled."]
    DIS = 0,
    #[doc = "1: Deglitcher enabled."]
    EN = 1,
}
impl From<I2C1DGEN1_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1DGEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1DGEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1DGEN1_A {
        match self.bits {
            false => I2C1DGEN1_A::DIS,
            true => I2C1DGEN1_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2C1DGEN1_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2C1DGEN1_A::EN
    }
}
#[doc = "Field `I2C1DGEN1` writer - I2C1 SCL Pad Deglitcher enable."]
pub type I2C1DGEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCTRL0_SPEC, I2C1DGEN1_A, O>;
impl<'a, const O: u8> I2C1DGEN1_W<'a, O> {
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2C1DGEN1_A::DIS)
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2C1DGEN1_A::EN)
    }
}
#[doc = "Field `I2C2DGEN0` reader - I2C2 SDA Pad Deglitcher enable."]
pub type I2C2DGEN0_R = crate::BitReader<I2C2DGEN0_A>;
#[doc = "I2C2 SDA Pad Deglitcher enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2DGEN0_A {
    #[doc = "0: Deglitcher disabled."]
    DIS = 0,
    #[doc = "1: Deglitcher enabled."]
    EN = 1,
}
impl From<I2C2DGEN0_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2DGEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C2DGEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C2DGEN0_A {
        match self.bits {
            false => I2C2DGEN0_A::DIS,
            true => I2C2DGEN0_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2C2DGEN0_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2C2DGEN0_A::EN
    }
}
#[doc = "Field `I2C2DGEN0` writer - I2C2 SDA Pad Deglitcher enable."]
pub type I2C2DGEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCTRL0_SPEC, I2C2DGEN0_A, O>;
impl<'a, const O: u8> I2C2DGEN0_W<'a, O> {
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2C2DGEN0_A::DIS)
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2C2DGEN0_A::EN)
    }
}
#[doc = "Field `I2C2DGEN1` reader - I2C2 SCL Pad Deglitcher enable."]
pub type I2C2DGEN1_R = crate::BitReader<I2C2DGEN1_A>;
#[doc = "I2C2 SCL Pad Deglitcher enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2DGEN1_A {
    #[doc = "0: Deglitcher disabled."]
    DIS = 0,
    #[doc = "1: Deglitcher enabled."]
    EN = 1,
}
impl From<I2C2DGEN1_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2DGEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C2DGEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C2DGEN1_A {
        match self.bits {
            false => I2C2DGEN1_A::DIS,
            true => I2C2DGEN1_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2C2DGEN1_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2C2DGEN1_A::EN
    }
}
#[doc = "Field `I2C2DGEN1` writer - I2C2 SCL Pad Deglitcher enable."]
pub type I2C2DGEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCTRL0_SPEC, I2C2DGEN1_A, O>;
impl<'a, const O: u8> I2C2DGEN1_W<'a, O> {
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2C2DGEN1_A::DIS)
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2C2DGEN1_A::EN)
    }
}
impl R {
    #[doc = "Bit 20 - I2C0 SDA Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c0dgen0(&self) -> I2C0DGEN0_R {
        I2C0DGEN0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C0 SCL Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c0dgen1(&self) -> I2C0DGEN1_R {
        I2C0DGEN1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C1 SDA Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c1dgen0(&self) -> I2C1DGEN0_R {
        I2C1DGEN0_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C1 SCL Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c1dgen1(&self) -> I2C1DGEN1_R {
        I2C1DGEN1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - I2C2 SDA Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c2dgen0(&self) -> I2C2DGEN0_R {
        I2C2DGEN0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - I2C2 SCL Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c2dgen1(&self) -> I2C2DGEN1_R {
        I2C2DGEN1_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - I2C0 SDA Pad Deglitcher enable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c0dgen0(&mut self) -> I2C0DGEN0_W<20> {
        I2C0DGEN0_W::new(self)
    }
    #[doc = "Bit 21 - I2C0 SCL Pad Deglitcher enable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c0dgen1(&mut self) -> I2C0DGEN1_W<21> {
        I2C0DGEN1_W::new(self)
    }
    #[doc = "Bit 22 - I2C1 SDA Pad Deglitcher enable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1dgen0(&mut self) -> I2C1DGEN0_W<22> {
        I2C1DGEN0_W::new(self)
    }
    #[doc = "Bit 23 - I2C1 SCL Pad Deglitcher enable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1dgen1(&mut self) -> I2C1DGEN1_W<23> {
        I2C1DGEN1_W::new(self)
    }
    #[doc = "Bit 24 - I2C2 SDA Pad Deglitcher enable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c2dgen0(&mut self) -> I2C2DGEN0_W<24> {
        I2C2DGEN0_W::new(self)
    }
    #[doc = "Bit 25 - I2C2 SCL Pad Deglitcher enable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c2dgen1(&mut self) -> I2C2DGEN1_W<25> {
        I2C2DGEN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Function Control 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctrl0](index.html) module"]
pub struct FCTRL0_SPEC;
impl crate::RegisterSpec for FCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fctrl0::R](R) reader structure"]
impl crate::Readable for FCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fctrl0::W](W) writer structure"]
impl crate::Writable for FCTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCTRL0 to value 0"]
impl crate::Resettable for FCTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
