#[doc = "Register `FSTAT` reader"]
pub struct R(crate::R<FSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "FPU Function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPU_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<FPU_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPU` reader - FPU Function."]
pub struct FPU_R(crate::FieldReader<bool, FPU_A>);
impl FPU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPU_A {
        match self.bits {
            false => FPU_A::NO,
            true => FPU_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == FPU_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == FPU_A::YES
    }
}
impl core::ops::Deref for FPU_R {
    type Target = crate::FieldReader<bool, FPU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "USB Device.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<USB_A> for bool {
    #[inline(always)]
    fn from(variant: USB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB` reader - USB Device."]
pub struct USB_R(crate::FieldReader<bool, USB_A>);
impl USB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_A {
        match self.bits {
            false => USB_A::NO,
            true => USB_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == USB_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == USB_A::YES
    }
}
impl core::ops::Deref for USB_R {
    type Target = crate::FieldReader<bool, USB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "10-bit Sigma Delta ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<ADC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC` reader - 10-bit Sigma Delta ADC."]
pub struct ADC_R(crate::FieldReader<bool, ADC_A>);
impl ADC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_A {
        match self.bits {
            false => ADC_A::NO,
            true => ADC_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == ADC_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == ADC_A::YES
    }
}
impl core::ops::Deref for ADC_R {
    type Target = crate::FieldReader<bool, ADC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "XiP function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XIP_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<XIP_A> for bool {
    #[inline(always)]
    fn from(variant: XIP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XIP` reader - XiP function."]
pub struct XIP_R(crate::FieldReader<bool, XIP_A>);
impl XIP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XIP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XIP_A {
        match self.bits {
            false => XIP_A::NO,
            true => XIP_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == XIP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == XIP_A::YES
    }
}
impl core::ops::Deref for XIP_R {
    type Target = crate::FieldReader<bool, XIP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PBM function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBM_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<PBM_A> for bool {
    #[inline(always)]
    fn from(variant: PBM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBM` reader - PBM function."]
pub struct PBM_R(crate::FieldReader<bool, PBM_A>);
impl PBM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PBM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBM_A {
        match self.bits {
            false => PBM_A::NO,
            true => PBM_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == PBM_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == PBM_A::YES
    }
}
impl core::ops::Deref for PBM_R {
    type Target = crate::FieldReader<bool, PBM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HBC function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HBC_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<HBC_A> for bool {
    #[inline(always)]
    fn from(variant: HBC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HBC` reader - HBC function."]
pub struct HBC_R(crate::FieldReader<bool, HBC_A>);
impl HBC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HBC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HBC_A {
        match self.bits {
            false => HBC_A::NO,
            true => HBC_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == HBC_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == HBC_A::YES
    }
}
impl core::ops::Deref for HBC_R {
    type Target = crate::FieldReader<bool, HBC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SDHC function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDHC_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<SDHC_A> for bool {
    #[inline(always)]
    fn from(variant: SDHC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDHC` reader - SDHC function."]
pub struct SDHC_R(crate::FieldReader<bool, SDHC_A>);
impl SDHC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDHC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDHC_A {
        match self.bits {
            false => SDHC_A::NO,
            true => SDHC_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == SDHC_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == SDHC_A::YES
    }
}
impl core::ops::Deref for SDHC_R {
    type Target = crate::FieldReader<bool, SDHC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SMPHR function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPHR_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<SMPHR_A> for bool {
    #[inline(always)]
    fn from(variant: SMPHR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPHR` reader - SMPHR function."]
pub struct SMPHR_R(crate::FieldReader<bool, SMPHR_A>);
impl SMPHR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMPHR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPHR_A {
        match self.bits {
            false => SMPHR_A::NO,
            true => SMPHR_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == SMPHR_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == SMPHR_A::YES
    }
}
impl core::ops::Deref for SMPHR_R {
    type Target = crate::FieldReader<bool, SMPHR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "System Cache function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCACHE_A {
    #[doc = "0: `0`"]
    NO = 0,
    #[doc = "1: `1`"]
    YES = 1,
}
impl From<SCACHE_A> for bool {
    #[inline(always)]
    fn from(variant: SCACHE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCACHE` reader - System Cache function."]
pub struct SCACHE_R(crate::FieldReader<bool, SCACHE_A>);
impl SCACHE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCACHE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCACHE_A {
        match self.bits {
            false => SCACHE_A::NO,
            true => SCACHE_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == SCACHE_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == SCACHE_A::YES
    }
}
impl core::ops::Deref for SCACHE_R {
    type Target = crate::FieldReader<bool, SCACHE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - FPU Function."]
    #[inline(always)]
    pub fn fpu(&self) -> FPU_R {
        FPU_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB Device."]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 10-bit Sigma Delta ADC."]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - XiP function."]
    #[inline(always)]
    pub fn xip(&self) -> XIP_R {
        XIP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PBM function."]
    #[inline(always)]
    pub fn pbm(&self) -> PBM_R {
        PBM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HBC function."]
    #[inline(always)]
    pub fn hbc(&self) -> HBC_R {
        HBC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SDHC function."]
    #[inline(always)]
    pub fn sdhc(&self) -> SDHC_R {
        SDHC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SMPHR function."]
    #[inline(always)]
    pub fn smphr(&self) -> SMPHR_R {
        SMPHR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - System Cache function."]
    #[inline(always)]
    pub fn scache(&self) -> SCACHE_R {
        SCACHE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "funcstat register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fstat](index.html) module"]
pub struct FSTAT_SPEC;
impl crate::RegisterSpec for FSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fstat::R](R) reader structure"]
impl crate::Readable for FSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FSTAT to value 0"]
impl crate::Resettable for FSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
