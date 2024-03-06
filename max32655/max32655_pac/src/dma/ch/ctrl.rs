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
#[doc = "Field `EN` reader - Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0."]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
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
#[doc = "Field `EN` writer - Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN_A::EN)
    }
}
#[doc = "Field `RLDEN` reader - Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed."]
pub type RLDEN_R = crate::BitReader<RLDEN_A>;
#[doc = "Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RLDEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<RLDEN_A> for bool {
    #[inline(always)]
    fn from(variant: RLDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RLDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RLDEN_A {
        match self.bits {
            false => RLDEN_A::DIS,
            true => RLDEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RLDEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RLDEN_A::EN
    }
}
#[doc = "Field `RLDEN` writer - Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed."]
pub type RLDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, RLDEN_A, O>;
impl<'a, const O: u8> RLDEN_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RLDEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RLDEN_A::EN)
    }
}
#[doc = "Field `PRI` reader - DMA Priority."]
pub type PRI_R = crate::FieldReader<u8, PRI_A>;
#[doc = "DMA Priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRI_A {
    #[doc = "0: Highest Priority."]
    HIGH = 0,
    #[doc = "1: Medium High Priority."]
    MED_HIGH = 1,
    #[doc = "2: Medium Low Priority."]
    MED_LOW = 2,
    #[doc = "3: Lowest Priority."]
    LOW = 3,
}
impl From<PRI_A> for u8 {
    #[inline(always)]
    fn from(variant: PRI_A) -> Self {
        variant as _
    }
}
impl PRI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRI_A {
        match self.bits {
            0 => PRI_A::HIGH,
            1 => PRI_A::MED_HIGH,
            2 => PRI_A::MED_LOW,
            3 => PRI_A::LOW,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PRI_A::HIGH
    }
    #[doc = "Checks if the value of the field is `MED_HIGH`"]
    #[inline(always)]
    pub fn is_med_high(&self) -> bool {
        *self == PRI_A::MED_HIGH
    }
    #[doc = "Checks if the value of the field is `MED_LOW`"]
    #[inline(always)]
    pub fn is_med_low(&self) -> bool {
        *self == PRI_A::MED_LOW
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PRI_A::LOW
    }
}
#[doc = "Field `PRI` writer - DMA Priority."]
pub type PRI_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, PRI_A, 2, O>;
impl<'a, const O: u8> PRI_W<'a, O> {
    #[doc = "Highest Priority."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PRI_A::HIGH)
    }
    #[doc = "Medium High Priority."]
    #[inline(always)]
    pub fn med_high(self) -> &'a mut W {
        self.variant(PRI_A::MED_HIGH)
    }
    #[doc = "Medium Low Priority."]
    #[inline(always)]
    pub fn med_low(self) -> &'a mut W {
        self.variant(PRI_A::MED_LOW)
    }
    #[doc = "Lowest Priority."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PRI_A::LOW)
    }
}
#[doc = "Field `REQUEST` reader - Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active."]
pub type REQUEST_R = crate::FieldReader<u8, REQUEST_A>;
#[doc = "Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REQUEST_A {
    #[doc = "0: Memory To Memory"]
    MEMTOMEM = 0,
    #[doc = "1: SPI1 RX"]
    SPI1RX = 1,
    #[doc = "4: UART0 RX"]
    UART0RX = 4,
    #[doc = "5: UART1 RX"]
    UART1RX = 5,
    #[doc = "7: I2C0 RX"]
    I2C0RX = 7,
    #[doc = "8: I2C1 RX"]
    I2C1RX = 8,
    #[doc = "9: ADC"]
    ADC = 9,
    #[doc = "10: I2C2 RX"]
    I2C2RX = 10,
    #[doc = "14: UART2 RX"]
    UART2RX = 14,
    #[doc = "15: SPI0 RX"]
    SPI0RX = 15,
    #[doc = "16: AES RX"]
    AESRX = 16,
    #[doc = "28: UART3 RX"]
    UART3RX = 28,
    #[doc = "30: I2S RX"]
    I2SRX = 30,
    #[doc = "33: SPI1 TX"]
    SPI1TX = 33,
    #[doc = "36: UART0 TX"]
    UART0TX = 36,
    #[doc = "37: UART1 TX"]
    UART1TX = 37,
    #[doc = "39: I2C0 TX"]
    I2C0TX = 39,
    #[doc = "40: I2C1 TX"]
    I2C1TX = 40,
    #[doc = "42: I2C2 TX"]
    I2C2TX = 42,
    #[doc = "44: CRC TX"]
    CRCTX = 44,
    #[doc = "46: UART2 TX"]
    UART2TX = 46,
    #[doc = "47: SPI0 TX"]
    SPI0TX = 47,
    #[doc = "48: AES TX"]
    AESTX = 48,
    #[doc = "60: UART3 TX"]
    UART3TX = 60,
    #[doc = "62: I2S TX"]
    I2STX = 62,
}
impl From<REQUEST_A> for u8 {
    #[inline(always)]
    fn from(variant: REQUEST_A) -> Self {
        variant as _
    }
}
impl REQUEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REQUEST_A> {
        match self.bits {
            0 => Some(REQUEST_A::MEMTOMEM),
            1 => Some(REQUEST_A::SPI1RX),
            4 => Some(REQUEST_A::UART0RX),
            5 => Some(REQUEST_A::UART1RX),
            7 => Some(REQUEST_A::I2C0RX),
            8 => Some(REQUEST_A::I2C1RX),
            9 => Some(REQUEST_A::ADC),
            10 => Some(REQUEST_A::I2C2RX),
            14 => Some(REQUEST_A::UART2RX),
            15 => Some(REQUEST_A::SPI0RX),
            16 => Some(REQUEST_A::AESRX),
            28 => Some(REQUEST_A::UART3RX),
            30 => Some(REQUEST_A::I2SRX),
            33 => Some(REQUEST_A::SPI1TX),
            36 => Some(REQUEST_A::UART0TX),
            37 => Some(REQUEST_A::UART1TX),
            39 => Some(REQUEST_A::I2C0TX),
            40 => Some(REQUEST_A::I2C1TX),
            42 => Some(REQUEST_A::I2C2TX),
            44 => Some(REQUEST_A::CRCTX),
            46 => Some(REQUEST_A::UART2TX),
            47 => Some(REQUEST_A::SPI0TX),
            48 => Some(REQUEST_A::AESTX),
            60 => Some(REQUEST_A::UART3TX),
            62 => Some(REQUEST_A::I2STX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MEMTOMEM`"]
    #[inline(always)]
    pub fn is_memtomem(&self) -> bool {
        *self == REQUEST_A::MEMTOMEM
    }
    #[doc = "Checks if the value of the field is `SPI1RX`"]
    #[inline(always)]
    pub fn is_spi1rx(&self) -> bool {
        *self == REQUEST_A::SPI1RX
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        *self == REQUEST_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        *self == REQUEST_A::UART1RX
    }
    #[doc = "Checks if the value of the field is `I2C0RX`"]
    #[inline(always)]
    pub fn is_i2c0rx(&self) -> bool {
        *self == REQUEST_A::I2C0RX
    }
    #[doc = "Checks if the value of the field is `I2C1RX`"]
    #[inline(always)]
    pub fn is_i2c1rx(&self) -> bool {
        *self == REQUEST_A::I2C1RX
    }
    #[doc = "Checks if the value of the field is `ADC`"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == REQUEST_A::ADC
    }
    #[doc = "Checks if the value of the field is `I2C2RX`"]
    #[inline(always)]
    pub fn is_i2c2rx(&self) -> bool {
        *self == REQUEST_A::I2C2RX
    }
    #[doc = "Checks if the value of the field is `UART2RX`"]
    #[inline(always)]
    pub fn is_uart2rx(&self) -> bool {
        *self == REQUEST_A::UART2RX
    }
    #[doc = "Checks if the value of the field is `SPI0RX`"]
    #[inline(always)]
    pub fn is_spi0rx(&self) -> bool {
        *self == REQUEST_A::SPI0RX
    }
    #[doc = "Checks if the value of the field is `AESRX`"]
    #[inline(always)]
    pub fn is_aesrx(&self) -> bool {
        *self == REQUEST_A::AESRX
    }
    #[doc = "Checks if the value of the field is `UART3RX`"]
    #[inline(always)]
    pub fn is_uart3rx(&self) -> bool {
        *self == REQUEST_A::UART3RX
    }
    #[doc = "Checks if the value of the field is `I2SRX`"]
    #[inline(always)]
    pub fn is_i2srx(&self) -> bool {
        *self == REQUEST_A::I2SRX
    }
    #[doc = "Checks if the value of the field is `SPI1TX`"]
    #[inline(always)]
    pub fn is_spi1tx(&self) -> bool {
        *self == REQUEST_A::SPI1TX
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        *self == REQUEST_A::UART0TX
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline(always)]
    pub fn is_uart1tx(&self) -> bool {
        *self == REQUEST_A::UART1TX
    }
    #[doc = "Checks if the value of the field is `I2C0TX`"]
    #[inline(always)]
    pub fn is_i2c0tx(&self) -> bool {
        *self == REQUEST_A::I2C0TX
    }
    #[doc = "Checks if the value of the field is `I2C1TX`"]
    #[inline(always)]
    pub fn is_i2c1tx(&self) -> bool {
        *self == REQUEST_A::I2C1TX
    }
    #[doc = "Checks if the value of the field is `I2C2TX`"]
    #[inline(always)]
    pub fn is_i2c2tx(&self) -> bool {
        *self == REQUEST_A::I2C2TX
    }
    #[doc = "Checks if the value of the field is `CRCTX`"]
    #[inline(always)]
    pub fn is_crctx(&self) -> bool {
        *self == REQUEST_A::CRCTX
    }
    #[doc = "Checks if the value of the field is `UART2TX`"]
    #[inline(always)]
    pub fn is_uart2tx(&self) -> bool {
        *self == REQUEST_A::UART2TX
    }
    #[doc = "Checks if the value of the field is `SPI0TX`"]
    #[inline(always)]
    pub fn is_spi0tx(&self) -> bool {
        *self == REQUEST_A::SPI0TX
    }
    #[doc = "Checks if the value of the field is `AESTX`"]
    #[inline(always)]
    pub fn is_aestx(&self) -> bool {
        *self == REQUEST_A::AESTX
    }
    #[doc = "Checks if the value of the field is `UART3TX`"]
    #[inline(always)]
    pub fn is_uart3tx(&self) -> bool {
        *self == REQUEST_A::UART3TX
    }
    #[doc = "Checks if the value of the field is `I2STX`"]
    #[inline(always)]
    pub fn is_i2stx(&self) -> bool {
        *self == REQUEST_A::I2STX
    }
}
#[doc = "Field `REQUEST` writer - Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active."]
pub type REQUEST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, REQUEST_A, 6, O>;
impl<'a, const O: u8> REQUEST_W<'a, O> {
    #[doc = "Memory To Memory"]
    #[inline(always)]
    pub fn memtomem(self) -> &'a mut W {
        self.variant(REQUEST_A::MEMTOMEM)
    }
    #[doc = "SPI1 RX"]
    #[inline(always)]
    pub fn spi1rx(self) -> &'a mut W {
        self.variant(REQUEST_A::SPI1RX)
    }
    #[doc = "UART0 RX"]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(REQUEST_A::UART0RX)
    }
    #[doc = "UART1 RX"]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(REQUEST_A::UART1RX)
    }
    #[doc = "I2C0 RX"]
    #[inline(always)]
    pub fn i2c0rx(self) -> &'a mut W {
        self.variant(REQUEST_A::I2C0RX)
    }
    #[doc = "I2C1 RX"]
    #[inline(always)]
    pub fn i2c1rx(self) -> &'a mut W {
        self.variant(REQUEST_A::I2C1RX)
    }
    #[doc = "ADC"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut W {
        self.variant(REQUEST_A::ADC)
    }
    #[doc = "I2C2 RX"]
    #[inline(always)]
    pub fn i2c2rx(self) -> &'a mut W {
        self.variant(REQUEST_A::I2C2RX)
    }
    #[doc = "UART2 RX"]
    #[inline(always)]
    pub fn uart2rx(self) -> &'a mut W {
        self.variant(REQUEST_A::UART2RX)
    }
    #[doc = "SPI0 RX"]
    #[inline(always)]
    pub fn spi0rx(self) -> &'a mut W {
        self.variant(REQUEST_A::SPI0RX)
    }
    #[doc = "AES RX"]
    #[inline(always)]
    pub fn aesrx(self) -> &'a mut W {
        self.variant(REQUEST_A::AESRX)
    }
    #[doc = "UART3 RX"]
    #[inline(always)]
    pub fn uart3rx(self) -> &'a mut W {
        self.variant(REQUEST_A::UART3RX)
    }
    #[doc = "I2S RX"]
    #[inline(always)]
    pub fn i2srx(self) -> &'a mut W {
        self.variant(REQUEST_A::I2SRX)
    }
    #[doc = "SPI1 TX"]
    #[inline(always)]
    pub fn spi1tx(self) -> &'a mut W {
        self.variant(REQUEST_A::SPI1TX)
    }
    #[doc = "UART0 TX"]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(REQUEST_A::UART0TX)
    }
    #[doc = "UART1 TX"]
    #[inline(always)]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(REQUEST_A::UART1TX)
    }
    #[doc = "I2C0 TX"]
    #[inline(always)]
    pub fn i2c0tx(self) -> &'a mut W {
        self.variant(REQUEST_A::I2C0TX)
    }
    #[doc = "I2C1 TX"]
    #[inline(always)]
    pub fn i2c1tx(self) -> &'a mut W {
        self.variant(REQUEST_A::I2C1TX)
    }
    #[doc = "I2C2 TX"]
    #[inline(always)]
    pub fn i2c2tx(self) -> &'a mut W {
        self.variant(REQUEST_A::I2C2TX)
    }
    #[doc = "CRC TX"]
    #[inline(always)]
    pub fn crctx(self) -> &'a mut W {
        self.variant(REQUEST_A::CRCTX)
    }
    #[doc = "UART2 TX"]
    #[inline(always)]
    pub fn uart2tx(self) -> &'a mut W {
        self.variant(REQUEST_A::UART2TX)
    }
    #[doc = "SPI0 TX"]
    #[inline(always)]
    pub fn spi0tx(self) -> &'a mut W {
        self.variant(REQUEST_A::SPI0TX)
    }
    #[doc = "AES TX"]
    #[inline(always)]
    pub fn aestx(self) -> &'a mut W {
        self.variant(REQUEST_A::AESTX)
    }
    #[doc = "UART3 TX"]
    #[inline(always)]
    pub fn uart3tx(self) -> &'a mut W {
        self.variant(REQUEST_A::UART3TX)
    }
    #[doc = "I2S TX"]
    #[inline(always)]
    pub fn i2stx(self) -> &'a mut W {
        self.variant(REQUEST_A::I2STX)
    }
}
#[doc = "Field `TO_WAIT` reader - Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive."]
pub type TO_WAIT_R = crate::BitReader<TO_WAIT_A>;
#[doc = "Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TO_WAIT_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<TO_WAIT_A> for bool {
    #[inline(always)]
    fn from(variant: TO_WAIT_A) -> Self {
        variant as u8 != 0
    }
}
impl TO_WAIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TO_WAIT_A {
        match self.bits {
            false => TO_WAIT_A::DIS,
            true => TO_WAIT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TO_WAIT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TO_WAIT_A::EN
    }
}
#[doc = "Field `TO_WAIT` writer - Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive."]
pub type TO_WAIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, TO_WAIT_A, O>;
impl<'a, const O: u8> TO_WAIT_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TO_WAIT_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TO_WAIT_A::EN)
    }
}
#[doc = "Field `TO_PER` reader - Timeout Period Select."]
pub type TO_PER_R = crate::FieldReader<u8, TO_PER_A>;
#[doc = "Timeout Period Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TO_PER_A {
    #[doc = "0: Timeout of 3 to 4 prescale clocks."]
    TO4 = 0,
    #[doc = "1: Timeout of 7 to 8 prescale clocks."]
    TO8 = 1,
    #[doc = "2: Timeout of 15 to 16 prescale clocks."]
    TO16 = 2,
    #[doc = "3: Timeout of 31 to 32 prescale clocks."]
    TO32 = 3,
    #[doc = "4: Timeout of 63 to 64 prescale clocks."]
    TO64 = 4,
    #[doc = "5: Timeout of 127 to 128 prescale clocks."]
    TO128 = 5,
    #[doc = "6: Timeout of 255 to 256 prescale clocks."]
    TO256 = 6,
    #[doc = "7: Timeout of 511 to 512 prescale clocks."]
    TO512 = 7,
}
impl From<TO_PER_A> for u8 {
    #[inline(always)]
    fn from(variant: TO_PER_A) -> Self {
        variant as _
    }
}
impl TO_PER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TO_PER_A {
        match self.bits {
            0 => TO_PER_A::TO4,
            1 => TO_PER_A::TO8,
            2 => TO_PER_A::TO16,
            3 => TO_PER_A::TO32,
            4 => TO_PER_A::TO64,
            5 => TO_PER_A::TO128,
            6 => TO_PER_A::TO256,
            7 => TO_PER_A::TO512,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TO4`"]
    #[inline(always)]
    pub fn is_to4(&self) -> bool {
        *self == TO_PER_A::TO4
    }
    #[doc = "Checks if the value of the field is `TO8`"]
    #[inline(always)]
    pub fn is_to8(&self) -> bool {
        *self == TO_PER_A::TO8
    }
    #[doc = "Checks if the value of the field is `TO16`"]
    #[inline(always)]
    pub fn is_to16(&self) -> bool {
        *self == TO_PER_A::TO16
    }
    #[doc = "Checks if the value of the field is `TO32`"]
    #[inline(always)]
    pub fn is_to32(&self) -> bool {
        *self == TO_PER_A::TO32
    }
    #[doc = "Checks if the value of the field is `TO64`"]
    #[inline(always)]
    pub fn is_to64(&self) -> bool {
        *self == TO_PER_A::TO64
    }
    #[doc = "Checks if the value of the field is `TO128`"]
    #[inline(always)]
    pub fn is_to128(&self) -> bool {
        *self == TO_PER_A::TO128
    }
    #[doc = "Checks if the value of the field is `TO256`"]
    #[inline(always)]
    pub fn is_to256(&self) -> bool {
        *self == TO_PER_A::TO256
    }
    #[doc = "Checks if the value of the field is `TO512`"]
    #[inline(always)]
    pub fn is_to512(&self) -> bool {
        *self == TO_PER_A::TO512
    }
}
#[doc = "Field `TO_PER` writer - Timeout Period Select."]
pub type TO_PER_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, TO_PER_A, 3, O>;
impl<'a, const O: u8> TO_PER_W<'a, O> {
    #[doc = "Timeout of 3 to 4 prescale clocks."]
    #[inline(always)]
    pub fn to4(self) -> &'a mut W {
        self.variant(TO_PER_A::TO4)
    }
    #[doc = "Timeout of 7 to 8 prescale clocks."]
    #[inline(always)]
    pub fn to8(self) -> &'a mut W {
        self.variant(TO_PER_A::TO8)
    }
    #[doc = "Timeout of 15 to 16 prescale clocks."]
    #[inline(always)]
    pub fn to16(self) -> &'a mut W {
        self.variant(TO_PER_A::TO16)
    }
    #[doc = "Timeout of 31 to 32 prescale clocks."]
    #[inline(always)]
    pub fn to32(self) -> &'a mut W {
        self.variant(TO_PER_A::TO32)
    }
    #[doc = "Timeout of 63 to 64 prescale clocks."]
    #[inline(always)]
    pub fn to64(self) -> &'a mut W {
        self.variant(TO_PER_A::TO64)
    }
    #[doc = "Timeout of 127 to 128 prescale clocks."]
    #[inline(always)]
    pub fn to128(self) -> &'a mut W {
        self.variant(TO_PER_A::TO128)
    }
    #[doc = "Timeout of 255 to 256 prescale clocks."]
    #[inline(always)]
    pub fn to256(self) -> &'a mut W {
        self.variant(TO_PER_A::TO256)
    }
    #[doc = "Timeout of 511 to 512 prescale clocks."]
    #[inline(always)]
    pub fn to512(self) -> &'a mut W {
        self.variant(TO_PER_A::TO512)
    }
}
#[doc = "Field `TO_CLKDIV` reader - Pre-Scale Select. Selects the Pre-Scale divider for timer clock input."]
pub type TO_CLKDIV_R = crate::FieldReader<u8, TO_CLKDIV_A>;
#[doc = "Pre-Scale Select. Selects the Pre-Scale divider for timer clock input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TO_CLKDIV_A {
    #[doc = "0: Disable timer."]
    DIS = 0,
    #[doc = "1: hclk / 256."]
    DIV256 = 1,
    #[doc = "2: hclk / 64k."]
    DIV64K = 2,
    #[doc = "3: hclk / 16M."]
    DIV16M = 3,
}
impl From<TO_CLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: TO_CLKDIV_A) -> Self {
        variant as _
    }
}
impl TO_CLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TO_CLKDIV_A {
        match self.bits {
            0 => TO_CLKDIV_A::DIS,
            1 => TO_CLKDIV_A::DIV256,
            2 => TO_CLKDIV_A::DIV64K,
            3 => TO_CLKDIV_A::DIV16M,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TO_CLKDIV_A::DIS
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == TO_CLKDIV_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV64K`"]
    #[inline(always)]
    pub fn is_div64k(&self) -> bool {
        *self == TO_CLKDIV_A::DIV64K
    }
    #[doc = "Checks if the value of the field is `DIV16M`"]
    #[inline(always)]
    pub fn is_div16m(&self) -> bool {
        *self == TO_CLKDIV_A::DIV16M
    }
}
#[doc = "Field `TO_CLKDIV` writer - Pre-Scale Select. Selects the Pre-Scale divider for timer clock input."]
pub type TO_CLKDIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, TO_CLKDIV_A, 2, O>;
impl<'a, const O: u8> TO_CLKDIV_W<'a, O> {
    #[doc = "Disable timer."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TO_CLKDIV_A::DIS)
    }
    #[doc = "hclk / 256."]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(TO_CLKDIV_A::DIV256)
    }
    #[doc = "hclk / 64k."]
    #[inline(always)]
    pub fn div64k(self) -> &'a mut W {
        self.variant(TO_CLKDIV_A::DIV64K)
    }
    #[doc = "hclk / 16M."]
    #[inline(always)]
    pub fn div16m(self) -> &'a mut W {
        self.variant(TO_CLKDIV_A::DIV16M)
    }
}
#[doc = "Field `SRCWD` reader - Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value."]
pub type SRCWD_R = crate::FieldReader<u8, SRCWD_A>;
#[doc = "Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRCWD_A {
    #[doc = "0: Byte."]
    BYTE = 0,
    #[doc = "1: Halfword."]
    HALF_WORD = 1,
    #[doc = "2: Word."]
    WORD = 2,
}
impl From<SRCWD_A> for u8 {
    #[inline(always)]
    fn from(variant: SRCWD_A) -> Self {
        variant as _
    }
}
impl SRCWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRCWD_A> {
        match self.bits {
            0 => Some(SRCWD_A::BYTE),
            1 => Some(SRCWD_A::HALF_WORD),
            2 => Some(SRCWD_A::WORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == SRCWD_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALF_WORD`"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == SRCWD_A::HALF_WORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == SRCWD_A::WORD
    }
}
#[doc = "Field `SRCWD` writer - Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value."]
pub type SRCWD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, SRCWD_A, 2, O>;
impl<'a, const O: u8> SRCWD_W<'a, O> {
    #[doc = "Byte."]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(SRCWD_A::BYTE)
    }
    #[doc = "Halfword."]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(SRCWD_A::HALF_WORD)
    }
    #[doc = "Word."]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(SRCWD_A::WORD)
    }
}
#[doc = "Field `SRCINC` reader - Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals."]
pub type SRCINC_R = crate::BitReader<SRCINC_A>;
#[doc = "Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRCINC_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<SRCINC_A> for bool {
    #[inline(always)]
    fn from(variant: SRCINC_A) -> Self {
        variant as u8 != 0
    }
}
impl SRCINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRCINC_A {
        match self.bits {
            false => SRCINC_A::DIS,
            true => SRCINC_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SRCINC_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SRCINC_A::EN
    }
}
#[doc = "Field `SRCINC` writer - Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals."]
pub type SRCINC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, SRCINC_A, O>;
impl<'a, const O: u8> SRCINC_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SRCINC_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SRCINC_A::EN)
    }
}
#[doc = "Field `DSTWD` reader - Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width)."]
pub type DSTWD_R = crate::FieldReader<u8, DSTWD_A>;
#[doc = "Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSTWD_A {
    #[doc = "0: Byte."]
    BYTE = 0,
    #[doc = "1: Halfword."]
    HALF_WORD = 1,
    #[doc = "2: Word."]
    WORD = 2,
}
impl From<DSTWD_A> for u8 {
    #[inline(always)]
    fn from(variant: DSTWD_A) -> Self {
        variant as _
    }
}
impl DSTWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DSTWD_A> {
        match self.bits {
            0 => Some(DSTWD_A::BYTE),
            1 => Some(DSTWD_A::HALF_WORD),
            2 => Some(DSTWD_A::WORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == DSTWD_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALF_WORD`"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == DSTWD_A::HALF_WORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == DSTWD_A::WORD
    }
}
#[doc = "Field `DSTWD` writer - Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width)."]
pub type DSTWD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, DSTWD_A, 2, O>;
impl<'a, const O: u8> DSTWD_W<'a, O> {
    #[doc = "Byte."]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(DSTWD_A::BYTE)
    }
    #[doc = "Halfword."]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(DSTWD_A::HALF_WORD)
    }
    #[doc = "Word."]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(DSTWD_A::WORD)
    }
}
#[doc = "Field `DSTINC` reader - Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals."]
pub type DSTINC_R = crate::BitReader<DSTINC_A>;
#[doc = "Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSTINC_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<DSTINC_A> for bool {
    #[inline(always)]
    fn from(variant: DSTINC_A) -> Self {
        variant as u8 != 0
    }
}
impl DSTINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSTINC_A {
        match self.bits {
            false => DSTINC_A::DIS,
            true => DSTINC_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DSTINC_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DSTINC_A::EN
    }
}
#[doc = "Field `DSTINC` writer - Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals."]
pub type DSTINC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, DSTINC_A, O>;
impl<'a, const O: u8> DSTINC_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DSTINC_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DSTINC_A::EN)
    }
}
#[doc = "Field `BURST_SIZE` reader - Burst Size. The number of bytes to be transferred into and out of the DMA FIFO in a single burst. Burst size equals 1 + value stored in this field."]
pub type BURST_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BURST_SIZE` writer - Burst Size. The number of bytes to be transferred into and out of the DMA FIFO in a single burst. Burst size equals 1 + value stored in this field."]
pub type BURST_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `DIS_IE` reader - Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0."]
pub type DIS_IE_R = crate::BitReader<DIS_IE_A>;
#[doc = "Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIS_IE_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<DIS_IE_A> for bool {
    #[inline(always)]
    fn from(variant: DIS_IE_A) -> Self {
        variant as u8 != 0
    }
}
impl DIS_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIS_IE_A {
        match self.bits {
            false => DIS_IE_A::DIS,
            true => DIS_IE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DIS_IE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DIS_IE_A::EN
    }
}
#[doc = "Field `DIS_IE` writer - Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0."]
pub type DIS_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, DIS_IE_A, O>;
impl<'a, const O: u8> DIS_IE_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DIS_IE_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DIS_IE_A::EN)
    }
}
#[doc = "Field `CTZ_IE` reader - Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs."]
pub type CTZ_IE_R = crate::BitReader<CTZ_IE_A>;
#[doc = "Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTZ_IE_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<CTZ_IE_A> for bool {
    #[inline(always)]
    fn from(variant: CTZ_IE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTZ_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTZ_IE_A {
        match self.bits {
            false => CTZ_IE_A::DIS,
            true => CTZ_IE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CTZ_IE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CTZ_IE_A::EN
    }
}
#[doc = "Field `CTZ_IE` writer - Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs."]
pub type CTZ_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, CTZ_IE_A, O>;
impl<'a, const O: u8> CTZ_IE_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CTZ_IE_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CTZ_IE_A::EN)
    }
}
impl R {
    #[doc = "Bit 0 - Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed."]
    #[inline(always)]
    pub fn rlden(&self) -> RLDEN_R {
        RLDEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - DMA Priority."]
    #[inline(always)]
    pub fn pri(&self) -> PRI_R {
        PRI_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:9 - Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active."]
    #[inline(always)]
    pub fn request(&self) -> REQUEST_R {
        REQUEST_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bit 10 - Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive."]
    #[inline(always)]
    pub fn to_wait(&self) -> TO_WAIT_R {
        TO_WAIT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - Timeout Period Select."]
    #[inline(always)]
    pub fn to_per(&self) -> TO_PER_R {
        TO_PER_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15 - Pre-Scale Select. Selects the Pre-Scale divider for timer clock input."]
    #[inline(always)]
    pub fn to_clkdiv(&self) -> TO_CLKDIV_R {
        TO_CLKDIV_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value."]
    #[inline(always)]
    pub fn srcwd(&self) -> SRCWD_R {
        SRCWD_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals."]
    #[inline(always)]
    pub fn srcinc(&self) -> SRCINC_R {
        SRCINC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width)."]
    #[inline(always)]
    pub fn dstwd(&self) -> DSTWD_R {
        DSTWD_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals."]
    #[inline(always)]
    pub fn dstinc(&self) -> DSTINC_R {
        DSTINC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Burst Size. The number of bytes to be transferred into and out of the DMA FIFO in a single burst. Burst size equals 1 + value stored in this field."]
    #[inline(always)]
    pub fn burst_size(&self) -> BURST_SIZE_R {
        BURST_SIZE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0."]
    #[inline(always)]
    pub fn dis_ie(&self) -> DIS_IE_R {
        DIS_IE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs."]
    #[inline(always)]
    pub fn ctz_ie(&self) -> CTZ_IE_R {
        CTZ_IE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed."]
    #[inline(always)]
    #[must_use]
    pub fn rlden(&mut self) -> RLDEN_W<1> {
        RLDEN_W::new(self)
    }
    #[doc = "Bits 2:3 - DMA Priority."]
    #[inline(always)]
    #[must_use]
    pub fn pri(&mut self) -> PRI_W<2> {
        PRI_W::new(self)
    }
    #[doc = "Bits 4:9 - Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active."]
    #[inline(always)]
    #[must_use]
    pub fn request(&mut self) -> REQUEST_W<4> {
        REQUEST_W::new(self)
    }
    #[doc = "Bit 10 - Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive."]
    #[inline(always)]
    #[must_use]
    pub fn to_wait(&mut self) -> TO_WAIT_W<10> {
        TO_WAIT_W::new(self)
    }
    #[doc = "Bits 11:13 - Timeout Period Select."]
    #[inline(always)]
    #[must_use]
    pub fn to_per(&mut self) -> TO_PER_W<11> {
        TO_PER_W::new(self)
    }
    #[doc = "Bits 14:15 - Pre-Scale Select. Selects the Pre-Scale divider for timer clock input."]
    #[inline(always)]
    #[must_use]
    pub fn to_clkdiv(&mut self) -> TO_CLKDIV_W<14> {
        TO_CLKDIV_W::new(self)
    }
    #[doc = "Bits 16:17 - Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value."]
    #[inline(always)]
    #[must_use]
    pub fn srcwd(&mut self) -> SRCWD_W<16> {
        SRCWD_W::new(self)
    }
    #[doc = "Bit 18 - Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals."]
    #[inline(always)]
    #[must_use]
    pub fn srcinc(&mut self) -> SRCINC_W<18> {
        SRCINC_W::new(self)
    }
    #[doc = "Bits 20:21 - Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width)."]
    #[inline(always)]
    #[must_use]
    pub fn dstwd(&mut self) -> DSTWD_W<20> {
        DSTWD_W::new(self)
    }
    #[doc = "Bit 22 - Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals."]
    #[inline(always)]
    #[must_use]
    pub fn dstinc(&mut self) -> DSTINC_W<22> {
        DSTINC_W::new(self)
    }
    #[doc = "Bits 24:28 - Burst Size. The number of bytes to be transferred into and out of the DMA FIFO in a single burst. Burst size equals 1 + value stored in this field."]
    #[inline(always)]
    #[must_use]
    pub fn burst_size(&mut self) -> BURST_SIZE_W<24> {
        BURST_SIZE_W::new(self)
    }
    #[doc = "Bit 30 - Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0."]
    #[inline(always)]
    #[must_use]
    pub fn dis_ie(&mut self) -> DIS_IE_W<30> {
        DIS_IE_W::new(self)
    }
    #[doc = "Bit 31 - Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ctz_ie(&mut self) -> CTZ_IE_W<31> {
        CTZ_IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
