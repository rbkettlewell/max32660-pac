#[doc = "Register `LP_CTRL` reader"]
pub struct R(crate::R<LP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LP_CTRL` writer"]
pub struct W(crate::W<LP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LP_CTRL_SPEC>;
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
impl From<crate::W<LP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "System RAM 0 Data retention in BACKUP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMRET_SEL0_A {
    #[doc = "0: Disabled."]
    DIS = 0,
    #[doc = "1: Enabled."]
    EN = 1,
}
impl From<RAMRET_SEL0_A> for bool {
    #[inline(always)]
    fn from(variant: RAMRET_SEL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMRET_SEL0` reader - System RAM 0 Data retention in BACKUP mode."]
pub struct RAMRET_SEL0_R(crate::FieldReader<bool, RAMRET_SEL0_A>);
impl RAMRET_SEL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAMRET_SEL0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMRET_SEL0_A {
        match self.bits {
            false => RAMRET_SEL0_A::DIS,
            true => RAMRET_SEL0_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RAMRET_SEL0_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == RAMRET_SEL0_A::EN
    }
}
impl core::ops::Deref for RAMRET_SEL0_R {
    type Target = crate::FieldReader<bool, RAMRET_SEL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMRET_SEL0` writer - System RAM 0 Data retention in BACKUP mode."]
pub struct RAMRET_SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMRET_SEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMRET_SEL0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RAMRET_SEL0_A::DIS)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RAMRET_SEL0_A::EN)
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
#[doc = "System RAM 1 Data retention in BACKUP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMRET_SEL1_A {
    #[doc = "0: Disabled."]
    DIS = 0,
    #[doc = "1: Enabled."]
    EN = 1,
}
impl From<RAMRET_SEL1_A> for bool {
    #[inline(always)]
    fn from(variant: RAMRET_SEL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMRET_SEL1` reader - System RAM 1 Data retention in BACKUP mode."]
pub struct RAMRET_SEL1_R(crate::FieldReader<bool, RAMRET_SEL1_A>);
impl RAMRET_SEL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAMRET_SEL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMRET_SEL1_A {
        match self.bits {
            false => RAMRET_SEL1_A::DIS,
            true => RAMRET_SEL1_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RAMRET_SEL1_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == RAMRET_SEL1_A::EN
    }
}
impl core::ops::Deref for RAMRET_SEL1_R {
    type Target = crate::FieldReader<bool, RAMRET_SEL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMRET_SEL1` writer - System RAM 1 Data retention in BACKUP mode."]
pub struct RAMRET_SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMRET_SEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMRET_SEL1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RAMRET_SEL1_A::DIS)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RAMRET_SEL1_A::EN)
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
#[doc = "System RAM 2 Data retention in BACKUP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMRET_SEL2_A {
    #[doc = "0: Disabled."]
    DIS = 0,
    #[doc = "1: Enabled."]
    EN = 1,
}
impl From<RAMRET_SEL2_A> for bool {
    #[inline(always)]
    fn from(variant: RAMRET_SEL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMRET_SEL2` reader - System RAM 2 Data retention in BACKUP mode."]
pub struct RAMRET_SEL2_R(crate::FieldReader<bool, RAMRET_SEL2_A>);
impl RAMRET_SEL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAMRET_SEL2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMRET_SEL2_A {
        match self.bits {
            false => RAMRET_SEL2_A::DIS,
            true => RAMRET_SEL2_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RAMRET_SEL2_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == RAMRET_SEL2_A::EN
    }
}
impl core::ops::Deref for RAMRET_SEL2_R {
    type Target = crate::FieldReader<bool, RAMRET_SEL2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMRET_SEL2` writer - System RAM 2 Data retention in BACKUP mode."]
pub struct RAMRET_SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMRET_SEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMRET_SEL2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RAMRET_SEL2_A::DIS)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RAMRET_SEL2_A::EN)
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
#[doc = "System RAM 3 Data retention in BACKUP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMRET_SEL3_A {
    #[doc = "0: Disabled."]
    DIS = 0,
    #[doc = "1: Enabled."]
    EN = 1,
}
impl From<RAMRET_SEL3_A> for bool {
    #[inline(always)]
    fn from(variant: RAMRET_SEL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMRET_SEL3` reader - System RAM 3 Data retention in BACKUP mode."]
pub struct RAMRET_SEL3_R(crate::FieldReader<bool, RAMRET_SEL3_A>);
impl RAMRET_SEL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAMRET_SEL3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMRET_SEL3_A {
        match self.bits {
            false => RAMRET_SEL3_A::DIS,
            true => RAMRET_SEL3_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RAMRET_SEL3_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == RAMRET_SEL3_A::EN
    }
}
impl core::ops::Deref for RAMRET_SEL3_R {
    type Target = crate::FieldReader<bool, RAMRET_SEL3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMRET_SEL3` writer - System RAM 3 Data retention in BACKUP mode."]
pub struct RAMRET_SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMRET_SEL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMRET_SEL3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RAMRET_SEL3_A::DIS)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RAMRET_SEL3_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Operating Voltage Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OVR_A {
    #[doc = "0: 0.9V 24MHz"]
    _0_9V = 0,
    #[doc = "1: 1.0V 48MHz"]
    _1_0V = 1,
    #[doc = "2: 1.1V 96MHz"]
    _1_1V = 2,
}
impl From<OVR_A> for u8 {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OVR` reader - Operating Voltage Range"]
pub struct OVR_R(crate::FieldReader<u8, OVR_A>);
impl OVR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OVR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OVR_A> {
        match self.bits {
            0 => Some(OVR_A::_0_9V),
            1 => Some(OVR_A::_1_0V),
            2 => Some(OVR_A::_1_1V),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0_9V`"]
    #[inline(always)]
    pub fn is_0_9v(&self) -> bool {
        **self == OVR_A::_0_9V
    }
    #[doc = "Checks if the value of the field is `_1_0V`"]
    #[inline(always)]
    pub fn is_1_0v(&self) -> bool {
        **self == OVR_A::_1_0V
    }
    #[doc = "Checks if the value of the field is `_1_1V`"]
    #[inline(always)]
    pub fn is_1_1v(&self) -> bool {
        **self == OVR_A::_1_1V
    }
}
impl core::ops::Deref for OVR_R {
    type Target = crate::FieldReader<u8, OVR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR` writer - Operating Voltage Range"]
pub struct OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0.9V 24MHz"]
    #[inline(always)]
    pub fn _0_9v(self) -> &'a mut W {
        self.variant(OVR_A::_0_9V)
    }
    #[doc = "1.0V 48MHz"]
    #[inline(always)]
    pub fn _1_0v(self) -> &'a mut W {
        self.variant(OVR_A::_1_0V)
    }
    #[doc = "1.1V 96MHz"]
    #[inline(always)]
    pub fn _1_1v(self) -> &'a mut W {
        self.variant(OVR_A::_1_1V)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Bypass V CORE External Supply Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCORE_DET_BYPASS_A {
    #[doc = "0: enable"]
    ENABLED = 0,
    #[doc = "1: disable"]
    DISABLE = 1,
}
impl From<VCORE_DET_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: VCORE_DET_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCORE_DET_BYPASS` reader - Bypass V CORE External Supply Detection"]
pub struct VCORE_DET_BYPASS_R(crate::FieldReader<bool, VCORE_DET_BYPASS_A>);
impl VCORE_DET_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCORE_DET_BYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCORE_DET_BYPASS_A {
        match self.bits {
            false => VCORE_DET_BYPASS_A::ENABLED,
            true => VCORE_DET_BYPASS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == VCORE_DET_BYPASS_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == VCORE_DET_BYPASS_A::DISABLE
    }
}
impl core::ops::Deref for VCORE_DET_BYPASS_R {
    type Target = crate::FieldReader<bool, VCORE_DET_BYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCORE_DET_BYPASS` writer - Bypass V CORE External Supply Detection"]
pub struct VCORE_DET_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> VCORE_DET_BYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCORE_DET_BYPASS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VCORE_DET_BYPASS_A::ENABLED)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VCORE_DET_BYPASS_A::DISABLE)
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
#[doc = "Retention Regulator Enable. This bit controls the retention regulator in BACKUP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RETREG_EN_A {
    #[doc = "0: Disabled."]
    DIS = 0,
    #[doc = "1: Enabled."]
    EN = 1,
}
impl From<RETREG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RETREG_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RETREG_EN` reader - Retention Regulator Enable. This bit controls the retention regulator in BACKUP mode."]
pub struct RETREG_EN_R(crate::FieldReader<bool, RETREG_EN_A>);
impl RETREG_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RETREG_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RETREG_EN_A {
        match self.bits {
            false => RETREG_EN_A::DIS,
            true => RETREG_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RETREG_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == RETREG_EN_A::EN
    }
}
impl core::ops::Deref for RETREG_EN_R {
    type Target = crate::FieldReader<bool, RETREG_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETREG_EN` writer - Retention Regulator Enable. This bit controls the retention regulator in BACKUP mode."]
pub struct RETREG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RETREG_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RETREG_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RETREG_EN_A::DIS)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RETREG_EN_A::EN)
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
#[doc = "Fast Wake-Up Mode. This bit enables fast wake-up from DeepSleep mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAST_WK_EN_A {
    #[doc = "0: Disabled."]
    DIS = 0,
    #[doc = "1: Enabled."]
    EN = 1,
}
impl From<FAST_WK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FAST_WK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAST_WK_EN` reader - Fast Wake-Up Mode. This bit enables fast wake-up from DeepSleep mode."]
pub struct FAST_WK_EN_R(crate::FieldReader<bool, FAST_WK_EN_A>);
impl FAST_WK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAST_WK_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAST_WK_EN_A {
        match self.bits {
            false => FAST_WK_EN_A::DIS,
            true => FAST_WK_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == FAST_WK_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == FAST_WK_EN_A::EN
    }
}
impl core::ops::Deref for FAST_WK_EN_R {
    type Target = crate::FieldReader<bool, FAST_WK_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAST_WK_EN` writer - Fast Wake-Up Mode. This bit enables fast wake-up from DeepSleep mode."]
pub struct FAST_WK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FAST_WK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAST_WK_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FAST_WK_EN_A::DIS)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FAST_WK_EN_A::EN)
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
#[doc = "Band Gap Disable for DEEPSLEEP and BACKUP Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BG_OFF_A {
    #[doc = "0: Bandgap is always ON."]
    ON = 0,
    #[doc = "1: Bandgap is OFF in DeepSleep mode(default)."]
    OFF = 1,
}
impl From<BG_OFF_A> for bool {
    #[inline(always)]
    fn from(variant: BG_OFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BG_OFF` reader - Band Gap Disable for DEEPSLEEP and BACKUP Mode"]
pub struct BG_OFF_R(crate::FieldReader<bool, BG_OFF_A>);
impl BG_OFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BG_OFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BG_OFF_A {
        match self.bits {
            false => BG_OFF_A::ON,
            true => BG_OFF_A::OFF,
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == BG_OFF_A::ON
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == BG_OFF_A::OFF
    }
}
impl core::ops::Deref for BG_OFF_R {
    type Target = crate::FieldReader<bool, BG_OFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BG_OFF` writer - Band Gap Disable for DEEPSLEEP and BACKUP Mode"]
pub struct BG_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> BG_OFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BG_OFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bandgap is always ON."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(BG_OFF_A::ON)
    }
    #[doc = "Bandgap is OFF in DeepSleep mode(default)."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(BG_OFF_A::OFF)
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
#[doc = "V CORE POR Disable for DEEPSLEEP and BACKUP Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCORE_POR_DIS_A {
    #[doc = "0: Disabled."]
    DIS = 0,
    #[doc = "1: Enabled."]
    EN = 1,
}
impl From<VCORE_POR_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: VCORE_POR_DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCORE_POR_DIS` reader - V CORE POR Disable for DEEPSLEEP and BACKUP Mode"]
pub struct VCORE_POR_DIS_R(crate::FieldReader<bool, VCORE_POR_DIS_A>);
impl VCORE_POR_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCORE_POR_DIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCORE_POR_DIS_A {
        match self.bits {
            false => VCORE_POR_DIS_A::DIS,
            true => VCORE_POR_DIS_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == VCORE_POR_DIS_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == VCORE_POR_DIS_A::EN
    }
}
impl core::ops::Deref for VCORE_POR_DIS_R {
    type Target = crate::FieldReader<bool, VCORE_POR_DIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCORE_POR_DIS` writer - V CORE POR Disable for DEEPSLEEP and BACKUP Mode"]
pub struct VCORE_POR_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> VCORE_POR_DIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCORE_POR_DIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VCORE_POR_DIS_A::DIS)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VCORE_POR_DIS_A::EN)
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
#[doc = "LDO Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDO_DIS_A {
    #[doc = "0: Enable if Bandgap is ON(default)"]
    EN = 0,
    #[doc = "1: Disabled."]
    DIS = 1,
}
impl From<LDO_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: LDO_DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDO_DIS` reader - LDO Disable"]
pub struct LDO_DIS_R(crate::FieldReader<bool, LDO_DIS_A>);
impl LDO_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_DIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDO_DIS_A {
        match self.bits {
            false => LDO_DIS_A::EN,
            true => LDO_DIS_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == LDO_DIS_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == LDO_DIS_A::DIS
    }
}
impl core::ops::Deref for LDO_DIS_R {
    type Target = crate::FieldReader<bool, LDO_DIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_DIS` writer - LDO Disable"]
pub struct LDO_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_DIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDO_DIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable if Bandgap is ON(default)"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(LDO_DIS_A::EN)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(LDO_DIS_A::DIS)
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
#[doc = "V CORE Supply Voltage Monitor Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCORE_SVM_DIS_A {
    #[doc = "0: Enable if Bandgap is ON(default)"]
    EN = 0,
    #[doc = "1: Disabled."]
    DIS = 1,
}
impl From<VCORE_SVM_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: VCORE_SVM_DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCORE_SVM_DIS` reader - V CORE Supply Voltage Monitor Disable"]
pub struct VCORE_SVM_DIS_R(crate::FieldReader<bool, VCORE_SVM_DIS_A>);
impl VCORE_SVM_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCORE_SVM_DIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCORE_SVM_DIS_A {
        match self.bits {
            false => VCORE_SVM_DIS_A::EN,
            true => VCORE_SVM_DIS_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == VCORE_SVM_DIS_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == VCORE_SVM_DIS_A::DIS
    }
}
impl core::ops::Deref for VCORE_SVM_DIS_R {
    type Target = crate::FieldReader<bool, VCORE_SVM_DIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCORE_SVM_DIS` writer - V CORE Supply Voltage Monitor Disable"]
pub struct VCORE_SVM_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> VCORE_SVM_DIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCORE_SVM_DIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable if Bandgap is ON(default)"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VCORE_SVM_DIS_A::EN)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VCORE_SVM_DIS_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "VDDIO Power-On Reset Monitor Disable. This bit controls the Power-On Reset monitor on VDDIO supply in all operating mods.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDIO_POR_DIS_A {
    #[doc = "0: Enabled."]
    EN = 0,
    #[doc = "1: Disabled."]
    DIS = 1,
}
impl From<VDDIO_POR_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: VDDIO_POR_DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDIO_POR_DIS` reader - VDDIO Power-On Reset Monitor Disable. This bit controls the Power-On Reset monitor on VDDIO supply in all operating mods."]
pub struct VDDIO_POR_DIS_R(crate::FieldReader<bool, VDDIO_POR_DIS_A>);
impl VDDIO_POR_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VDDIO_POR_DIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDIO_POR_DIS_A {
        match self.bits {
            false => VDDIO_POR_DIS_A::EN,
            true => VDDIO_POR_DIS_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == VDDIO_POR_DIS_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == VDDIO_POR_DIS_A::DIS
    }
}
impl core::ops::Deref for VDDIO_POR_DIS_R {
    type Target = crate::FieldReader<bool, VDDIO_POR_DIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDIO_POR_DIS` writer - VDDIO Power-On Reset Monitor Disable. This bit controls the Power-On Reset monitor on VDDIO supply in all operating mods."]
pub struct VDDIO_POR_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDIO_POR_DIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDIO_POR_DIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VDDIO_POR_DIS_A::EN)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VDDIO_POR_DIS_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - System RAM 0 Data retention in BACKUP mode."]
    #[inline(always)]
    pub fn ramret_sel0(&self) -> RAMRET_SEL0_R {
        RAMRET_SEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - System RAM 1 Data retention in BACKUP mode."]
    #[inline(always)]
    pub fn ramret_sel1(&self) -> RAMRET_SEL1_R {
        RAMRET_SEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - System RAM 2 Data retention in BACKUP mode."]
    #[inline(always)]
    pub fn ramret_sel2(&self) -> RAMRET_SEL2_R {
        RAMRET_SEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - System RAM 3 Data retention in BACKUP mode."]
    #[inline(always)]
    pub fn ramret_sel3(&self) -> RAMRET_SEL3_R {
        RAMRET_SEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Operating Voltage Range"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Bypass V CORE External Supply Detection"]
    #[inline(always)]
    pub fn vcore_det_bypass(&self) -> VCORE_DET_BYPASS_R {
        VCORE_DET_BYPASS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Retention Regulator Enable. This bit controls the retention regulator in BACKUP mode."]
    #[inline(always)]
    pub fn retreg_en(&self) -> RETREG_EN_R {
        RETREG_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Fast Wake-Up Mode. This bit enables fast wake-up from DeepSleep mode."]
    #[inline(always)]
    pub fn fast_wk_en(&self) -> FAST_WK_EN_R {
        FAST_WK_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Band Gap Disable for DEEPSLEEP and BACKUP Mode"]
    #[inline(always)]
    pub fn bg_off(&self) -> BG_OFF_R {
        BG_OFF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - V CORE POR Disable for DEEPSLEEP and BACKUP Mode"]
    #[inline(always)]
    pub fn vcore_por_dis(&self) -> VCORE_POR_DIS_R {
        VCORE_POR_DIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LDO Disable"]
    #[inline(always)]
    pub fn ldo_dis(&self) -> LDO_DIS_R {
        LDO_DIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - V CORE Supply Voltage Monitor Disable"]
    #[inline(always)]
    pub fn vcore_svm_dis(&self) -> VCORE_SVM_DIS_R {
        VCORE_SVM_DIS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 25 - VDDIO Power-On Reset Monitor Disable. This bit controls the Power-On Reset monitor on VDDIO supply in all operating mods."]
    #[inline(always)]
    pub fn vddio_por_dis(&self) -> VDDIO_POR_DIS_R {
        VDDIO_POR_DIS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System RAM 0 Data retention in BACKUP mode."]
    #[inline(always)]
    pub fn ramret_sel0(&mut self) -> RAMRET_SEL0_W {
        RAMRET_SEL0_W { w: self }
    }
    #[doc = "Bit 1 - System RAM 1 Data retention in BACKUP mode."]
    #[inline(always)]
    pub fn ramret_sel1(&mut self) -> RAMRET_SEL1_W {
        RAMRET_SEL1_W { w: self }
    }
    #[doc = "Bit 2 - System RAM 2 Data retention in BACKUP mode."]
    #[inline(always)]
    pub fn ramret_sel2(&mut self) -> RAMRET_SEL2_W {
        RAMRET_SEL2_W { w: self }
    }
    #[doc = "Bit 3 - System RAM 3 Data retention in BACKUP mode."]
    #[inline(always)]
    pub fn ramret_sel3(&mut self) -> RAMRET_SEL3_W {
        RAMRET_SEL3_W { w: self }
    }
    #[doc = "Bits 4:5 - Operating Voltage Range"]
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W {
        OVR_W { w: self }
    }
    #[doc = "Bit 6 - Bypass V CORE External Supply Detection"]
    #[inline(always)]
    pub fn vcore_det_bypass(&mut self) -> VCORE_DET_BYPASS_W {
        VCORE_DET_BYPASS_W { w: self }
    }
    #[doc = "Bit 8 - Retention Regulator Enable. This bit controls the retention regulator in BACKUP mode."]
    #[inline(always)]
    pub fn retreg_en(&mut self) -> RETREG_EN_W {
        RETREG_EN_W { w: self }
    }
    #[doc = "Bit 10 - Fast Wake-Up Mode. This bit enables fast wake-up from DeepSleep mode."]
    #[inline(always)]
    pub fn fast_wk_en(&mut self) -> FAST_WK_EN_W {
        FAST_WK_EN_W { w: self }
    }
    #[doc = "Bit 11 - Band Gap Disable for DEEPSLEEP and BACKUP Mode"]
    #[inline(always)]
    pub fn bg_off(&mut self) -> BG_OFF_W {
        BG_OFF_W { w: self }
    }
    #[doc = "Bit 12 - V CORE POR Disable for DEEPSLEEP and BACKUP Mode"]
    #[inline(always)]
    pub fn vcore_por_dis(&mut self) -> VCORE_POR_DIS_W {
        VCORE_POR_DIS_W { w: self }
    }
    #[doc = "Bit 16 - LDO Disable"]
    #[inline(always)]
    pub fn ldo_dis(&mut self) -> LDO_DIS_W {
        LDO_DIS_W { w: self }
    }
    #[doc = "Bit 20 - V CORE Supply Voltage Monitor Disable"]
    #[inline(always)]
    pub fn vcore_svm_dis(&mut self) -> VCORE_SVM_DIS_W {
        VCORE_SVM_DIS_W { w: self }
    }
    #[doc = "Bit 25 - VDDIO Power-On Reset Monitor Disable. This bit controls the Power-On Reset monitor on VDDIO supply in all operating mods."]
    #[inline(always)]
    pub fn vddio_por_dis(&mut self) -> VDDIO_POR_DIS_W {
        VDDIO_POR_DIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Power Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lp_ctrl](index.html) module"]
pub struct LP_CTRL_SPEC;
impl crate::RegisterSpec for LP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lp_ctrl::R](R) reader structure"]
impl crate::Readable for LP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lp_ctrl::W](W) writer structure"]
impl crate::Writable for LP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LP_CTRL to value 0"]
impl crate::Resettable for LP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
