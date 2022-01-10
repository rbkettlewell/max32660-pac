#[doc = "Register `INTSCN` reader"]
pub struct R(crate::R<INTSCN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSCN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSCN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSCN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSCN` writer"]
pub struct W(crate::W<INTSCN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSCN_SPEC>;
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
impl From<crate::W<INTSCN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTSCN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Die Shield Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHIELD_EN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<SHIELD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SHIELD_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHIELD_EN` reader - Die Shield Enable."]
pub struct SHIELD_EN_R(crate::FieldReader<bool, SHIELD_EN_A>);
impl SHIELD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SHIELD_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHIELD_EN_A {
        match self.bits {
            false => SHIELD_EN_A::DIS,
            true => SHIELD_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == SHIELD_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == SHIELD_EN_A::EN
    }
}
impl core::ops::Deref for SHIELD_EN_R {
    type Target = crate::FieldReader<bool, SHIELD_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHIELD_EN` writer - Die Shield Enable."]
pub struct SHIELD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIELD_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHIELD_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SHIELD_EN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SHIELD_EN_A::EN)
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
#[doc = "Temperature Sensor Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMP_EN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<TEMP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TEMP_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEMP_EN` reader - Temperature Sensor Enable."]
pub struct TEMP_EN_R(crate::FieldReader<bool, TEMP_EN_A>);
impl TEMP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEMP_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEMP_EN_A {
        match self.bits {
            false => TEMP_EN_A::DIS,
            true => TEMP_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TEMP_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TEMP_EN_A::EN
    }
}
impl core::ops::Deref for TEMP_EN_R {
    type Target = crate::FieldReader<bool, TEMP_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEMP_EN` writer - Temperature Sensor Enable."]
pub struct TEMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMP_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEMP_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TEMP_EN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TEMP_EN_A::EN)
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
#[doc = "Battery Monitor Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBAT_EN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<VBAT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: VBAT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBAT_EN` reader - Battery Monitor Enable."]
pub struct VBAT_EN_R(crate::FieldReader<bool, VBAT_EN_A>);
impl VBAT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBAT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBAT_EN_A {
        match self.bits {
            false => VBAT_EN_A::DIS,
            true => VBAT_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == VBAT_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == VBAT_EN_A::EN
    }
}
impl core::ops::Deref for VBAT_EN_R {
    type Target = crate::FieldReader<bool, VBAT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBAT_EN` writer - Battery Monitor Enable."]
pub struct VBAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBAT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBAT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VBAT_EN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VBAT_EN_A::EN)
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
#[doc = "Low Temperature Detection Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOTEMP_SEL_A {
    #[doc = "0: -50 degrees C."]
    NEG50C = 0,
    #[doc = "1: -30 degrees C."]
    NEG30C = 1,
}
impl From<LOTEMP_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: LOTEMP_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOTEMP_SEL` reader - Low Temperature Detection Select."]
pub struct LOTEMP_SEL_R(crate::FieldReader<bool, LOTEMP_SEL_A>);
impl LOTEMP_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOTEMP_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOTEMP_SEL_A {
        match self.bits {
            false => LOTEMP_SEL_A::NEG50C,
            true => LOTEMP_SEL_A::NEG30C,
        }
    }
    #[doc = "Checks if the value of the field is `NEG50C`"]
    #[inline(always)]
    pub fn is_neg50c(&self) -> bool {
        **self == LOTEMP_SEL_A::NEG50C
    }
    #[doc = "Checks if the value of the field is `NEG30C`"]
    #[inline(always)]
    pub fn is_neg30c(&self) -> bool {
        **self == LOTEMP_SEL_A::NEG30C
    }
}
impl core::ops::Deref for LOTEMP_SEL_R {
    type Target = crate::FieldReader<bool, LOTEMP_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOTEMP_SEL` writer - Low Temperature Detection Select."]
pub struct LOTEMP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LOTEMP_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOTEMP_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "-50 degrees C."]
    #[inline(always)]
    pub fn neg50c(self) -> &'a mut W {
        self.variant(LOTEMP_SEL_A::NEG50C)
    }
    #[doc = "-30 degrees C."]
    #[inline(always)]
    pub fn neg30c(self) -> &'a mut W {
        self.variant(LOTEMP_SEL_A::NEG30C)
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
#[doc = "VCORE Undervoltage Detect Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCORELOEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<VCORELOEN_A> for bool {
    #[inline(always)]
    fn from(variant: VCORELOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCORELOEN` reader - VCORE Undervoltage Detect Enable."]
pub struct VCORELOEN_R(crate::FieldReader<bool, VCORELOEN_A>);
impl VCORELOEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCORELOEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCORELOEN_A {
        match self.bits {
            false => VCORELOEN_A::DIS,
            true => VCORELOEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == VCORELOEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == VCORELOEN_A::EN
    }
}
impl core::ops::Deref for VCORELOEN_R {
    type Target = crate::FieldReader<bool, VCORELOEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCORELOEN` writer - VCORE Undervoltage Detect Enable."]
pub struct VCORELOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VCORELOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCORELOEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VCORELOEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VCORELOEN_A::EN)
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
#[doc = "VCORE Overvoltage Detect Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOREHIEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<VCOREHIEN_A> for bool {
    #[inline(always)]
    fn from(variant: VCOREHIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOREHIEN` reader - VCORE Overvoltage Detect Enable."]
pub struct VCOREHIEN_R(crate::FieldReader<bool, VCOREHIEN_A>);
impl VCOREHIEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCOREHIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCOREHIEN_A {
        match self.bits {
            false => VCOREHIEN_A::DIS,
            true => VCOREHIEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == VCOREHIEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == VCOREHIEN_A::EN
    }
}
impl core::ops::Deref for VCOREHIEN_R {
    type Target = crate::FieldReader<bool, VCOREHIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCOREHIEN` writer - VCORE Overvoltage Detect Enable."]
pub struct VCOREHIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VCOREHIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCOREHIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VCOREHIEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VCOREHIEN_A::EN)
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
#[doc = "VDD Undervoltage Detect Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDLOEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<VDDLOEN_A> for bool {
    #[inline(always)]
    fn from(variant: VDDLOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDLOEN` reader - VDD Undervoltage Detect Enable."]
pub struct VDDLOEN_R(crate::FieldReader<bool, VDDLOEN_A>);
impl VDDLOEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VDDLOEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDLOEN_A {
        match self.bits {
            false => VDDLOEN_A::DIS,
            true => VDDLOEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == VDDLOEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == VDDLOEN_A::EN
    }
}
impl core::ops::Deref for VDDLOEN_R {
    type Target = crate::FieldReader<bool, VDDLOEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDLOEN` writer - VDD Undervoltage Detect Enable."]
pub struct VDDLOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDLOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDLOEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VDDLOEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VDDLOEN_A::EN)
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
#[doc = "VDD Overvoltage Detect Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDHIEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<VDDHIEN_A> for bool {
    #[inline(always)]
    fn from(variant: VDDHIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDHIEN` reader - VDD Overvoltage Detect Enable."]
pub struct VDDHIEN_R(crate::FieldReader<bool, VDDHIEN_A>);
impl VDDHIEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VDDHIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDHIEN_A {
        match self.bits {
            false => VDDHIEN_A::DIS,
            true => VDDHIEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == VDDHIEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == VDDHIEN_A::EN
    }
}
impl core::ops::Deref for VDDHIEN_R {
    type Target = crate::FieldReader<bool, VDDHIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDHIEN` writer - VDD Overvoltage Detect Enable."]
pub struct VDDHIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDHIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDHIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VDDHIEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VDDHIEN_A::EN)
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
#[doc = "Voltage Glitch Detection Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VGLEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<VGLEN_A> for bool {
    #[inline(always)]
    fn from(variant: VGLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VGLEN` reader - Voltage Glitch Detection Enable."]
pub struct VGLEN_R(crate::FieldReader<bool, VGLEN_A>);
impl VGLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VGLEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VGLEN_A {
        match self.bits {
            false => VGLEN_A::DIS,
            true => VGLEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == VGLEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == VGLEN_A::EN
    }
}
impl core::ops::Deref for VGLEN_R {
    type Target = crate::FieldReader<bool, VGLEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VGLEN` writer - Voltage Glitch Detection Enable."]
pub struct VGLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VGLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VGLEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VGLEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(VGLEN_A::EN)
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
#[doc = "Lock Register. Once locked, the INTSCN register can no longer be modified. Only a battery disconnect will clear this bit. VBAT powers this register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: Unlocked."]
    UNLOCKED = 0,
    #[doc = "1: Locked."]
    LOCKED = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - Lock Register. Once locked, the INTSCN register can no longer be modified. Only a battery disconnect will clear this bit. VBAT powers this register."]
pub struct LOCK_R(crate::FieldReader<bool, LOCK_A>);
impl LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::UNLOCKED,
            true => LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == LOCK_A::LOCKED
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK` writer - Lock Register. Once locked, the INTSCN register can no longer be modified. Only a battery disconnect will clear this bit. VBAT powers this register."]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Unlocked."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCK_A::UNLOCKED)
    }
    #[doc = "Locked."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCK_A::LOCKED)
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
    #[doc = "Bit 0 - Die Shield Enable."]
    #[inline(always)]
    pub fn shield_en(&self) -> SHIELD_EN_R {
        SHIELD_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Temperature Sensor Enable."]
    #[inline(always)]
    pub fn temp_en(&self) -> TEMP_EN_R {
        TEMP_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Battery Monitor Enable."]
    #[inline(always)]
    pub fn vbat_en(&self) -> VBAT_EN_R {
        VBAT_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Low Temperature Detection Select."]
    #[inline(always)]
    pub fn lotemp_sel(&self) -> LOTEMP_SEL_R {
        LOTEMP_SEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - VCORE Undervoltage Detect Enable."]
    #[inline(always)]
    pub fn vcoreloen(&self) -> VCORELOEN_R {
        VCORELOEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - VCORE Overvoltage Detect Enable."]
    #[inline(always)]
    pub fn vcorehien(&self) -> VCOREHIEN_R {
        VCOREHIEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - VDD Undervoltage Detect Enable."]
    #[inline(always)]
    pub fn vddloen(&self) -> VDDLOEN_R {
        VDDLOEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - VDD Overvoltage Detect Enable."]
    #[inline(always)]
    pub fn vddhien(&self) -> VDDHIEN_R {
        VDDHIEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Voltage Glitch Detection Enable."]
    #[inline(always)]
    pub fn vglen(&self) -> VGLEN_R {
        VGLEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Lock Register. Once locked, the INTSCN register can no longer be modified. Only a battery disconnect will clear this bit. VBAT powers this register."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Die Shield Enable."]
    #[inline(always)]
    pub fn shield_en(&mut self) -> SHIELD_EN_W {
        SHIELD_EN_W { w: self }
    }
    #[doc = "Bit 1 - Temperature Sensor Enable."]
    #[inline(always)]
    pub fn temp_en(&mut self) -> TEMP_EN_W {
        TEMP_EN_W { w: self }
    }
    #[doc = "Bit 2 - Battery Monitor Enable."]
    #[inline(always)]
    pub fn vbat_en(&mut self) -> VBAT_EN_W {
        VBAT_EN_W { w: self }
    }
    #[doc = "Bit 16 - Low Temperature Detection Select."]
    #[inline(always)]
    pub fn lotemp_sel(&mut self) -> LOTEMP_SEL_W {
        LOTEMP_SEL_W { w: self }
    }
    #[doc = "Bit 18 - VCORE Undervoltage Detect Enable."]
    #[inline(always)]
    pub fn vcoreloen(&mut self) -> VCORELOEN_W {
        VCORELOEN_W { w: self }
    }
    #[doc = "Bit 19 - VCORE Overvoltage Detect Enable."]
    #[inline(always)]
    pub fn vcorehien(&mut self) -> VCOREHIEN_W {
        VCOREHIEN_W { w: self }
    }
    #[doc = "Bit 20 - VDD Undervoltage Detect Enable."]
    #[inline(always)]
    pub fn vddloen(&mut self) -> VDDLOEN_W {
        VDDLOEN_W { w: self }
    }
    #[doc = "Bit 21 - VDD Overvoltage Detect Enable."]
    #[inline(always)]
    pub fn vddhien(&mut self) -> VDDHIEN_W {
        VDDHIEN_W { w: self }
    }
    #[doc = "Bit 22 - Voltage Glitch Detection Enable."]
    #[inline(always)]
    pub fn vglen(&mut self) -> VGLEN_W {
        VGLEN_W { w: self }
    }
    #[doc = "Bit 31 - Lock Register. Once locked, the INTSCN register can no longer be modified. Only a battery disconnect will clear this bit. VBAT powers this register."]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal Sensor Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intscn](index.html) module"]
pub struct INTSCN_SPEC;
impl crate::RegisterSpec for INTSCN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intscn::R](R) reader structure"]
impl crate::Readable for INTSCN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intscn::W](W) writer structure"]
impl crate::Writable for INTSCN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTSCN to value 0"]
impl crate::Resettable for INTSCN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
