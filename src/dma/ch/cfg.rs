#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<CHEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN` reader - Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0."]
pub struct CHEN_R(crate::FieldReader<bool, CHEN_A>);
impl CHEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN_A {
        match self.bits {
            false => CHEN_A::DIS,
            true => CHEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == CHEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == CHEN_A::EN
    }
}
impl core::ops::Deref for CHEN_R {
    type Target = crate::FieldReader<bool, CHEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN` writer - Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0."]
pub struct CHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CHEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CHEN_A::EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `RLDEN` reader - Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed."]
pub struct RLDEN_R(crate::FieldReader<bool, RLDEN_A>);
impl RLDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RLDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RLDEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == RLDEN_A::EN
    }
}
impl core::ops::Deref for RLDEN_R {
    type Target = crate::FieldReader<bool, RLDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RLDEN` writer - Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed."]
pub struct RLDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RLDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RLDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "DMA Priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRI_A {
    #[doc = "0: Highest Priority."]
    HIGH = 0,
    #[doc = "1: Medium High Priority."]
    MEDHIGH = 1,
    #[doc = "2: Medium Low Priority."]
    MEDLOW = 2,
    #[doc = "3: Lowest Priority."]
    LOW = 3,
}
impl From<PRI_A> for u8 {
    #[inline(always)]
    fn from(variant: PRI_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRI` reader - DMA Priority."]
pub struct PRI_R(crate::FieldReader<u8, PRI_A>);
impl PRI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRI_A {
        match self.bits {
            0 => PRI_A::HIGH,
            1 => PRI_A::MEDHIGH,
            2 => PRI_A::MEDLOW,
            3 => PRI_A::LOW,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PRI_A::HIGH
    }
    #[doc = "Checks if the value of the field is `MEDHIGH`"]
    #[inline(always)]
    pub fn is_med_high(&self) -> bool {
        **self == PRI_A::MEDHIGH
    }
    #[doc = "Checks if the value of the field is `MEDLOW`"]
    #[inline(always)]
    pub fn is_med_low(&self) -> bool {
        **self == PRI_A::MEDLOW
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PRI_A::LOW
    }
}
impl core::ops::Deref for PRI_R {
    type Target = crate::FieldReader<u8, PRI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI` writer - DMA Priority."]
pub struct PRI_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRI_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Highest Priority."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PRI_A::HIGH)
    }
    #[doc = "Medium High Priority."]
    #[inline(always)]
    pub fn med_high(self) -> &'a mut W {
        self.variant(PRI_A::MEDHIGH)
    }
    #[doc = "Medium Low Priority."]
    #[inline(always)]
    pub fn med_low(self) -> &'a mut W {
        self.variant(PRI_A::MEDLOW)
    }
    #[doc = "Lowest Priority."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PRI_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REQSEL_A {
    #[doc = "0: Memory To Memory"]
    MEMTOMEM = 0,
    #[doc = "1: SPI0 RX"]
    SPI0RX = 1,
    #[doc = "2: SPI1 RX"]
    SPI1RX = 2,
    #[doc = "4: UART0 RX"]
    UART0RX = 4,
    #[doc = "5: UART1 RX"]
    UART1RX = 5,
    #[doc = "7: I2C0 RX"]
    I2C0RX = 7,
    #[doc = "8: I2C1 RX"]
    I2C1RX = 8,
    #[doc = "33: SPI0 TX"]
    SPI0TX = 33,
    #[doc = "34: SPI1 TX"]
    SPI1TX = 34,
    #[doc = "36: UART0 TX"]
    UART0TX = 36,
    #[doc = "37: UART1 TX"]
    UART1TX = 37,
    #[doc = "39: I2C0 TX"]
    I2C0TX = 39,
    #[doc = "40: I2C1 TX"]
    I2C1TX = 40,
}
impl From<REQSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REQSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REQSEL` reader - Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active."]
pub struct REQSEL_R(crate::FieldReader<u8, REQSEL_A>);
impl REQSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REQSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REQSEL_A> {
        match self.bits {
            0 => Some(REQSEL_A::MEMTOMEM),
            1 => Some(REQSEL_A::SPI0RX),
            2 => Some(REQSEL_A::SPI1RX),
            4 => Some(REQSEL_A::UART0RX),
            5 => Some(REQSEL_A::UART1RX),
            7 => Some(REQSEL_A::I2C0RX),
            8 => Some(REQSEL_A::I2C1RX),
            33 => Some(REQSEL_A::SPI0TX),
            34 => Some(REQSEL_A::SPI1TX),
            36 => Some(REQSEL_A::UART0TX),
            37 => Some(REQSEL_A::UART1TX),
            39 => Some(REQSEL_A::I2C0TX),
            40 => Some(REQSEL_A::I2C1TX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MEMTOMEM`"]
    #[inline(always)]
    pub fn is_memtomem(&self) -> bool {
        **self == REQSEL_A::MEMTOMEM
    }
    #[doc = "Checks if the value of the field is `SPI0RX`"]
    #[inline(always)]
    pub fn is_spi0rx(&self) -> bool {
        **self == REQSEL_A::SPI0RX
    }
    #[doc = "Checks if the value of the field is `SPI1RX`"]
    #[inline(always)]
    pub fn is_spi1rx(&self) -> bool {
        **self == REQSEL_A::SPI1RX
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        **self == REQSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        **self == REQSEL_A::UART1RX
    }
    #[doc = "Checks if the value of the field is `I2C0RX`"]
    #[inline(always)]
    pub fn is_i2c0rx(&self) -> bool {
        **self == REQSEL_A::I2C0RX
    }
    #[doc = "Checks if the value of the field is `I2C1RX`"]
    #[inline(always)]
    pub fn is_i2c1rx(&self) -> bool {
        **self == REQSEL_A::I2C1RX
    }
    #[doc = "Checks if the value of the field is `SPI0TX`"]
    #[inline(always)]
    pub fn is_spi0tx(&self) -> bool {
        **self == REQSEL_A::SPI0TX
    }
    #[doc = "Checks if the value of the field is `SPI1TX`"]
    #[inline(always)]
    pub fn is_spi1tx(&self) -> bool {
        **self == REQSEL_A::SPI1TX
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        **self == REQSEL_A::UART0TX
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline(always)]
    pub fn is_uart1tx(&self) -> bool {
        **self == REQSEL_A::UART1TX
    }
    #[doc = "Checks if the value of the field is `I2C0TX`"]
    #[inline(always)]
    pub fn is_i2c0tx(&self) -> bool {
        **self == REQSEL_A::I2C0TX
    }
    #[doc = "Checks if the value of the field is `I2C1TX`"]
    #[inline(always)]
    pub fn is_i2c1tx(&self) -> bool {
        **self == REQSEL_A::I2C1TX
    }
}
impl core::ops::Deref for REQSEL_R {
    type Target = crate::FieldReader<u8, REQSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQSEL` writer - Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active."]
pub struct REQSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REQSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REQSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Memory To Memory"]
    #[inline(always)]
    pub fn memtomem(self) -> &'a mut W {
        self.variant(REQSEL_A::MEMTOMEM)
    }
    #[doc = "SPI0 RX"]
    #[inline(always)]
    pub fn spi0rx(self) -> &'a mut W {
        self.variant(REQSEL_A::SPI0RX)
    }
    #[doc = "SPI1 RX"]
    #[inline(always)]
    pub fn spi1rx(self) -> &'a mut W {
        self.variant(REQSEL_A::SPI1RX)
    }
    #[doc = "UART0 RX"]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(REQSEL_A::UART0RX)
    }
    #[doc = "UART1 RX"]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(REQSEL_A::UART1RX)
    }
    #[doc = "I2C0 RX"]
    #[inline(always)]
    pub fn i2c0rx(self) -> &'a mut W {
        self.variant(REQSEL_A::I2C0RX)
    }
    #[doc = "I2C1 RX"]
    #[inline(always)]
    pub fn i2c1rx(self) -> &'a mut W {
        self.variant(REQSEL_A::I2C1RX)
    }
    #[doc = "SPI0 TX"]
    #[inline(always)]
    pub fn spi0tx(self) -> &'a mut W {
        self.variant(REQSEL_A::SPI0TX)
    }
    #[doc = "SPI1 TX"]
    #[inline(always)]
    pub fn spi1tx(self) -> &'a mut W {
        self.variant(REQSEL_A::SPI1TX)
    }
    #[doc = "UART0 TX"]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(REQSEL_A::UART0TX)
    }
    #[doc = "UART1 TX"]
    #[inline(always)]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(REQSEL_A::UART1TX)
    }
    #[doc = "I2C0 TX"]
    #[inline(always)]
    pub fn i2c0tx(self) -> &'a mut W {
        self.variant(REQSEL_A::I2C0TX)
    }
    #[doc = "I2C1 TX"]
    #[inline(always)]
    pub fn i2c1tx(self) -> &'a mut W {
        self.variant(REQSEL_A::I2C1TX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | ((value as u32 & 0x3f) << 4);
        self.w
    }
}
#[doc = "Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQWAIT_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<REQWAIT_A> for bool {
    #[inline(always)]
    fn from(variant: REQWAIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQWAIT` reader - Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive."]
pub struct REQWAIT_R(crate::FieldReader<bool, REQWAIT_A>);
impl REQWAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REQWAIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQWAIT_A {
        match self.bits {
            false => REQWAIT_A::DIS,
            true => REQWAIT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == REQWAIT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == REQWAIT_A::EN
    }
}
impl core::ops::Deref for REQWAIT_R {
    type Target = crate::FieldReader<bool, REQWAIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQWAIT` writer - Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive."]
pub struct REQWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> REQWAIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REQWAIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(REQWAIT_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(REQWAIT_A::EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Time-Out Select. Selects the number of prescale clocks seen by the channel timer before a time-out conditions is generated for this channel. Important note: since the prescaler runs independent of the individual channel timers, the actual number of Pre-Scale clock edges seen has a margin of error equal to a single Pre-Scale clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TOSEL_A {
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
impl From<TOSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TOSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TOSEL` reader - Time-Out Select. Selects the number of prescale clocks seen by the channel timer before a time-out conditions is generated for this channel. Important note: since the prescaler runs independent of the individual channel timers, the actual number of Pre-Scale clock edges seen has a margin of error equal to a single Pre-Scale clock."]
pub struct TOSEL_R(crate::FieldReader<u8, TOSEL_A>);
impl TOSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOSEL_A {
        match self.bits {
            0 => TOSEL_A::TO4,
            1 => TOSEL_A::TO8,
            2 => TOSEL_A::TO16,
            3 => TOSEL_A::TO32,
            4 => TOSEL_A::TO64,
            5 => TOSEL_A::TO128,
            6 => TOSEL_A::TO256,
            7 => TOSEL_A::TO512,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TO4`"]
    #[inline(always)]
    pub fn is_to4(&self) -> bool {
        **self == TOSEL_A::TO4
    }
    #[doc = "Checks if the value of the field is `TO8`"]
    #[inline(always)]
    pub fn is_to8(&self) -> bool {
        **self == TOSEL_A::TO8
    }
    #[doc = "Checks if the value of the field is `TO16`"]
    #[inline(always)]
    pub fn is_to16(&self) -> bool {
        **self == TOSEL_A::TO16
    }
    #[doc = "Checks if the value of the field is `TO32`"]
    #[inline(always)]
    pub fn is_to32(&self) -> bool {
        **self == TOSEL_A::TO32
    }
    #[doc = "Checks if the value of the field is `TO64`"]
    #[inline(always)]
    pub fn is_to64(&self) -> bool {
        **self == TOSEL_A::TO64
    }
    #[doc = "Checks if the value of the field is `TO128`"]
    #[inline(always)]
    pub fn is_to128(&self) -> bool {
        **self == TOSEL_A::TO128
    }
    #[doc = "Checks if the value of the field is `TO256`"]
    #[inline(always)]
    pub fn is_to256(&self) -> bool {
        **self == TOSEL_A::TO256
    }
    #[doc = "Checks if the value of the field is `TO512`"]
    #[inline(always)]
    pub fn is_to512(&self) -> bool {
        **self == TOSEL_A::TO512
    }
}
impl core::ops::Deref for TOSEL_R {
    type Target = crate::FieldReader<u8, TOSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOSEL` writer - Time-Out Select. Selects the number of prescale clocks seen by the channel timer before a time-out conditions is generated for this channel. Important note: since the prescaler runs independent of the individual channel timers, the actual number of Pre-Scale clock edges seen has a margin of error equal to a single Pre-Scale clock."]
pub struct TOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Timeout of 3 to 4 prescale clocks."]
    #[inline(always)]
    pub fn to4(self) -> &'a mut W {
        self.variant(TOSEL_A::TO4)
    }
    #[doc = "Timeout of 7 to 8 prescale clocks."]
    #[inline(always)]
    pub fn to8(self) -> &'a mut W {
        self.variant(TOSEL_A::TO8)
    }
    #[doc = "Timeout of 15 to 16 prescale clocks."]
    #[inline(always)]
    pub fn to16(self) -> &'a mut W {
        self.variant(TOSEL_A::TO16)
    }
    #[doc = "Timeout of 31 to 32 prescale clocks."]
    #[inline(always)]
    pub fn to32(self) -> &'a mut W {
        self.variant(TOSEL_A::TO32)
    }
    #[doc = "Timeout of 63 to 64 prescale clocks."]
    #[inline(always)]
    pub fn to64(self) -> &'a mut W {
        self.variant(TOSEL_A::TO64)
    }
    #[doc = "Timeout of 127 to 128 prescale clocks."]
    #[inline(always)]
    pub fn to128(self) -> &'a mut W {
        self.variant(TOSEL_A::TO128)
    }
    #[doc = "Timeout of 255 to 256 prescale clocks."]
    #[inline(always)]
    pub fn to256(self) -> &'a mut W {
        self.variant(TOSEL_A::TO256)
    }
    #[doc = "Timeout of 511 to 512 prescale clocks."]
    #[inline(always)]
    pub fn to512(self) -> &'a mut W {
        self.variant(TOSEL_A::TO512)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
#[doc = "Pre-Scale Select. Selects the Pre-Scale divider for timer clock input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSSEL_A {
    #[doc = "0: Disable timer."]
    DIS = 0,
    #[doc = "1: hclk / 256."]
    DIV256 = 1,
    #[doc = "2: hclk / 64k."]
    DIV64K = 2,
    #[doc = "3: hclk / 16M."]
    DIV16M = 3,
}
impl From<PSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSSEL` reader - Pre-Scale Select. Selects the Pre-Scale divider for timer clock input."]
pub struct PSSEL_R(crate::FieldReader<u8, PSSEL_A>);
impl PSSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PSSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSSEL_A {
        match self.bits {
            0 => PSSEL_A::DIS,
            1 => PSSEL_A::DIV256,
            2 => PSSEL_A::DIV64K,
            3 => PSSEL_A::DIV16M,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PSSEL_A::DIS
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        **self == PSSEL_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV64K`"]
    #[inline(always)]
    pub fn is_div64k(&self) -> bool {
        **self == PSSEL_A::DIV64K
    }
    #[doc = "Checks if the value of the field is `DIV16M`"]
    #[inline(always)]
    pub fn is_div16m(&self) -> bool {
        **self == PSSEL_A::DIV16M
    }
}
impl core::ops::Deref for PSSEL_R {
    type Target = crate::FieldReader<u8, PSSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSSEL` writer - Pre-Scale Select. Selects the Pre-Scale divider for timer clock input."]
pub struct PSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Disable timer."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PSSEL_A::DIS)
    }
    #[doc = "hclk / 256."]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PSSEL_A::DIV256)
    }
    #[doc = "hclk / 64k."]
    #[inline(always)]
    pub fn div64k(self) -> &'a mut W {
        self.variant(PSSEL_A::DIV64K)
    }
    #[doc = "hclk / 16M."]
    #[inline(always)]
    pub fn div16m(self) -> &'a mut W {
        self.variant(PSSEL_A::DIV16M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRCWD_A {
    #[doc = "0: Byte."]
    BYTE = 0,
    #[doc = "1: Halfword."]
    HALFWORD = 1,
    #[doc = "2: Word."]
    WORD = 2,
}
impl From<SRCWD_A> for u8 {
    #[inline(always)]
    fn from(variant: SRCWD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRCWD` reader - Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value."]
pub struct SRCWD_R(crate::FieldReader<u8, SRCWD_A>);
impl SRCWD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRCWD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRCWD_A> {
        match self.bits {
            0 => Some(SRCWD_A::BYTE),
            1 => Some(SRCWD_A::HALFWORD),
            2 => Some(SRCWD_A::WORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        **self == SRCWD_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        **self == SRCWD_A::HALFWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        **self == SRCWD_A::WORD
    }
}
impl core::ops::Deref for SRCWD_R {
    type Target = crate::FieldReader<u8, SRCWD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRCWD` writer - Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value."]
pub struct SRCWD_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRCWD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Byte."]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(SRCWD_A::BYTE)
    }
    #[doc = "Halfword."]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(SRCWD_A::HALFWORD)
    }
    #[doc = "Word."]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(SRCWD_A::WORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `SRCINC` reader - Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals."]
pub struct SRCINC_R(crate::FieldReader<bool, SRCINC_A>);
impl SRCINC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRCINC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == SRCINC_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == SRCINC_A::EN
    }
}
impl core::ops::Deref for SRCINC_R {
    type Target = crate::FieldReader<bool, SRCINC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRCINC` writer - Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals."]
pub struct SRCINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRCINC_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSTWD_A {
    #[doc = "0: Byte."]
    BYTE = 0,
    #[doc = "1: Halfword."]
    HALFWORD = 1,
    #[doc = "2: Word."]
    WORD = 2,
}
impl From<DSTWD_A> for u8 {
    #[inline(always)]
    fn from(variant: DSTWD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DSTWD` reader - Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width)."]
pub struct DSTWD_R(crate::FieldReader<u8, DSTWD_A>);
impl DSTWD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DSTWD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DSTWD_A> {
        match self.bits {
            0 => Some(DSTWD_A::BYTE),
            1 => Some(DSTWD_A::HALFWORD),
            2 => Some(DSTWD_A::WORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        **self == DSTWD_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        **self == DSTWD_A::HALFWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        **self == DSTWD_A::WORD
    }
}
impl core::ops::Deref for DSTWD_R {
    type Target = crate::FieldReader<u8, DSTWD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSTWD` writer - Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width)."]
pub struct DSTWD_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSTWD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Byte."]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(DSTWD_A::BYTE)
    }
    #[doc = "Halfword."]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(DSTWD_A::HALFWORD)
    }
    #[doc = "Word."]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(DSTWD_A::WORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DSTINC` reader - Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals."]
pub struct DSTINC_R(crate::FieldReader<bool, DSTINC_A>);
impl DSTINC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSTINC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == DSTINC_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == DSTINC_A::EN
    }
}
impl core::ops::Deref for DSTINC_R {
    type Target = crate::FieldReader<bool, DSTINC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSTINC` writer - Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals."]
pub struct DSTINC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSTINC_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `BRST` reader - Burst Size. The number of bytes to be transferred into and out of the DMA FIFO in a single burst. Burst size equals 1 + value stored in this field."]
pub struct BRST_R(crate::FieldReader<u8, u8>);
impl BRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRST` writer - Burst Size. The number of bytes to be transferred into and out of the DMA FIFO in a single burst. Burst size equals 1 + value stored in this field."]
pub struct BRST_W<'a> {
    w: &'a mut W,
}
impl<'a> BRST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
#[doc = "Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHDIEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<CHDIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHDIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHDIEN` reader - Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0."]
pub struct CHDIEN_R(crate::FieldReader<bool, CHDIEN_A>);
impl CHDIEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHDIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHDIEN_A {
        match self.bits {
            false => CHDIEN_A::DIS,
            true => CHDIEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == CHDIEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == CHDIEN_A::EN
    }
}
impl core::ops::Deref for CHDIEN_R {
    type Target = crate::FieldReader<bool, CHDIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHDIEN` writer - Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0."]
pub struct CHDIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHDIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHDIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CHDIEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CHDIEN_A::EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTZIEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<CTZIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTZIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTZIEN` reader - Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs."]
pub struct CTZIEN_R(crate::FieldReader<bool, CTZIEN_A>);
impl CTZIEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTZIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTZIEN_A {
        match self.bits {
            false => CTZIEN_A::DIS,
            true => CTZIEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == CTZIEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == CTZIEN_A::EN
    }
}
impl core::ops::Deref for CTZIEN_R {
    type Target = crate::FieldReader<bool, CTZIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTZIEN` writer - Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs."]
pub struct CTZIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTZIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTZIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CTZIEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CTZIEN_A::EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0."]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed."]
    #[inline(always)]
    pub fn rlden(&self) -> RLDEN_R {
        RLDEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - DMA Priority."]
    #[inline(always)]
    pub fn pri(&self) -> PRI_R {
        PRI_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:9 - Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active."]
    #[inline(always)]
    pub fn reqsel(&self) -> REQSEL_R {
        REQSEL_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bit 10 - Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive."]
    #[inline(always)]
    pub fn reqwait(&self) -> REQWAIT_R {
        REQWAIT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - Time-Out Select. Selects the number of prescale clocks seen by the channel timer before a time-out conditions is generated for this channel. Important note: since the prescaler runs independent of the individual channel timers, the actual number of Pre-Scale clock edges seen has a margin of error equal to a single Pre-Scale clock."]
    #[inline(always)]
    pub fn tosel(&self) -> TOSEL_R {
        TOSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 14:15 - Pre-Scale Select. Selects the Pre-Scale divider for timer clock input."]
    #[inline(always)]
    pub fn pssel(&self) -> PSSEL_R {
        PSSEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value."]
    #[inline(always)]
    pub fn srcwd(&self) -> SRCWD_R {
        SRCWD_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals."]
    #[inline(always)]
    pub fn srcinc(&self) -> SRCINC_R {
        SRCINC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width)."]
    #[inline(always)]
    pub fn dstwd(&self) -> DSTWD_R {
        DSTWD_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 22 - Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals."]
    #[inline(always)]
    pub fn dstinc(&self) -> DSTINC_R {
        DSTINC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - Burst Size. The number of bytes to be transferred into and out of the DMA FIFO in a single burst. Burst size equals 1 + value stored in this field."]
    #[inline(always)]
    pub fn brst(&self) -> BRST_R {
        BRST_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0."]
    #[inline(always)]
    pub fn chdien(&self) -> CHDIEN_R {
        CHDIEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs."]
    #[inline(always)]
    pub fn ctzien(&self) -> CTZIEN_R {
        CTZIEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0."]
    #[inline(always)]
    pub fn chen(&mut self) -> CHEN_W {
        CHEN_W { w: self }
    }
    #[doc = "Bit 1 - Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed."]
    #[inline(always)]
    pub fn rlden(&mut self) -> RLDEN_W {
        RLDEN_W { w: self }
    }
    #[doc = "Bits 2:3 - DMA Priority."]
    #[inline(always)]
    pub fn pri(&mut self) -> PRI_W {
        PRI_W { w: self }
    }
    #[doc = "Bits 4:9 - Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active."]
    #[inline(always)]
    pub fn reqsel(&mut self) -> REQSEL_W {
        REQSEL_W { w: self }
    }
    #[doc = "Bit 10 - Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive."]
    #[inline(always)]
    pub fn reqwait(&mut self) -> REQWAIT_W {
        REQWAIT_W { w: self }
    }
    #[doc = "Bits 11:13 - Time-Out Select. Selects the number of prescale clocks seen by the channel timer before a time-out conditions is generated for this channel. Important note: since the prescaler runs independent of the individual channel timers, the actual number of Pre-Scale clock edges seen has a margin of error equal to a single Pre-Scale clock."]
    #[inline(always)]
    pub fn tosel(&mut self) -> TOSEL_W {
        TOSEL_W { w: self }
    }
    #[doc = "Bits 14:15 - Pre-Scale Select. Selects the Pre-Scale divider for timer clock input."]
    #[inline(always)]
    pub fn pssel(&mut self) -> PSSEL_W {
        PSSEL_W { w: self }
    }
    #[doc = "Bits 16:17 - Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value."]
    #[inline(always)]
    pub fn srcwd(&mut self) -> SRCWD_W {
        SRCWD_W { w: self }
    }
    #[doc = "Bit 18 - Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals."]
    #[inline(always)]
    pub fn srcinc(&mut self) -> SRCINC_W {
        SRCINC_W { w: self }
    }
    #[doc = "Bits 20:21 - Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width)."]
    #[inline(always)]
    pub fn dstwd(&mut self) -> DSTWD_W {
        DSTWD_W { w: self }
    }
    #[doc = "Bit 22 - Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals."]
    #[inline(always)]
    pub fn dstinc(&mut self) -> DSTINC_W {
        DSTINC_W { w: self }
    }
    #[doc = "Bits 24:28 - Burst Size. The number of bytes to be transferred into and out of the DMA FIFO in a single burst. Burst size equals 1 + value stored in this field."]
    #[inline(always)]
    pub fn brst(&mut self) -> BRST_W {
        BRST_W { w: self }
    }
    #[doc = "Bit 30 - Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0."]
    #[inline(always)]
    pub fn chdien(&mut self) -> CHDIEN_W {
        CHDIEN_W { w: self }
    }
    #[doc = "Bit 31 - Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs."]
    #[inline(always)]
    pub fn ctzien(&mut self) -> CTZIEN_W {
        CTZIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Configuration Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
