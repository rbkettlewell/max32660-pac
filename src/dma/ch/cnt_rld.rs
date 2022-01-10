#[doc = "Register `CNT_RLD` reader"]
pub struct R(crate::R<CNT_RLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNT_RLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNT_RLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNT_RLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNT_RLD` writer"]
pub struct W(crate::W<CNT_RLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNT_RLD_SPEC>;
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
impl From<crate::W<CNT_RLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNT_RLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT_RLD` reader - Count Reload Value. The value of this register is loaded into DMA0_CNT upon a count-to-zero condition."]
pub struct CNT_RLD_R(crate::FieldReader<u32, u32>);
impl CNT_RLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CNT_RLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_RLD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT_RLD` writer - Count Reload Value. The value of this register is loaded into DMA0_CNT upon a count-to-zero condition."]
pub struct CNT_RLD_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_RLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Reload Enable. This bit should be set after the address reload registers have been programmed. This bit is automatically cleared to 0 when reload occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RLDEN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<RLDEN_A> for bool {
    #[inline(always)]
    fn from(variant: RLDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RLDEN` reader - Reload Enable. This bit should be set after the address reload registers have been programmed. This bit is automatically cleared to 0 when reload occurs."]
pub struct RLDEN_R(crate::FieldReader<bool, RLDEN_A>);
impl RLDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RLDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RLDEN_A {
        match self.bits {
            false => RLDEN_A::DIS,
            true => RLDEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RLDEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == RLDEN_A::EN
    }
}
impl core::ops::Deref for RLDEN_R {
    type Target = crate::FieldReader<bool, RLDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RLDEN` writer - Reload Enable. This bit should be set after the address reload registers have been programmed. This bit is automatically cleared to 0 when reload occurs."]
pub struct RLDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RLDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RLDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RLDEN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RLDEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Count Reload Value. The value of this register is loaded into DMA0_CNT upon a count-to-zero condition."]
    #[inline(always)]
    pub fn cnt_rld(&self) -> CNT_RLD_R {
        CNT_RLD_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 31 - Reload Enable. This bit should be set after the address reload registers have been programmed. This bit is automatically cleared to 0 when reload occurs."]
    #[inline(always)]
    pub fn rlden(&self) -> RLDEN_R {
        RLDEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Count Reload Value. The value of this register is loaded into DMA0_CNT upon a count-to-zero condition."]
    #[inline(always)]
    pub fn cnt_rld(&mut self) -> CNT_RLD_W {
        CNT_RLD_W { w: self }
    }
    #[doc = "Bit 31 - Reload Enable. This bit should be set after the address reload registers have been programmed. This bit is automatically cleared to 0 when reload occurs."]
    #[inline(always)]
    pub fn rlden(&mut self) -> RLDEN_W {
        RLDEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Count Reload Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt_rld](index.html) module"]
pub struct CNT_RLD_SPEC;
impl crate::RegisterSpec for CNT_RLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnt_rld::R](R) reader structure"]
impl crate::Readable for CNT_RLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnt_rld::W](W) writer structure"]
impl crate::Writable for CNT_RLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNT_RLD to value 0"]
impl crate::Resettable for CNT_RLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
