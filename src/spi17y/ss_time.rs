#[doc = "Register `SS_TIME` reader"]
pub struct R(crate::R<SS_TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SS_TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SS_TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SS_TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SS_TIME` writer"]
pub struct W(crate::W<SS_TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SS_TIME_SPEC>;
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
impl From<crate::W<SS_TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SS_TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Slave Select Pre delay 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRE_A {
    #[doc = "0: 256 system clocks between SS active and first serial clock edge."]
    _256 = 0,
}
impl From<PRE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRE` reader - Slave Select Pre delay 1."]
pub struct PRE_R(crate::FieldReader<u8, PRE_A>);
impl PRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRE_A> {
        match self.bits {
            0 => Some(PRE_A::_256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        **self == PRE_A::_256
    }
}
impl core::ops::Deref for PRE_R {
    type Target = crate::FieldReader<u8, PRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRE` writer - Slave Select Pre delay 1."]
pub struct PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "256 system clocks between SS active and first serial clock edge."]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(PRE_A::_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Slave Select Post delay 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POST_A {
    #[doc = "0: 256 system clocks between last serial clock edge and SS inactive."]
    _256 = 0,
}
impl From<POST_A> for u8 {
    #[inline(always)]
    fn from(variant: POST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `POST` reader - Slave Select Post delay 2."]
pub struct POST_R(crate::FieldReader<u8, POST_A>);
impl POST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        POST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<POST_A> {
        match self.bits {
            0 => Some(POST_A::_256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        **self == POST_A::_256
    }
}
impl core::ops::Deref for POST_R {
    type Target = crate::FieldReader<u8, POST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POST` writer - Slave Select Post delay 2."]
pub struct POST_W<'a> {
    w: &'a mut W,
}
impl<'a> POST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POST_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "256 system clocks between last serial clock edge and SS inactive."]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(POST_A::_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Slave Select Inactive delay.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INACT_A {
    #[doc = "0: 256 system clocks between transactions."]
    _256 = 0,
}
impl From<INACT_A> for u8 {
    #[inline(always)]
    fn from(variant: INACT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INACT` reader - Slave Select Inactive delay."]
pub struct INACT_R(crate::FieldReader<u8, INACT_A>);
impl INACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INACT_A> {
        match self.bits {
            0 => Some(INACT_A::_256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        **self == INACT_A::_256
    }
}
impl core::ops::Deref for INACT_R {
    type Target = crate::FieldReader<u8, INACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INACT` writer - Slave Select Inactive delay."]
pub struct INACT_W<'a> {
    w: &'a mut W,
}
impl<'a> INACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INACT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "256 system clocks between transactions."]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(INACT_A::_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Slave Select Pre delay 1."]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Slave Select Post delay 2."]
    #[inline(always)]
    pub fn post(&self) -> POST_R {
        POST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Slave Select Inactive delay."]
    #[inline(always)]
    pub fn inact(&self) -> INACT_R {
        INACT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slave Select Pre delay 1."]
    #[inline(always)]
    pub fn pre(&mut self) -> PRE_W {
        PRE_W { w: self }
    }
    #[doc = "Bits 8:15 - Slave Select Post delay 2."]
    #[inline(always)]
    pub fn post(&mut self) -> POST_W {
        POST_W { w: self }
    }
    #[doc = "Bits 16:23 - Slave Select Inactive delay."]
    #[inline(always)]
    pub fn inact(&mut self) -> INACT_W {
        INACT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for controlling SPI peripheral/Slave Select Timing.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ss_time](index.html) module"]
pub struct SS_TIME_SPEC;
impl crate::RegisterSpec for SS_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ss_time::R](R) reader structure"]
impl crate::Readable for SS_TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ss_time::W](W) writer structure"]
impl crate::Writable for SS_TIME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SS_TIME to value 0"]
impl crate::Resettable for SS_TIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
