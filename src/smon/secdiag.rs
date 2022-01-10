#[doc = "Register `SECDIAG` reader"]
pub struct R(crate::R<SECDIAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECDIAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECDIAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECDIAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Battery-On-Reset Flag. This bit is set once the back up battery is conneted.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BORF_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<BORF_A> for bool {
    #[inline(always)]
    fn from(variant: BORF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BORF` reader - Battery-On-Reset Flag. This bit is set once the back up battery is conneted."]
pub struct BORF_R(crate::FieldReader<bool, BORF_A>);
impl BORF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BORF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BORF_A {
        match self.bits {
            false => BORF_A::NOEVENT,
            true => BORF_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == BORF_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == BORF_A::OCCURRED
    }
}
impl core::ops::Deref for BORF_R {
    type Target = crate::FieldReader<bool, BORF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Dynamic Sensor Flag. This bit is set to 1 when any of the EXTSTAT bits are set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DYNF_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<DYNF_A> for bool {
    #[inline(always)]
    fn from(variant: DYNF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DYNF` reader - Dynamic Sensor Flag. This bit is set to 1 when any of the EXTSTAT bits are set."]
pub struct DYNF_R(crate::FieldReader<bool, DYNF_A>);
impl DYNF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DYNF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DYNF_A {
        match self.bits {
            false => DYNF_A::NOEVENT,
            true => DYNF_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == DYNF_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == DYNF_A::OCCURRED
    }
}
impl core::ops::Deref for DYNF_R {
    type Target = crate::FieldReader<bool, DYNF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "AES Key Transfer. This bit is set to 1 when AES Key has been transferred from the TRNG to the battery backed AES key register. This bit can only be reset by a BOR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESKT_A {
    #[doc = "0: Key has not been transferred."]
    INCOMPLETE = 0,
    #[doc = "1: Key has been transferred."]
    COMPLETE = 1,
}
impl From<AESKT_A> for bool {
    #[inline(always)]
    fn from(variant: AESKT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESKT` reader - AES Key Transfer. This bit is set to 1 when AES Key has been transferred from the TRNG to the battery backed AES key register. This bit can only be reset by a BOR."]
pub struct AESKT_R(crate::FieldReader<bool, AESKT_A>);
impl AESKT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AESKT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESKT_A {
        match self.bits {
            false => AESKT_A::INCOMPLETE,
            true => AESKT_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `INCOMPLETE`"]
    #[inline(always)]
    pub fn is_incomplete(&self) -> bool {
        **self == AESKT_A::INCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        **self == AESKT_A::COMPLETE
    }
}
impl core::ops::Deref for AESKT_R {
    type Target = crate::FieldReader<bool, AESKT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "External Sensor 0 Detect.\n\nValue on reset: 0"]
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
#[doc = "Field `EXTSTAT0` reader - External Sensor 0 Detect."]
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
#[doc = "External Sensor 1 Detect.\n\nValue on reset: 0"]
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
#[doc = "Field `EXTSTAT1` reader - External Sensor 1 Detect."]
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
#[doc = "External Sensor 2 Detect.\n\nValue on reset: 0"]
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
#[doc = "Field `EXTSTAT2` reader - External Sensor 2 Detect."]
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
#[doc = "External Sensor 3 Detect.\n\nValue on reset: 0"]
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
#[doc = "Field `EXTSTAT3` reader - External Sensor 3 Detect."]
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
#[doc = "External Sensor 4 Detect.\n\nValue on reset: 0"]
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
#[doc = "Field `EXTSTAT4` reader - External Sensor 4 Detect."]
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
#[doc = "External Sensor 5 Detect.\n\nValue on reset: 0"]
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
#[doc = "Field `EXTSTAT5` reader - External Sensor 5 Detect."]
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
impl R {
    #[doc = "Bit 0 - Battery-On-Reset Flag. This bit is set once the back up battery is conneted."]
    #[inline(always)]
    pub fn borf(&self) -> BORF_R {
        BORF_R::new((self.bits & 0x01) != 0)
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
    #[doc = "Bit 7 - Dynamic Sensor Flag. This bit is set to 1 when any of the EXTSTAT bits are set."]
    #[inline(always)]
    pub fn dynf(&self) -> DYNF_R {
        DYNF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AES Key Transfer. This bit is set to 1 when AES Key has been transferred from the TRNG to the battery backed AES key register. This bit can only be reset by a BOR."]
    #[inline(always)]
    pub fn aeskt(&self) -> AESKT_R {
        AESKT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - External Sensor 0 Detect."]
    #[inline(always)]
    pub fn extstat0(&self) -> EXTSTAT0_R {
        EXTSTAT0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - External Sensor 1 Detect."]
    #[inline(always)]
    pub fn extstat1(&self) -> EXTSTAT1_R {
        EXTSTAT1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - External Sensor 2 Detect."]
    #[inline(always)]
    pub fn extstat2(&self) -> EXTSTAT2_R {
        EXTSTAT2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - External Sensor 3 Detect."]
    #[inline(always)]
    pub fn extstat3(&self) -> EXTSTAT3_R {
        EXTSTAT3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - External Sensor 4 Detect."]
    #[inline(always)]
    pub fn extstat4(&self) -> EXTSTAT4_R {
        EXTSTAT4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - External Sensor 5 Detect."]
    #[inline(always)]
    pub fn extstat5(&self) -> EXTSTAT5_R {
        EXTSTAT5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
#[doc = "Security Diagnostic Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secdiag](index.html) module"]
pub struct SECDIAG_SPEC;
impl crate::RegisterSpec for SECDIAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secdiag::R](R) reader structure"]
impl crate::Readable for SECDIAG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SECDIAG to value 0x01"]
impl crate::Resettable for SECDIAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
