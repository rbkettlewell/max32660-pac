#[doc = "Register `MOD` reader"]
pub struct R(crate::R<MOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOD` writer"]
pub struct W(crate::W<MOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOD_SPEC>;
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
impl From<crate::W<MOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Slave Select Value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSV_A {
    #[doc = "0: The SSEL pin will be driven low."]
    LO = 0,
    #[doc = "1: The SSEL pin will be driven high."]
    HI = 1,
}
impl From<SSV_A> for bool {
    #[inline(always)]
    fn from(variant: SSV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSV` reader - Slave Select Value."]
pub struct SSV_R(crate::FieldReader<bool, SSV_A>);
impl SSV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSV_A {
        match self.bits {
            false => SSV_A::LO,
            true => SSV_A::HI,
        }
    }
    #[doc = "Checks if the value of the field is `LO`"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        **self == SSV_A::LO
    }
    #[doc = "Checks if the value of the field is `HI`"]
    #[inline(always)]
    pub fn is_hi(&self) -> bool {
        **self == SSV_A::HI
    }
}
impl core::ops::Deref for SSV_R {
    type Target = crate::FieldReader<bool, SSV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSV` writer - Slave Select Value."]
pub struct SSV_W<'a> {
    w: &'a mut W,
}
impl<'a> SSV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The SSEL pin will be driven low."]
    #[inline(always)]
    pub fn lo(self) -> &'a mut W {
        self.variant(SSV_A::LO)
    }
    #[doc = "The SSEL pin will be driven high."]
    #[inline(always)]
    pub fn hi(self) -> &'a mut W {
        self.variant(SSV_A::HI)
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
#[doc = "Slave Select I/O.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSIO_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "1: `1`"]
    OUTPUT = 1,
}
impl From<SSIO_A> for bool {
    #[inline(always)]
    fn from(variant: SSIO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSIO` reader - Slave Select I/O."]
pub struct SSIO_R(crate::FieldReader<bool, SSIO_A>);
impl SSIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSIO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSIO_A {
        match self.bits {
            false => SSIO_A::INPUT,
            true => SSIO_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == SSIO_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == SSIO_A::OUTPUT
    }
}
impl core::ops::Deref for SSIO_R {
    type Target = crate::FieldReader<bool, SSIO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSIO` writer - Slave Select I/O."]
pub struct SSIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SSIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSIO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(SSIO_A::INPUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(SSIO_A::OUTPUT)
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NUMBITS_A {
    #[doc = "0: `0`"]
    BITS16 = 0,
    #[doc = "1: `1`"]
    BITS1 = 1,
    #[doc = "2: `10`"]
    BITS2 = 2,
    #[doc = "3: `11`"]
    BITS3 = 3,
    #[doc = "4: `100`"]
    BITS4 = 4,
    #[doc = "5: `101`"]
    BITS5 = 5,
    #[doc = "6: `110`"]
    BITS6 = 6,
    #[doc = "7: `111`"]
    BITS7 = 7,
    #[doc = "8: `1000`"]
    BITS8 = 8,
    #[doc = "9: `1001`"]
    BITS9 = 9,
    #[doc = "10: `1010`"]
    BITS10 = 10,
    #[doc = "11: `1011`"]
    BITS11 = 11,
    #[doc = "12: `1100`"]
    BITS12 = 12,
    #[doc = "13: `1101`"]
    BITS13 = 13,
    #[doc = "14: `1110`"]
    BITS14 = 14,
    #[doc = "15: `1111`"]
    BITS15 = 15,
}
impl From<NUMBITS_A> for u8 {
    #[inline(always)]
    fn from(variant: NUMBITS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NUMBITS` reader - "]
pub struct NUMBITS_R(crate::FieldReader<u8, NUMBITS_A>);
impl NUMBITS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NUMBITS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NUMBITS_A {
        match self.bits {
            0 => NUMBITS_A::BITS16,
            1 => NUMBITS_A::BITS1,
            2 => NUMBITS_A::BITS2,
            3 => NUMBITS_A::BITS3,
            4 => NUMBITS_A::BITS4,
            5 => NUMBITS_A::BITS5,
            6 => NUMBITS_A::BITS6,
            7 => NUMBITS_A::BITS7,
            8 => NUMBITS_A::BITS8,
            9 => NUMBITS_A::BITS9,
            10 => NUMBITS_A::BITS10,
            11 => NUMBITS_A::BITS11,
            12 => NUMBITS_A::BITS12,
            13 => NUMBITS_A::BITS13,
            14 => NUMBITS_A::BITS14,
            15 => NUMBITS_A::BITS15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BITS16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        **self == NUMBITS_A::BITS16
    }
    #[doc = "Checks if the value of the field is `BITS1`"]
    #[inline(always)]
    pub fn is_bits1(&self) -> bool {
        **self == NUMBITS_A::BITS1
    }
    #[doc = "Checks if the value of the field is `BITS2`"]
    #[inline(always)]
    pub fn is_bits2(&self) -> bool {
        **self == NUMBITS_A::BITS2
    }
    #[doc = "Checks if the value of the field is `BITS3`"]
    #[inline(always)]
    pub fn is_bits3(&self) -> bool {
        **self == NUMBITS_A::BITS3
    }
    #[doc = "Checks if the value of the field is `BITS4`"]
    #[inline(always)]
    pub fn is_bits4(&self) -> bool {
        **self == NUMBITS_A::BITS4
    }
    #[doc = "Checks if the value of the field is `BITS5`"]
    #[inline(always)]
    pub fn is_bits5(&self) -> bool {
        **self == NUMBITS_A::BITS5
    }
    #[doc = "Checks if the value of the field is `BITS6`"]
    #[inline(always)]
    pub fn is_bits6(&self) -> bool {
        **self == NUMBITS_A::BITS6
    }
    #[doc = "Checks if the value of the field is `BITS7`"]
    #[inline(always)]
    pub fn is_bits7(&self) -> bool {
        **self == NUMBITS_A::BITS7
    }
    #[doc = "Checks if the value of the field is `BITS8`"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        **self == NUMBITS_A::BITS8
    }
    #[doc = "Checks if the value of the field is `BITS9`"]
    #[inline(always)]
    pub fn is_bits9(&self) -> bool {
        **self == NUMBITS_A::BITS9
    }
    #[doc = "Checks if the value of the field is `BITS10`"]
    #[inline(always)]
    pub fn is_bits10(&self) -> bool {
        **self == NUMBITS_A::BITS10
    }
    #[doc = "Checks if the value of the field is `BITS11`"]
    #[inline(always)]
    pub fn is_bits11(&self) -> bool {
        **self == NUMBITS_A::BITS11
    }
    #[doc = "Checks if the value of the field is `BITS12`"]
    #[inline(always)]
    pub fn is_bits12(&self) -> bool {
        **self == NUMBITS_A::BITS12
    }
    #[doc = "Checks if the value of the field is `BITS13`"]
    #[inline(always)]
    pub fn is_bits13(&self) -> bool {
        **self == NUMBITS_A::BITS13
    }
    #[doc = "Checks if the value of the field is `BITS14`"]
    #[inline(always)]
    pub fn is_bits14(&self) -> bool {
        **self == NUMBITS_A::BITS14
    }
    #[doc = "Checks if the value of the field is `BITS15`"]
    #[inline(always)]
    pub fn is_bits15(&self) -> bool {
        **self == NUMBITS_A::BITS15
    }
}
impl core::ops::Deref for NUMBITS_R {
    type Target = crate::FieldReader<u8, NUMBITS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUMBITS` writer - "]
pub struct NUMBITS_W<'a> {
    w: &'a mut W,
}
impl<'a> NUMBITS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NUMBITS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(NUMBITS_A::BITS16)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn bits1(self) -> &'a mut W {
        self.variant(NUMBITS_A::BITS1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn bits2(self) -> &'a mut W {
        self.variant(NUMBITS_A::BITS2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn bits3(self) -> &'a mut W {
        self.variant(NUMBITS_A::BITS3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn bits4(self) -> &'a mut W {
        self.variant(NUMBITS_A::BITS4)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn bits5(self) -> &'a mut W {
        self.variant(NUMBITS_A::BITS5)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn bits6(self) -> &'a mut W {
        self.variant(NUMBITS_A::BITS6)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn bits7(self) -> &'a mut W {
        self.variant(NUMBITS_A::BITS7)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(NUMBITS_A::BITS8)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn bits9(self) -> &'a mut W {
        self.variant(NUMBITS_A::BITS9)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn bits10(self) -> &'a mut W {
        self.variant(NUMBITS_A::BITS10)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn bits11(self) -> &'a mut W {
        self.variant(NUMBITS_A::BITS11)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn bits12(self) -> &'a mut W {
        self.variant(NUMBITS_A::BITS12)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn bits13(self) -> &'a mut W {
        self.variant(NUMBITS_A::BITS13)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn bits14(self) -> &'a mut W {
        self.variant(NUMBITS_A::BITS14)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn bits15(self) -> &'a mut W {
        self.variant(NUMBITS_A::BITS15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | ((value as u32 & 0x0f) << 2);
        self.w
    }
}
#[doc = "Transmit Left Justify.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_LJ_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_LJ_A> for bool {
    #[inline(always)]
    fn from(variant: TX_LJ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_LJ` reader - Transmit Left Justify."]
pub struct TX_LJ_R(crate::FieldReader<bool, TX_LJ_A>);
impl TX_LJ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_LJ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_LJ_A {
        match self.bits {
            false => TX_LJ_A::DISABLE,
            true => TX_LJ_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TX_LJ_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TX_LJ_A::ENABLE
    }
}
impl core::ops::Deref for TX_LJ_R {
    type Target = crate::FieldReader<bool, TX_LJ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_LJ` writer - Transmit Left Justify."]
pub struct TX_LJ_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LJ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_LJ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TX_LJ_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TX_LJ_A::ENABLE)
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
#[doc = "Slave Select 1. If SPI is enabled and in master mode, the SSEL_1 is driven according to this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSL1_A {
    #[doc = "0: High."]
    HI = 0,
    #[doc = "1: Low."]
    LO = 1,
}
impl From<SSL1_A> for bool {
    #[inline(always)]
    fn from(variant: SSL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSL1` reader - Slave Select 1. If SPI is enabled and in master mode, the SSEL_1 is driven according to this bit."]
pub struct SSL1_R(crate::FieldReader<bool, SSL1_A>);
impl SSL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSL1_A {
        match self.bits {
            false => SSL1_A::HI,
            true => SSL1_A::LO,
        }
    }
    #[doc = "Checks if the value of the field is `HI`"]
    #[inline(always)]
    pub fn is_hi(&self) -> bool {
        **self == SSL1_A::HI
    }
    #[doc = "Checks if the value of the field is `LO`"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        **self == SSL1_A::LO
    }
}
impl core::ops::Deref for SSL1_R {
    type Target = crate::FieldReader<bool, SSL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSL1` writer - Slave Select 1. If SPI is enabled and in master mode, the SSEL_1 is driven according to this bit."]
pub struct SSL1_W<'a> {
    w: &'a mut W,
}
impl<'a> SSL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSL1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "High."]
    #[inline(always)]
    pub fn hi(self) -> &'a mut W {
        self.variant(SSL1_A::HI)
    }
    #[doc = "Low."]
    #[inline(always)]
    pub fn lo(self) -> &'a mut W {
        self.variant(SSL1_A::LO)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Slave Select 2. If SPI is enabled and in master mode, the SSEL_2 is driven according to this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSL2_A {
    #[doc = "0: High."]
    HI = 0,
    #[doc = "1: Low."]
    LO = 1,
}
impl From<SSL2_A> for bool {
    #[inline(always)]
    fn from(variant: SSL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSL2` reader - Slave Select 2. If SPI is enabled and in master mode, the SSEL_2 is driven according to this bit."]
pub struct SSL2_R(crate::FieldReader<bool, SSL2_A>);
impl SSL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSL2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSL2_A {
        match self.bits {
            false => SSL2_A::HI,
            true => SSL2_A::LO,
        }
    }
    #[doc = "Checks if the value of the field is `HI`"]
    #[inline(always)]
    pub fn is_hi(&self) -> bool {
        **self == SSL2_A::HI
    }
    #[doc = "Checks if the value of the field is `LO`"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        **self == SSL2_A::LO
    }
}
impl core::ops::Deref for SSL2_R {
    type Target = crate::FieldReader<bool, SSL2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSL2` writer - Slave Select 2. If SPI is enabled and in master mode, the SSEL_2 is driven according to this bit."]
pub struct SSL2_W<'a> {
    w: &'a mut W,
}
impl<'a> SSL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSL2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "High."]
    #[inline(always)]
    pub fn hi(self) -> &'a mut W {
        self.variant(SSL2_A::HI)
    }
    #[doc = "Low."]
    #[inline(always)]
    pub fn lo(self) -> &'a mut W {
        self.variant(SSL2_A::LO)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Slave Select 3. If SPI is enabled and in master mode, the SSEL_3 is driven according to this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSL3_A {
    #[doc = "0: High."]
    HI = 0,
    #[doc = "1: Low."]
    LO = 1,
}
impl From<SSL3_A> for bool {
    #[inline(always)]
    fn from(variant: SSL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSL3` reader - Slave Select 3. If SPI is enabled and in master mode, the SSEL_3 is driven according to this bit."]
pub struct SSL3_R(crate::FieldReader<bool, SSL3_A>);
impl SSL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSL3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSL3_A {
        match self.bits {
            false => SSL3_A::HI,
            true => SSL3_A::LO,
        }
    }
    #[doc = "Checks if the value of the field is `HI`"]
    #[inline(always)]
    pub fn is_hi(&self) -> bool {
        **self == SSL3_A::HI
    }
    #[doc = "Checks if the value of the field is `LO`"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        **self == SSL3_A::LO
    }
}
impl core::ops::Deref for SSL3_R {
    type Target = crate::FieldReader<bool, SSL3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSL3` writer - Slave Select 3. If SPI is enabled and in master mode, the SSEL_3 is driven according to this bit."]
pub struct SSL3_W<'a> {
    w: &'a mut W,
}
impl<'a> SSL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSL3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "High."]
    #[inline(always)]
    pub fn hi(self) -> &'a mut W {
        self.variant(SSL3_A::HI)
    }
    #[doc = "Low."]
    #[inline(always)]
    pub fn lo(self) -> &'a mut W {
        self.variant(SSL3_A::LO)
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
impl R {
    #[doc = "Bit 0 - Slave Select Value."]
    #[inline(always)]
    pub fn ssv(&self) -> SSV_R {
        SSV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Slave Select I/O."]
    #[inline(always)]
    pub fn ssio(&self) -> SSIO_R {
        SSIO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn numbits(&self) -> NUMBITS_R {
        NUMBITS_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Transmit Left Justify."]
    #[inline(always)]
    pub fn tx_lj(&self) -> TX_LJ_R {
        TX_LJ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Slave Select 1. If SPI is enabled and in master mode, the SSEL_1 is driven according to this bit."]
    #[inline(always)]
    pub fn ssl1(&self) -> SSL1_R {
        SSL1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Slave Select 2. If SPI is enabled and in master mode, the SSEL_2 is driven according to this bit."]
    #[inline(always)]
    pub fn ssl2(&self) -> SSL2_R {
        SSL2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Slave Select 3. If SPI is enabled and in master mode, the SSEL_3 is driven according to this bit."]
    #[inline(always)]
    pub fn ssl3(&self) -> SSL3_R {
        SSL3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Select Value."]
    #[inline(always)]
    pub fn ssv(&mut self) -> SSV_W {
        SSV_W { w: self }
    }
    #[doc = "Bit 1 - Slave Select I/O."]
    #[inline(always)]
    pub fn ssio(&mut self) -> SSIO_W {
        SSIO_W { w: self }
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn numbits(&mut self) -> NUMBITS_W {
        NUMBITS_W { w: self }
    }
    #[doc = "Bit 7 - Transmit Left Justify."]
    #[inline(always)]
    pub fn tx_lj(&mut self) -> TX_LJ_W {
        TX_LJ_W { w: self }
    }
    #[doc = "Bit 8 - Slave Select 1. If SPI is enabled and in master mode, the SSEL_1 is driven according to this bit."]
    #[inline(always)]
    pub fn ssl1(&mut self) -> SSL1_W {
        SSL1_W { w: self }
    }
    #[doc = "Bit 9 - Slave Select 2. If SPI is enabled and in master mode, the SSEL_2 is driven according to this bit."]
    #[inline(always)]
    pub fn ssl2(&mut self) -> SSL2_W {
        SSL2_W { w: self }
    }
    #[doc = "Bit 10 - Slave Select 3. If SPI is enabled and in master mode, the SSEL_3 is driven according to this bit."]
    #[inline(always)]
    pub fn ssl3(&mut self) -> SSL3_W {
        SSL3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Mode Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mod_](index.html) module"]
pub struct MOD_SPEC;
impl crate::RegisterSpec for MOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mod_::R](R) reader structure"]
impl crate::Readable for MOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mod_::W](W) writer structure"]
impl crate::Writable for MOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MOD to value 0"]
impl crate::Resettable for MOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
