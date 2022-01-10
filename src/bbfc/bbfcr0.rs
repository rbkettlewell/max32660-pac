#[doc = "Register `BBFCR0` reader"]
pub struct R(crate::R<BBFCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BBFCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BBFCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BBFCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BBFCR0` writer"]
pub struct W(crate::W<BBFCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BBFCR0_SPEC>;
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
impl From<crate::W<BBFCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BBFCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKPDRV` reader - Hyperbus CK Pad Driver Control."]
pub struct CKPDRV_R(crate::FieldReader<u8, u8>);
impl CKPDRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CKPDRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKPDRV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKPDRV` writer - Hyperbus CK Pad Driver Control."]
pub struct CKPDRV_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPDRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `CKNPDRV` reader - Hyperbus CKN Pad Driver Control."]
pub struct CKNPDRV_R(crate::FieldReader<u8, u8>);
impl CKNPDRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CKNPDRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKNPDRV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKNPDRV` writer - Hyperbus CKN Pad Driver Control."]
pub struct CKNPDRV_W<'a> {
    w: &'a mut W,
}
impl<'a> CKNPDRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Hyperbus RDS DLL Power Up Control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDSDLLEN_A {
    #[doc = "0: Disabled."]
    DIS = 0,
    #[doc = "1: Enabled."]
    EN = 1,
}
impl From<RDSDLLEN_A> for bool {
    #[inline(always)]
    fn from(variant: RDSDLLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDSDLLEN` reader - Hyperbus RDS DLL Power Up Control."]
pub struct RDSDLLEN_R(crate::FieldReader<bool, RDSDLLEN_A>);
impl RDSDLLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDSDLLEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDSDLLEN_A {
        match self.bits {
            false => RDSDLLEN_A::DIS,
            true => RDSDLLEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RDSDLLEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == RDSDLLEN_A::EN
    }
}
impl core::ops::Deref for RDSDLLEN_R {
    type Target = crate::FieldReader<bool, RDSDLLEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDSDLLEN` writer - Hyperbus RDS DLL Power Up Control."]
pub struct RDSDLLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RDSDLLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDSDLLEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RDSDLLEN_A::DIS)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RDSDLLEN_A::EN)
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
impl R {
    #[doc = "Bits 0:3 - Hyperbus CK Pad Driver Control."]
    #[inline(always)]
    pub fn ckpdrv(&self) -> CKPDRV_R {
        CKPDRV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Hyperbus CKN Pad Driver Control."]
    #[inline(always)]
    pub fn cknpdrv(&self) -> CKNPDRV_R {
        CKNPDRV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Hyperbus RDS DLL Power Up Control."]
    #[inline(always)]
    pub fn rdsdllen(&self) -> RDSDLLEN_R {
        RDSDLLEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Hyperbus CK Pad Driver Control."]
    #[inline(always)]
    pub fn ckpdrv(&mut self) -> CKPDRV_W {
        CKPDRV_W { w: self }
    }
    #[doc = "Bits 4:7 - Hyperbus CKN Pad Driver Control."]
    #[inline(always)]
    pub fn cknpdrv(&mut self) -> CKNPDRV_W {
        CKNPDRV_W { w: self }
    }
    #[doc = "Bit 8 - Hyperbus RDS DLL Power Up Control."]
    #[inline(always)]
    pub fn rdsdllen(&mut self) -> RDSDLLEN_W {
        RDSDLLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Function Control Register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bbfcr0](index.html) module"]
pub struct BBFCR0_SPEC;
impl crate::RegisterSpec for BBFCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bbfcr0::R](R) reader structure"]
impl crate::Readable for BBFCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bbfcr0::W](W) writer structure"]
impl crate::Writable for BBFCR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BBFCR0 to value 0"]
impl crate::Resettable for BBFCR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
