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
#[doc = "Real Time Clock Enable. This bit enables the Real Time Clock. This bit can only be written when WE=1 and BUSY =0. Change to this bit is effective only after BUSY is cleared from 1 to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCE_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<RTCE_A> for bool {
    #[inline(always)]
    fn from(variant: RTCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCE` reader - Real Time Clock Enable. This bit enables the Real Time Clock. This bit can only be written when WE=1 and BUSY =0. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
pub struct RTCE_R(crate::FieldReader<bool, RTCE_A>);
impl RTCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTCE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCE_A {
        match self.bits {
            false => RTCE_A::DIS,
            true => RTCE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RTCE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == RTCE_A::EN
    }
}
impl core::ops::Deref for RTCE_R {
    type Target = crate::FieldReader<bool, RTCE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCE` writer - Real Time Clock Enable. This bit enables the Real Time Clock. This bit can only be written when WE=1 and BUSY =0. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
pub struct RTCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RTCE_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RTCE_A::EN)
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
#[doc = "Alarm Time-of-Day Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADE_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<ADE_A> for bool {
    #[inline(always)]
    fn from(variant: ADE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADE` reader - Alarm Time-of-Day Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
pub struct ADE_R(crate::FieldReader<bool, ADE_A>);
impl ADE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADE_A {
        match self.bits {
            false => ADE_A::DIS,
            true => ADE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == ADE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == ADE_A::EN
    }
}
impl core::ops::Deref for ADE_R {
    type Target = crate::FieldReader<bool, ADE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADE` writer - Alarm Time-of-Day Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
pub struct ADE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ADE_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ADE_A::EN)
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
#[doc = "Alarm Sub-second Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASE_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<ASE_A> for bool {
    #[inline(always)]
    fn from(variant: ASE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASE` reader - Alarm Sub-second Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
pub struct ASE_R(crate::FieldReader<bool, ASE_A>);
impl ASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ASE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASE_A {
        match self.bits {
            false => ASE_A::DIS,
            true => ASE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == ASE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == ASE_A::EN
    }
}
impl core::ops::Deref for ASE_R {
    type Target = crate::FieldReader<bool, ASE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASE` writer - Alarm Sub-second Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
pub struct ASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ASE_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ASE_A::EN)
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
#[doc = "RTC Busy. This bit is set to 1 by hardware when changes to RTC registers required a synchronized version of the register to be in place. This bit is automatically cleared by hardware.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: Idle."]
    IDLE = 0,
    #[doc = "1: Busy."]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - RTC Busy. This bit is set to 1 by hardware when changes to RTC registers required a synchronized version of the register to be in place. This bit is automatically cleared by hardware."]
pub struct BUSY_R(crate::FieldReader<bool, BUSY_A>);
impl BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::IDLE,
            true => BUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == BUSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == BUSY_A::BUSY
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, BUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RTC Ready. This bit is set to 1 by hardware when the RTC count registers update. It can be cleared to 0 by software at any time. It will also be cleared to 0 by hardware just prior to an update of the RTC count register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDY_A {
    #[doc = "0: Register has not updated."]
    BUSY = 0,
    #[doc = "1: Ready."]
    READY = 1,
}
impl From<RDY_A> for bool {
    #[inline(always)]
    fn from(variant: RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDY` reader - RTC Ready. This bit is set to 1 by hardware when the RTC count registers update. It can be cleared to 0 by software at any time. It will also be cleared to 0 by hardware just prior to an update of the RTC count register."]
pub struct RDY_R(crate::FieldReader<bool, RDY_A>);
impl RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDY_A {
        match self.bits {
            false => RDY_A::BUSY,
            true => RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == RDY_A::BUSY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == RDY_A::READY
    }
}
impl core::ops::Deref for RDY_R {
    type Target = crate::FieldReader<bool, RDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDY` writer - RTC Ready. This bit is set to 1 by hardware when the RTC count registers update. It can be cleared to 0 by software at any time. It will also be cleared to 0 by hardware just prior to an update of the RTC count register."]
pub struct RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register has not updated."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(RDY_A::BUSY)
    }
    #[doc = "Ready."]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(RDY_A::READY)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "RTC Ready Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDYE_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<RDYE_A> for bool {
    #[inline(always)]
    fn from(variant: RDYE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDYE` reader - RTC Ready Interrupt Enable."]
pub struct RDYE_R(crate::FieldReader<bool, RDYE_A>);
impl RDYE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDYE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDYE_A {
        match self.bits {
            false => RDYE_A::DIS,
            true => RDYE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RDYE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == RDYE_A::EN
    }
}
impl core::ops::Deref for RDYE_R {
    type Target = crate::FieldReader<bool, RDYE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDYE` writer - RTC Ready Interrupt Enable."]
pub struct RDYE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDYE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDYE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RDYE_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RDYE_A::EN)
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
#[doc = "Time-of-Day Alarm Interrupt Flag. This alarm is qualified as wake-up source to the processor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALDF_A {
    #[doc = "0: Not active"]
    INACTIVE = 0,
    #[doc = "1: Active"]
    PENDING = 1,
}
impl From<ALDF_A> for bool {
    #[inline(always)]
    fn from(variant: ALDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALDF` reader - Time-of-Day Alarm Interrupt Flag. This alarm is qualified as wake-up source to the processor."]
pub struct ALDF_R(crate::FieldReader<bool, ALDF_A>);
impl ALDF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALDF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALDF_A {
        match self.bits {
            false => ALDF_A::INACTIVE,
            true => ALDF_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == ALDF_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == ALDF_A::PENDING
    }
}
impl core::ops::Deref for ALDF_R {
    type Target = crate::FieldReader<bool, ALDF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sub-second Alarm Interrupt Flag. This alarm is qualified as wake-up source to the processor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALSF_A {
    #[doc = "0: Not active"]
    INACTIVE = 0,
    #[doc = "1: Active"]
    PENDING = 1,
}
impl From<ALSF_A> for bool {
    #[inline(always)]
    fn from(variant: ALSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALSF` reader - Sub-second Alarm Interrupt Flag. This alarm is qualified as wake-up source to the processor."]
pub struct ALSF_R(crate::FieldReader<bool, ALSF_A>);
impl ALSF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALSF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALSF_A {
        match self.bits {
            false => ALSF_A::INACTIVE,
            true => ALSF_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == ALSF_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == ALSF_A::PENDING
    }
}
impl core::ops::Deref for ALSF_R {
    type Target = crate::FieldReader<bool, ALSF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Square Wave Output Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SQE_A {
    #[doc = "0: Not active"]
    INACTIVE = 0,
    #[doc = "1: Active"]
    PENDING = 1,
}
impl From<SQE_A> for bool {
    #[inline(always)]
    fn from(variant: SQE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SQE` reader - Square Wave Output Enable."]
pub struct SQE_R(crate::FieldReader<bool, SQE_A>);
impl SQE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SQE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SQE_A {
        match self.bits {
            false => SQE_A::INACTIVE,
            true => SQE_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == SQE_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == SQE_A::PENDING
    }
}
impl core::ops::Deref for SQE_R {
    type Target = crate::FieldReader<bool, SQE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQE` writer - Square Wave Output Enable."]
pub struct SQE_W<'a> {
    w: &'a mut W,
}
impl<'a> SQE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SQE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not active"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(SQE_A::INACTIVE)
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(SQE_A::PENDING)
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
#[doc = "Frequency Output Selection. When SQE=1, these bits specify the output frequency on the SQW pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FT_A {
    #[doc = "0: 1 Hz (Compensated)."]
    FREQ1HZ = 0,
    #[doc = "1: 512 Hz (Compensated)."]
    FREQ512HZ = 1,
    #[doc = "2: 4 KHz."]
    FREQ4KHZ = 2,
    #[doc = "3: RTC Input Clock / 8."]
    CLKDIV8 = 3,
}
impl From<FT_A> for u8 {
    #[inline(always)]
    fn from(variant: FT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FT` reader - Frequency Output Selection. When SQE=1, these bits specify the output frequency on the SQW pin."]
pub struct FT_R(crate::FieldReader<u8, FT_A>);
impl FT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FT_A {
        match self.bits {
            0 => FT_A::FREQ1HZ,
            1 => FT_A::FREQ512HZ,
            2 => FT_A::FREQ4KHZ,
            3 => FT_A::CLKDIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FREQ1HZ`"]
    #[inline(always)]
    pub fn is_freq1hz(&self) -> bool {
        **self == FT_A::FREQ1HZ
    }
    #[doc = "Checks if the value of the field is `FREQ512HZ`"]
    #[inline(always)]
    pub fn is_freq512hz(&self) -> bool {
        **self == FT_A::FREQ512HZ
    }
    #[doc = "Checks if the value of the field is `FREQ4KHZ`"]
    #[inline(always)]
    pub fn is_freq4khz(&self) -> bool {
        **self == FT_A::FREQ4KHZ
    }
    #[doc = "Checks if the value of the field is `CLKDIV8`"]
    #[inline(always)]
    pub fn is_clk_div8(&self) -> bool {
        **self == FT_A::CLKDIV8
    }
}
impl core::ops::Deref for FT_R {
    type Target = crate::FieldReader<u8, FT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT` writer - Frequency Output Selection. When SQE=1, these bits specify the output frequency on the SQW pin."]
pub struct FT_W<'a> {
    w: &'a mut W,
}
impl<'a> FT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1 Hz (Compensated)."]
    #[inline(always)]
    pub fn freq1hz(self) -> &'a mut W {
        self.variant(FT_A::FREQ1HZ)
    }
    #[doc = "512 Hz (Compensated)."]
    #[inline(always)]
    pub fn freq512hz(self) -> &'a mut W {
        self.variant(FT_A::FREQ512HZ)
    }
    #[doc = "4 KHz."]
    #[inline(always)]
    pub fn freq4khz(self) -> &'a mut W {
        self.variant(FT_A::FREQ4KHZ)
    }
    #[doc = "RTC Input Clock / 8."]
    #[inline(always)]
    pub fn clk_div8(self) -> &'a mut W {
        self.variant(FT_A::CLKDIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "32KHz Oscillator Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum X32KMD_A {
    #[doc = "0: Always operate in Noise Immune Mode. Oscillator warm-up required."]
    NOISEIMMUNEMODE = 0,
    #[doc = "1: Always operate in Quiet Mode. No oscillator warm-up required."]
    QUIETMODE = 1,
    #[doc = "2: Operate in Noise Immune Mode normally, switch to Quiet Mode on Stop Mode entry. Will wait for 32K oscillator warm-up before code execution on Stop Mode exit."]
    QUIETINSTOPWITHWARMUP = 2,
    #[doc = "3: Operate in Noise Immune Mode normally, switch to Quiet Mode on Stop Mode entry. Will not wait for 32K oscillator warm-up before code execution on Stop Mode exit."]
    QUIETINSTOPNOWARMUP = 3,
}
impl From<X32KMD_A> for u8 {
    #[inline(always)]
    fn from(variant: X32KMD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `X32KMD` reader - 32KHz Oscillator Mode."]
pub struct X32KMD_R(crate::FieldReader<u8, X32KMD_A>);
impl X32KMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        X32KMD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> X32KMD_A {
        match self.bits {
            0 => X32KMD_A::NOISEIMMUNEMODE,
            1 => X32KMD_A::QUIETMODE,
            2 => X32KMD_A::QUIETINSTOPWITHWARMUP,
            3 => X32KMD_A::QUIETINSTOPNOWARMUP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOISEIMMUNEMODE`"]
    #[inline(always)]
    pub fn is_noise_immune_mode(&self) -> bool {
        **self == X32KMD_A::NOISEIMMUNEMODE
    }
    #[doc = "Checks if the value of the field is `QUIETMODE`"]
    #[inline(always)]
    pub fn is_quiet_mode(&self) -> bool {
        **self == X32KMD_A::QUIETMODE
    }
    #[doc = "Checks if the value of the field is `QUIETINSTOPWITHWARMUP`"]
    #[inline(always)]
    pub fn is_quiet_in_stop_with_warmup(&self) -> bool {
        **self == X32KMD_A::QUIETINSTOPWITHWARMUP
    }
    #[doc = "Checks if the value of the field is `QUIETINSTOPNOWARMUP`"]
    #[inline(always)]
    pub fn is_quiet_in_stop_no_warmup(&self) -> bool {
        **self == X32KMD_A::QUIETINSTOPNOWARMUP
    }
}
impl core::ops::Deref for X32KMD_R {
    type Target = crate::FieldReader<u8, X32KMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32KMD` writer - 32KHz Oscillator Mode."]
pub struct X32KMD_W<'a> {
    w: &'a mut W,
}
impl<'a> X32KMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: X32KMD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Always operate in Noise Immune Mode. Oscillator warm-up required."]
    #[inline(always)]
    pub fn noise_immune_mode(self) -> &'a mut W {
        self.variant(X32KMD_A::NOISEIMMUNEMODE)
    }
    #[doc = "Always operate in Quiet Mode. No oscillator warm-up required."]
    #[inline(always)]
    pub fn quiet_mode(self) -> &'a mut W {
        self.variant(X32KMD_A::QUIETMODE)
    }
    #[doc = "Operate in Noise Immune Mode normally, switch to Quiet Mode on Stop Mode entry. Will wait for 32K oscillator warm-up before code execution on Stop Mode exit."]
    #[inline(always)]
    pub fn quiet_in_stop_with_warmup(self) -> &'a mut W {
        self.variant(X32KMD_A::QUIETINSTOPWITHWARMUP)
    }
    #[doc = "Operate in Noise Immune Mode normally, switch to Quiet Mode on Stop Mode entry. Will not wait for 32K oscillator warm-up before code execution on Stop Mode exit."]
    #[inline(always)]
    pub fn quiet_in_stop_no_warmup(self) -> &'a mut W {
        self.variant(X32KMD_A::QUIETINSTOPNOWARMUP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Write Enable. This register bit serves as a protection mechanism against unintentional writes to critical RTC bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WE_A {
    #[doc = "0: Not active"]
    INACTIVE = 0,
    #[doc = "1: Active"]
    PENDING = 1,
}
impl From<WE_A> for bool {
    #[inline(always)]
    fn from(variant: WE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE` reader - Write Enable. This register bit serves as a protection mechanism against unintentional writes to critical RTC bits."]
pub struct WE_R(crate::FieldReader<bool, WE_A>);
impl WE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WE_A {
        match self.bits {
            false => WE_A::INACTIVE,
            true => WE_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == WE_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == WE_A::PENDING
    }
}
impl core::ops::Deref for WE_R {
    type Target = crate::FieldReader<bool, WE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WE` writer - Write Enable. This register bit serves as a protection mechanism against unintentional writes to critical RTC bits."]
pub struct WE_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not active"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(WE_A::INACTIVE)
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(WE_A::PENDING)
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
impl R {
    #[doc = "Bit 0 - Real Time Clock Enable. This bit enables the Real Time Clock. This bit can only be written when WE=1 and BUSY =0. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
    #[inline(always)]
    pub fn rtce(&self) -> RTCE_R {
        RTCE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Alarm Time-of-Day Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
    #[inline(always)]
    pub fn ade(&self) -> ADE_R {
        ADE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Alarm Sub-second Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
    #[inline(always)]
    pub fn ase(&self) -> ASE_R {
        ASE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC Busy. This bit is set to 1 by hardware when changes to RTC registers required a synchronized version of the register to be in place. This bit is automatically cleared by hardware."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RTC Ready. This bit is set to 1 by hardware when the RTC count registers update. It can be cleared to 0 by software at any time. It will also be cleared to 0 by hardware just prior to an update of the RTC count register."]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RTC Ready Interrupt Enable."]
    #[inline(always)]
    pub fn rdye(&self) -> RDYE_R {
        RDYE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Time-of-Day Alarm Interrupt Flag. This alarm is qualified as wake-up source to the processor."]
    #[inline(always)]
    pub fn aldf(&self) -> ALDF_R {
        ALDF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Sub-second Alarm Interrupt Flag. This alarm is qualified as wake-up source to the processor."]
    #[inline(always)]
    pub fn alsf(&self) -> ALSF_R {
        ALSF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Square Wave Output Enable."]
    #[inline(always)]
    pub fn sqe(&self) -> SQE_R {
        SQE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - Frequency Output Selection. When SQE=1, these bits specify the output frequency on the SQW pin."]
    #[inline(always)]
    pub fn ft(&self) -> FT_R {
        FT_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 11:12 - 32KHz Oscillator Mode."]
    #[inline(always)]
    pub fn x32kmd(&self) -> X32KMD_R {
        X32KMD_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Write Enable. This register bit serves as a protection mechanism against unintentional writes to critical RTC bits."]
    #[inline(always)]
    pub fn we(&self) -> WE_R {
        WE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock Enable. This bit enables the Real Time Clock. This bit can only be written when WE=1 and BUSY =0. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
    #[inline(always)]
    pub fn rtce(&mut self) -> RTCE_W {
        RTCE_W { w: self }
    }
    #[doc = "Bit 1 - Alarm Time-of-Day Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
    #[inline(always)]
    pub fn ade(&mut self) -> ADE_W {
        ADE_W { w: self }
    }
    #[doc = "Bit 2 - Alarm Sub-second Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
    #[inline(always)]
    pub fn ase(&mut self) -> ASE_W {
        ASE_W { w: self }
    }
    #[doc = "Bit 4 - RTC Ready. This bit is set to 1 by hardware when the RTC count registers update. It can be cleared to 0 by software at any time. It will also be cleared to 0 by hardware just prior to an update of the RTC count register."]
    #[inline(always)]
    pub fn rdy(&mut self) -> RDY_W {
        RDY_W { w: self }
    }
    #[doc = "Bit 5 - RTC Ready Interrupt Enable."]
    #[inline(always)]
    pub fn rdye(&mut self) -> RDYE_W {
        RDYE_W { w: self }
    }
    #[doc = "Bit 8 - Square Wave Output Enable."]
    #[inline(always)]
    pub fn sqe(&mut self) -> SQE_W {
        SQE_W { w: self }
    }
    #[doc = "Bits 9:10 - Frequency Output Selection. When SQE=1, these bits specify the output frequency on the SQW pin."]
    #[inline(always)]
    pub fn ft(&mut self) -> FT_W {
        FT_W { w: self }
    }
    #[doc = "Bits 11:12 - 32KHz Oscillator Mode."]
    #[inline(always)]
    pub fn x32kmd(&mut self) -> X32KMD_W {
        X32KMD_W { w: self }
    }
    #[doc = "Bit 15 - Write Enable. This register bit serves as a protection mechanism against unintentional writes to critical RTC bits."]
    #[inline(always)]
    pub fn we(&mut self) -> WE_W {
        WE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
}
#[doc = "`reset()` method sets CTRL to value 0x08"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
