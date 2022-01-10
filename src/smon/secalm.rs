#[doc = "Register `SECALM` reader"]
pub struct R(crate::R<SECALM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECALM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECALM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECALM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECALM` writer"]
pub struct W(crate::W<SECALM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECALM_SPEC>;
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
impl From<crate::W<SECALM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECALM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Destructive Reset Trigger. Setting this bit will generate a DRS. This bit is self-cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRS_A {
    #[doc = "0: No operation/complete."]
    COMPLETE = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<DRS_A> for bool {
    #[inline(always)]
    fn from(variant: DRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRS` reader - Destructive Reset Trigger. Setting this bit will generate a DRS. This bit is self-cleared by hardware."]
pub struct DRS_R(crate::FieldReader<bool, DRS_A>);
impl DRS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRS_A {
        match self.bits {
            false => DRS_A::COMPLETE,
            true => DRS_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        **self == DRS_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == DRS_A::START
    }
}
impl core::ops::Deref for DRS_R {
    type Target = crate::FieldReader<bool, DRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRS` writer - Destructive Reset Trigger. Setting this bit will generate a DRS. This bit is self-cleared by hardware."]
pub struct DRS_W<'a> {
    w: &'a mut W,
}
impl<'a> DRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(DRS_A::COMPLETE)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(DRS_A::START)
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
#[doc = "Key Wipe Trigger. Set to 1 to initiate a wipe of the AES key register. It does not reset the part, or log a timestamp. AES and DES registers are not affected by this bit. This bit is automatically cleared to 0 after the keys have been wiped.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYWIPE_A {
    #[doc = "0: No operation/complete."]
    COMPLETE = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<KEYWIPE_A> for bool {
    #[inline(always)]
    fn from(variant: KEYWIPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYWIPE` reader - Key Wipe Trigger. Set to 1 to initiate a wipe of the AES key register. It does not reset the part, or log a timestamp. AES and DES registers are not affected by this bit. This bit is automatically cleared to 0 after the keys have been wiped."]
pub struct KEYWIPE_R(crate::FieldReader<bool, KEYWIPE_A>);
impl KEYWIPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEYWIPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYWIPE_A {
        match self.bits {
            false => KEYWIPE_A::COMPLETE,
            true => KEYWIPE_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        **self == KEYWIPE_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == KEYWIPE_A::START
    }
}
impl core::ops::Deref for KEYWIPE_R {
    type Target = crate::FieldReader<bool, KEYWIPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEYWIPE` writer - Key Wipe Trigger. Set to 1 to initiate a wipe of the AES key register. It does not reset the part, or log a timestamp. AES and DES registers are not affected by this bit. This bit is automatically cleared to 0 after the keys have been wiped."]
pub struct KEYWIPE_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYWIPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEYWIPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(KEYWIPE_A::COMPLETE)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(KEYWIPE_A::START)
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
#[doc = "Die Shield Flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHIELDF_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<SHIELDF_A> for bool {
    #[inline(always)]
    fn from(variant: SHIELDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHIELDF` reader - Die Shield Flag."]
pub struct SHIELDF_R(crate::FieldReader<bool, SHIELDF_A>);
impl SHIELDF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SHIELDF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHIELDF_A {
        match self.bits {
            false => SHIELDF_A::NOEVENT,
            true => SHIELDF_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == SHIELDF_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == SHIELDF_A::OCCURRED
    }
}
impl core::ops::Deref for SHIELDF_R {
    type Target = crate::FieldReader<bool, SHIELDF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHIELDF` writer - Die Shield Flag."]
pub struct SHIELDF_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIELDF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHIELDF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(SHIELDF_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(SHIELDF_A::OCCURRED)
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
#[doc = "Low Temperature Detect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOTEMP_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<LOTEMP_A> for bool {
    #[inline(always)]
    fn from(variant: LOTEMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOTEMP` reader - Low Temperature Detect."]
pub struct LOTEMP_R(crate::FieldReader<bool, LOTEMP_A>);
impl LOTEMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOTEMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOTEMP_A {
        match self.bits {
            false => LOTEMP_A::NOEVENT,
            true => LOTEMP_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == LOTEMP_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == LOTEMP_A::OCCURRED
    }
}
impl core::ops::Deref for LOTEMP_R {
    type Target = crate::FieldReader<bool, LOTEMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOTEMP` writer - Low Temperature Detect."]
pub struct LOTEMP_W<'a> {
    w: &'a mut W,
}
impl<'a> LOTEMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOTEMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(LOTEMP_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(LOTEMP_A::OCCURRED)
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
#[doc = "High Temperature Detect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HITEMP_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<HITEMP_A> for bool {
    #[inline(always)]
    fn from(variant: HITEMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HITEMP` reader - High Temperature Detect."]
pub struct HITEMP_R(crate::FieldReader<bool, HITEMP_A>);
impl HITEMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HITEMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HITEMP_A {
        match self.bits {
            false => HITEMP_A::NOEVENT,
            true => HITEMP_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == HITEMP_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == HITEMP_A::OCCURRED
    }
}
impl core::ops::Deref for HITEMP_R {
    type Target = crate::FieldReader<bool, HITEMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HITEMP` writer - High Temperature Detect."]
pub struct HITEMP_W<'a> {
    w: &'a mut W,
}
impl<'a> HITEMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HITEMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(HITEMP_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(HITEMP_A::OCCURRED)
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
#[doc = "Battery Undervoltage Detect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BATLO_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<BATLO_A> for bool {
    #[inline(always)]
    fn from(variant: BATLO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BATLO` reader - Battery Undervoltage Detect."]
pub struct BATLO_R(crate::FieldReader<bool, BATLO_A>);
impl BATLO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BATLO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BATLO_A {
        match self.bits {
            false => BATLO_A::NOEVENT,
            true => BATLO_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == BATLO_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == BATLO_A::OCCURRED
    }
}
impl core::ops::Deref for BATLO_R {
    type Target = crate::FieldReader<bool, BATLO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BATLO` writer - Battery Undervoltage Detect."]
pub struct BATLO_W<'a> {
    w: &'a mut W,
}
impl<'a> BATLO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BATLO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(BATLO_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(BATLO_A::OCCURRED)
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
#[doc = "Battery Overvoltage Detect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BATHI_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<BATHI_A> for bool {
    #[inline(always)]
    fn from(variant: BATHI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BATHI` reader - Battery Overvoltage Detect."]
pub struct BATHI_R(crate::FieldReader<bool, BATHI_A>);
impl BATHI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BATHI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BATHI_A {
        match self.bits {
            false => BATHI_A::NOEVENT,
            true => BATHI_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == BATHI_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == BATHI_A::OCCURRED
    }
}
impl core::ops::Deref for BATHI_R {
    type Target = crate::FieldReader<bool, BATHI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BATHI` writer - Battery Overvoltage Detect."]
pub struct BATHI_W<'a> {
    w: &'a mut W,
}
impl<'a> BATHI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BATHI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(BATHI_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(BATHI_A::OCCURRED)
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
#[doc = "External Sensor Flag. This bit is set to 1 when any of the EXTSTAT bits are set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTF_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<EXTF_A> for bool {
    #[inline(always)]
    fn from(variant: EXTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTF` reader - External Sensor Flag. This bit is set to 1 when any of the EXTSTAT bits are set."]
pub struct EXTF_R(crate::FieldReader<bool, EXTF_A>);
impl EXTF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTF_A {
        match self.bits {
            false => EXTF_A::NOEVENT,
            true => EXTF_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == EXTF_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == EXTF_A::OCCURRED
    }
}
impl core::ops::Deref for EXTF_R {
    type Target = crate::FieldReader<bool, EXTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTF` writer - External Sensor Flag. This bit is set to 1 when any of the EXTSTAT bits are set."]
pub struct EXTF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(EXTF_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(EXTF_A::OCCURRED)
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
#[doc = "VDD Undervoltage Detect Flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDLO_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<VDDLO_A> for bool {
    #[inline(always)]
    fn from(variant: VDDLO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDLO` reader - VDD Undervoltage Detect Flag."]
pub struct VDDLO_R(crate::FieldReader<bool, VDDLO_A>);
impl VDDLO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VDDLO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDLO_A {
        match self.bits {
            false => VDDLO_A::NOEVENT,
            true => VDDLO_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == VDDLO_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == VDDLO_A::OCCURRED
    }
}
impl core::ops::Deref for VDDLO_R {
    type Target = crate::FieldReader<bool, VDDLO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDLO` writer - VDD Undervoltage Detect Flag."]
pub struct VDDLO_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDLO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDLO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(VDDLO_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(VDDLO_A::OCCURRED)
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
#[doc = "VCORE Undervoltage Detect Flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCORELO_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<VCORELO_A> for bool {
    #[inline(always)]
    fn from(variant: VCORELO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCORELO` reader - VCORE Undervoltage Detect Flag."]
pub struct VCORELO_R(crate::FieldReader<bool, VCORELO_A>);
impl VCORELO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCORELO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCORELO_A {
        match self.bits {
            false => VCORELO_A::NOEVENT,
            true => VCORELO_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == VCORELO_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == VCORELO_A::OCCURRED
    }
}
impl core::ops::Deref for VCORELO_R {
    type Target = crate::FieldReader<bool, VCORELO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCORELO` writer - VCORE Undervoltage Detect Flag."]
pub struct VCORELO_W<'a> {
    w: &'a mut W,
}
impl<'a> VCORELO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCORELO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(VCORELO_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(VCORELO_A::OCCURRED)
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
#[doc = "VCORE Overvoltage Detect Flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOREHI_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<VCOREHI_A> for bool {
    #[inline(always)]
    fn from(variant: VCOREHI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOREHI` reader - VCORE Overvoltage Detect Flag."]
pub struct VCOREHI_R(crate::FieldReader<bool, VCOREHI_A>);
impl VCOREHI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCOREHI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCOREHI_A {
        match self.bits {
            false => VCOREHI_A::NOEVENT,
            true => VCOREHI_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == VCOREHI_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == VCOREHI_A::OCCURRED
    }
}
impl core::ops::Deref for VCOREHI_R {
    type Target = crate::FieldReader<bool, VCOREHI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCOREHI` writer - VCORE Overvoltage Detect Flag."]
pub struct VCOREHI_W<'a> {
    w: &'a mut W,
}
impl<'a> VCOREHI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCOREHI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(VCOREHI_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(VCOREHI_A::OCCURRED)
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
#[doc = "VDD Overvoltage Flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDHI_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<VDDHI_A> for bool {
    #[inline(always)]
    fn from(variant: VDDHI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDHI` reader - VDD Overvoltage Flag."]
pub struct VDDHI_R(crate::FieldReader<bool, VDDHI_A>);
impl VDDHI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VDDHI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDHI_A {
        match self.bits {
            false => VDDHI_A::NOEVENT,
            true => VDDHI_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == VDDHI_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == VDDHI_A::OCCURRED
    }
}
impl core::ops::Deref for VDDHI_R {
    type Target = crate::FieldReader<bool, VDDHI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDHI` writer - VDD Overvoltage Flag."]
pub struct VDDHI_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDHI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDHI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(VDDHI_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(VDDHI_A::OCCURRED)
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
#[doc = "Voltage Glitch Detection Flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VGL_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<VGL_A> for bool {
    #[inline(always)]
    fn from(variant: VGL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VGL` reader - Voltage Glitch Detection Flag."]
pub struct VGL_R(crate::FieldReader<bool, VGL_A>);
impl VGL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VGL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VGL_A {
        match self.bits {
            false => VGL_A::NOEVENT,
            true => VGL_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == VGL_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == VGL_A::OCCURRED
    }
}
impl core::ops::Deref for VGL_R {
    type Target = crate::FieldReader<bool, VGL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VGL` writer - Voltage Glitch Detection Flag."]
pub struct VGL_W<'a> {
    w: &'a mut W,
}
impl<'a> VGL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VGL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(VGL_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(VGL_A::OCCURRED)
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
#[doc = "External Sensor 0 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTSTAT0_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<EXTSTAT0_A> for bool {
    #[inline(always)]
    fn from(variant: EXTSTAT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTSTAT0` reader - External Sensor 0 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
pub struct EXTSTAT0_R(crate::FieldReader<bool, EXTSTAT0_A>);
impl EXTSTAT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTSTAT0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTSTAT0_A {
        match self.bits {
            false => EXTSTAT0_A::NOEVENT,
            true => EXTSTAT0_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == EXTSTAT0_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == EXTSTAT0_A::OCCURRED
    }
}
impl core::ops::Deref for EXTSTAT0_R {
    type Target = crate::FieldReader<bool, EXTSTAT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTSTAT0` writer - External Sensor 0 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
pub struct EXTSTAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSTAT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTSTAT0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(EXTSTAT0_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(EXTSTAT0_A::OCCURRED)
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
#[doc = "External Sensor 1 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTSTAT1_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<EXTSTAT1_A> for bool {
    #[inline(always)]
    fn from(variant: EXTSTAT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTSTAT1` reader - External Sensor 1 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
pub struct EXTSTAT1_R(crate::FieldReader<bool, EXTSTAT1_A>);
impl EXTSTAT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTSTAT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTSTAT1_A {
        match self.bits {
            false => EXTSTAT1_A::NOEVENT,
            true => EXTSTAT1_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == EXTSTAT1_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == EXTSTAT1_A::OCCURRED
    }
}
impl core::ops::Deref for EXTSTAT1_R {
    type Target = crate::FieldReader<bool, EXTSTAT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTSTAT1` writer - External Sensor 1 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
pub struct EXTSTAT1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSTAT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTSTAT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(EXTSTAT1_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(EXTSTAT1_A::OCCURRED)
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
#[doc = "External Sensor 2 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTSTAT2_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<EXTSTAT2_A> for bool {
    #[inline(always)]
    fn from(variant: EXTSTAT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTSTAT2` reader - External Sensor 2 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
pub struct EXTSTAT2_R(crate::FieldReader<bool, EXTSTAT2_A>);
impl EXTSTAT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTSTAT2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTSTAT2_A {
        match self.bits {
            false => EXTSTAT2_A::NOEVENT,
            true => EXTSTAT2_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == EXTSTAT2_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == EXTSTAT2_A::OCCURRED
    }
}
impl core::ops::Deref for EXTSTAT2_R {
    type Target = crate::FieldReader<bool, EXTSTAT2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTSTAT2` writer - External Sensor 2 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
pub struct EXTSTAT2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSTAT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTSTAT2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(EXTSTAT2_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(EXTSTAT2_A::OCCURRED)
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
#[doc = "External Sensor 3 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTSTAT3_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<EXTSTAT3_A> for bool {
    #[inline(always)]
    fn from(variant: EXTSTAT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTSTAT3` reader - External Sensor 3 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
pub struct EXTSTAT3_R(crate::FieldReader<bool, EXTSTAT3_A>);
impl EXTSTAT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTSTAT3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTSTAT3_A {
        match self.bits {
            false => EXTSTAT3_A::NOEVENT,
            true => EXTSTAT3_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == EXTSTAT3_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == EXTSTAT3_A::OCCURRED
    }
}
impl core::ops::Deref for EXTSTAT3_R {
    type Target = crate::FieldReader<bool, EXTSTAT3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTSTAT3` writer - External Sensor 3 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
pub struct EXTSTAT3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSTAT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTSTAT3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(EXTSTAT3_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(EXTSTAT3_A::OCCURRED)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "External Sensor 4 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTSTAT4_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<EXTSTAT4_A> for bool {
    #[inline(always)]
    fn from(variant: EXTSTAT4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTSTAT4` reader - External Sensor 4 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
pub struct EXTSTAT4_R(crate::FieldReader<bool, EXTSTAT4_A>);
impl EXTSTAT4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTSTAT4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTSTAT4_A {
        match self.bits {
            false => EXTSTAT4_A::NOEVENT,
            true => EXTSTAT4_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == EXTSTAT4_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == EXTSTAT4_A::OCCURRED
    }
}
impl core::ops::Deref for EXTSTAT4_R {
    type Target = crate::FieldReader<bool, EXTSTAT4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTSTAT4` writer - External Sensor 4 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
pub struct EXTSTAT4_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSTAT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTSTAT4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(EXTSTAT4_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(EXTSTAT4_A::OCCURRED)
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
#[doc = "External Sensor 5 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTSTAT5_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<EXTSTAT5_A> for bool {
    #[inline(always)]
    fn from(variant: EXTSTAT5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTSTAT5` reader - External Sensor 5 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
pub struct EXTSTAT5_R(crate::FieldReader<bool, EXTSTAT5_A>);
impl EXTSTAT5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTSTAT5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTSTAT5_A {
        match self.bits {
            false => EXTSTAT5_A::NOEVENT,
            true => EXTSTAT5_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == EXTSTAT5_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == EXTSTAT5_A::OCCURRED
    }
}
impl core::ops::Deref for EXTSTAT5_R {
    type Target = crate::FieldReader<bool, EXTSTAT5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTSTAT5` writer - External Sensor 5 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
pub struct EXTSTAT5_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSTAT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTSTAT5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(EXTSTAT5_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(EXTSTAT5_A::OCCURRED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "External Sensor 0 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTSWARN0_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<EXTSWARN0_A> for bool {
    #[inline(always)]
    fn from(variant: EXTSWARN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTSWARN0` reader - External Sensor 0 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
pub struct EXTSWARN0_R(crate::FieldReader<bool, EXTSWARN0_A>);
impl EXTSWARN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTSWARN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTSWARN0_A {
        match self.bits {
            false => EXTSWARN0_A::NOEVENT,
            true => EXTSWARN0_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == EXTSWARN0_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == EXTSWARN0_A::OCCURRED
    }
}
impl core::ops::Deref for EXTSWARN0_R {
    type Target = crate::FieldReader<bool, EXTSWARN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTSWARN0` writer - External Sensor 0 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
pub struct EXTSWARN0_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSWARN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTSWARN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(EXTSWARN0_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(EXTSWARN0_A::OCCURRED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "External Sensor 1 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTSWARN1_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<EXTSWARN1_A> for bool {
    #[inline(always)]
    fn from(variant: EXTSWARN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTSWARN1` reader - External Sensor 1 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
pub struct EXTSWARN1_R(crate::FieldReader<bool, EXTSWARN1_A>);
impl EXTSWARN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTSWARN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTSWARN1_A {
        match self.bits {
            false => EXTSWARN1_A::NOEVENT,
            true => EXTSWARN1_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == EXTSWARN1_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == EXTSWARN1_A::OCCURRED
    }
}
impl core::ops::Deref for EXTSWARN1_R {
    type Target = crate::FieldReader<bool, EXTSWARN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTSWARN1` writer - External Sensor 1 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
pub struct EXTSWARN1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSWARN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTSWARN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(EXTSWARN1_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(EXTSWARN1_A::OCCURRED)
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
#[doc = "External Sensor 2 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTSWARN2_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<EXTSWARN2_A> for bool {
    #[inline(always)]
    fn from(variant: EXTSWARN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTSWARN2` reader - External Sensor 2 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
pub struct EXTSWARN2_R(crate::FieldReader<bool, EXTSWARN2_A>);
impl EXTSWARN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTSWARN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTSWARN2_A {
        match self.bits {
            false => EXTSWARN2_A::NOEVENT,
            true => EXTSWARN2_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == EXTSWARN2_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == EXTSWARN2_A::OCCURRED
    }
}
impl core::ops::Deref for EXTSWARN2_R {
    type Target = crate::FieldReader<bool, EXTSWARN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTSWARN2` writer - External Sensor 2 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
pub struct EXTSWARN2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSWARN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTSWARN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(EXTSWARN2_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(EXTSWARN2_A::OCCURRED)
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
#[doc = "External Sensor 3 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTSWARN3_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<EXTSWARN3_A> for bool {
    #[inline(always)]
    fn from(variant: EXTSWARN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTSWARN3` reader - External Sensor 3 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
pub struct EXTSWARN3_R(crate::FieldReader<bool, EXTSWARN3_A>);
impl EXTSWARN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTSWARN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTSWARN3_A {
        match self.bits {
            false => EXTSWARN3_A::NOEVENT,
            true => EXTSWARN3_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == EXTSWARN3_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == EXTSWARN3_A::OCCURRED
    }
}
impl core::ops::Deref for EXTSWARN3_R {
    type Target = crate::FieldReader<bool, EXTSWARN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTSWARN3` writer - External Sensor 3 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
pub struct EXTSWARN3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSWARN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTSWARN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(EXTSWARN3_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(EXTSWARN3_A::OCCURRED)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "External Sensor 4 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTSWARN4_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<EXTSWARN4_A> for bool {
    #[inline(always)]
    fn from(variant: EXTSWARN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTSWARN4` reader - External Sensor 4 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
pub struct EXTSWARN4_R(crate::FieldReader<bool, EXTSWARN4_A>);
impl EXTSWARN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTSWARN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTSWARN4_A {
        match self.bits {
            false => EXTSWARN4_A::NOEVENT,
            true => EXTSWARN4_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == EXTSWARN4_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == EXTSWARN4_A::OCCURRED
    }
}
impl core::ops::Deref for EXTSWARN4_R {
    type Target = crate::FieldReader<bool, EXTSWARN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTSWARN4` writer - External Sensor 4 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
pub struct EXTSWARN4_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSWARN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTSWARN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(EXTSWARN4_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(EXTSWARN4_A::OCCURRED)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "External Sensor 5 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTSWARN5_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<EXTSWARN5_A> for bool {
    #[inline(always)]
    fn from(variant: EXTSWARN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTSWARN5` reader - External Sensor 5 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
pub struct EXTSWARN5_R(crate::FieldReader<bool, EXTSWARN5_A>);
impl EXTSWARN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTSWARN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTSWARN5_A {
        match self.bits {
            false => EXTSWARN5_A::NOEVENT,
            true => EXTSWARN5_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == EXTSWARN5_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == EXTSWARN5_A::OCCURRED
    }
}
impl core::ops::Deref for EXTSWARN5_R {
    type Target = crate::FieldReader<bool, EXTSWARN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTSWARN5` writer - External Sensor 5 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
pub struct EXTSWARN5_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSWARN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTSWARN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(EXTSWARN5_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(EXTSWARN5_A::OCCURRED)
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
    #[doc = "Bit 0 - Destructive Reset Trigger. Setting this bit will generate a DRS. This bit is self-cleared by hardware."]
    #[inline(always)]
    pub fn drs(&self) -> DRS_R {
        DRS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Key Wipe Trigger. Set to 1 to initiate a wipe of the AES key register. It does not reset the part, or log a timestamp. AES and DES registers are not affected by this bit. This bit is automatically cleared to 0 after the keys have been wiped."]
    #[inline(always)]
    pub fn keywipe(&self) -> KEYWIPE_R {
        KEYWIPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Die Shield Flag."]
    #[inline(always)]
    pub fn shieldf(&self) -> SHIELDF_R {
        SHIELDF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Low Temperature Detect."]
    #[inline(always)]
    pub fn lotemp(&self) -> LOTEMP_R {
        LOTEMP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - High Temperature Detect."]
    #[inline(always)]
    pub fn hitemp(&self) -> HITEMP_R {
        HITEMP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Battery Undervoltage Detect."]
    #[inline(always)]
    pub fn batlo(&self) -> BATLO_R {
        BATLO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Battery Overvoltage Detect."]
    #[inline(always)]
    pub fn bathi(&self) -> BATHI_R {
        BATHI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Sensor Flag. This bit is set to 1 when any of the EXTSTAT bits are set."]
    #[inline(always)]
    pub fn extf(&self) -> EXTF_R {
        EXTF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - VDD Undervoltage Detect Flag."]
    #[inline(always)]
    pub fn vddlo(&self) -> VDDLO_R {
        VDDLO_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - VCORE Undervoltage Detect Flag."]
    #[inline(always)]
    pub fn vcorelo(&self) -> VCORELO_R {
        VCORELO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - VCORE Overvoltage Detect Flag."]
    #[inline(always)]
    pub fn vcorehi(&self) -> VCOREHI_R {
        VCOREHI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - VDD Overvoltage Flag."]
    #[inline(always)]
    pub fn vddhi(&self) -> VDDHI_R {
        VDDHI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Voltage Glitch Detection Flag."]
    #[inline(always)]
    pub fn vgl(&self) -> VGL_R {
        VGL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - External Sensor 0 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
    #[inline(always)]
    pub fn extstat0(&self) -> EXTSTAT0_R {
        EXTSTAT0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - External Sensor 1 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
    #[inline(always)]
    pub fn extstat1(&self) -> EXTSTAT1_R {
        EXTSTAT1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - External Sensor 2 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
    #[inline(always)]
    pub fn extstat2(&self) -> EXTSTAT2_R {
        EXTSTAT2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - External Sensor 3 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
    #[inline(always)]
    pub fn extstat3(&self) -> EXTSTAT3_R {
        EXTSTAT3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - External Sensor 4 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
    #[inline(always)]
    pub fn extstat4(&self) -> EXTSTAT4_R {
        EXTSTAT4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - External Sensor 5 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
    #[inline(always)]
    pub fn extstat5(&self) -> EXTSTAT5_R {
        EXTSTAT5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - External Sensor 0 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
    #[inline(always)]
    pub fn extswarn0(&self) -> EXTSWARN0_R {
        EXTSWARN0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - External Sensor 1 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
    #[inline(always)]
    pub fn extswarn1(&self) -> EXTSWARN1_R {
        EXTSWARN1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - External Sensor 2 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
    #[inline(always)]
    pub fn extswarn2(&self) -> EXTSWARN2_R {
        EXTSWARN2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - External Sensor 3 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
    #[inline(always)]
    pub fn extswarn3(&self) -> EXTSWARN3_R {
        EXTSWARN3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - External Sensor 4 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
    #[inline(always)]
    pub fn extswarn4(&self) -> EXTSWARN4_R {
        EXTSWARN4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - External Sensor 5 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
    #[inline(always)]
    pub fn extswarn5(&self) -> EXTSWARN5_R {
        EXTSWARN5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Destructive Reset Trigger. Setting this bit will generate a DRS. This bit is self-cleared by hardware."]
    #[inline(always)]
    pub fn drs(&mut self) -> DRS_W {
        DRS_W { w: self }
    }
    #[doc = "Bit 1 - Key Wipe Trigger. Set to 1 to initiate a wipe of the AES key register. It does not reset the part, or log a timestamp. AES and DES registers are not affected by this bit. This bit is automatically cleared to 0 after the keys have been wiped."]
    #[inline(always)]
    pub fn keywipe(&mut self) -> KEYWIPE_W {
        KEYWIPE_W { w: self }
    }
    #[doc = "Bit 2 - Die Shield Flag."]
    #[inline(always)]
    pub fn shieldf(&mut self) -> SHIELDF_W {
        SHIELDF_W { w: self }
    }
    #[doc = "Bit 3 - Low Temperature Detect."]
    #[inline(always)]
    pub fn lotemp(&mut self) -> LOTEMP_W {
        LOTEMP_W { w: self }
    }
    #[doc = "Bit 4 - High Temperature Detect."]
    #[inline(always)]
    pub fn hitemp(&mut self) -> HITEMP_W {
        HITEMP_W { w: self }
    }
    #[doc = "Bit 5 - Battery Undervoltage Detect."]
    #[inline(always)]
    pub fn batlo(&mut self) -> BATLO_W {
        BATLO_W { w: self }
    }
    #[doc = "Bit 6 - Battery Overvoltage Detect."]
    #[inline(always)]
    pub fn bathi(&mut self) -> BATHI_W {
        BATHI_W { w: self }
    }
    #[doc = "Bit 7 - External Sensor Flag. This bit is set to 1 when any of the EXTSTAT bits are set."]
    #[inline(always)]
    pub fn extf(&mut self) -> EXTF_W {
        EXTF_W { w: self }
    }
    #[doc = "Bit 8 - VDD Undervoltage Detect Flag."]
    #[inline(always)]
    pub fn vddlo(&mut self) -> VDDLO_W {
        VDDLO_W { w: self }
    }
    #[doc = "Bit 9 - VCORE Undervoltage Detect Flag."]
    #[inline(always)]
    pub fn vcorelo(&mut self) -> VCORELO_W {
        VCORELO_W { w: self }
    }
    #[doc = "Bit 10 - VCORE Overvoltage Detect Flag."]
    #[inline(always)]
    pub fn vcorehi(&mut self) -> VCOREHI_W {
        VCOREHI_W { w: self }
    }
    #[doc = "Bit 11 - VDD Overvoltage Flag."]
    #[inline(always)]
    pub fn vddhi(&mut self) -> VDDHI_W {
        VDDHI_W { w: self }
    }
    #[doc = "Bit 12 - Voltage Glitch Detection Flag."]
    #[inline(always)]
    pub fn vgl(&mut self) -> VGL_W {
        VGL_W { w: self }
    }
    #[doc = "Bit 16 - External Sensor 0 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
    #[inline(always)]
    pub fn extstat0(&mut self) -> EXTSTAT0_W {
        EXTSTAT0_W { w: self }
    }
    #[doc = "Bit 17 - External Sensor 1 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
    #[inline(always)]
    pub fn extstat1(&mut self) -> EXTSTAT1_W {
        EXTSTAT1_W { w: self }
    }
    #[doc = "Bit 18 - External Sensor 2 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
    #[inline(always)]
    pub fn extstat2(&mut self) -> EXTSTAT2_W {
        EXTSTAT2_W { w: self }
    }
    #[doc = "Bit 19 - External Sensor 3 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
    #[inline(always)]
    pub fn extstat3(&mut self) -> EXTSTAT3_W {
        EXTSTAT3_W { w: self }
    }
    #[doc = "Bit 20 - External Sensor 4 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
    #[inline(always)]
    pub fn extstat4(&mut self) -> EXTSTAT4_W {
        EXTSTAT4_W { w: self }
    }
    #[doc = "Bit 21 - External Sensor 5 Detect. The tamper detect is only active when it is enabled. This bits needs to be cleared in software after a tamper event to re-arm the sensor."]
    #[inline(always)]
    pub fn extstat5(&mut self) -> EXTSTAT5_W {
        EXTSTAT5_W { w: self }
    }
    #[doc = "Bit 24 - External Sensor 0 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
    #[inline(always)]
    pub fn extswarn0(&mut self) -> EXTSWARN0_W {
        EXTSWARN0_W { w: self }
    }
    #[doc = "Bit 25 - External Sensor 1 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
    #[inline(always)]
    pub fn extswarn1(&mut self) -> EXTSWARN1_W {
        EXTSWARN1_W { w: self }
    }
    #[doc = "Bit 26 - External Sensor 2 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
    #[inline(always)]
    pub fn extswarn2(&mut self) -> EXTSWARN2_W {
        EXTSWARN2_W { w: self }
    }
    #[doc = "Bit 27 - External Sensor 3 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
    #[inline(always)]
    pub fn extswarn3(&mut self) -> EXTSWARN3_W {
        EXTSWARN3_W { w: self }
    }
    #[doc = "Bit 28 - External Sensor 4 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
    #[inline(always)]
    pub fn extswarn4(&mut self) -> EXTSWARN4_W {
        EXTSWARN4_W { w: self }
    }
    #[doc = "Bit 29 - External Sensor 5 Warning Ready flag. The tamper detect warning flags are set, regardless of whether the external sensors are enabled."]
    #[inline(always)]
    pub fn extswarn5(&mut self) -> EXTSWARN5_W {
        EXTSWARN5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security Alarm Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secalm](index.html) module"]
pub struct SECALM_SPEC;
impl crate::RegisterSpec for SECALM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secalm::R](R) reader structure"]
impl crate::Readable for SECALM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secalm::W](W) writer structure"]
impl crate::Writable for SECALM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECALM to value 0"]
impl crate::Resettable for SECALM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
