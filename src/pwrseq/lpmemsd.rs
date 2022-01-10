#[doc = "Register `LPMEMSD` reader"]
pub struct R(crate::R<LPMEMSD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMEMSD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMEMSD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMEMSD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMEMSD` writer"]
pub struct W(crate::W<LPMEMSD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMEMSD_SPEC>;
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
impl From<crate::W<LPMEMSD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMEMSD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "System RAM block 0 Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM0_OFF_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<SRAM0_OFF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM0_OFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM0_OFF` reader - System RAM block 0 Shut Down."]
pub struct SRAM0_OFF_R(crate::FieldReader<bool, SRAM0_OFF_A>);
impl SRAM0_OFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM0_OFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM0_OFF_A {
        match self.bits {
            false => SRAM0_OFF_A::NORMAL,
            true => SRAM0_OFF_A::SHUTDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == SRAM0_OFF_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        **self == SRAM0_OFF_A::SHUTDOWN
    }
}
impl core::ops::Deref for SRAM0_OFF_R {
    type Target = crate::FieldReader<bool, SRAM0_OFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM0_OFF` writer - System RAM block 0 Shut Down."]
pub struct SRAM0_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM0_OFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM0_OFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SRAM0_OFF_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(SRAM0_OFF_A::SHUTDOWN)
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
#[doc = "System RAM block 1 Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM1_OFF_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<SRAM1_OFF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM1_OFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM1_OFF` reader - System RAM block 1 Shut Down."]
pub struct SRAM1_OFF_R(crate::FieldReader<bool, SRAM1_OFF_A>);
impl SRAM1_OFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM1_OFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM1_OFF_A {
        match self.bits {
            false => SRAM1_OFF_A::NORMAL,
            true => SRAM1_OFF_A::SHUTDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == SRAM1_OFF_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        **self == SRAM1_OFF_A::SHUTDOWN
    }
}
impl core::ops::Deref for SRAM1_OFF_R {
    type Target = crate::FieldReader<bool, SRAM1_OFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM1_OFF` writer - System RAM block 1 Shut Down."]
pub struct SRAM1_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1_OFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM1_OFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SRAM1_OFF_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(SRAM1_OFF_A::SHUTDOWN)
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
#[doc = "System RAM block 2 Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM2_OFF_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<SRAM2_OFF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2_OFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM2_OFF` reader - System RAM block 2 Shut Down."]
pub struct SRAM2_OFF_R(crate::FieldReader<bool, SRAM2_OFF_A>);
impl SRAM2_OFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM2_OFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM2_OFF_A {
        match self.bits {
            false => SRAM2_OFF_A::NORMAL,
            true => SRAM2_OFF_A::SHUTDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == SRAM2_OFF_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        **self == SRAM2_OFF_A::SHUTDOWN
    }
}
impl core::ops::Deref for SRAM2_OFF_R {
    type Target = crate::FieldReader<bool, SRAM2_OFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM2_OFF` writer - System RAM block 2 Shut Down."]
pub struct SRAM2_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2_OFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM2_OFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SRAM2_OFF_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(SRAM2_OFF_A::SHUTDOWN)
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
#[doc = "System RAM block 3 Shut Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM3_OFF_A {
    #[doc = "0: Normal Operating Mode."]
    NORMAL = 0,
    #[doc = "1: Shutdown Mode."]
    SHUTDOWN = 1,
}
impl From<SRAM3_OFF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM3_OFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM3_OFF` reader - System RAM block 3 Shut Down."]
pub struct SRAM3_OFF_R(crate::FieldReader<bool, SRAM3_OFF_A>);
impl SRAM3_OFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM3_OFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM3_OFF_A {
        match self.bits {
            false => SRAM3_OFF_A::NORMAL,
            true => SRAM3_OFF_A::SHUTDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == SRAM3_OFF_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        **self == SRAM3_OFF_A::SHUTDOWN
    }
}
impl core::ops::Deref for SRAM3_OFF_R {
    type Target = crate::FieldReader<bool, SRAM3_OFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM3_OFF` writer - System RAM block 3 Shut Down."]
pub struct SRAM3_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM3_OFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM3_OFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal Operating Mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SRAM3_OFF_A::NORMAL)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(SRAM3_OFF_A::SHUTDOWN)
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
impl R {
    #[doc = "Bit 0 - System RAM block 0 Shut Down."]
    #[inline(always)]
    pub fn sram0_off(&self) -> SRAM0_OFF_R {
        SRAM0_OFF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - System RAM block 1 Shut Down."]
    #[inline(always)]
    pub fn sram1_off(&self) -> SRAM1_OFF_R {
        SRAM1_OFF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - System RAM block 2 Shut Down."]
    #[inline(always)]
    pub fn sram2_off(&self) -> SRAM2_OFF_R {
        SRAM2_OFF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - System RAM block 3 Shut Down."]
    #[inline(always)]
    pub fn sram3_off(&self) -> SRAM3_OFF_R {
        SRAM3_OFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System RAM block 0 Shut Down."]
    #[inline(always)]
    pub fn sram0_off(&mut self) -> SRAM0_OFF_W {
        SRAM0_OFF_W { w: self }
    }
    #[doc = "Bit 1 - System RAM block 1 Shut Down."]
    #[inline(always)]
    pub fn sram1_off(&mut self) -> SRAM1_OFF_W {
        SRAM1_OFF_W { w: self }
    }
    #[doc = "Bit 2 - System RAM block 2 Shut Down."]
    #[inline(always)]
    pub fn sram2_off(&mut self) -> SRAM2_OFF_W {
        SRAM2_OFF_W { w: self }
    }
    #[doc = "Bit 3 - System RAM block 3 Shut Down."]
    #[inline(always)]
    pub fn sram3_off(&mut self) -> SRAM3_OFF_W {
        SRAM3_OFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Power Memory Shutdown Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmemsd](index.html) module"]
pub struct LPMEMSD_SPEC;
impl crate::RegisterSpec for LPMEMSD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpmemsd::R](R) reader structure"]
impl crate::Readable for LPMEMSD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmemsd::W](W) writer structure"]
impl crate::Writable for LPMEMSD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPMEMSD to value 0"]
impl crate::Resettable for LPMEMSD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
