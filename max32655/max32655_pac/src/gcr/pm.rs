#[doc = "Register `PM` reader"]
pub struct R(crate::R<PM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PM` writer"]
pub struct W(crate::W<PM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PM_SPEC>;
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
impl From<crate::W<PM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode."]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Active Mode."]
    ACTIVE = 0,
    #[doc = "1: Cortex-M4 Active, RISC-V Sleep Mode."]
    SLEEP = 1,
    #[doc = "2: Standby Mode."]
    STANDBY = 2,
    #[doc = "4: Backup Mode."]
    BACKUP = 4,
    #[doc = "8: LPM or CM4 Deep Sleep Mode."]
    LPM = 8,
    #[doc = "9: UPM."]
    UPM = 9,
    #[doc = "10: Power Down Mode."]
    POWERDOWN = 10,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::ACTIVE),
            1 => Some(MODE_A::SLEEP),
            2 => Some(MODE_A::STANDBY),
            4 => Some(MODE_A::BACKUP),
            8 => Some(MODE_A::LPM),
            9 => Some(MODE_A::UPM),
            10 => Some(MODE_A::POWERDOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == MODE_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `SLEEP`"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == MODE_A::SLEEP
    }
    #[doc = "Checks if the value of the field is `STANDBY`"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == MODE_A::STANDBY
    }
    #[doc = "Checks if the value of the field is `BACKUP`"]
    #[inline(always)]
    pub fn is_backup(&self) -> bool {
        *self == MODE_A::BACKUP
    }
    #[doc = "Checks if the value of the field is `LPM`"]
    #[inline(always)]
    pub fn is_lpm(&self) -> bool {
        *self == MODE_A::LPM
    }
    #[doc = "Checks if the value of the field is `UPM`"]
    #[inline(always)]
    pub fn is_upm(&self) -> bool {
        *self == MODE_A::UPM
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline(always)]
    pub fn is_powerdown(&self) -> bool {
        *self == MODE_A::POWERDOWN
    }
}
#[doc = "Field `MODE` writer - Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PM_SPEC, u8, MODE_A, 4, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Active Mode."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(MODE_A::ACTIVE)
    }
    #[doc = "Cortex-M4 Active, RISC-V Sleep Mode."]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut W {
        self.variant(MODE_A::SLEEP)
    }
    #[doc = "Standby Mode."]
    #[inline(always)]
    pub fn standby(self) -> &'a mut W {
        self.variant(MODE_A::STANDBY)
    }
    #[doc = "Backup Mode."]
    #[inline(always)]
    pub fn backup(self) -> &'a mut W {
        self.variant(MODE_A::BACKUP)
    }
    #[doc = "LPM or CM4 Deep Sleep Mode."]
    #[inline(always)]
    pub fn lpm(self) -> &'a mut W {
        self.variant(MODE_A::LPM)
    }
    #[doc = "UPM."]
    #[inline(always)]
    pub fn upm(self) -> &'a mut W {
        self.variant(MODE_A::UPM)
    }
    #[doc = "Power Down Mode."]
    #[inline(always)]
    pub fn powerdown(self) -> &'a mut W {
        self.variant(MODE_A::POWERDOWN)
    }
}
#[doc = "Field `GPIO_WE` reader - GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set."]
pub type GPIO_WE_R = crate::BitReader<GPIO_WE_A>;
#[doc = "GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_WE_A {
    #[doc = "0: Wake Up Disable."]
    DIS = 0,
    #[doc = "1: Wake Up Enable."]
    EN = 1,
}
impl From<GPIO_WE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_WE_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_WE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_WE_A {
        match self.bits {
            false => GPIO_WE_A::DIS,
            true => GPIO_WE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO_WE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GPIO_WE_A::EN
    }
}
#[doc = "Field `GPIO_WE` writer - GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set."]
pub type GPIO_WE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PM_SPEC, GPIO_WE_A, O>;
impl<'a, const O: u8> GPIO_WE_W<'a, O> {
    #[doc = "Wake Up Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO_WE_A::DIS)
    }
    #[doc = "Wake Up Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(GPIO_WE_A::EN)
    }
}
#[doc = "Field `RTC_WE` reader - RTC Alarm Wake Up Enable. This bit enables RTC alarm as wakeup source. If enabled, the desired RTC alarm must be configured via the RTC control registers."]
pub type RTC_WE_R = crate::BitReader<RTC_WE_A>;
#[doc = "RTC Alarm Wake Up Enable. This bit enables RTC alarm as wakeup source. If enabled, the desired RTC alarm must be configured via the RTC control registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_WE_A {
    #[doc = "0: Wake Up Disable."]
    DIS = 0,
    #[doc = "1: Wake Up Enable."]
    EN = 1,
}
impl From<RTC_WE_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_WE_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_WE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_WE_A {
        match self.bits {
            false => RTC_WE_A::DIS,
            true => RTC_WE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RTC_WE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RTC_WE_A::EN
    }
}
#[doc = "Field `RTC_WE` writer - RTC Alarm Wake Up Enable. This bit enables RTC alarm as wakeup source. If enabled, the desired RTC alarm must be configured via the RTC control registers."]
pub type RTC_WE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PM_SPEC, RTC_WE_A, O>;
impl<'a, const O: u8> RTC_WE_W<'a, O> {
    #[doc = "Wake Up Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RTC_WE_A::DIS)
    }
    #[doc = "Wake Up Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RTC_WE_A::EN)
    }
}
#[doc = "Field `WUT_WE` reader - WUT Wake Up Enable. This bit enables the Wake-Up Timer as wakeup source."]
pub type WUT_WE_R = crate::BitReader<WUT_WE_A>;
#[doc = "WUT Wake Up Enable. This bit enables the Wake-Up Timer as wakeup source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUT_WE_A {
    #[doc = "0: Wake Up Disable."]
    DIS = 0,
    #[doc = "1: Wake Up Enable."]
    EN = 1,
}
impl From<WUT_WE_A> for bool {
    #[inline(always)]
    fn from(variant: WUT_WE_A) -> Self {
        variant as u8 != 0
    }
}
impl WUT_WE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUT_WE_A {
        match self.bits {
            false => WUT_WE_A::DIS,
            true => WUT_WE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WUT_WE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WUT_WE_A::EN
    }
}
#[doc = "Field `WUT_WE` writer - WUT Wake Up Enable. This bit enables the Wake-Up Timer as wakeup source."]
pub type WUT_WE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PM_SPEC, WUT_WE_A, O>;
impl<'a, const O: u8> WUT_WE_W<'a, O> {
    #[doc = "Wake Up Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(WUT_WE_A::DIS)
    }
    #[doc = "Wake Up Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(WUT_WE_A::EN)
    }
}
#[doc = "Field `AINCOMP_WE` reader - AIN COMP Wake Up Enable. This bit enables AIN COMP as wakeup source."]
pub type AINCOMP_WE_R = crate::BitReader<AINCOMP_WE_A>;
#[doc = "AIN COMP Wake Up Enable. This bit enables AIN COMP as wakeup source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AINCOMP_WE_A {
    #[doc = "0: Wake Up Disable."]
    DIS = 0,
    #[doc = "1: Wake Up Enable."]
    EN = 1,
}
impl From<AINCOMP_WE_A> for bool {
    #[inline(always)]
    fn from(variant: AINCOMP_WE_A) -> Self {
        variant as u8 != 0
    }
}
impl AINCOMP_WE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AINCOMP_WE_A {
        match self.bits {
            false => AINCOMP_WE_A::DIS,
            true => AINCOMP_WE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == AINCOMP_WE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == AINCOMP_WE_A::EN
    }
}
#[doc = "Field `AINCOMP_WE` writer - AIN COMP Wake Up Enable. This bit enables AIN COMP as wakeup source."]
pub type AINCOMP_WE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PM_SPEC, AINCOMP_WE_A, O>;
impl<'a, const O: u8> AINCOMP_WE_W<'a, O> {
    #[doc = "Wake Up Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(AINCOMP_WE_A::DIS)
    }
    #[doc = "Wake Up Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(AINCOMP_WE_A::EN)
    }
}
#[doc = "Field `ISO_PD` reader - 60 MHz power down. This bit selects the 60 MHz clock power state in DEEPSLEEP mode."]
pub type ISO_PD_R = crate::BitReader<ISO_PD_A>;
#[doc = "60 MHz power down. This bit selects the 60 MHz clock power state in DEEPSLEEP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISO_PD_A {
    #[doc = "0: Mode is Active."]
    ACTIVE = 0,
    #[doc = "1: Powered down in DEEPSLEEP."]
    DEEPSLEEP = 1,
}
impl From<ISO_PD_A> for bool {
    #[inline(always)]
    fn from(variant: ISO_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl ISO_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISO_PD_A {
        match self.bits {
            false => ISO_PD_A::ACTIVE,
            true => ISO_PD_A::DEEPSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ISO_PD_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == ISO_PD_A::DEEPSLEEP
    }
}
#[doc = "Field `ISO_PD` writer - 60 MHz power down. This bit selects the 60 MHz clock power state in DEEPSLEEP mode."]
pub type ISO_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PM_SPEC, ISO_PD_A, O>;
impl<'a, const O: u8> ISO_PD_W<'a, O> {
    #[doc = "Mode is Active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(ISO_PD_A::ACTIVE)
    }
    #[doc = "Powered down in DEEPSLEEP."]
    #[inline(always)]
    pub fn deepsleep(self) -> &'a mut W {
        self.variant(ISO_PD_A::DEEPSLEEP)
    }
}
#[doc = "Field `IPO_PD` reader - 100 MHz power down. This bit selects 100 MHz clock power state in DEEPSLEEP mode."]
pub type IPO_PD_R = crate::BitReader<IPO_PD_A>;
#[doc = "100 MHz power down. This bit selects 100 MHz clock power state in DEEPSLEEP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPO_PD_A {
    #[doc = "0: Mode is Active."]
    ACTIVE = 0,
    #[doc = "1: Powered down in DEEPSLEEP."]
    DEEPSLEEP = 1,
}
impl From<IPO_PD_A> for bool {
    #[inline(always)]
    fn from(variant: IPO_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl IPO_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPO_PD_A {
        match self.bits {
            false => IPO_PD_A::ACTIVE,
            true => IPO_PD_A::DEEPSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == IPO_PD_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == IPO_PD_A::DEEPSLEEP
    }
}
#[doc = "Field `IPO_PD` writer - 100 MHz power down. This bit selects 100 MHz clock power state in DEEPSLEEP mode."]
pub type IPO_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PM_SPEC, IPO_PD_A, O>;
impl<'a, const O: u8> IPO_PD_W<'a, O> {
    #[doc = "Mode is Active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(IPO_PD_A::ACTIVE)
    }
    #[doc = "Powered down in DEEPSLEEP."]
    #[inline(always)]
    pub fn deepsleep(self) -> &'a mut W {
        self.variant(IPO_PD_A::DEEPSLEEP)
    }
}
#[doc = "Field `IBRO_PD` reader - 7.3725 MHz power down. This bit selects 7.3725 MHz clock power state in DEEPSLEEP mode."]
pub type IBRO_PD_R = crate::BitReader<IBRO_PD_A>;
#[doc = "7.3725 MHz power down. This bit selects 7.3725 MHz clock power state in DEEPSLEEP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IBRO_PD_A {
    #[doc = "0: Mode is Active."]
    ACTIVE = 0,
    #[doc = "1: Powered down in DEEPSLEEP."]
    DEEPSLEEP = 1,
}
impl From<IBRO_PD_A> for bool {
    #[inline(always)]
    fn from(variant: IBRO_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl IBRO_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBRO_PD_A {
        match self.bits {
            false => IBRO_PD_A::ACTIVE,
            true => IBRO_PD_A::DEEPSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == IBRO_PD_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == IBRO_PD_A::DEEPSLEEP
    }
}
#[doc = "Field `IBRO_PD` writer - 7.3725 MHz power down. This bit selects 7.3725 MHz clock power state in DEEPSLEEP mode."]
pub type IBRO_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PM_SPEC, IBRO_PD_A, O>;
impl<'a, const O: u8> IBRO_PD_W<'a, O> {
    #[doc = "Mode is Active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(IBRO_PD_A::ACTIVE)
    }
    #[doc = "Powered down in DEEPSLEEP."]
    #[inline(always)]
    pub fn deepsleep(self) -> &'a mut W {
        self.variant(IBRO_PD_A::DEEPSLEEP)
    }
}
#[doc = "Field `ERFO_BP` reader - 32MHz Oscillator Bypass"]
pub type ERFO_BP_R = crate::BitReader<bool>;
#[doc = "Field `ERFO_BP` writer - 32MHz Oscillator Bypass"]
pub type ERFO_BP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PM_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set."]
    #[inline(always)]
    pub fn gpio_we(&self) -> GPIO_WE_R {
        GPIO_WE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC Alarm Wake Up Enable. This bit enables RTC alarm as wakeup source. If enabled, the desired RTC alarm must be configured via the RTC control registers."]
    #[inline(always)]
    pub fn rtc_we(&self) -> RTC_WE_R {
        RTC_WE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - WUT Wake Up Enable. This bit enables the Wake-Up Timer as wakeup source."]
    #[inline(always)]
    pub fn wut_we(&self) -> WUT_WE_R {
        WUT_WE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - AIN COMP Wake Up Enable. This bit enables AIN COMP as wakeup source."]
    #[inline(always)]
    pub fn aincomp_we(&self) -> AINCOMP_WE_R {
        AINCOMP_WE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - 60 MHz power down. This bit selects the 60 MHz clock power state in DEEPSLEEP mode."]
    #[inline(always)]
    pub fn iso_pd(&self) -> ISO_PD_R {
        ISO_PD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 100 MHz power down. This bit selects 100 MHz clock power state in DEEPSLEEP mode."]
    #[inline(always)]
    pub fn ipo_pd(&self) -> IPO_PD_R {
        IPO_PD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 7.3725 MHz power down. This bit selects 7.3725 MHz clock power state in DEEPSLEEP mode."]
    #[inline(always)]
    pub fn ibro_pd(&self) -> IBRO_PD_R {
        IBRO_PD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - 32MHz Oscillator Bypass"]
    #[inline(always)]
    pub fn erfo_bp(&self) -> ERFO_BP_R {
        ERFO_BP_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 4 - GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_we(&mut self) -> GPIO_WE_W<4> {
        GPIO_WE_W::new(self)
    }
    #[doc = "Bit 5 - RTC Alarm Wake Up Enable. This bit enables RTC alarm as wakeup source. If enabled, the desired RTC alarm must be configured via the RTC control registers."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_we(&mut self) -> RTC_WE_W<5> {
        RTC_WE_W::new(self)
    }
    #[doc = "Bit 7 - WUT Wake Up Enable. This bit enables the Wake-Up Timer as wakeup source."]
    #[inline(always)]
    #[must_use]
    pub fn wut_we(&mut self) -> WUT_WE_W<7> {
        WUT_WE_W::new(self)
    }
    #[doc = "Bit 9 - AIN COMP Wake Up Enable. This bit enables AIN COMP as wakeup source."]
    #[inline(always)]
    #[must_use]
    pub fn aincomp_we(&mut self) -> AINCOMP_WE_W<9> {
        AINCOMP_WE_W::new(self)
    }
    #[doc = "Bit 15 - 60 MHz power down. This bit selects the 60 MHz clock power state in DEEPSLEEP mode."]
    #[inline(always)]
    #[must_use]
    pub fn iso_pd(&mut self) -> ISO_PD_W<15> {
        ISO_PD_W::new(self)
    }
    #[doc = "Bit 16 - 100 MHz power down. This bit selects 100 MHz clock power state in DEEPSLEEP mode."]
    #[inline(always)]
    #[must_use]
    pub fn ipo_pd(&mut self) -> IPO_PD_W<16> {
        IPO_PD_W::new(self)
    }
    #[doc = "Bit 17 - 7.3725 MHz power down. This bit selects 7.3725 MHz clock power state in DEEPSLEEP mode."]
    #[inline(always)]
    #[must_use]
    pub fn ibro_pd(&mut self) -> IBRO_PD_W<17> {
        IBRO_PD_W::new(self)
    }
    #[doc = "Bit 20 - 32MHz Oscillator Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn erfo_bp(&mut self) -> ERFO_BP_W<20> {
        ERFO_BP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Management.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pm](index.html) module"]
pub struct PM_SPEC;
impl crate::RegisterSpec for PM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pm::R](R) reader structure"]
impl crate::Readable for PM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pm::W](W) writer structure"]
impl crate::Writable for PM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PM to value 0"]
impl crate::Resettable for PM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
