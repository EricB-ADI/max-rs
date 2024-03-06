#[doc = "Register `SYSCTRL` reader"]
pub struct R(crate::R<SYSCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCTRL` writer"]
pub struct W(crate::W<SYSCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCTRL_SPEC>;
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
impl From<crate::W<SYSCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BSTAPEN` reader - Boundary Scan TAP enable. When enabled, the JTAG port is conneted to the Boundary Scan TAP instead of the ARM ICE."]
pub type BSTAPEN_R = crate::BitReader<bool>;
#[doc = "Field `BSTAPEN` writer - Boundary Scan TAP enable. When enabled, the JTAG port is conneted to the Boundary Scan TAP instead of the ARM ICE."]
pub type BSTAPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTRL_SPEC, bool, O>;
#[doc = "Field `FLASH_PAGE_FLIP` reader - Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer."]
pub type FLASH_PAGE_FLIP_R = crate::BitReader<FLASH_PAGE_FLIP_A>;
#[doc = "Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASH_PAGE_FLIP_A {
    #[doc = "0: Physical layout matches logical layout."]
    NORMAL = 0,
    #[doc = "1: Bottom half mapped to logical top half and vice versa."]
    SWAPPED = 1,
}
impl From<FLASH_PAGE_FLIP_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_PAGE_FLIP_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASH_PAGE_FLIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_PAGE_FLIP_A {
        match self.bits {
            false => FLASH_PAGE_FLIP_A::NORMAL,
            true => FLASH_PAGE_FLIP_A::SWAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == FLASH_PAGE_FLIP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SWAPPED`"]
    #[inline(always)]
    pub fn is_swapped(&self) -> bool {
        *self == FLASH_PAGE_FLIP_A::SWAPPED
    }
}
#[doc = "Field `FLASH_PAGE_FLIP` writer - Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer."]
pub type FLASH_PAGE_FLIP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYSCTRL_SPEC, FLASH_PAGE_FLIP_A, O>;
impl<'a, const O: u8> FLASH_PAGE_FLIP_W<'a, O> {
    #[doc = "Physical layout matches logical layout."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(FLASH_PAGE_FLIP_A::NORMAL)
    }
    #[doc = "Bottom half mapped to logical top half and vice versa."]
    #[inline(always)]
    pub fn swapped(self) -> &'a mut W {
        self.variant(FLASH_PAGE_FLIP_A::SWAPPED)
    }
}
#[doc = "Field `ICC0_FLUSH` reader - Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4."]
pub type ICC0_FLUSH_R = crate::BitReader<ICC0_FLUSH_A>;
#[doc = "Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICC0_FLUSH_A {
    #[doc = "0: Normal Code Cache Operation"]
    NORMAL = 0,
    #[doc = "1: Code Caches and CPU instruction buffer are flushed"]
    FLUSH = 1,
}
impl From<ICC0_FLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: ICC0_FLUSH_A) -> Self {
        variant as u8 != 0
    }
}
impl ICC0_FLUSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICC0_FLUSH_A {
        match self.bits {
            false => ICC0_FLUSH_A::NORMAL,
            true => ICC0_FLUSH_A::FLUSH,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ICC0_FLUSH_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `FLUSH`"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == ICC0_FLUSH_A::FLUSH
    }
}
#[doc = "Field `ICC0_FLUSH` writer - Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4."]
pub type ICC0_FLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTRL_SPEC, ICC0_FLUSH_A, O>;
impl<'a, const O: u8> ICC0_FLUSH_W<'a, O> {
    #[doc = "Normal Code Cache Operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(ICC0_FLUSH_A::NORMAL)
    }
    #[doc = "Code Caches and CPU instruction buffer are flushed"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(ICC0_FLUSH_A::FLUSH)
    }
}
#[doc = "Field `ROMDONE` reader - ROM_DONE status. Used to disable SWD interface during system initialization procedure"]
pub type ROMDONE_R = crate::BitReader<bool>;
#[doc = "Field `ROMDONE` writer - ROM_DONE status. Used to disable SWD interface during system initialization procedure"]
pub type ROMDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTRL_SPEC, bool, O>;
#[doc = "Field `CCHK` reader - Compute ROM Checksum. This bit is self-cleared when calculation is completed. Once set, software clearing this bit is ignored and the bit will remain set until the operation is completed."]
pub type CCHK_R = crate::BitReader<CCHK_A>;
#[doc = "Compute ROM Checksum. This bit is self-cleared when calculation is completed. Once set, software clearing this bit is ignored and the bit will remain set until the operation is completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCHK_A {
    #[doc = "0: No operation/complete."]
    COMPLETE = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<CCHK_A> for bool {
    #[inline(always)]
    fn from(variant: CCHK_A) -> Self {
        variant as u8 != 0
    }
}
impl CCHK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCHK_A {
        match self.bits {
            false => CCHK_A::COMPLETE,
            true => CCHK_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == CCHK_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == CCHK_A::START
    }
}
#[doc = "Field `CCHK` writer - Compute ROM Checksum. This bit is self-cleared when calculation is completed. Once set, software clearing this bit is ignored and the bit will remain set until the operation is completed."]
pub type CCHK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTRL_SPEC, CCHK_A, O>;
impl<'a, const O: u8> CCHK_W<'a, O> {
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(CCHK_A::COMPLETE)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(CCHK_A::START)
    }
}
#[doc = "Field `SWD_DIS` reader - Serial Wire Debug Disable. This bit is used to disable the serial wire debug interface This bit is only writeable if (FMV lock word is not programmed) or if (ICE lock word is not programmed and the ROM_DONE bit is not set)."]
pub type SWD_DIS_R = crate::BitReader<bool>;
#[doc = "Field `SWD_DIS` writer - Serial Wire Debug Disable. This bit is used to disable the serial wire debug interface This bit is only writeable if (FMV lock word is not programmed) or if (ICE lock word is not programmed and the ROM_DONE bit is not set)."]
pub type SWD_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTRL_SPEC, bool, O>;
#[doc = "Field `CHKRES` reader - ROM Checksum Result. This bit is only valid when CHKRD=1."]
pub type CHKRES_R = crate::BitReader<CHKRES_A>;
#[doc = "ROM Checksum Result. This bit is only valid when CHKRD=1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHKRES_A {
    #[doc = "0: ROM Checksum Correct."]
    PASS = 0,
    #[doc = "1: ROM Checksum Fail."]
    FAIL = 1,
}
impl From<CHKRES_A> for bool {
    #[inline(always)]
    fn from(variant: CHKRES_A) -> Self {
        variant as u8 != 0
    }
}
impl CHKRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHKRES_A {
        match self.bits {
            false => CHKRES_A::PASS,
            true => CHKRES_A::FAIL,
        }
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == CHKRES_A::PASS
    }
    #[doc = "Checks if the value of the field is `FAIL`"]
    #[inline(always)]
    pub fn is_fail(&self) -> bool {
        *self == CHKRES_A::FAIL
    }
}
#[doc = "Field `CHKRES` writer - ROM Checksum Result. This bit is only valid when CHKRD=1."]
pub type CHKRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTRL_SPEC, CHKRES_A, O>;
impl<'a, const O: u8> CHKRES_W<'a, O> {
    #[doc = "ROM Checksum Correct."]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(CHKRES_A::PASS)
    }
    #[doc = "ROM Checksum Fail."]
    #[inline(always)]
    pub fn fail(self) -> &'a mut W {
        self.variant(CHKRES_A::FAIL)
    }
}
#[doc = "Field `OVR` reader - Operating Voltage Range."]
pub type OVR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OVR` writer - Operating Voltage Range."]
pub type OVR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSCTRL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 1 - Boundary Scan TAP enable. When enabled, the JTAG port is conneted to the Boundary Scan TAP instead of the ARM ICE."]
    #[inline(always)]
    pub fn bstapen(&self) -> BSTAPEN_R {
        BSTAPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer."]
    #[inline(always)]
    pub fn flash_page_flip(&self) -> FLASH_PAGE_FLIP_R {
        FLASH_PAGE_FLIP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4."]
    #[inline(always)]
    pub fn icc0_flush(&self) -> ICC0_FLUSH_R {
        ICC0_FLUSH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - ROM_DONE status. Used to disable SWD interface during system initialization procedure"]
    #[inline(always)]
    pub fn romdone(&self) -> ROMDONE_R {
        ROMDONE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Compute ROM Checksum. This bit is self-cleared when calculation is completed. Once set, software clearing this bit is ignored and the bit will remain set until the operation is completed."]
    #[inline(always)]
    pub fn cchk(&self) -> CCHK_R {
        CCHK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Serial Wire Debug Disable. This bit is used to disable the serial wire debug interface This bit is only writeable if (FMV lock word is not programmed) or if (ICE lock word is not programmed and the ROM_DONE bit is not set)."]
    #[inline(always)]
    pub fn swd_dis(&self) -> SWD_DIS_R {
        SWD_DIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ROM Checksum Result. This bit is only valid when CHKRD=1."]
    #[inline(always)]
    pub fn chkres(&self) -> CHKRES_R {
        CHKRES_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Operating Voltage Range."]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Boundary Scan TAP enable. When enabled, the JTAG port is conneted to the Boundary Scan TAP instead of the ARM ICE."]
    #[inline(always)]
    #[must_use]
    pub fn bstapen(&mut self) -> BSTAPEN_W<1> {
        BSTAPEN_W::new(self)
    }
    #[doc = "Bit 4 - Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer."]
    #[inline(always)]
    #[must_use]
    pub fn flash_page_flip(&mut self) -> FLASH_PAGE_FLIP_W<4> {
        FLASH_PAGE_FLIP_W::new(self)
    }
    #[doc = "Bit 6 - Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4."]
    #[inline(always)]
    #[must_use]
    pub fn icc0_flush(&mut self) -> ICC0_FLUSH_W<6> {
        ICC0_FLUSH_W::new(self)
    }
    #[doc = "Bit 12 - ROM_DONE status. Used to disable SWD interface during system initialization procedure"]
    #[inline(always)]
    #[must_use]
    pub fn romdone(&mut self) -> ROMDONE_W<12> {
        ROMDONE_W::new(self)
    }
    #[doc = "Bit 13 - Compute ROM Checksum. This bit is self-cleared when calculation is completed. Once set, software clearing this bit is ignored and the bit will remain set until the operation is completed."]
    #[inline(always)]
    #[must_use]
    pub fn cchk(&mut self) -> CCHK_W<13> {
        CCHK_W::new(self)
    }
    #[doc = "Bit 14 - Serial Wire Debug Disable. This bit is used to disable the serial wire debug interface This bit is only writeable if (FMV lock word is not programmed) or if (ICE lock word is not programmed and the ROM_DONE bit is not set)."]
    #[inline(always)]
    #[must_use]
    pub fn swd_dis(&mut self) -> SWD_DIS_W<14> {
        SWD_DIS_W::new(self)
    }
    #[doc = "Bit 15 - ROM Checksum Result. This bit is only valid when CHKRD=1."]
    #[inline(always)]
    #[must_use]
    pub fn chkres(&mut self) -> CHKRES_W<15> {
        CHKRES_W::new(self)
    }
    #[doc = "Bits 16:17 - Operating Voltage Range."]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<16> {
        OVR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl](index.html) module"]
pub struct SYSCTRL_SPEC;
impl crate::RegisterSpec for SYSCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysctrl::R](R) reader structure"]
impl crate::Readable for SYSCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysctrl::W](W) writer structure"]
impl crate::Writable for SYSCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCTRL to value 0"]
impl crate::Resettable for SYSCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
