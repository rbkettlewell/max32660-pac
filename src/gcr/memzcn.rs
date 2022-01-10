#[doc = "Register `MEMZCN` reader"]
pub struct R(crate::R<MEMZCN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMZCN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMZCN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMZCN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEMZCN` writer"]
pub struct W(crate::W<MEMZCN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMZCN_SPEC>;
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
impl From<crate::W<MEMZCN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMZCN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "System RAM Block 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM0Z_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<SRAM0Z_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM0Z_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM0Z` reader - System RAM Block 0."]
pub struct SRAM0Z_R(crate::FieldReader<bool, SRAM0Z_A>);
impl SRAM0Z_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM0Z_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM0Z_A {
        match self.bits {
            false => SRAM0Z_A::NOP,
            true => SRAM0Z_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        **self == SRAM0Z_A::NOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == SRAM0Z_A::START
    }
}
impl core::ops::Deref for SRAM0Z_R {
    type Target = crate::FieldReader<bool, SRAM0Z_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM0Z` writer - System RAM Block 0."]
pub struct SRAM0Z_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM0Z_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM0Z_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(SRAM0Z_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SRAM0Z_A::START)
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
#[doc = "Instruction Cache.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICACHEZ_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<ICACHEZ_A> for bool {
    #[inline(always)]
    fn from(variant: ICACHEZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICACHEZ` reader - Instruction Cache."]
pub struct ICACHEZ_R(crate::FieldReader<bool, ICACHEZ_A>);
impl ICACHEZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICACHEZ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICACHEZ_A {
        match self.bits {
            false => ICACHEZ_A::NOP,
            true => ICACHEZ_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        **self == ICACHEZ_A::NOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == ICACHEZ_A::START
    }
}
impl core::ops::Deref for ICACHEZ_R {
    type Target = crate::FieldReader<bool, ICACHEZ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHEZ` writer - Instruction Cache."]
pub struct ICACHEZ_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHEZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICACHEZ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(ICACHEZ_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(ICACHEZ_A::START)
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
impl R {
    #[doc = "Bit 0 - System RAM Block 0."]
    #[inline(always)]
    pub fn sram0z(&self) -> SRAM0Z_R {
        SRAM0Z_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Instruction Cache."]
    #[inline(always)]
    pub fn icachez(&self) -> ICACHEZ_R {
        ICACHEZ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System RAM Block 0."]
    #[inline(always)]
    pub fn sram0z(&mut self) -> SRAM0Z_W {
        SRAM0Z_W { w: self }
    }
    #[doc = "Bit 1 - Instruction Cache."]
    #[inline(always)]
    pub fn icachez(&mut self) -> ICACHEZ_W {
        ICACHEZ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Zeroize Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memzcn](index.html) module"]
pub struct MEMZCN_SPEC;
impl crate::RegisterSpec for MEMZCN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [memzcn::R](R) reader structure"]
impl crate::Readable for MEMZCN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [memzcn::W](W) writer structure"]
impl crate::Writable for MEMZCN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEMZCN to value 0"]
impl crate::Resettable for MEMZCN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
