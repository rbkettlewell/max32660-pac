#[doc = "Register `MEMCKCN` reader"]
pub struct R(crate::R<MEMCKCN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMCKCN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMCKCN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMCKCN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEMCKCN` writer"]
pub struct W(crate::W<MEMCKCN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMCKCN_SPEC>;
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
impl From<crate::W<MEMCKCN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMCKCN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FWS` reader - Flash Wait State. These bits define the number of wait-state cycles per Flash data read access. Minimum wait state is 2."]
pub struct FWS_R(crate::FieldReader<u8, u8>);
impl FWS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FWS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FWS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWS` writer - Flash Wait State. These bits define the number of wait-state cycles per Flash data read access. Minimum wait state is 2."]
pub struct FWS_W<'a> {
    w: &'a mut W,
}
impl<'a> FWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "System RAM 0 Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRAM0LS_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<SYSRAM0LS_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRAM0LS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSRAM0LS` reader - System RAM 0 Light Sleep Mode."]
pub struct SYSRAM0LS_R(crate::FieldReader<bool, SYSRAM0LS_A>);
impl SYSRAM0LS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSRAM0LS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRAM0LS_A {
        match self.bits {
            false => SYSRAM0LS_A::ACTIVE,
            true => SYSRAM0LS_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == SYSRAM0LS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `LIGHT_SLEEP`"]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        **self == SYSRAM0LS_A::LIGHT_SLEEP
    }
}
impl core::ops::Deref for SYSRAM0LS_R {
    type Target = crate::FieldReader<bool, SYSRAM0LS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSRAM0LS` writer - System RAM 0 Light Sleep Mode."]
pub struct SYSRAM0LS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRAM0LS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRAM0LS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(SYSRAM0LS_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut W {
        self.variant(SYSRAM0LS_A::LIGHT_SLEEP)
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
#[doc = "System RAM 1 Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRAM1LS_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<SYSRAM1LS_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRAM1LS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSRAM1LS` reader - System RAM 1 Light Sleep Mode."]
pub struct SYSRAM1LS_R(crate::FieldReader<bool, SYSRAM1LS_A>);
impl SYSRAM1LS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSRAM1LS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRAM1LS_A {
        match self.bits {
            false => SYSRAM1LS_A::ACTIVE,
            true => SYSRAM1LS_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == SYSRAM1LS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `LIGHT_SLEEP`"]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        **self == SYSRAM1LS_A::LIGHT_SLEEP
    }
}
impl core::ops::Deref for SYSRAM1LS_R {
    type Target = crate::FieldReader<bool, SYSRAM1LS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSRAM1LS` writer - System RAM 1 Light Sleep Mode."]
pub struct SYSRAM1LS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRAM1LS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRAM1LS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(SYSRAM1LS_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut W {
        self.variant(SYSRAM1LS_A::LIGHT_SLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "System RAM 2 Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRAM2LS_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<SYSRAM2LS_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRAM2LS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSRAM2LS` reader - System RAM 2 Light Sleep Mode."]
pub struct SYSRAM2LS_R(crate::FieldReader<bool, SYSRAM2LS_A>);
impl SYSRAM2LS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSRAM2LS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRAM2LS_A {
        match self.bits {
            false => SYSRAM2LS_A::ACTIVE,
            true => SYSRAM2LS_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == SYSRAM2LS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `LIGHT_SLEEP`"]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        **self == SYSRAM2LS_A::LIGHT_SLEEP
    }
}
impl core::ops::Deref for SYSRAM2LS_R {
    type Target = crate::FieldReader<bool, SYSRAM2LS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSRAM2LS` writer - System RAM 2 Light Sleep Mode."]
pub struct SYSRAM2LS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRAM2LS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRAM2LS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(SYSRAM2LS_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut W {
        self.variant(SYSRAM2LS_A::LIGHT_SLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "System RAM 3 Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRAM3LS_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<SYSRAM3LS_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRAM3LS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSRAM3LS` reader - System RAM 3 Light Sleep Mode."]
pub struct SYSRAM3LS_R(crate::FieldReader<bool, SYSRAM3LS_A>);
impl SYSRAM3LS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSRAM3LS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRAM3LS_A {
        match self.bits {
            false => SYSRAM3LS_A::ACTIVE,
            true => SYSRAM3LS_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == SYSRAM3LS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `LIGHT_SLEEP`"]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        **self == SYSRAM3LS_A::LIGHT_SLEEP
    }
}
impl core::ops::Deref for SYSRAM3LS_R {
    type Target = crate::FieldReader<bool, SYSRAM3LS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSRAM3LS` writer - System RAM 3 Light Sleep Mode."]
pub struct SYSRAM3LS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRAM3LS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRAM3LS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(SYSRAM3LS_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut W {
        self.variant(SYSRAM3LS_A::LIGHT_SLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "ICache RAM Light Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICACHELS_A {
    #[doc = "0: Memory is active."]
    ACTIVE = 0,
    #[doc = "1: Memory is in Light Sleep mode."]
    LIGHT_SLEEP = 1,
}
impl From<ICACHELS_A> for bool {
    #[inline(always)]
    fn from(variant: ICACHELS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICACHELS` reader - ICache RAM Light Sleep Mode."]
pub struct ICACHELS_R(crate::FieldReader<bool, ICACHELS_A>);
impl ICACHELS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICACHELS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICACHELS_A {
        match self.bits {
            false => ICACHELS_A::ACTIVE,
            true => ICACHELS_A::LIGHT_SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == ICACHELS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `LIGHT_SLEEP`"]
    #[inline(always)]
    pub fn is_light_sleep(&self) -> bool {
        **self == ICACHELS_A::LIGHT_SLEEP
    }
}
impl core::ops::Deref for ICACHELS_R {
    type Target = crate::FieldReader<bool, ICACHELS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHELS` writer - ICache RAM Light Sleep Mode."]
pub struct ICACHELS_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHELS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICACHELS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Memory is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(ICACHELS_A::ACTIVE)
    }
    #[doc = "Memory is in Light Sleep mode."]
    #[inline(always)]
    pub fn light_sleep(self) -> &'a mut W {
        self.variant(ICACHELS_A::LIGHT_SLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Flash Wait State. These bits define the number of wait-state cycles per Flash data read access. Minimum wait state is 2."]
    #[inline(always)]
    pub fn fws(&self) -> FWS_R {
        FWS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - System RAM 0 Light Sleep Mode."]
    #[inline(always)]
    pub fn sysram0ls(&self) -> SYSRAM0LS_R {
        SYSRAM0LS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - System RAM 1 Light Sleep Mode."]
    #[inline(always)]
    pub fn sysram1ls(&self) -> SYSRAM1LS_R {
        SYSRAM1LS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - System RAM 2 Light Sleep Mode."]
    #[inline(always)]
    pub fn sysram2ls(&self) -> SYSRAM2LS_R {
        SYSRAM2LS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - System RAM 3 Light Sleep Mode."]
    #[inline(always)]
    pub fn sysram3ls(&self) -> SYSRAM3LS_R {
        SYSRAM3LS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ICache RAM Light Sleep Mode."]
    #[inline(always)]
    pub fn icachels(&self) -> ICACHELS_R {
        ICACHELS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Flash Wait State. These bits define the number of wait-state cycles per Flash data read access. Minimum wait state is 2."]
    #[inline(always)]
    pub fn fws(&mut self) -> FWS_W {
        FWS_W { w: self }
    }
    #[doc = "Bit 8 - System RAM 0 Light Sleep Mode."]
    #[inline(always)]
    pub fn sysram0ls(&mut self) -> SYSRAM0LS_W {
        SYSRAM0LS_W { w: self }
    }
    #[doc = "Bit 9 - System RAM 1 Light Sleep Mode."]
    #[inline(always)]
    pub fn sysram1ls(&mut self) -> SYSRAM1LS_W {
        SYSRAM1LS_W { w: self }
    }
    #[doc = "Bit 10 - System RAM 2 Light Sleep Mode."]
    #[inline(always)]
    pub fn sysram2ls(&mut self) -> SYSRAM2LS_W {
        SYSRAM2LS_W { w: self }
    }
    #[doc = "Bit 11 - System RAM 3 Light Sleep Mode."]
    #[inline(always)]
    pub fn sysram3ls(&mut self) -> SYSRAM3LS_W {
        SYSRAM3LS_W { w: self }
    }
    #[doc = "Bit 12 - ICache RAM Light Sleep Mode."]
    #[inline(always)]
    pub fn icachels(&mut self) -> ICACHELS_W {
        ICACHELS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Clock Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memckcn](index.html) module"]
pub struct MEMCKCN_SPEC;
impl crate::RegisterSpec for MEMCKCN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [memckcn::R](R) reader structure"]
impl crate::Readable for MEMCKCN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [memckcn::W](W) writer structure"]
impl crate::Writable for MEMCKCN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEMCKCN to value 0"]
impl crate::Resettable for MEMCKCN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
