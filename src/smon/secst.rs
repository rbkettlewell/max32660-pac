#[doc = "Register `SECST` reader"]
pub struct R(crate::R<SECST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "External Sensor Control Register Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTSRS_A {
    #[doc = "0: Access authorized."]
    ALLOWED = 0,
    #[doc = "1: Access not authorized."]
    NOTALLOWED = 1,
}
impl From<EXTSRS_A> for bool {
    #[inline(always)]
    fn from(variant: EXTSRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTSRS` reader - External Sensor Control Register Status."]
pub struct EXTSRS_R(crate::FieldReader<bool, EXTSRS_A>);
impl EXTSRS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTSRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTSRS_A {
        match self.bits {
            false => EXTSRS_A::ALLOWED,
            true => EXTSRS_A::NOTALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        **self == EXTSRS_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `NOTALLOWED`"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        **self == EXTSRS_A::NOTALLOWED
    }
}
impl core::ops::Deref for EXTSRS_R {
    type Target = crate::FieldReader<bool, EXTSRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Internal Sensor Control Register Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSRS_A {
    #[doc = "0: Access authorized."]
    ALLOWED = 0,
    #[doc = "1: Access not authorized."]
    NOTALLOWED = 1,
}
impl From<INTSRS_A> for bool {
    #[inline(always)]
    fn from(variant: INTSRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSRS` reader - Internal Sensor Control Register Status."]
pub struct INTSRS_R(crate::FieldReader<bool, INTSRS_A>);
impl INTSRS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTSRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSRS_A {
        match self.bits {
            false => INTSRS_A::ALLOWED,
            true => INTSRS_A::NOTALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        **self == INTSRS_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `NOTALLOWED`"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        **self == INTSRS_A::NOTALLOWED
    }
}
impl core::ops::Deref for INTSRS_R {
    type Target = crate::FieldReader<bool, INTSRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Security Alarm Register Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECALRS_A {
    #[doc = "0: Access authorized."]
    ALLOWED = 0,
    #[doc = "1: Access not authorized."]
    NOTALLOWED = 1,
}
impl From<SECALRS_A> for bool {
    #[inline(always)]
    fn from(variant: SECALRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECALRS` reader - Security Alarm Register Status."]
pub struct SECALRS_R(crate::FieldReader<bool, SECALRS_A>);
impl SECALRS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SECALRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECALRS_A {
        match self.bits {
            false => SECALRS_A::ALLOWED,
            true => SECALRS_A::NOTALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        **self == SECALRS_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `NOTALLOWED`"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        **self == SECALRS_A::NOTALLOWED
    }
}
impl core::ops::Deref for SECALRS_R {
    type Target = crate::FieldReader<bool, SECALRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - External Sensor Control Register Status."]
    #[inline(always)]
    pub fn extsrs(&self) -> EXTSRS_R {
        EXTSRS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Internal Sensor Control Register Status."]
    #[inline(always)]
    pub fn intsrs(&self) -> INTSRS_R {
        INTSRS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Security Alarm Register Status."]
    #[inline(always)]
    pub fn secalrs(&self) -> SECALRS_R {
        SECALRS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "Security Monitor Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secst](index.html) module"]
pub struct SECST_SPEC;
impl crate::RegisterSpec for SECST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secst::R](R) reader structure"]
impl crate::Readable for SECST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SECST to value 0"]
impl crate::Resettable for SECST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
