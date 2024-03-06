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
#[doc = "Field `EN` reader - I2C Enable."]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "I2C Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: Disable I2C."]
    DIS = 0,
    #[doc = "1: enable I2C."]
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
#[doc = "Field `EN` writer - I2C Enable."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Disable I2C."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN_A::DIS)
    }
    #[doc = "enable I2C."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN_A::EN)
    }
}
#[doc = "Field `MST_MODE` reader - Master Mode Enable."]
pub type MST_MODE_R = crate::BitReader<MST_MODE_A>;
#[doc = "Master Mode Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MST_MODE_A {
    #[doc = "0: Slave Mode."]
    SLAVE_MODE = 0,
    #[doc = "1: Master Mode."]
    MASTER_MODE = 1,
}
impl From<MST_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MST_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MST_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MST_MODE_A {
        match self.bits {
            false => MST_MODE_A::SLAVE_MODE,
            true => MST_MODE_A::MASTER_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE_MODE`"]
    #[inline(always)]
    pub fn is_slave_mode(&self) -> bool {
        *self == MST_MODE_A::SLAVE_MODE
    }
    #[doc = "Checks if the value of the field is `MASTER_MODE`"]
    #[inline(always)]
    pub fn is_master_mode(&self) -> bool {
        *self == MST_MODE_A::MASTER_MODE
    }
}
#[doc = "Field `MST_MODE` writer - Master Mode Enable."]
pub type MST_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, MST_MODE_A, O>;
impl<'a, const O: u8> MST_MODE_W<'a, O> {
    #[doc = "Slave Mode."]
    #[inline(always)]
    pub fn slave_mode(self) -> &'a mut W {
        self.variant(MST_MODE_A::SLAVE_MODE)
    }
    #[doc = "Master Mode."]
    #[inline(always)]
    pub fn master_mode(self) -> &'a mut W {
        self.variant(MST_MODE_A::MASTER_MODE)
    }
}
#[doc = "Field `GC_ADDR_EN` reader - General Call Address Enable."]
pub type GC_ADDR_EN_R = crate::BitReader<GC_ADDR_EN_A>;
#[doc = "General Call Address Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GC_ADDR_EN_A {
    #[doc = "0: Ignore Gneral Call Address."]
    DIS = 0,
    #[doc = "1: Acknowledge general call address."]
    EN = 1,
}
impl From<GC_ADDR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: GC_ADDR_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl GC_ADDR_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GC_ADDR_EN_A {
        match self.bits {
            false => GC_ADDR_EN_A::DIS,
            true => GC_ADDR_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GC_ADDR_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GC_ADDR_EN_A::EN
    }
}
#[doc = "Field `GC_ADDR_EN` writer - General Call Address Enable."]
pub type GC_ADDR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, GC_ADDR_EN_A, O>;
impl<'a, const O: u8> GC_ADDR_EN_W<'a, O> {
    #[doc = "Ignore Gneral Call Address."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GC_ADDR_EN_A::DIS)
    }
    #[doc = "Acknowledge general call address."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(GC_ADDR_EN_A::EN)
    }
}
#[doc = "Field `IRXM_EN` reader - Interactive Receive Mode."]
pub type IRXM_EN_R = crate::BitReader<IRXM_EN_A>;
#[doc = "Interactive Receive Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRXM_EN_A {
    #[doc = "0: Disable Interactive Receive Mode."]
    DIS = 0,
    #[doc = "1: Enable Interactive Receive Mode."]
    EN = 1,
}
impl From<IRXM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: IRXM_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl IRXM_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRXM_EN_A {
        match self.bits {
            false => IRXM_EN_A::DIS,
            true => IRXM_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == IRXM_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == IRXM_EN_A::EN
    }
}
#[doc = "Field `IRXM_EN` writer - Interactive Receive Mode."]
pub type IRXM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, IRXM_EN_A, O>;
impl<'a, const O: u8> IRXM_EN_W<'a, O> {
    #[doc = "Disable Interactive Receive Mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(IRXM_EN_A::DIS)
    }
    #[doc = "Enable Interactive Receive Mode."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(IRXM_EN_A::EN)
    }
}
#[doc = "Field `IRXM_ACK` reader - Data Acknowledge. This bit defines the acknowledge bit returned by the I2C receiver while IRXM = 1 HW forces ACK to 0 when IRXM = 0."]
pub type IRXM_ACK_R = crate::BitReader<IRXM_ACK_A>;
#[doc = "Data Acknowledge. This bit defines the acknowledge bit returned by the I2C receiver while IRXM = 1 HW forces ACK to 0 when IRXM = 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRXM_ACK_A {
    #[doc = "0: return ACK (pulling SDA LOW)."]
    ACK = 0,
    #[doc = "1: return NACK (leaving SDA HIGH)."]
    NACK = 1,
}
impl From<IRXM_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: IRXM_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl IRXM_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRXM_ACK_A {
        match self.bits {
            false => IRXM_ACK_A::ACK,
            true => IRXM_ACK_A::NACK,
        }
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == IRXM_ACK_A::ACK
    }
    #[doc = "Checks if the value of the field is `NACK`"]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == IRXM_ACK_A::NACK
    }
}
#[doc = "Field `IRXM_ACK` writer - Data Acknowledge. This bit defines the acknowledge bit returned by the I2C receiver while IRXM = 1 HW forces ACK to 0 when IRXM = 0."]
pub type IRXM_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, IRXM_ACK_A, O>;
impl<'a, const O: u8> IRXM_ACK_W<'a, O> {
    #[doc = "return ACK (pulling SDA LOW)."]
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(IRXM_ACK_A::ACK)
    }
    #[doc = "return NACK (leaving SDA HIGH)."]
    #[inline(always)]
    pub fn nack(self) -> &'a mut W {
        self.variant(IRXM_ACK_A::NACK)
    }
}
#[doc = "Field `SCL_OUT` reader - SCL Output. This bits control SCL output when SWOE =1."]
pub type SCL_OUT_R = crate::BitReader<SCL_OUT_A>;
#[doc = "SCL Output. This bits control SCL output when SWOE =1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCL_OUT_A {
    #[doc = "0: Drive SCL low."]
    DRIVE_SCL_LOW = 0,
    #[doc = "1: Release SCL."]
    RELEASE_SCL = 1,
}
impl From<SCL_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: SCL_OUT_A) -> Self {
        variant as u8 != 0
    }
}
impl SCL_OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCL_OUT_A {
        match self.bits {
            false => SCL_OUT_A::DRIVE_SCL_LOW,
            true => SCL_OUT_A::RELEASE_SCL,
        }
    }
    #[doc = "Checks if the value of the field is `DRIVE_SCL_LOW`"]
    #[inline(always)]
    pub fn is_drive_scl_low(&self) -> bool {
        *self == SCL_OUT_A::DRIVE_SCL_LOW
    }
    #[doc = "Checks if the value of the field is `RELEASE_SCL`"]
    #[inline(always)]
    pub fn is_release_scl(&self) -> bool {
        *self == SCL_OUT_A::RELEASE_SCL
    }
}
#[doc = "Field `SCL_OUT` writer - SCL Output. This bits control SCL output when SWOE =1."]
pub type SCL_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, SCL_OUT_A, O>;
impl<'a, const O: u8> SCL_OUT_W<'a, O> {
    #[doc = "Drive SCL low."]
    #[inline(always)]
    pub fn drive_scl_low(self) -> &'a mut W {
        self.variant(SCL_OUT_A::DRIVE_SCL_LOW)
    }
    #[doc = "Release SCL."]
    #[inline(always)]
    pub fn release_scl(self) -> &'a mut W {
        self.variant(SCL_OUT_A::RELEASE_SCL)
    }
}
#[doc = "Field `SDA_OUT` reader - SDA Output. This bits control SDA output when SWOE = 1."]
pub type SDA_OUT_R = crate::BitReader<SDA_OUT_A>;
#[doc = "SDA Output. This bits control SDA output when SWOE = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDA_OUT_A {
    #[doc = "0: Drive SDA low."]
    DRIVE_SDA_LOW = 0,
    #[doc = "1: Release SDA."]
    RELEASE_SDA = 1,
}
impl From<SDA_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: SDA_OUT_A) -> Self {
        variant as u8 != 0
    }
}
impl SDA_OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDA_OUT_A {
        match self.bits {
            false => SDA_OUT_A::DRIVE_SDA_LOW,
            true => SDA_OUT_A::RELEASE_SDA,
        }
    }
    #[doc = "Checks if the value of the field is `DRIVE_SDA_LOW`"]
    #[inline(always)]
    pub fn is_drive_sda_low(&self) -> bool {
        *self == SDA_OUT_A::DRIVE_SDA_LOW
    }
    #[doc = "Checks if the value of the field is `RELEASE_SDA`"]
    #[inline(always)]
    pub fn is_release_sda(&self) -> bool {
        *self == SDA_OUT_A::RELEASE_SDA
    }
}
#[doc = "Field `SDA_OUT` writer - SDA Output. This bits control SDA output when SWOE = 1."]
pub type SDA_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, SDA_OUT_A, O>;
impl<'a, const O: u8> SDA_OUT_W<'a, O> {
    #[doc = "Drive SDA low."]
    #[inline(always)]
    pub fn drive_sda_low(self) -> &'a mut W {
        self.variant(SDA_OUT_A::DRIVE_SDA_LOW)
    }
    #[doc = "Release SDA."]
    #[inline(always)]
    pub fn release_sda(self) -> &'a mut W {
        self.variant(SDA_OUT_A::RELEASE_SDA)
    }
}
#[doc = "Field `SCL` reader - SCL status. This bit reflects the logic gate of SCL signal."]
pub type SCL_R = crate::BitReader<bool>;
#[doc = "Field `SDA` reader - SDA status. THis bit reflects the logic gate of SDA signal."]
pub type SDA_R = crate::BitReader<bool>;
#[doc = "Field `BB_MODE` reader - Software Output Enable."]
pub type BB_MODE_R = crate::BitReader<BB_MODE_A>;
#[doc = "Software Output Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BB_MODE_A {
    #[doc = "0: I2C Outputs SCLO and SDAO disabled."]
    OUTPUTS_DISABLE = 0,
    #[doc = "1: I2C Outputs SCLO and SDAO enabled."]
    OUTPUTS_ENABLE = 1,
}
impl From<BB_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: BB_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl BB_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB_MODE_A {
        match self.bits {
            false => BB_MODE_A::OUTPUTS_DISABLE,
            true => BB_MODE_A::OUTPUTS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUTS_DISABLE`"]
    #[inline(always)]
    pub fn is_outputs_disable(&self) -> bool {
        *self == BB_MODE_A::OUTPUTS_DISABLE
    }
    #[doc = "Checks if the value of the field is `OUTPUTS_ENABLE`"]
    #[inline(always)]
    pub fn is_outputs_enable(&self) -> bool {
        *self == BB_MODE_A::OUTPUTS_ENABLE
    }
}
#[doc = "Field `BB_MODE` writer - Software Output Enable."]
pub type BB_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, BB_MODE_A, O>;
impl<'a, const O: u8> BB_MODE_W<'a, O> {
    #[doc = "I2C Outputs SCLO and SDAO disabled."]
    #[inline(always)]
    pub fn outputs_disable(self) -> &'a mut W {
        self.variant(BB_MODE_A::OUTPUTS_DISABLE)
    }
    #[doc = "I2C Outputs SCLO and SDAO enabled."]
    #[inline(always)]
    pub fn outputs_enable(self) -> &'a mut W {
        self.variant(BB_MODE_A::OUTPUTS_ENABLE)
    }
}
#[doc = "Field `READ` reader - Read. This bit reflects the R/W bit of an address match (AMI = 1) or general call match (GCI = 1). This bit is valid 3 cycles after the relevant interrupt bit is set."]
pub type READ_R = crate::BitReader<READ_A>;
#[doc = "Read. This bit reflects the R/W bit of an address match (AMI = 1) or general call match (GCI = 1). This bit is valid 3 cycles after the relevant interrupt bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READ_A {
    #[doc = "0: Write."]
    WRITE = 0,
    #[doc = "1: Read."]
    READ = 1,
}
impl From<READ_A> for bool {
    #[inline(always)]
    fn from(variant: READ_A) -> Self {
        variant as u8 != 0
    }
}
impl READ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_A {
        match self.bits {
            false => READ_A::WRITE,
            true => READ_A::READ,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == READ_A::WRITE
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == READ_A::READ
    }
}
#[doc = "Field `CLKSTR_DIS` reader - This bit will disable slave clock stretching when set."]
pub type CLKSTR_DIS_R = crate::BitReader<CLKSTR_DIS_A>;
#[doc = "This bit will disable slave clock stretching when set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKSTR_DIS_A {
    #[doc = "0: Slave clock stretching enabled."]
    EN = 0,
    #[doc = "1: Slave clock stretching disabled."]
    DIS = 1,
}
impl From<CLKSTR_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSTR_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKSTR_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSTR_DIS_A {
        match self.bits {
            false => CLKSTR_DIS_A::EN,
            true => CLKSTR_DIS_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CLKSTR_DIS_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CLKSTR_DIS_A::DIS
    }
}
#[doc = "Field `CLKSTR_DIS` writer - This bit will disable slave clock stretching when set."]
pub type CLKSTR_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, CLKSTR_DIS_A, O>;
impl<'a, const O: u8> CLKSTR_DIS_W<'a, O> {
    #[doc = "Slave clock stretching enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CLKSTR_DIS_A::EN)
    }
    #[doc = "Slave clock stretching disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CLKSTR_DIS_A::DIS)
    }
}
#[doc = "Field `ONE_MST_MODE` reader - SCL Push-Pull Mode. This bit controls whether SCL is operated in a the I2C standard open-drain mode, or in a non-standard push-pull mode where the Hi-Z output isreplaced with Drive-1. The non-standard mode should only be used when operating as a master and communicating with slaves that are guaranteed to never drive SCL low."]
pub type ONE_MST_MODE_R = crate::BitReader<ONE_MST_MODE_A>;
#[doc = "SCL Push-Pull Mode. This bit controls whether SCL is operated in a the I2C standard open-drain mode, or in a non-standard push-pull mode where the Hi-Z output isreplaced with Drive-1. The non-standard mode should only be used when operating as a master and communicating with slaves that are guaranteed to never drive SCL low.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONE_MST_MODE_A {
    #[doc = "0: Standard open-drain operation: drive low for 0, Hi-Z for 1"]
    DIS = 0,
    #[doc = "1: Non-standard push-pull operation: drive low for 0, drive high for 1"]
    EN = 1,
}
impl From<ONE_MST_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: ONE_MST_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl ONE_MST_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONE_MST_MODE_A {
        match self.bits {
            false => ONE_MST_MODE_A::DIS,
            true => ONE_MST_MODE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ONE_MST_MODE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ONE_MST_MODE_A::EN
    }
}
#[doc = "Field `ONE_MST_MODE` writer - SCL Push-Pull Mode. This bit controls whether SCL is operated in a the I2C standard open-drain mode, or in a non-standard push-pull mode where the Hi-Z output isreplaced with Drive-1. The non-standard mode should only be used when operating as a master and communicating with slaves that are guaranteed to never drive SCL low."]
pub type ONE_MST_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, ONE_MST_MODE_A, O>;
impl<'a, const O: u8> ONE_MST_MODE_W<'a, O> {
    #[doc = "Standard open-drain operation: drive low for 0, Hi-Z for 1"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ONE_MST_MODE_A::DIS)
    }
    #[doc = "Non-standard push-pull operation: drive low for 0, drive high for 1"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ONE_MST_MODE_A::EN)
    }
}
#[doc = "Field `HS_EN` reader - High speed mode enable"]
pub type HS_EN_R = crate::BitReader<bool>;
#[doc = "Field `HS_EN` writer - High speed mode enable"]
pub type HS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - I2C Enable."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Mode Enable."]
    #[inline(always)]
    pub fn mst_mode(&self) -> MST_MODE_R {
        MST_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - General Call Address Enable."]
    #[inline(always)]
    pub fn gc_addr_en(&self) -> GC_ADDR_EN_R {
        GC_ADDR_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interactive Receive Mode."]
    #[inline(always)]
    pub fn irxm_en(&self) -> IRXM_EN_R {
        IRXM_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Acknowledge. This bit defines the acknowledge bit returned by the I2C receiver while IRXM = 1 HW forces ACK to 0 when IRXM = 0."]
    #[inline(always)]
    pub fn irxm_ack(&self) -> IRXM_ACK_R {
        IRXM_ACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - SCL Output. This bits control SCL output when SWOE =1."]
    #[inline(always)]
    pub fn scl_out(&self) -> SCL_OUT_R {
        SCL_OUT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SDA Output. This bits control SDA output when SWOE = 1."]
    #[inline(always)]
    pub fn sda_out(&self) -> SDA_OUT_R {
        SDA_OUT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCL status. This bit reflects the logic gate of SCL signal."]
    #[inline(always)]
    pub fn scl(&self) -> SCL_R {
        SCL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SDA status. THis bit reflects the logic gate of SDA signal."]
    #[inline(always)]
    pub fn sda(&self) -> SDA_R {
        SDA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software Output Enable."]
    #[inline(always)]
    pub fn bb_mode(&self) -> BB_MODE_R {
        BB_MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Read. This bit reflects the R/W bit of an address match (AMI = 1) or general call match (GCI = 1). This bit is valid 3 cycles after the relevant interrupt bit is set."]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit will disable slave clock stretching when set."]
    #[inline(always)]
    pub fn clkstr_dis(&self) -> CLKSTR_DIS_R {
        CLKSTR_DIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCL Push-Pull Mode. This bit controls whether SCL is operated in a the I2C standard open-drain mode, or in a non-standard push-pull mode where the Hi-Z output isreplaced with Drive-1. The non-standard mode should only be used when operating as a master and communicating with slaves that are guaranteed to never drive SCL low."]
    #[inline(always)]
    pub fn one_mst_mode(&self) -> ONE_MST_MODE_R {
        ONE_MST_MODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - High speed mode enable"]
    #[inline(always)]
    pub fn hs_en(&self) -> HS_EN_R {
        HS_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Enable."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Master Mode Enable."]
    #[inline(always)]
    #[must_use]
    pub fn mst_mode(&mut self) -> MST_MODE_W<1> {
        MST_MODE_W::new(self)
    }
    #[doc = "Bit 2 - General Call Address Enable."]
    #[inline(always)]
    #[must_use]
    pub fn gc_addr_en(&mut self) -> GC_ADDR_EN_W<2> {
        GC_ADDR_EN_W::new(self)
    }
    #[doc = "Bit 3 - Interactive Receive Mode."]
    #[inline(always)]
    #[must_use]
    pub fn irxm_en(&mut self) -> IRXM_EN_W<3> {
        IRXM_EN_W::new(self)
    }
    #[doc = "Bit 4 - Data Acknowledge. This bit defines the acknowledge bit returned by the I2C receiver while IRXM = 1 HW forces ACK to 0 when IRXM = 0."]
    #[inline(always)]
    #[must_use]
    pub fn irxm_ack(&mut self) -> IRXM_ACK_W<4> {
        IRXM_ACK_W::new(self)
    }
    #[doc = "Bit 6 - SCL Output. This bits control SCL output when SWOE =1."]
    #[inline(always)]
    #[must_use]
    pub fn scl_out(&mut self) -> SCL_OUT_W<6> {
        SCL_OUT_W::new(self)
    }
    #[doc = "Bit 7 - SDA Output. This bits control SDA output when SWOE = 1."]
    #[inline(always)]
    #[must_use]
    pub fn sda_out(&mut self) -> SDA_OUT_W<7> {
        SDA_OUT_W::new(self)
    }
    #[doc = "Bit 10 - Software Output Enable."]
    #[inline(always)]
    #[must_use]
    pub fn bb_mode(&mut self) -> BB_MODE_W<10> {
        BB_MODE_W::new(self)
    }
    #[doc = "Bit 12 - This bit will disable slave clock stretching when set."]
    #[inline(always)]
    #[must_use]
    pub fn clkstr_dis(&mut self) -> CLKSTR_DIS_W<12> {
        CLKSTR_DIS_W::new(self)
    }
    #[doc = "Bit 13 - SCL Push-Pull Mode. This bit controls whether SCL is operated in a the I2C standard open-drain mode, or in a non-standard push-pull mode where the Hi-Z output isreplaced with Drive-1. The non-standard mode should only be used when operating as a master and communicating with slaves that are guaranteed to never drive SCL low."]
    #[inline(always)]
    #[must_use]
    pub fn one_mst_mode(&mut self) -> ONE_MST_MODE_W<13> {
        ONE_MST_MODE_W::new(self)
    }
    #[doc = "Bit 15 - High speed mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn hs_en(&mut self) -> HS_EN_W<15> {
        HS_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
