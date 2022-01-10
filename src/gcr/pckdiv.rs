#[doc = "Register `PCKDIV` reader"]
pub struct R(crate::R<PCKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCKDIV` writer"]
pub struct W(crate::W<PCKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCKDIV_SPEC>;
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
impl From<crate::W<PCKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Always-ON(AON) domain CLock Divider. These bits define the AON domain clock divider.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AONCD_A {
    #[doc = "0: PCLK divide by 4."]
    DIV_4 = 0,
    #[doc = "1: PCLK divide by 8."]
    DIV_8 = 1,
    #[doc = "2: PCLK divide by 16."]
    DIV_16 = 2,
    #[doc = "3: PCLK divide by 32."]
    DIV_32 = 3,
}
impl From<AONCD_A> for u8 {
    #[inline(always)]
    fn from(variant: AONCD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AONCD` reader - Always-ON(AON) domain CLock Divider. These bits define the AON domain clock divider."]
pub struct AONCD_R(crate::FieldReader<u8, AONCD_A>);
impl AONCD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AONCD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AONCD_A {
        match self.bits {
            0 => AONCD_A::DIV_4,
            1 => AONCD_A::DIV_8,
            2 => AONCD_A::DIV_16,
            3 => AONCD_A::DIV_32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV_4`"]
    #[inline(always)]
    pub fn is_div_4(&self) -> bool {
        **self == AONCD_A::DIV_4
    }
    #[doc = "Checks if the value of the field is `DIV_8`"]
    #[inline(always)]
    pub fn is_div_8(&self) -> bool {
        **self == AONCD_A::DIV_8
    }
    #[doc = "Checks if the value of the field is `DIV_16`"]
    #[inline(always)]
    pub fn is_div_16(&self) -> bool {
        **self == AONCD_A::DIV_16
    }
    #[doc = "Checks if the value of the field is `DIV_32`"]
    #[inline(always)]
    pub fn is_div_32(&self) -> bool {
        **self == AONCD_A::DIV_32
    }
}
impl core::ops::Deref for AONCD_R {
    type Target = crate::FieldReader<u8, AONCD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AONCD` writer - Always-ON(AON) domain CLock Divider. These bits define the AON domain clock divider."]
pub struct AONCD_W<'a> {
    w: &'a mut W,
}
impl<'a> AONCD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AONCD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PCLK divide by 4."]
    #[inline(always)]
    pub fn div_4(self) -> &'a mut W {
        self.variant(AONCD_A::DIV_4)
    }
    #[doc = "PCLK divide by 8."]
    #[inline(always)]
    pub fn div_8(self) -> &'a mut W {
        self.variant(AONCD_A::DIV_8)
    }
    #[doc = "PCLK divide by 16."]
    #[inline(always)]
    pub fn div_16(self) -> &'a mut W {
        self.variant(AONCD_A::DIV_16)
    }
    #[doc = "PCLK divide by 32."]
    #[inline(always)]
    pub fn div_32(self) -> &'a mut W {
        self.variant(AONCD_A::DIV_32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Always-ON(AON) domain CLock Divider. These bits define the AON domain clock divider."]
    #[inline(always)]
    pub fn aoncd(&self) -> AONCD_R {
        AONCD_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Always-ON(AON) domain CLock Divider. These bits define the AON domain clock divider."]
    #[inline(always)]
    pub fn aoncd(&mut self) -> AONCD_W {
        AONCD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Clock Divider.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pckdiv](index.html) module"]
pub struct PCKDIV_SPEC;
impl crate::RegisterSpec for PCKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pckdiv::R](R) reader structure"]
impl crate::Readable for PCKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pckdiv::W](W) writer structure"]
impl crate::Writable for PCKDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCKDIV to value 0x01"]
impl crate::Resettable for PCKDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
