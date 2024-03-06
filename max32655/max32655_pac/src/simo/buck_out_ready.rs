#[doc = "Register `BUCK_OUT_READY` reader"]
pub struct R(crate::R<BUCK_OUT_READY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUCK_OUT_READY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUCK_OUT_READY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUCK_OUT_READY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUCKOUTRDYA` reader - When set, indicates that the output voltage has reached its regulated value"]
pub type BUCKOUTRDYA_R = crate::BitReader<BUCKOUTRDYA_A>;
#[doc = "When set, indicates that the output voltage has reached its regulated value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUCKOUTRDYA_A {
    #[doc = "0: Output voltage not in range"]
    NOTRDY = 0,
    #[doc = "1: Output voltage in range"]
    RDY = 1,
}
impl From<BUCKOUTRDYA_A> for bool {
    #[inline(always)]
    fn from(variant: BUCKOUTRDYA_A) -> Self {
        variant as u8 != 0
    }
}
impl BUCKOUTRDYA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUCKOUTRDYA_A {
        match self.bits {
            false => BUCKOUTRDYA_A::NOTRDY,
            true => BUCKOUTRDYA_A::RDY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRDY`"]
    #[inline(always)]
    pub fn is_notrdy(&self) -> bool {
        *self == BUCKOUTRDYA_A::NOTRDY
    }
    #[doc = "Checks if the value of the field is `RDY`"]
    #[inline(always)]
    pub fn is_rdy(&self) -> bool {
        *self == BUCKOUTRDYA_A::RDY
    }
}
#[doc = "Field `BUCKOUTRDYB` reader - When set, indicates that the output voltage has reached its regulated value"]
pub type BUCKOUTRDYB_R = crate::BitReader<BUCKOUTRDYB_A>;
#[doc = "When set, indicates that the output voltage has reached its regulated value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUCKOUTRDYB_A {
    #[doc = "0: Output voltage not in range"]
    NOTRDY = 0,
    #[doc = "1: Output voltage in range"]
    RDY = 1,
}
impl From<BUCKOUTRDYB_A> for bool {
    #[inline(always)]
    fn from(variant: BUCKOUTRDYB_A) -> Self {
        variant as u8 != 0
    }
}
impl BUCKOUTRDYB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUCKOUTRDYB_A {
        match self.bits {
            false => BUCKOUTRDYB_A::NOTRDY,
            true => BUCKOUTRDYB_A::RDY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRDY`"]
    #[inline(always)]
    pub fn is_notrdy(&self) -> bool {
        *self == BUCKOUTRDYB_A::NOTRDY
    }
    #[doc = "Checks if the value of the field is `RDY`"]
    #[inline(always)]
    pub fn is_rdy(&self) -> bool {
        *self == BUCKOUTRDYB_A::RDY
    }
}
#[doc = "Field `BUCKOUTRDYC` reader - When set, indicates that the output voltage has reached its regulated value"]
pub type BUCKOUTRDYC_R = crate::BitReader<BUCKOUTRDYC_A>;
#[doc = "When set, indicates that the output voltage has reached its regulated value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUCKOUTRDYC_A {
    #[doc = "0: Output voltage not in range"]
    NOTRDY = 0,
    #[doc = "1: Output voltage in range"]
    RDY = 1,
}
impl From<BUCKOUTRDYC_A> for bool {
    #[inline(always)]
    fn from(variant: BUCKOUTRDYC_A) -> Self {
        variant as u8 != 0
    }
}
impl BUCKOUTRDYC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUCKOUTRDYC_A {
        match self.bits {
            false => BUCKOUTRDYC_A::NOTRDY,
            true => BUCKOUTRDYC_A::RDY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRDY`"]
    #[inline(always)]
    pub fn is_notrdy(&self) -> bool {
        *self == BUCKOUTRDYC_A::NOTRDY
    }
    #[doc = "Checks if the value of the field is `RDY`"]
    #[inline(always)]
    pub fn is_rdy(&self) -> bool {
        *self == BUCKOUTRDYC_A::RDY
    }
}
#[doc = "Field `BUCKOUTRDYD` reader - When set, indicates that the output voltage has reached its regulated value"]
pub type BUCKOUTRDYD_R = crate::BitReader<BUCKOUTRDYD_A>;
#[doc = "When set, indicates that the output voltage has reached its regulated value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUCKOUTRDYD_A {
    #[doc = "0: Output voltage not in range"]
    NOTRDY = 0,
    #[doc = "1: Output voltage in range"]
    RDY = 1,
}
impl From<BUCKOUTRDYD_A> for bool {
    #[inline(always)]
    fn from(variant: BUCKOUTRDYD_A) -> Self {
        variant as u8 != 0
    }
}
impl BUCKOUTRDYD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUCKOUTRDYD_A {
        match self.bits {
            false => BUCKOUTRDYD_A::NOTRDY,
            true => BUCKOUTRDYD_A::RDY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRDY`"]
    #[inline(always)]
    pub fn is_notrdy(&self) -> bool {
        *self == BUCKOUTRDYD_A::NOTRDY
    }
    #[doc = "Checks if the value of the field is `RDY`"]
    #[inline(always)]
    pub fn is_rdy(&self) -> bool {
        *self == BUCKOUTRDYD_A::RDY
    }
}
impl R {
    #[doc = "Bit 0 - When set, indicates that the output voltage has reached its regulated value"]
    #[inline(always)]
    pub fn buckoutrdya(&self) -> BUCKOUTRDYA_R {
        BUCKOUTRDYA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set, indicates that the output voltage has reached its regulated value"]
    #[inline(always)]
    pub fn buckoutrdyb(&self) -> BUCKOUTRDYB_R {
        BUCKOUTRDYB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set, indicates that the output voltage has reached its regulated value"]
    #[inline(always)]
    pub fn buckoutrdyc(&self) -> BUCKOUTRDYC_R {
        BUCKOUTRDYC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When set, indicates that the output voltage has reached its regulated value"]
    #[inline(always)]
    pub fn buckoutrdyd(&self) -> BUCKOUTRDYD_R {
        BUCKOUTRDYD_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Buck Regulator Output Ready Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buck_out_ready](index.html) module"]
pub struct BUCK_OUT_READY_SPEC;
impl crate::RegisterSpec for BUCK_OUT_READY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buck_out_ready::R](R) reader structure"]
impl crate::Readable for BUCK_OUT_READY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUCK_OUT_READY to value 0"]
impl crate::Resettable for BUCK_OUT_READY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
