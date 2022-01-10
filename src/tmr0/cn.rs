#[doc = "Register `CN` reader"]
pub struct R(crate::R<CN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CN` writer"]
pub struct W(crate::W<CN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CN_SPEC>;
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
impl From<crate::W<CN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timer Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMODE_A {
    #[doc = "0: One Shot Mode."]
    ONESHOT = 0,
    #[doc = "1: Continuous Mode."]
    CONTINUOUS = 1,
    #[doc = "2: Counter Mode."]
    COUNTER = 2,
    #[doc = "3: PWM Mode."]
    PWM = 3,
    #[doc = "4: Capture Mode."]
    CAPTURE = 4,
    #[doc = "5: Compare Mode."]
    COMPARE = 5,
    #[doc = "6: Gated Mode."]
    GATED = 6,
    #[doc = "7: Capture/Compare Mode."]
    CAPTURECOMPARE = 7,
}
impl From<TMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMODE` reader - Timer Mode."]
pub struct TMODE_R(crate::FieldReader<u8, TMODE_A>);
impl TMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMODE_A {
        match self.bits {
            0 => TMODE_A::ONESHOT,
            1 => TMODE_A::CONTINUOUS,
            2 => TMODE_A::COUNTER,
            3 => TMODE_A::PWM,
            4 => TMODE_A::CAPTURE,
            5 => TMODE_A::COMPARE,
            6 => TMODE_A::GATED,
            7 => TMODE_A::CAPTURECOMPARE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONESHOT`"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        **self == TMODE_A::ONESHOT
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        **self == TMODE_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `COUNTER`"]
    #[inline(always)]
    pub fn is_counter(&self) -> bool {
        **self == TMODE_A::COUNTER
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        **self == TMODE_A::PWM
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        **self == TMODE_A::CAPTURE
    }
    #[doc = "Checks if the value of the field is `COMPARE`"]
    #[inline(always)]
    pub fn is_compare(&self) -> bool {
        **self == TMODE_A::COMPARE
    }
    #[doc = "Checks if the value of the field is `GATED`"]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        **self == TMODE_A::GATED
    }
    #[doc = "Checks if the value of the field is `CAPTURECOMPARE`"]
    #[inline(always)]
    pub fn is_capture_compare(&self) -> bool {
        **self == TMODE_A::CAPTURECOMPARE
    }
}
impl core::ops::Deref for TMODE_R {
    type Target = crate::FieldReader<u8, TMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMODE` writer - Timer Mode."]
pub struct TMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "One Shot Mode."]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut W {
        self.variant(TMODE_A::ONESHOT)
    }
    #[doc = "Continuous Mode."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMODE_A::CONTINUOUS)
    }
    #[doc = "Counter Mode."]
    #[inline(always)]
    pub fn counter(self) -> &'a mut W {
        self.variant(TMODE_A::COUNTER)
    }
    #[doc = "PWM Mode."]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(TMODE_A::PWM)
    }
    #[doc = "Capture Mode."]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(TMODE_A::CAPTURE)
    }
    #[doc = "Compare Mode."]
    #[inline(always)]
    pub fn compare(self) -> &'a mut W {
        self.variant(TMODE_A::COMPARE)
    }
    #[doc = "Gated Mode."]
    #[inline(always)]
    pub fn gated(self) -> &'a mut W {
        self.variant(TMODE_A::GATED)
    }
    #[doc = "Capture/Compare Mode."]
    #[inline(always)]
    pub fn capture_compare(self) -> &'a mut W {
        self.variant(TMODE_A::CAPTURECOMPARE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Prescaler. Set the Timer's prescaler value. The prescaler divides the PCLK input to the timer and sets the Timer's Count Clock, F_CNT_CLK = PCLK(HZ)/prescaler. The Timer's prescaler setting is a 4-bit value with pres3:pres\\[2:0\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRES_A {
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
impl From<PRES_A> for u8 {
    #[inline(always)]
    fn from(variant: PRES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRES` reader - Prescaler. Set the Timer's prescaler value. The prescaler divides the PCLK input to the timer and sets the Timer's Count Clock, F_CNT_CLK = PCLK(HZ)/prescaler. The Timer's prescaler setting is a 4-bit value with pres3:pres\\[2:0\\]."]
pub struct PRES_R(crate::FieldReader<u8, PRES_A>);
impl PRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRES_A {
        match self.bits {
            0 => PRES_A::DIV1,
            1 => PRES_A::DIV2,
            2 => PRES_A::DIV4,
            3 => PRES_A::DIV8,
            4 => PRES_A::DIV16,
            5 => PRES_A::DIV32,
            6 => PRES_A::DIV64,
            7 => PRES_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == PRES_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PRES_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == PRES_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == PRES_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == PRES_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        **self == PRES_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        **self == PRES_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        **self == PRES_A::DIV128
    }
}
impl core::ops::Deref for PRES_R {
    type Target = crate::FieldReader<u8, PRES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRES` writer - Prescaler. Set the Timer's prescaler value. The prescaler divides the PCLK input to the timer and sets the Timer's Count Clock, F_CNT_CLK = PCLK(HZ)/prescaler. The Timer's prescaler setting is a 4-bit value with pres3:pres\\[2:0\\]."]
pub struct PRES_W<'a> {
    w: &'a mut W,
}
impl<'a> PRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRES_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRES_A::DIV1)
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRES_A::DIV2)
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRES_A::DIV4)
    }
    #[doc = "Divide by 8."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRES_A::DIV8)
    }
    #[doc = "Divide by 16."]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRES_A::DIV16)
    }
    #[doc = "Divide by 32."]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRES_A::DIV32)
    }
    #[doc = "Divide by 64."]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRES_A::DIV64)
    }
    #[doc = "Divide by 128."]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRES_A::DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Timer input/output polarity bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPOL_A {
    #[doc = "0: Active High."]
    ACTIVEHI = 0,
    #[doc = "1: Active Low."]
    ACTIVELO = 1,
}
impl From<TPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPOL` reader - Timer input/output polarity bit."]
pub struct TPOL_R(crate::FieldReader<bool, TPOL_A>);
impl TPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPOL_A {
        match self.bits {
            false => TPOL_A::ACTIVEHI,
            true => TPOL_A::ACTIVELO,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVEHI`"]
    #[inline(always)]
    pub fn is_active_hi(&self) -> bool {
        **self == TPOL_A::ACTIVEHI
    }
    #[doc = "Checks if the value of the field is `ACTIVELO`"]
    #[inline(always)]
    pub fn is_active_lo(&self) -> bool {
        **self == TPOL_A::ACTIVELO
    }
}
impl core::ops::Deref for TPOL_R {
    type Target = crate::FieldReader<bool, TPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPOL` writer - Timer input/output polarity bit."]
pub struct TPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Active High."]
    #[inline(always)]
    pub fn active_hi(self) -> &'a mut W {
        self.variant(TPOL_A::ACTIVEHI)
    }
    #[doc = "Active Low."]
    #[inline(always)]
    pub fn active_lo(self) -> &'a mut W {
        self.variant(TPOL_A::ACTIVELO)
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
#[doc = "Timer Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<TEN_A> for bool {
    #[inline(always)]
    fn from(variant: TEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEN` reader - Timer Enable."]
pub struct TEN_R(crate::FieldReader<bool, TEN_A>);
impl TEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEN_A {
        match self.bits {
            false => TEN_A::DIS,
            true => TEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TEN_A::EN
    }
}
impl core::ops::Deref for TEN_R {
    type Target = crate::FieldReader<bool, TEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEN` writer - Timer Enable."]
pub struct TEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TEN_A::EN)
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
#[doc = "Field `PRES3` reader - MSB of prescaler value."]
pub struct PRES3_R(crate::FieldReader<bool, bool>);
impl PRES3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRES3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRES3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRES3` writer - MSB of prescaler value."]
pub struct PRES3_W<'a> {
    w: &'a mut W,
}
impl<'a> PRES3_W<'a> {
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
#[doc = "Timer PWM Synchronization Mode Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMSYNC_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<PWMSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: PWMSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMSYNC` reader - Timer PWM Synchronization Mode Enable."]
pub struct PWMSYNC_R(crate::FieldReader<bool, PWMSYNC_A>);
impl PWMSYNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWMSYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMSYNC_A {
        match self.bits {
            false => PWMSYNC_A::DIS,
            true => PWMSYNC_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PWMSYNC_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PWMSYNC_A::EN
    }
}
impl core::ops::Deref for PWMSYNC_R {
    type Target = crate::FieldReader<bool, PWMSYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMSYNC` writer - Timer PWM Synchronization Mode Enable."]
pub struct PWMSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMSYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWMSYNC_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWMSYNC_A::EN)
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
#[doc = "Timer PWM output 0A polarity bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOLHPOL_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<NOLHPOL_A> for bool {
    #[inline(always)]
    fn from(variant: NOLHPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOLHPOL` reader - Timer PWM output 0A polarity bit."]
pub struct NOLHPOL_R(crate::FieldReader<bool, NOLHPOL_A>);
impl NOLHPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NOLHPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOLHPOL_A {
        match self.bits {
            false => NOLHPOL_A::DIS,
            true => NOLHPOL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == NOLHPOL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == NOLHPOL_A::EN
    }
}
impl core::ops::Deref for NOLHPOL_R {
    type Target = crate::FieldReader<bool, NOLHPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOLHPOL` writer - Timer PWM output 0A polarity bit."]
pub struct NOLHPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> NOLHPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOLHPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(NOLHPOL_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(NOLHPOL_A::EN)
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
#[doc = "Timer PWM output 0A' polarity bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOLLPOL_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<NOLLPOL_A> for bool {
    #[inline(always)]
    fn from(variant: NOLLPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOLLPOL` reader - Timer PWM output 0A' polarity bit."]
pub struct NOLLPOL_R(crate::FieldReader<bool, NOLLPOL_A>);
impl NOLLPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NOLLPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOLLPOL_A {
        match self.bits {
            false => NOLLPOL_A::DIS,
            true => NOLLPOL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == NOLLPOL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == NOLLPOL_A::EN
    }
}
impl core::ops::Deref for NOLLPOL_R {
    type Target = crate::FieldReader<bool, NOLLPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOLLPOL` writer - Timer PWM output 0A' polarity bit."]
pub struct NOLLPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> NOLLPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOLLPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(NOLLPOL_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(NOLLPOL_A::EN)
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
#[doc = "Timer PWM output 0A Mode Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMCKBD_A {
    #[doc = "1: Disable."]
    DIS = 1,
    #[doc = "0: Enable."]
    EN = 0,
}
impl From<PWMCKBD_A> for bool {
    #[inline(always)]
    fn from(variant: PWMCKBD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMCKBD` reader - Timer PWM output 0A Mode Disable."]
pub struct PWMCKBD_R(crate::FieldReader<bool, PWMCKBD_A>);
impl PWMCKBD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWMCKBD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMCKBD_A {
        match self.bits {
            true => PWMCKBD_A::DIS,
            false => PWMCKBD_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PWMCKBD_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PWMCKBD_A::EN
    }
}
impl core::ops::Deref for PWMCKBD_R {
    type Target = crate::FieldReader<bool, PWMCKBD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMCKBD` writer - Timer PWM output 0A Mode Disable."]
pub struct PWMCKBD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCKBD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMCKBD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWMCKBD_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWMCKBD_A::EN)
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
impl R {
    #[doc = "Bits 0:2 - Timer Mode."]
    #[inline(always)]
    pub fn tmode(&self) -> TMODE_R {
        TMODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Prescaler. Set the Timer's prescaler value. The prescaler divides the PCLK input to the timer and sets the Timer's Count Clock, F_CNT_CLK = PCLK(HZ)/prescaler. The Timer's prescaler setting is a 4-bit value with pres3:pres\\[2:0\\]."]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 6 - Timer input/output polarity bit."]
    #[inline(always)]
    pub fn tpol(&self) -> TPOL_R {
        TPOL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Timer Enable."]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MSB of prescaler value."]
    #[inline(always)]
    pub fn pres3(&self) -> PRES3_R {
        PRES3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Timer PWM Synchronization Mode Enable."]
    #[inline(always)]
    pub fn pwmsync(&self) -> PWMSYNC_R {
        PWMSYNC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Timer PWM output 0A polarity bit."]
    #[inline(always)]
    pub fn nolhpol(&self) -> NOLHPOL_R {
        NOLHPOL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Timer PWM output 0A' polarity bit."]
    #[inline(always)]
    pub fn nollpol(&self) -> NOLLPOL_R {
        NOLLPOL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Timer PWM output 0A Mode Disable."]
    #[inline(always)]
    pub fn pwmckbd(&self) -> PWMCKBD_R {
        PWMCKBD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Timer Mode."]
    #[inline(always)]
    pub fn tmode(&mut self) -> TMODE_W {
        TMODE_W { w: self }
    }
    #[doc = "Bits 3:5 - Prescaler. Set the Timer's prescaler value. The prescaler divides the PCLK input to the timer and sets the Timer's Count Clock, F_CNT_CLK = PCLK(HZ)/prescaler. The Timer's prescaler setting is a 4-bit value with pres3:pres\\[2:0\\]."]
    #[inline(always)]
    pub fn pres(&mut self) -> PRES_W {
        PRES_W { w: self }
    }
    #[doc = "Bit 6 - Timer input/output polarity bit."]
    #[inline(always)]
    pub fn tpol(&mut self) -> TPOL_W {
        TPOL_W { w: self }
    }
    #[doc = "Bit 7 - Timer Enable."]
    #[inline(always)]
    pub fn ten(&mut self) -> TEN_W {
        TEN_W { w: self }
    }
    #[doc = "Bit 8 - MSB of prescaler value."]
    #[inline(always)]
    pub fn pres3(&mut self) -> PRES3_W {
        PRES3_W { w: self }
    }
    #[doc = "Bit 9 - Timer PWM Synchronization Mode Enable."]
    #[inline(always)]
    pub fn pwmsync(&mut self) -> PWMSYNC_W {
        PWMSYNC_W { w: self }
    }
    #[doc = "Bit 10 - Timer PWM output 0A polarity bit."]
    #[inline(always)]
    pub fn nolhpol(&mut self) -> NOLHPOL_W {
        NOLHPOL_W { w: self }
    }
    #[doc = "Bit 11 - Timer PWM output 0A' polarity bit."]
    #[inline(always)]
    pub fn nollpol(&mut self) -> NOLLPOL_W {
        NOLLPOL_W { w: self }
    }
    #[doc = "Bit 12 - Timer PWM output 0A Mode Disable."]
    #[inline(always)]
    pub fn pwmckbd(&mut self) -> PWMCKBD_W {
        PWMCKBD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cn](index.html) module"]
pub struct CN_SPEC;
impl crate::RegisterSpec for CN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cn::R](R) reader structure"]
impl crate::Readable for CN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cn::W](W) writer structure"]
impl crate::Writable for CN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CN to value 0"]
impl crate::Resettable for CN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
