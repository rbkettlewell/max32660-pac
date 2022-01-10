#[doc = "Register `CLKCN` reader"]
pub struct R(crate::R<CLKCN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKCN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKCN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKCN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKCN` writer"]
pub struct W(crate::W<CLKCN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKCN_SPEC>;
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
impl From<crate::W<CLKCN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKCN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Prescaler Select. This 3 bit field sets the system operating frequency by controlling the prescaler that divides the output of the PLL0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSC_A {
    #[doc = "0: Divide by 1."]
    DIV1 = 0,
    #[doc = "1: Divide by 2."]
    DIV2 = 1,
    #[doc = "2: Divide by 4."]
    DIV4 = 2,
    #[doc = "3: Divide by 8."]
    DIV8 = 3,
    #[doc = "4: Divide by 16."]
    DIV16 = 4,
    #[doc = "5: Divide by 32."]
    DIV32 = 5,
    #[doc = "6: Divide by 64."]
    DIV64 = 6,
    #[doc = "7: Divide by 128."]
    DIV128 = 7,
}
impl From<PSC_A> for u8 {
    #[inline(always)]
    fn from(variant: PSC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSC` reader - Prescaler Select. This 3 bit field sets the system operating frequency by controlling the prescaler that divides the output of the PLL0."]
pub struct PSC_R(crate::FieldReader<u8, PSC_A>);
impl PSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PSC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSC_A {
        match self.bits {
            0 => PSC_A::DIV1,
            1 => PSC_A::DIV2,
            2 => PSC_A::DIV4,
            3 => PSC_A::DIV8,
            4 => PSC_A::DIV16,
            5 => PSC_A::DIV32,
            6 => PSC_A::DIV64,
            7 => PSC_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == PSC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PSC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == PSC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == PSC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == PSC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        **self == PSC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        **self == PSC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        **self == PSC_A::DIV128
    }
}
impl core::ops::Deref for PSC_R {
    type Target = crate::FieldReader<u8, PSC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSC` writer - Prescaler Select. This 3 bit field sets the system operating frequency by controlling the prescaler that divides the output of the PLL0."]
pub struct PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PSC_A::DIV1)
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PSC_A::DIV2)
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PSC_A::DIV4)
    }
    #[doc = "Divide by 8."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PSC_A::DIV8)
    }
    #[doc = "Divide by 16."]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PSC_A::DIV16)
    }
    #[doc = "Divide by 32."]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PSC_A::DIV32)
    }
    #[doc = "Divide by 64."]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PSC_A::DIV64)
    }
    #[doc = "Divide by 128."]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PSC_A::DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Clock Source Select. This 3 bit field selects the source for the system clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: The internal 96 MHz oscillator is used for the system clock."]
    HIRC = 0,
    #[doc = "3: The nano-ring output is used for the system clock."]
    NANORING = 3,
    #[doc = "6: HFXIN is used for the system clock."]
    HFXIN = 6,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKSEL` reader - Clock Source Select. This 3 bit field selects the source for the system clock."]
pub struct CLKSEL_R(crate::FieldReader<u8, CLKSEL_A>);
impl CLKSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            0 => Some(CLKSEL_A::HIRC),
            3 => Some(CLKSEL_A::NANORING),
            6 => Some(CLKSEL_A::HFXIN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HIRC`"]
    #[inline(always)]
    pub fn is_hirc(&self) -> bool {
        **self == CLKSEL_A::HIRC
    }
    #[doc = "Checks if the value of the field is `NANORING`"]
    #[inline(always)]
    pub fn is_nano_ring(&self) -> bool {
        **self == CLKSEL_A::NANORING
    }
    #[doc = "Checks if the value of the field is `HFXIN`"]
    #[inline(always)]
    pub fn is_hfx_in(&self) -> bool {
        **self == CLKSEL_A::HFXIN
    }
}
impl core::ops::Deref for CLKSEL_R {
    type Target = crate::FieldReader<u8, CLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKSEL` writer - Clock Source Select. This 3 bit field selects the source for the system clock."]
pub struct CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The internal 96 MHz oscillator is used for the system clock."]
    #[inline(always)]
    pub fn hirc(self) -> &'a mut W {
        self.variant(CLKSEL_A::HIRC)
    }
    #[doc = "The nano-ring output is used for the system clock."]
    #[inline(always)]
    pub fn nano_ring(self) -> &'a mut W {
        self.variant(CLKSEL_A::NANORING)
    }
    #[doc = "HFXIN is used for the system clock."]
    #[inline(always)]
    pub fn hfx_in(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFXIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Clock Ready. This read only bit reflects whether the currently selected system clock source is running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKRDY_A {
    #[doc = "0: Switchover to the new clock source (as selected by CLKSEL) has not yet occurred."]
    BUSY = 0,
    #[doc = "1: System clock running from CLKSEL clock source."]
    READY = 1,
}
impl From<CKRDY_A> for bool {
    #[inline(always)]
    fn from(variant: CKRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKRDY` reader - Clock Ready. This read only bit reflects whether the currently selected system clock source is running."]
pub struct CKRDY_R(crate::FieldReader<bool, CKRDY_A>);
impl CKRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CKRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKRDY_A {
        match self.bits {
            false => CKRDY_A::BUSY,
            true => CKRDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == CKRDY_A::BUSY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == CKRDY_A::READY
    }
}
impl core::ops::Deref for CKRDY_R {
    type Target = crate::FieldReader<bool, CKRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "32kHz Crystal Oscillator Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum X32K_EN_A {
    #[doc = "0: Is Disabled."]
    DIS = 0,
    #[doc = "1: Is Enabled."]
    EN = 1,
}
impl From<X32K_EN_A> for bool {
    #[inline(always)]
    fn from(variant: X32K_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `X32K_EN` reader - 32kHz Crystal Oscillator Enable."]
pub struct X32K_EN_R(crate::FieldReader<bool, X32K_EN_A>);
impl X32K_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32K_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> X32K_EN_A {
        match self.bits {
            false => X32K_EN_A::DIS,
            true => X32K_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == X32K_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == X32K_EN_A::EN
    }
}
impl core::ops::Deref for X32K_EN_R {
    type Target = crate::FieldReader<bool, X32K_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32K_EN` writer - 32kHz Crystal Oscillator Enable."]
pub struct X32K_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> X32K_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: X32K_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Is Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(X32K_EN_A::DIS)
    }
    #[doc = "Is Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(X32K_EN_A::EN)
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
#[doc = "60MHz High Frequency Internal Reference Clock Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIRC_EN_A {
    #[doc = "0: Is Disabled."]
    DIS = 0,
    #[doc = "1: Is Enabled."]
    EN = 1,
}
impl From<HIRC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HIRC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIRC_EN` reader - 60MHz High Frequency Internal Reference Clock Enable."]
pub struct HIRC_EN_R(crate::FieldReader<bool, HIRC_EN_A>);
impl HIRC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HIRC_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIRC_EN_A {
        match self.bits {
            false => HIRC_EN_A::DIS,
            true => HIRC_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == HIRC_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == HIRC_EN_A::EN
    }
}
impl core::ops::Deref for HIRC_EN_R {
    type Target = crate::FieldReader<bool, HIRC_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIRC_EN` writer - 60MHz High Frequency Internal Reference Clock Enable."]
pub struct HIRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HIRC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIRC_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Is Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(HIRC_EN_A::DIS)
    }
    #[doc = "Is Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(HIRC_EN_A::EN)
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
#[doc = "32kHz Crystal Oscillator Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum X32K_RDY_A {
    #[doc = "0: Not Ready"]
    NOT = 0,
    #[doc = "1: X32K Ready"]
    READY = 1,
}
impl From<X32K_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: X32K_RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `X32K_RDY` reader - 32kHz Crystal Oscillator Ready"]
pub struct X32K_RDY_R(crate::FieldReader<bool, X32K_RDY_A>);
impl X32K_RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32K_RDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> X32K_RDY_A {
        match self.bits {
            false => X32K_RDY_A::NOT,
            true => X32K_RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        **self == X32K_RDY_A::NOT
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == X32K_RDY_A::READY
    }
}
impl core::ops::Deref for X32K_RDY_R {
    type Target = crate::FieldReader<bool, X32K_RDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "60MHz HIRC Ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIRC_RDY_A {
    #[doc = "0: Not Ready"]
    NOT = 0,
    #[doc = "1: HIRC Ready"]
    READY = 1,
}
impl From<HIRC_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: HIRC_RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIRC_RDY` reader - 60MHz HIRC Ready."]
pub struct HIRC_RDY_R(crate::FieldReader<bool, HIRC_RDY_A>);
impl HIRC_RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HIRC_RDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIRC_RDY_A {
        match self.bits {
            false => HIRC_RDY_A::NOT,
            true => HIRC_RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        **self == HIRC_RDY_A::NOT
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == HIRC_RDY_A::READY
    }
}
impl core::ops::Deref for HIRC_RDY_R {
    type Target = crate::FieldReader<bool, HIRC_RDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIRC_RDY` writer - 60MHz HIRC Ready."]
pub struct HIRC_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HIRC_RDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIRC_RDY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn not(self) -> &'a mut W {
        self.variant(HIRC_RDY_A::NOT)
    }
    #[doc = "HIRC Ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(HIRC_RDY_A::READY)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "8kHz Low Frequency Reference Clock Ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LIRC8K_RDY_A {
    #[doc = "0: Not Ready"]
    NOT = 0,
    #[doc = "1: Clock Ready"]
    READY = 1,
}
impl From<LIRC8K_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: LIRC8K_RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIRC8K_RDY` reader - 8kHz Low Frequency Reference Clock Ready."]
pub struct LIRC8K_RDY_R(crate::FieldReader<bool, LIRC8K_RDY_A>);
impl LIRC8K_RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LIRC8K_RDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIRC8K_RDY_A {
        match self.bits {
            false => LIRC8K_RDY_A::NOT,
            true => LIRC8K_RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        **self == LIRC8K_RDY_A::NOT
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == LIRC8K_RDY_A::READY
    }
}
impl core::ops::Deref for LIRC8K_RDY_R {
    type Target = crate::FieldReader<bool, LIRC8K_RDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LIRC8K_RDY` writer - 8kHz Low Frequency Reference Clock Ready."]
pub struct LIRC8K_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> LIRC8K_RDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LIRC8K_RDY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not Ready"]
    #[inline(always)]
    pub fn not(self) -> &'a mut W {
        self.variant(LIRC8K_RDY_A::NOT)
    }
    #[doc = "Clock Ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(LIRC8K_RDY_A::READY)
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
impl R {
    #[doc = "Bits 6:8 - Prescaler Select. This 3 bit field sets the system operating frequency by controlling the prescaler that divides the output of the PLL0."]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - Clock Source Select. This 3 bit field selects the source for the system clock."]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 13 - Clock Ready. This read only bit reflects whether the currently selected system clock source is running."]
    #[inline(always)]
    pub fn ckrdy(&self) -> CKRDY_R {
        CKRDY_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 32kHz Crystal Oscillator Enable."]
    #[inline(always)]
    pub fn x32k_en(&self) -> X32K_EN_R {
        X32K_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 60MHz High Frequency Internal Reference Clock Enable."]
    #[inline(always)]
    pub fn hirc_en(&self) -> HIRC_EN_R {
        HIRC_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 32kHz Crystal Oscillator Ready"]
    #[inline(always)]
    pub fn x32k_rdy(&self) -> X32K_RDY_R {
        X32K_RDY_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 60MHz HIRC Ready."]
    #[inline(always)]
    pub fn hirc_rdy(&self) -> HIRC_RDY_R {
        HIRC_RDY_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 8kHz Low Frequency Reference Clock Ready."]
    #[inline(always)]
    pub fn lirc8k_rdy(&self) -> LIRC8K_RDY_R {
        LIRC8K_RDY_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 6:8 - Prescaler Select. This 3 bit field sets the system operating frequency by controlling the prescaler that divides the output of the PLL0."]
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W { w: self }
    }
    #[doc = "Bits 9:11 - Clock Source Select. This 3 bit field selects the source for the system clock."]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W { w: self }
    }
    #[doc = "Bit 17 - 32kHz Crystal Oscillator Enable."]
    #[inline(always)]
    pub fn x32k_en(&mut self) -> X32K_EN_W {
        X32K_EN_W { w: self }
    }
    #[doc = "Bit 18 - 60MHz High Frequency Internal Reference Clock Enable."]
    #[inline(always)]
    pub fn hirc_en(&mut self) -> HIRC_EN_W {
        HIRC_EN_W { w: self }
    }
    #[doc = "Bit 26 - 60MHz HIRC Ready."]
    #[inline(always)]
    pub fn hirc_rdy(&mut self) -> HIRC_RDY_W {
        HIRC_RDY_W { w: self }
    }
    #[doc = "Bit 29 - 8kHz Low Frequency Reference Clock Ready."]
    #[inline(always)]
    pub fn lirc8k_rdy(&mut self) -> LIRC8K_RDY_W {
        LIRC8K_RDY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkcn](index.html) module"]
pub struct CLKCN_SPEC;
impl crate::RegisterSpec for CLKCN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkcn::R](R) reader structure"]
impl crate::Readable for CLKCN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkcn::W](W) writer structure"]
impl crate::Writable for CLKCN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKCN to value 0x08"]
impl crate::Resettable for CLKCN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
