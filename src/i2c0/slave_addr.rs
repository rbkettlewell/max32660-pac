#[doc = "Register `SLAVE_ADDR` reader"]
pub struct R(crate::R<SLAVE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLAVE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLAVE_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLAVE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLAVE_ADDR` writer"]
pub struct W(crate::W<SLAVE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLAVE_ADDR_SPEC>;
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
impl From<crate::W<SLAVE_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLAVE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLAVE_ADDR` reader - Slave Address."]
pub struct SLAVE_ADDR_R(crate::FieldReader<u16, u16>);
impl SLAVE_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SLAVE_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_ADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_ADDR` writer - Slave Address."]
pub struct SLAVE_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `SLAVE_ADDR_DIS` reader - Slave Address DIS."]
pub struct SLAVE_ADDR_DIS_R(crate::FieldReader<bool, bool>);
impl SLAVE_ADDR_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_ADDR_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_ADDR_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_ADDR_DIS` writer - Slave Address DIS."]
pub struct SLAVE_ADDR_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_ADDR_DIS_W<'a> {
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
#[doc = "Field `SLAVE_ADDR_IDX` reader - Slave Address Index."]
pub struct SLAVE_ADDR_IDX_R(crate::FieldReader<u8, u8>);
impl SLAVE_ADDR_IDX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLAVE_ADDR_IDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_ADDR_IDX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_ADDR_IDX` writer - Slave Address Index."]
pub struct SLAVE_ADDR_IDX_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_ADDR_IDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | ((value as u32 & 0x0f) << 11);
        self.w
    }
}
#[doc = "Extended Address Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EX_ADDR_A {
    #[doc = "0: 7-bit address."]
    _7_BITS_ADDRESS = 0,
    #[doc = "1: 10-bit address."]
    _10_BITS_ADDRESS = 1,
}
impl From<EX_ADDR_A> for bool {
    #[inline(always)]
    fn from(variant: EX_ADDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EX_ADDR` reader - Extended Address Select."]
pub struct EX_ADDR_R(crate::FieldReader<bool, EX_ADDR_A>);
impl EX_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EX_ADDR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EX_ADDR_A {
        match self.bits {
            false => EX_ADDR_A::_7_BITS_ADDRESS,
            true => EX_ADDR_A::_10_BITS_ADDRESS,
        }
    }
    #[doc = "Checks if the value of the field is `_7_BITS_ADDRESS`"]
    #[inline(always)]
    pub fn is_7_bits_address(&self) -> bool {
        **self == EX_ADDR_A::_7_BITS_ADDRESS
    }
    #[doc = "Checks if the value of the field is `_10_BITS_ADDRESS`"]
    #[inline(always)]
    pub fn is_10_bits_address(&self) -> bool {
        **self == EX_ADDR_A::_10_BITS_ADDRESS
    }
}
impl core::ops::Deref for EX_ADDR_R {
    type Target = crate::FieldReader<bool, EX_ADDR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EX_ADDR` writer - Extended Address Select."]
pub struct EX_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> EX_ADDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EX_ADDR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "7-bit address."]
    #[inline(always)]
    pub fn _7_bits_address(self) -> &'a mut W {
        self.variant(EX_ADDR_A::_7_BITS_ADDRESS)
    }
    #[doc = "10-bit address."]
    #[inline(always)]
    pub fn _10_bits_address(self) -> &'a mut W {
        self.variant(EX_ADDR_A::_10_BITS_ADDRESS)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Slave Address."]
    #[inline(always)]
    pub fn slave_addr(&self) -> SLAVE_ADDR_R {
        SLAVE_ADDR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Slave Address DIS."]
    #[inline(always)]
    pub fn slave_addr_dis(&self) -> SLAVE_ADDR_DIS_R {
        SLAVE_ADDR_DIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:14 - Slave Address Index."]
    #[inline(always)]
    pub fn slave_addr_idx(&self) -> SLAVE_ADDR_IDX_R {
        SLAVE_ADDR_IDX_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Extended Address Select."]
    #[inline(always)]
    pub fn ex_addr(&self) -> EX_ADDR_R {
        EX_ADDR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave Address."]
    #[inline(always)]
    pub fn slave_addr(&mut self) -> SLAVE_ADDR_W {
        SLAVE_ADDR_W { w: self }
    }
    #[doc = "Bit 10 - Slave Address DIS."]
    #[inline(always)]
    pub fn slave_addr_dis(&mut self) -> SLAVE_ADDR_DIS_W {
        SLAVE_ADDR_DIS_W { w: self }
    }
    #[doc = "Bits 11:14 - Slave Address Index."]
    #[inline(always)]
    pub fn slave_addr_idx(&mut self) -> SLAVE_ADDR_IDX_W {
        SLAVE_ADDR_IDX_W { w: self }
    }
    #[doc = "Bit 15 - Extended Address Select."]
    #[inline(always)]
    pub fn ex_addr(&mut self) -> EX_ADDR_W {
        EX_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Address Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave_addr](index.html) module"]
pub struct SLAVE_ADDR_SPEC;
impl crate::RegisterSpec for SLAVE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slave_addr::R](R) reader structure"]
impl crate::Readable for SLAVE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slave_addr::W](W) writer structure"]
impl crate::Writable for SLAVE_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLAVE_ADDR to value 0"]
impl crate::Resettable for SLAVE_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
