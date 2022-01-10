#[doc = "Register `SFSTAT` reader"]
pub struct R(crate::R<SFSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "TRNG function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRNG_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<TRNG_A> for bool {
    #[inline(always)]
    fn from(variant: TRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRNG` reader - TRNG function."]
pub struct TRNG_R(crate::FieldReader<bool, TRNG_A>);
impl TRNG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRNG_A {
        match self.bits {
            false => TRNG_A::NO,
            true => TRNG_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == TRNG_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == TRNG_A::YES
    }
}
impl core::ops::Deref for TRNG_R {
    type Target = crate::FieldReader<bool, TRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "AES function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AES_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<AES_A> for bool {
    #[inline(always)]
    fn from(variant: AES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AES` reader - AES function."]
pub struct AES_R(crate::FieldReader<bool, AES_A>);
impl AES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AES_A {
        match self.bits {
            false => AES_A::NO,
            true => AES_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == AES_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == AES_A::YES
    }
}
impl core::ops::Deref for AES_R {
    type Target = crate::FieldReader<bool, AES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SHA function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHA_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<SHA_A> for bool {
    #[inline(always)]
    fn from(variant: SHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHA` reader - SHA function."]
pub struct SHA_R(crate::FieldReader<bool, SHA_A>);
impl SHA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SHA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHA_A {
        match self.bits {
            false => SHA_A::NO,
            true => SHA_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == SHA_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == SHA_A::YES
    }
}
impl core::ops::Deref for SHA_R {
    type Target = crate::FieldReader<bool, SHA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "MAA function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAA_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<MAA_A> for bool {
    #[inline(always)]
    fn from(variant: MAA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAA` reader - MAA function."]
pub struct MAA_R(crate::FieldReader<bool, MAA_A>);
impl MAA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAA_A {
        match self.bits {
            false => MAA_A::NO,
            true => MAA_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == MAA_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == MAA_A::YES
    }
}
impl core::ops::Deref for MAA_R {
    type Target = crate::FieldReader<bool, MAA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 2 - TRNG function."]
    #[inline(always)]
    pub fn trng(&self) -> TRNG_R {
        TRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AES function."]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SHA function."]
    #[inline(always)]
    pub fn sha(&self) -> SHA_R {
        SHA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MAA function."]
    #[inline(always)]
    pub fn maa(&self) -> MAA_R {
        MAA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "secfuncstat register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfstat](index.html) module"]
pub struct SFSTAT_SPEC;
impl crate::RegisterSpec for SFSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfstat::R](R) reader structure"]
impl crate::Readable for SFSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SFSTAT to value 0"]
impl crate::Resettable for SFSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
