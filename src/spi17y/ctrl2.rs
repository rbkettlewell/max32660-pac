#[doc = "Register `CTRL2` reader"]
pub struct R(crate::R<CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL2` writer"]
pub struct W(crate::W<CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL2_SPEC>;
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
impl From<crate::W<CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock Phase.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: Data Sampled on clock rising edge. Use when in SPI Mode 0 and Mode 2"]
    RISING_EDGE = 0,
    #[doc = "1: Data Sampled on clock falling edge. Use when in SPI Mode 1 and Mode 3"]
    FALLING_EDGE = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Clock Phase."]
pub struct CPHA_R(crate::FieldReader<bool, CPHA_A>);
impl CPHA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPHA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::RISING_EDGE,
            true => CPHA_A::FALLING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == CPHA_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == CPHA_A::FALLING_EDGE
    }
}
impl core::ops::Deref for CPHA_R {
    type Target = crate::FieldReader<bool, CPHA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPHA` writer - Clock Phase."]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPHA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data Sampled on clock rising edge. Use when in SPI Mode 0 and Mode 2"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CPHA_A::RISING_EDGE)
    }
    #[doc = "Data Sampled on clock falling edge. Use when in SPI Mode 1 and Mode 3"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CPHA_A::FALLING_EDGE)
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
#[doc = "Clock Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: Normal Clock. Use when in SPI Mode 0 and Mode 1"]
    NORMAL = 0,
    #[doc = "1: Inverted Clock. Use when in SPI Mode 2 and Mode 3"]
    INVERTED = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity."]
pub struct CPOL_R(crate::FieldReader<bool, CPOL_A>);
impl CPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::NORMAL,
            true => CPOL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == CPOL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == CPOL_A::INVERTED
    }
}
impl core::ops::Deref for CPOL_R {
    type Target = crate::FieldReader<bool, CPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPOL` writer - Clock Polarity."]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal Clock. Use when in SPI Mode 0 and Mode 1"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CPOL_A::NORMAL)
    }
    #[doc = "Inverted Clock. Use when in SPI Mode 2 and Mode 3"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(CPOL_A::INVERTED)
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
#[doc = "Field `SCLK_INV` reader - Reserved - Must Always Be Cleared to 0."]
pub struct SCLK_INV_R(crate::FieldReader<bool, bool>);
impl SCLK_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCLK_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLK_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLK_INV` writer - Reserved - Must Always Be Cleared to 0."]
pub struct SCLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Number of Bits per character.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NUMBITS_A {
    #[doc = "0: 16 bits per character."]
    _0 = 0,
}
impl From<NUMBITS_A> for u8 {
    #[inline(always)]
    fn from(variant: NUMBITS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NUMBITS` reader - Number of Bits per character."]
pub struct NUMBITS_R(crate::FieldReader<u8, NUMBITS_A>);
impl NUMBITS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NUMBITS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NUMBITS_A> {
        match self.bits {
            0 => Some(NUMBITS_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == NUMBITS_A::_0
    }
}
impl core::ops::Deref for NUMBITS_R {
    type Target = crate::FieldReader<u8, NUMBITS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUMBITS` writer - Number of Bits per character."]
pub struct NUMBITS_W<'a> {
    w: &'a mut W,
}
impl<'a> NUMBITS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NUMBITS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "16 bits per character."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NUMBITS_A::_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "SPI Data width.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATA_WIDTH_A {
    #[doc = "0: 1 data pin."]
    MONO = 0,
    #[doc = "1: 2 data pins."]
    DUAL = 1,
    #[doc = "2: 4 data pins."]
    QUAD = 2,
}
impl From<DATA_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DATA_WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DATA_WIDTH` reader - SPI Data width."]
pub struct DATA_WIDTH_R(crate::FieldReader<u8, DATA_WIDTH_A>);
impl DATA_WIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_WIDTH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATA_WIDTH_A> {
        match self.bits {
            0 => Some(DATA_WIDTH_A::MONO),
            1 => Some(DATA_WIDTH_A::DUAL),
            2 => Some(DATA_WIDTH_A::QUAD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MONO`"]
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        **self == DATA_WIDTH_A::MONO
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        **self == DATA_WIDTH_A::DUAL
    }
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        **self == DATA_WIDTH_A::QUAD
    }
}
impl core::ops::Deref for DATA_WIDTH_R {
    type Target = crate::FieldReader<u8, DATA_WIDTH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_WIDTH` writer - SPI Data width."]
pub struct DATA_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_WIDTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 data pin."]
    #[inline(always)]
    pub fn mono(self) -> &'a mut W {
        self.variant(DATA_WIDTH_A::MONO)
    }
    #[doc = "2 data pins."]
    #[inline(always)]
    pub fn dual(self) -> &'a mut W {
        self.variant(DATA_WIDTH_A::DUAL)
    }
    #[doc = "4 data pins."]
    #[inline(always)]
    pub fn quad(self) -> &'a mut W {
        self.variant(DATA_WIDTH_A::QUAD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Three Wire mode. MOSI/MISO pin(s) shared. Only Mono mode suports Four-Wire.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THREE_WIRE_A {
    #[doc = "0: Use four wire mode (Mono only)."]
    DIS = 0,
    #[doc = "1: Use three wire mode."]
    EN = 1,
}
impl From<THREE_WIRE_A> for bool {
    #[inline(always)]
    fn from(variant: THREE_WIRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `THREE_WIRE` reader - Three Wire mode. MOSI/MISO pin(s) shared. Only Mono mode suports Four-Wire."]
pub struct THREE_WIRE_R(crate::FieldReader<bool, THREE_WIRE_A>);
impl THREE_WIRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THREE_WIRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THREE_WIRE_A {
        match self.bits {
            false => THREE_WIRE_A::DIS,
            true => THREE_WIRE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == THREE_WIRE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == THREE_WIRE_A::EN
    }
}
impl core::ops::Deref for THREE_WIRE_R {
    type Target = crate::FieldReader<bool, THREE_WIRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THREE_WIRE` writer - Three Wire mode. MOSI/MISO pin(s) shared. Only Mono mode suports Four-Wire."]
pub struct THREE_WIRE_W<'a> {
    w: &'a mut W,
}
impl<'a> THREE_WIRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THREE_WIRE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use four wire mode (Mono only)."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(THREE_WIRE_A::DIS)
    }
    #[doc = "Use three wire mode."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(THREE_WIRE_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Slave Select Polarity, each Slave Select can have unique polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SS_POL_A {
    #[doc = "1: SS0 active high."]
    SS0_HIGH = 1,
    #[doc = "2: SS1 active high."]
    SS1_HIGH = 2,
    #[doc = "4: SS2 active high."]
    SS2_HIGH = 4,
    #[doc = "8: SS3 active high."]
    SS3_HIGH = 8,
}
impl From<SS_POL_A> for u8 {
    #[inline(always)]
    fn from(variant: SS_POL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SS_POL` reader - Slave Select Polarity, each Slave Select can have unique polarity."]
pub struct SS_POL_R(crate::FieldReader<u8, SS_POL_A>);
impl SS_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SS_POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SS_POL_A> {
        match self.bits {
            1 => Some(SS_POL_A::SS0_HIGH),
            2 => Some(SS_POL_A::SS1_HIGH),
            4 => Some(SS_POL_A::SS2_HIGH),
            8 => Some(SS_POL_A::SS3_HIGH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SS0_HIGH`"]
    #[inline(always)]
    pub fn is_ss0_high(&self) -> bool {
        **self == SS_POL_A::SS0_HIGH
    }
    #[doc = "Checks if the value of the field is `SS1_HIGH`"]
    #[inline(always)]
    pub fn is_ss1_high(&self) -> bool {
        **self == SS_POL_A::SS1_HIGH
    }
    #[doc = "Checks if the value of the field is `SS2_HIGH`"]
    #[inline(always)]
    pub fn is_ss2_high(&self) -> bool {
        **self == SS_POL_A::SS2_HIGH
    }
    #[doc = "Checks if the value of the field is `SS3_HIGH`"]
    #[inline(always)]
    pub fn is_ss3_high(&self) -> bool {
        **self == SS_POL_A::SS3_HIGH
    }
}
impl core::ops::Deref for SS_POL_R {
    type Target = crate::FieldReader<u8, SS_POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SS_POL` writer - Slave Select Polarity, each Slave Select can have unique polarity."]
pub struct SS_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS_POL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SS0 active high."]
    #[inline(always)]
    pub fn ss0_high(self) -> &'a mut W {
        self.variant(SS_POL_A::SS0_HIGH)
    }
    #[doc = "SS1 active high."]
    #[inline(always)]
    pub fn ss1_high(self) -> &'a mut W {
        self.variant(SS_POL_A::SS1_HIGH)
    }
    #[doc = "SS2 active high."]
    #[inline(always)]
    pub fn ss2_high(self) -> &'a mut W {
        self.variant(SS_POL_A::SS2_HIGH)
    }
    #[doc = "SS3 active high."]
    #[inline(always)]
    pub fn ss3_high(self) -> &'a mut W {
        self.variant(SS_POL_A::SS3_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Slave Ready Polarity, each Slave Ready can have unique polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRPOL_A {
    #[doc = "1: SR0 active high."]
    SR0_HIGH = 1,
    #[doc = "2: SR1 active high."]
    SR1_HIGH = 2,
    #[doc = "4: SR2 active high."]
    SR2_HIGH = 4,
    #[doc = "8: SR3 active high."]
    SR3_HIGH = 8,
    #[doc = "16: SR4 active high."]
    SR4_HIGH = 16,
    #[doc = "32: SR5 active high."]
    SR5_HIGH = 32,
    #[doc = "64: SR6 active high."]
    SR6_HIGH = 64,
    #[doc = "128: SR7 active high."]
    SR7_HIGH = 128,
}
impl From<SRPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: SRPOL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRPOL` reader - Slave Ready Polarity, each Slave Ready can have unique polarity."]
pub struct SRPOL_R(crate::FieldReader<u8, SRPOL_A>);
impl SRPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRPOL_A> {
        match self.bits {
            1 => Some(SRPOL_A::SR0_HIGH),
            2 => Some(SRPOL_A::SR1_HIGH),
            4 => Some(SRPOL_A::SR2_HIGH),
            8 => Some(SRPOL_A::SR3_HIGH),
            16 => Some(SRPOL_A::SR4_HIGH),
            32 => Some(SRPOL_A::SR5_HIGH),
            64 => Some(SRPOL_A::SR6_HIGH),
            128 => Some(SRPOL_A::SR7_HIGH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR0_HIGH`"]
    #[inline(always)]
    pub fn is_sr0_high(&self) -> bool {
        **self == SRPOL_A::SR0_HIGH
    }
    #[doc = "Checks if the value of the field is `SR1_HIGH`"]
    #[inline(always)]
    pub fn is_sr1_high(&self) -> bool {
        **self == SRPOL_A::SR1_HIGH
    }
    #[doc = "Checks if the value of the field is `SR2_HIGH`"]
    #[inline(always)]
    pub fn is_sr2_high(&self) -> bool {
        **self == SRPOL_A::SR2_HIGH
    }
    #[doc = "Checks if the value of the field is `SR3_HIGH`"]
    #[inline(always)]
    pub fn is_sr3_high(&self) -> bool {
        **self == SRPOL_A::SR3_HIGH
    }
    #[doc = "Checks if the value of the field is `SR4_HIGH`"]
    #[inline(always)]
    pub fn is_sr4_high(&self) -> bool {
        **self == SRPOL_A::SR4_HIGH
    }
    #[doc = "Checks if the value of the field is `SR5_HIGH`"]
    #[inline(always)]
    pub fn is_sr5_high(&self) -> bool {
        **self == SRPOL_A::SR5_HIGH
    }
    #[doc = "Checks if the value of the field is `SR6_HIGH`"]
    #[inline(always)]
    pub fn is_sr6_high(&self) -> bool {
        **self == SRPOL_A::SR6_HIGH
    }
    #[doc = "Checks if the value of the field is `SR7_HIGH`"]
    #[inline(always)]
    pub fn is_sr7_high(&self) -> bool {
        **self == SRPOL_A::SR7_HIGH
    }
}
impl core::ops::Deref for SRPOL_R {
    type Target = crate::FieldReader<u8, SRPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRPOL` writer - Slave Ready Polarity, each Slave Ready can have unique polarity."]
pub struct SRPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRPOL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SR0 active high."]
    #[inline(always)]
    pub fn sr0_high(self) -> &'a mut W {
        self.variant(SRPOL_A::SR0_HIGH)
    }
    #[doc = "SR1 active high."]
    #[inline(always)]
    pub fn sr1_high(self) -> &'a mut W {
        self.variant(SRPOL_A::SR1_HIGH)
    }
    #[doc = "SR2 active high."]
    #[inline(always)]
    pub fn sr2_high(self) -> &'a mut W {
        self.variant(SRPOL_A::SR2_HIGH)
    }
    #[doc = "SR3 active high."]
    #[inline(always)]
    pub fn sr3_high(self) -> &'a mut W {
        self.variant(SRPOL_A::SR3_HIGH)
    }
    #[doc = "SR4 active high."]
    #[inline(always)]
    pub fn sr4_high(self) -> &'a mut W {
        self.variant(SRPOL_A::SR4_HIGH)
    }
    #[doc = "SR5 active high."]
    #[inline(always)]
    pub fn sr5_high(self) -> &'a mut W {
        self.variant(SRPOL_A::SR5_HIGH)
    }
    #[doc = "SR6 active high."]
    #[inline(always)]
    pub fn sr6_high(self) -> &'a mut W {
        self.variant(SRPOL_A::SR6_HIGH)
    }
    #[doc = "SR7 active high."]
    #[inline(always)]
    pub fn sr7_high(self) -> &'a mut W {
        self.variant(SRPOL_A::SR7_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock Phase."]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock Polarity."]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reserved - Must Always Be Cleared to 0."]
    #[inline(always)]
    pub fn sclk_inv(&self) -> SCLK_INV_R {
        SCLK_INV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Number of Bits per character."]
    #[inline(always)]
    pub fn numbits(&self) -> NUMBITS_R {
        NUMBITS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - SPI Data width."]
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Three Wire mode. MOSI/MISO pin(s) shared. Only Mono mode suports Four-Wire."]
    #[inline(always)]
    pub fn three_wire(&self) -> THREE_WIRE_R {
        THREE_WIRE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Slave Select Polarity, each Slave Select can have unique polarity."]
    #[inline(always)]
    pub fn ss_pol(&self) -> SS_POL_R {
        SS_POL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Slave Ready Polarity, each Slave Ready can have unique polarity."]
    #[inline(always)]
    pub fn srpol(&self) -> SRPOL_R {
        SRPOL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Phase."]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bit 1 - Clock Polarity."]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 4 - Reserved - Must Always Be Cleared to 0."]
    #[inline(always)]
    pub fn sclk_inv(&mut self) -> SCLK_INV_W {
        SCLK_INV_W { w: self }
    }
    #[doc = "Bits 8:11 - Number of Bits per character."]
    #[inline(always)]
    pub fn numbits(&mut self) -> NUMBITS_W {
        NUMBITS_W { w: self }
    }
    #[doc = "Bits 12:13 - SPI Data width."]
    #[inline(always)]
    pub fn data_width(&mut self) -> DATA_WIDTH_W {
        DATA_WIDTH_W { w: self }
    }
    #[doc = "Bit 15 - Three Wire mode. MOSI/MISO pin(s) shared. Only Mono mode suports Four-Wire."]
    #[inline(always)]
    pub fn three_wire(&mut self) -> THREE_WIRE_W {
        THREE_WIRE_W { w: self }
    }
    #[doc = "Bits 16:23 - Slave Select Polarity, each Slave Select can have unique polarity."]
    #[inline(always)]
    pub fn ss_pol(&mut self) -> SS_POL_W {
        SS_POL_W { w: self }
    }
    #[doc = "Bits 24:31 - Slave Ready Polarity, each Slave Ready can have unique polarity."]
    #[inline(always)]
    pub fn srpol(&mut self) -> SRPOL_W {
        SRPOL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for controlling SPI peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](index.html) module"]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl2::R](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
