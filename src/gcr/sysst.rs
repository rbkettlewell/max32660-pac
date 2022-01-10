#[doc = "Register `SYSST` reader"]
pub struct R(crate::R<SYSST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSST` writer"]
pub struct W(crate::W<SYSST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSST_SPEC>;
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
impl From<crate::W<SYSST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ARM ICE Lock Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICECLOCK_A {
    #[doc = "0: ICE is unlocked."]
    UNLOCKED = 0,
    #[doc = "1: ICE is locked."]
    LOCKED = 1,
}
impl From<ICECLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: ICECLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICECLOCK` reader - ARM ICE Lock Status."]
pub struct ICECLOCK_R(crate::FieldReader<bool, ICECLOCK_A>);
impl ICECLOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICECLOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICECLOCK_A {
        match self.bits {
            false => ICECLOCK_A::UNLOCKED,
            true => ICECLOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == ICECLOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == ICECLOCK_A::LOCKED
    }
}
impl core::ops::Deref for ICECLOCK_R {
    type Target = crate::FieldReader<bool, ICECLOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICECLOCK` writer - ARM ICE Lock Status."]
pub struct ICECLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ICECLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICECLOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ICE is unlocked."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(ICECLOCK_A::UNLOCKED)
    }
    #[doc = "ICE is locked."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(ICECLOCK_A::LOCKED)
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
#[doc = "Code Integrity Error Flag. This bit indicates a code integrity error has occured in XiP interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CODEINTERR_A {
    #[doc = "0: Normal Operating Condition."]
    NORM = 0,
    #[doc = "1: Code Integrity Error."]
    CODE = 1,
}
impl From<CODEINTERR_A> for bool {
    #[inline(always)]
    fn from(variant: CODEINTERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CODEINTERR` reader - Code Integrity Error Flag. This bit indicates a code integrity error has occured in XiP interface."]
pub struct CODEINTERR_R(crate::FieldReader<bool, CODEINTERR_A>);
impl CODEINTERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CODEINTERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CODEINTERR_A {
        match self.bits {
            false => CODEINTERR_A::NORM,
            true => CODEINTERR_A::CODE,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        **self == CODEINTERR_A::NORM
    }
    #[doc = "Checks if the value of the field is `CODE`"]
    #[inline(always)]
    pub fn is_code(&self) -> bool {
        **self == CODEINTERR_A::CODE
    }
}
impl core::ops::Deref for CODEINTERR_R {
    type Target = crate::FieldReader<bool, CODEINTERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CODEINTERR` writer - Code Integrity Error Flag. This bit indicates a code integrity error has occured in XiP interface."]
pub struct CODEINTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CODEINTERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CODEINTERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal Operating Condition."]
    #[inline(always)]
    pub fn norm(self) -> &'a mut W {
        self.variant(CODEINTERR_A::NORM)
    }
    #[doc = "Code Integrity Error."]
    #[inline(always)]
    pub fn code(self) -> &'a mut W {
        self.variant(CODEINTERR_A::CODE)
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
#[doc = "System Cache Memory Fault Flag. This bit indicates a memory fault has occured in the System Cache while receiving data from the Hyperbus Interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCMEMF_A {
    #[doc = "0: Normal Operating Condition."]
    NORM = 0,
    #[doc = "1: Memory Fault."]
    MEMORY = 1,
}
impl From<SCMEMF_A> for bool {
    #[inline(always)]
    fn from(variant: SCMEMF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCMEMF` reader - System Cache Memory Fault Flag. This bit indicates a memory fault has occured in the System Cache while receiving data from the Hyperbus Interface."]
pub struct SCMEMF_R(crate::FieldReader<bool, SCMEMF_A>);
impl SCMEMF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCMEMF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCMEMF_A {
        match self.bits {
            false => SCMEMF_A::NORM,
            true => SCMEMF_A::MEMORY,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        **self == SCMEMF_A::NORM
    }
    #[doc = "Checks if the value of the field is `MEMORY`"]
    #[inline(always)]
    pub fn is_memory(&self) -> bool {
        **self == SCMEMF_A::MEMORY
    }
}
impl core::ops::Deref for SCMEMF_R {
    type Target = crate::FieldReader<bool, SCMEMF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCMEMF` writer - System Cache Memory Fault Flag. This bit indicates a memory fault has occured in the System Cache while receiving data from the Hyperbus Interface."]
pub struct SCMEMF_W<'a> {
    w: &'a mut W,
}
impl<'a> SCMEMF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCMEMF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal Operating Condition."]
    #[inline(always)]
    pub fn norm(self) -> &'a mut W {
        self.variant(SCMEMF_A::NORM)
    }
    #[doc = "Memory Fault."]
    #[inline(always)]
    pub fn memory(self) -> &'a mut W {
        self.variant(SCMEMF_A::MEMORY)
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
    #[doc = "Bit 0 - ARM ICE Lock Status."]
    #[inline(always)]
    pub fn iceclock(&self) -> ICECLOCK_R {
        ICECLOCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Code Integrity Error Flag. This bit indicates a code integrity error has occured in XiP interface."]
    #[inline(always)]
    pub fn codeinterr(&self) -> CODEINTERR_R {
        CODEINTERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - System Cache Memory Fault Flag. This bit indicates a memory fault has occured in the System Cache while receiving data from the Hyperbus Interface."]
    #[inline(always)]
    pub fn scmemf(&self) -> SCMEMF_R {
        SCMEMF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ARM ICE Lock Status."]
    #[inline(always)]
    pub fn iceclock(&mut self) -> ICECLOCK_W {
        ICECLOCK_W { w: self }
    }
    #[doc = "Bit 1 - Code Integrity Error Flag. This bit indicates a code integrity error has occured in XiP interface."]
    #[inline(always)]
    pub fn codeinterr(&mut self) -> CODEINTERR_W {
        CODEINTERR_W { w: self }
    }
    #[doc = "Bit 5 - System Cache Memory Fault Flag. This bit indicates a memory fault has occured in the System Cache while receiving data from the Hyperbus Interface."]
    #[inline(always)]
    pub fn scmemf(&mut self) -> SCMEMF_W {
        SCMEMF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysst](index.html) module"]
pub struct SYSST_SPEC;
impl crate::RegisterSpec for SYSST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysst::R](R) reader structure"]
impl crate::Readable for SYSST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysst::W](W) writer structure"]
impl crate::Writable for SYSST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSST to value 0"]
impl crate::Resettable for SYSST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
