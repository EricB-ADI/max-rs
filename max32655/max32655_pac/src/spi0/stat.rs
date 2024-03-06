#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSY` reader - SPI active status. In Master mode, set when transaction starts, cleared when last bit of last character is acted upon and Slave Select de-assertion would occur. In Slave mode, set when Slave Select is asserted, cleared when Slave Select is de-asserted. Not used in Timer mode."]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "SPI active status. In Master mode, set when transaction starts, cleared when last bit of last character is acted upon and Slave Select de-assertion would occur. In Slave mode, set when Slave Select is asserted, cleared when Slave Select is de-asserted. Not used in Timer mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: SPI not active."]
    NOT = 0,
    #[doc = "1: SPI active."]
    ACTIVE = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::NOT,
            true => BUSY_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == BUSY_A::NOT
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BUSY_A::ACTIVE
    }
}
impl R {
    #[doc = "Bit 0 - SPI active status. In Master mode, set when transaction starts, cleared when last bit of last character is acted upon and Slave Select de-assertion would occur. In Slave mode, set when Slave Select is asserted, cleared when Slave Select is de-asserted. Not used in Timer mode."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SPI Status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
