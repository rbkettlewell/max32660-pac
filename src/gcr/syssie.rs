#[doc = "Register `SYSSIE` reader"]
pub struct R(crate::R<SYSSIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSSIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSSIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSSIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSSIE` writer"]
pub struct W(crate::W<SYSSIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSSIE_SPEC>;
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
impl From<crate::W<SYSSIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSSIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ARM ICE Unlock Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICEULIE_A {
    #[doc = "0: disabled."]
    DIS = 0,
    #[doc = "1: enabled."]
    EN = 1,
}
impl From<ICEULIE_A> for bool {
    #[inline(always)]
    fn from(variant: ICEULIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICEULIE` reader - ARM ICE Unlock Interrupt Enable."]
pub struct ICEULIE_R(crate::FieldReader<bool, ICEULIE_A>);
impl ICEULIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICEULIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICEULIE_A {
        match self.bits {
            false => ICEULIE_A::DIS,
            true => ICEULIE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == ICEULIE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == ICEULIE_A::EN
    }
}
impl core::ops::Deref for ICEULIE_R {
    type Target = crate::FieldReader<bool, ICEULIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICEULIE` writer - ARM ICE Unlock Interrupt Enable."]
pub struct ICEULIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEULIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICEULIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ICEULIE_A::DIS)
    }
    #[doc = "enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ICEULIE_A::EN)
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
#[doc = "Code Integrity Error Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIEIE_A {
    #[doc = "0: disabled."]
    DIS = 0,
    #[doc = "1: enabled."]
    EN = 1,
}
impl From<CIEIE_A> for bool {
    #[inline(always)]
    fn from(variant: CIEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIEIE` reader - Code Integrity Error Interrupt Enable."]
pub struct CIEIE_R(crate::FieldReader<bool, CIEIE_A>);
impl CIEIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CIEIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIEIE_A {
        match self.bits {
            false => CIEIE_A::DIS,
            true => CIEIE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == CIEIE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == CIEIE_A::EN
    }
}
impl core::ops::Deref for CIEIE_R {
    type Target = crate::FieldReader<bool, CIEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CIEIE` writer - Code Integrity Error Interrupt Enable."]
pub struct CIEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CIEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CIEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CIEIE_A::DIS)
    }
    #[doc = "enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CIEIE_A::EN)
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
#[doc = "System Cache Memory Fault Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCMFIE_A {
    #[doc = "0: disabled."]
    DIS = 0,
    #[doc = "1: enabled."]
    EN = 1,
}
impl From<SCMFIE_A> for bool {
    #[inline(always)]
    fn from(variant: SCMFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCMFIE` reader - System Cache Memory Fault Interrupt Enable."]
pub struct SCMFIE_R(crate::FieldReader<bool, SCMFIE_A>);
impl SCMFIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCMFIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCMFIE_A {
        match self.bits {
            false => SCMFIE_A::DIS,
            true => SCMFIE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == SCMFIE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == SCMFIE_A::EN
    }
}
impl core::ops::Deref for SCMFIE_R {
    type Target = crate::FieldReader<bool, SCMFIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCMFIE` writer - System Cache Memory Fault Interrupt Enable."]
pub struct SCMFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCMFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCMFIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SCMFIE_A::DIS)
    }
    #[doc = "enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SCMFIE_A::EN)
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
impl R {
    #[doc = "Bit 0 - ARM ICE Unlock Interrupt Enable."]
    #[inline(always)]
    pub fn iceulie(&self) -> ICEULIE_R {
        ICEULIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Code Integrity Error Interrupt Enable."]
    #[inline(always)]
    pub fn cieie(&self) -> CIEIE_R {
        CIEIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - System Cache Memory Fault Interrupt Enable."]
    #[inline(always)]
    pub fn scmfie(&self) -> SCMFIE_R {
        SCMFIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ARM ICE Unlock Interrupt Enable."]
    #[inline(always)]
    pub fn iceulie(&mut self) -> ICEULIE_W {
        ICEULIE_W { w: self }
    }
    #[doc = "Bit 1 - Code Integrity Error Interrupt Enable."]
    #[inline(always)]
    pub fn cieie(&mut self) -> CIEIE_W {
        CIEIE_W { w: self }
    }
    #[doc = "Bit 5 - System Cache Memory Fault Interrupt Enable."]
    #[inline(always)]
    pub fn scmfie(&mut self) -> SCMFIE_W {
        SCMFIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Status Interrupt Enable Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syssie](index.html) module"]
pub struct SYSSIE_SPEC;
impl crate::RegisterSpec for SYSSIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syssie::R](R) reader structure"]
impl crate::Readable for SYSSIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syssie::W](W) writer structure"]
impl crate::Writable for SYSSIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSSIE to value 0"]
impl crate::Resettable for SYSSIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
