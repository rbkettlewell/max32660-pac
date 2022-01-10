#[doc = "Register `RSTR0` reader"]
pub struct R(crate::R<RSTR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTR0` writer"]
pub struct W(crate::W<RSTR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTR0_SPEC>;
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
impl From<crate::W<RSTR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DMA Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - DMA Reset."]
pub struct DMA_R(crate::FieldReader<bool, DMA_A>);
impl DMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::RESET_DONE,
            true => DMA_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        **self == DMA_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == DMA_A::BUSY
    }
}
impl core::ops::Deref for DMA_R {
    type Target = crate::FieldReader<bool, DMA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "DMA Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<DMA_AW> for bool {
    #[inline(always)]
    fn from(variant: DMA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` writer - DMA Reset."]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(DMA_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA_AW::RESET)
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
#[doc = "Watchdog Timer Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<WDT_A> for bool {
    #[inline(always)]
    fn from(variant: WDT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT` reader - Watchdog Timer Reset."]
pub struct WDT_R(crate::FieldReader<bool, WDT_A>);
impl WDT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT_A {
        match self.bits {
            false => WDT_A::RESET_DONE,
            true => WDT_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        **self == WDT_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == WDT_A::BUSY
    }
}
impl core::ops::Deref for WDT_R {
    type Target = crate::FieldReader<bool, WDT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Watchdog Timer Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<WDT_AW> for bool {
    #[inline(always)]
    fn from(variant: WDT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT` writer - Watchdog Timer Reset."]
pub struct WDT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDT_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(WDT_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(WDT_AW::RESET)
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
#[doc = "GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO0_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<GPIO0_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO0` reader - GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states."]
pub struct GPIO0_R(crate::FieldReader<bool, GPIO0_A>);
impl GPIO0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0_A {
        match self.bits {
            false => GPIO0_A::RESET_DONE,
            true => GPIO0_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        **self == GPIO0_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == GPIO0_A::BUSY
    }
}
impl core::ops::Deref for GPIO0_R {
    type Target = crate::FieldReader<bool, GPIO0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO0_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<GPIO0_AW> for bool {
    #[inline(always)]
    fn from(variant: GPIO0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO0` writer - GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states."]
pub struct GPIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(GPIO0_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIO0_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Timer0 Reset. Setting this bit to 1 resets Timer 0 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER0_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<TIMER0_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER0` reader - Timer0 Reset. Setting this bit to 1 resets Timer 0 blocks."]
pub struct TIMER0_R(crate::FieldReader<bool, TIMER0_A>);
impl TIMER0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER0_A {
        match self.bits {
            false => TIMER0_A::RESET_DONE,
            true => TIMER0_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        **self == TIMER0_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == TIMER0_A::BUSY
    }
}
impl core::ops::Deref for TIMER0_R {
    type Target = crate::FieldReader<bool, TIMER0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Timer0 Reset. Setting this bit to 1 resets Timer 0 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER0_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<TIMER0_AW> for bool {
    #[inline(always)]
    fn from(variant: TIMER0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER0` writer - Timer0 Reset. Setting this bit to 1 resets Timer 0 blocks."]
pub struct TIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(TIMER0_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIMER0_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Timer1 Reset. Setting this bit to 1 resets Timer 1 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER1_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<TIMER1_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER1` reader - Timer1 Reset. Setting this bit to 1 resets Timer 1 blocks."]
pub struct TIMER1_R(crate::FieldReader<bool, TIMER1_A>);
impl TIMER1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER1_A {
        match self.bits {
            false => TIMER1_A::RESET_DONE,
            true => TIMER1_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        **self == TIMER1_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == TIMER1_A::BUSY
    }
}
impl core::ops::Deref for TIMER1_R {
    type Target = crate::FieldReader<bool, TIMER1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Timer1 Reset. Setting this bit to 1 resets Timer 1 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER1_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<TIMER1_AW> for bool {
    #[inline(always)]
    fn from(variant: TIMER1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER1` writer - Timer1 Reset. Setting this bit to 1 resets Timer 1 blocks."]
pub struct TIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(TIMER1_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIMER1_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Timer2 Reset. Setting this bit to 1 resets Timer 2 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER2_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<TIMER2_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER2` reader - Timer2 Reset. Setting this bit to 1 resets Timer 2 blocks."]
pub struct TIMER2_R(crate::FieldReader<bool, TIMER2_A>);
impl TIMER2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER2_A {
        match self.bits {
            false => TIMER2_A::RESET_DONE,
            true => TIMER2_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        **self == TIMER2_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == TIMER2_A::BUSY
    }
}
impl core::ops::Deref for TIMER2_R {
    type Target = crate::FieldReader<bool, TIMER2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Timer2 Reset. Setting this bit to 1 resets Timer 2 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER2_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<TIMER2_AW> for bool {
    #[inline(always)]
    fn from(variant: TIMER2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER2` writer - Timer2 Reset. Setting this bit to 1 resets Timer 2 blocks."]
pub struct TIMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(TIMER2_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIMER2_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "UART0 Reset. Setting this bit to 1 resets all UART 0 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<UART0_A> for bool {
    #[inline(always)]
    fn from(variant: UART0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART0` reader - UART0 Reset. Setting this bit to 1 resets all UART 0 blocks."]
pub struct UART0_R(crate::FieldReader<bool, UART0_A>);
impl UART0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0_A {
        match self.bits {
            false => UART0_A::RESET_DONE,
            true => UART0_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        **self == UART0_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == UART0_A::BUSY
    }
}
impl core::ops::Deref for UART0_R {
    type Target = crate::FieldReader<bool, UART0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "UART0 Reset. Setting this bit to 1 resets all UART 0 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<UART0_AW> for bool {
    #[inline(always)]
    fn from(variant: UART0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART0` writer - UART0 Reset. Setting this bit to 1 resets all UART 0 blocks."]
pub struct UART0_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(UART0_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(UART0_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "UART1 Reset. Setting this bit to 1 resets all UART 1 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<UART1_A> for bool {
    #[inline(always)]
    fn from(variant: UART1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART1` reader - UART1 Reset. Setting this bit to 1 resets all UART 1 blocks."]
pub struct UART1_R(crate::FieldReader<bool, UART1_A>);
impl UART1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1_A {
        match self.bits {
            false => UART1_A::RESET_DONE,
            true => UART1_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        **self == UART1_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == UART1_A::BUSY
    }
}
impl core::ops::Deref for UART1_R {
    type Target = crate::FieldReader<bool, UART1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "UART1 Reset. Setting this bit to 1 resets all UART 1 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<UART1_AW> for bool {
    #[inline(always)]
    fn from(variant: UART1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART1` writer - UART1 Reset. Setting this bit to 1 resets all UART 1 blocks."]
pub struct UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(UART1_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(UART1_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "SPI0 Reset. Setting this bit to 1 resets all SPI 0 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<SPI0_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI0` reader - SPI0 Reset. Setting this bit to 1 resets all SPI 0 blocks."]
pub struct SPI0_R(crate::FieldReader<bool, SPI0_A>);
impl SPI0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_A {
        match self.bits {
            false => SPI0_A::RESET_DONE,
            true => SPI0_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        **self == SPI0_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == SPI0_A::BUSY
    }
}
impl core::ops::Deref for SPI0_R {
    type Target = crate::FieldReader<bool, SPI0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SPI0 Reset. Setting this bit to 1 resets all SPI 0 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<SPI0_AW> for bool {
    #[inline(always)]
    fn from(variant: SPI0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI0` writer - SPI0 Reset. Setting this bit to 1 resets all SPI 0 blocks."]
pub struct SPI0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(SPI0_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SPI0_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "SPI1 Reset. Setting this bit to 1 resets all SPI 1 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<SPI1_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI1` reader - SPI1 Reset. Setting this bit to 1 resets all SPI 1 blocks."]
pub struct SPI1_R(crate::FieldReader<bool, SPI1_A>);
impl SPI1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_A {
        match self.bits {
            false => SPI1_A::RESET_DONE,
            true => SPI1_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        **self == SPI1_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == SPI1_A::BUSY
    }
}
impl core::ops::Deref for SPI1_R {
    type Target = crate::FieldReader<bool, SPI1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SPI1 Reset. Setting this bit to 1 resets all SPI 1 blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<SPI1_AW> for bool {
    #[inline(always)]
    fn from(variant: SPI1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI1` writer - SPI1 Reset. Setting this bit to 1 resets all SPI 1 blocks."]
pub struct SPI1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(SPI1_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SPI1_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "I2C0 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<I2C0_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C0` reader - I2C0 Reset."]
pub struct I2C0_R(crate::FieldReader<bool, I2C0_A>);
impl I2C0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0_A {
        match self.bits {
            false => I2C0_A::RESET_DONE,
            true => I2C0_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        **self == I2C0_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == I2C0_A::BUSY
    }
}
impl core::ops::Deref for I2C0_R {
    type Target = crate::FieldReader<bool, I2C0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "I2C0 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<I2C0_AW> for bool {
    #[inline(always)]
    fn from(variant: I2C0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C0` writer - I2C0 Reset."]
pub struct I2C0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(I2C0_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C0_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Real Time Clock Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<RTC_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC` reader - Real Time Clock Reset."]
pub struct RTC_R(crate::FieldReader<bool, RTC_A>);
impl RTC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_A {
        match self.bits {
            false => RTC_A::RESET_DONE,
            true => RTC_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        **self == RTC_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == RTC_A::BUSY
    }
}
impl core::ops::Deref for RTC_R {
    type Target = crate::FieldReader<bool, RTC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Real Time Clock Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<RTC_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC` writer - Real Time Clock Reset."]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(RTC_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RTC_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Soft Reset.Write 1 to perform a Soft Reset. A soft reset performs a Peripheral Reset and also resets the GPIO peripheral but does not reset the CPU or Watchdog Timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRST_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<SRST_A> for bool {
    #[inline(always)]
    fn from(variant: SRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRST` reader - Soft Reset.Write 1 to perform a Soft Reset. A soft reset performs a Peripheral Reset and also resets the GPIO peripheral but does not reset the CPU or Watchdog Timer."]
pub struct SRST_R(crate::FieldReader<bool, SRST_A>);
impl SRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRST_A {
        match self.bits {
            false => SRST_A::RESET_DONE,
            true => SRST_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        **self == SRST_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == SRST_A::BUSY
    }
}
impl core::ops::Deref for SRST_R {
    type Target = crate::FieldReader<bool, SRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Soft Reset.Write 1 to perform a Soft Reset. A soft reset performs a Peripheral Reset and also resets the GPIO peripheral but does not reset the CPU or Watchdog Timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRST_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<SRST_AW> for bool {
    #[inline(always)]
    fn from(variant: SRST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRST` writer - Soft Reset.Write 1 to perform a Soft Reset. A soft reset performs a Peripheral Reset and also resets the GPIO peripheral but does not reset the CPU or Watchdog Timer."]
pub struct SRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRST_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(SRST_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SRST_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRST_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<PRST_A> for bool {
    #[inline(always)]
    fn from(variant: PRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRST` reader - Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset."]
pub struct PRST_R(crate::FieldReader<bool, PRST_A>);
impl PRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRST_A {
        match self.bits {
            false => PRST_A::RESET_DONE,
            true => PRST_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        **self == PRST_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == PRST_A::BUSY
    }
}
impl core::ops::Deref for PRST_R {
    type Target = crate::FieldReader<bool, PRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRST_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<PRST_AW> for bool {
    #[inline(always)]
    fn from(variant: PRST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRST` writer - Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset."]
pub struct PRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRST_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(PRST_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(PRST_AW::RESET)
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
#[doc = "System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSTEM_A {
    #[doc = "0: Reset Complete"]
    RESET_DONE = 0,
    #[doc = "1: Reset Busy"]
    BUSY = 1,
}
impl From<SYSTEM_A> for bool {
    #[inline(always)]
    fn from(variant: SYSTEM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSTEM` reader - System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer."]
pub struct SYSTEM_R(crate::FieldReader<bool, SYSTEM_A>);
impl SYSTEM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSTEM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSTEM_A {
        match self.bits {
            false => SYSTEM_A::RESET_DONE,
            true => SYSTEM_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DONE`"]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        **self == SYSTEM_A::RESET_DONE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == SYSTEM_A::BUSY
    }
}
impl core::ops::Deref for SYSTEM_R {
    type Target = crate::FieldReader<bool, SYSTEM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSTEM_AW {
    #[doc = "0: Reserved. Do not use."]
    RFU = 0,
    #[doc = "1: Starts reset operation."]
    RESET = 1,
}
impl From<SYSTEM_AW> for bool {
    #[inline(always)]
    fn from(variant: SYSTEM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSTEM` writer - System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer."]
pub struct SYSTEM_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSTEM_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reserved. Do not use."]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(SYSTEM_AW::RFU)
    }
    #[doc = "Starts reset operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SYSTEM_AW::RESET)
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
    #[doc = "Bit 0 - DMA Reset."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Watchdog Timer Reset."]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states."]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer0 Reset. Setting this bit to 1 resets Timer 0 blocks."]
    #[inline(always)]
    pub fn timer0(&self) -> TIMER0_R {
        TIMER0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Timer1 Reset. Setting this bit to 1 resets Timer 1 blocks."]
    #[inline(always)]
    pub fn timer1(&self) -> TIMER1_R {
        TIMER1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Timer2 Reset. Setting this bit to 1 resets Timer 2 blocks."]
    #[inline(always)]
    pub fn timer2(&self) -> TIMER2_R {
        TIMER2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 11 - UART0 Reset. Setting this bit to 1 resets all UART 0 blocks."]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - UART1 Reset. Setting this bit to 1 resets all UART 1 blocks."]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SPI0 Reset. Setting this bit to 1 resets all SPI 0 blocks."]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SPI1 Reset. Setting this bit to 1 resets all SPI 1 blocks."]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - I2C0 Reset."]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Real Time Clock Reset."]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Soft Reset.Write 1 to perform a Soft Reset. A soft reset performs a Peripheral Reset and also resets the GPIO peripheral but does not reset the CPU or Watchdog Timer."]
    #[inline(always)]
    pub fn srst(&self) -> SRST_R {
        SRST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset."]
    #[inline(always)]
    pub fn prst(&self) -> PRST_R {
        PRST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer."]
    #[inline(always)]
    pub fn system(&self) -> SYSTEM_R {
        SYSTEM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Reset."]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 1 - Watchdog Timer Reset."]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W {
        WDT_W { w: self }
    }
    #[doc = "Bit 2 - GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states."]
    #[inline(always)]
    pub fn gpio0(&mut self) -> GPIO0_W {
        GPIO0_W { w: self }
    }
    #[doc = "Bit 5 - Timer0 Reset. Setting this bit to 1 resets Timer 0 blocks."]
    #[inline(always)]
    pub fn timer0(&mut self) -> TIMER0_W {
        TIMER0_W { w: self }
    }
    #[doc = "Bit 6 - Timer1 Reset. Setting this bit to 1 resets Timer 1 blocks."]
    #[inline(always)]
    pub fn timer1(&mut self) -> TIMER1_W {
        TIMER1_W { w: self }
    }
    #[doc = "Bit 7 - Timer2 Reset. Setting this bit to 1 resets Timer 2 blocks."]
    #[inline(always)]
    pub fn timer2(&mut self) -> TIMER2_W {
        TIMER2_W { w: self }
    }
    #[doc = "Bit 11 - UART0 Reset. Setting this bit to 1 resets all UART 0 blocks."]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART0_W {
        UART0_W { w: self }
    }
    #[doc = "Bit 12 - UART1 Reset. Setting this bit to 1 resets all UART 1 blocks."]
    #[inline(always)]
    pub fn uart1(&mut self) -> UART1_W {
        UART1_W { w: self }
    }
    #[doc = "Bit 13 - SPI0 Reset. Setting this bit to 1 resets all SPI 0 blocks."]
    #[inline(always)]
    pub fn spi0(&mut self) -> SPI0_W {
        SPI0_W { w: self }
    }
    #[doc = "Bit 14 - SPI1 Reset. Setting this bit to 1 resets all SPI 1 blocks."]
    #[inline(always)]
    pub fn spi1(&mut self) -> SPI1_W {
        SPI1_W { w: self }
    }
    #[doc = "Bit 16 - I2C0 Reset."]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2C0_W {
        I2C0_W { w: self }
    }
    #[doc = "Bit 17 - Real Time Clock Reset."]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
    #[doc = "Bit 29 - Soft Reset.Write 1 to perform a Soft Reset. A soft reset performs a Peripheral Reset and also resets the GPIO peripheral but does not reset the CPU or Watchdog Timer."]
    #[inline(always)]
    pub fn srst(&mut self) -> SRST_W {
        SRST_W { w: self }
    }
    #[doc = "Bit 30 - Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset."]
    #[inline(always)]
    pub fn prst(&mut self) -> PRST_W {
        PRST_W { w: self }
    }
    #[doc = "Bit 31 - System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer."]
    #[inline(always)]
    pub fn system(&mut self) -> SYSTEM_W {
        SYSTEM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstr0](index.html) module"]
pub struct RSTR0_SPEC;
impl crate::RegisterSpec for RSTR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstr0::R](R) reader structure"]
impl crate::Readable for RSTR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstr0::W](W) writer structure"]
impl crate::Writable for RSTR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTR0 to value 0"]
impl crate::Resettable for RSTR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
