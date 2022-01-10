#[doc = "Register `CACHE_CTRL` reader"]
pub struct R(crate::R<CACHE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_CTRL` writer"]
pub struct W(crate::W<CACHE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_CTRL_SPEC>;
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
impl From<crate::W<CACHE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Cache Enable. Controls whether the cache is bypassed or is in use. Changing the state of this bit will cause the instruction cache to be flushed and its contents invalidated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHE_EN_A {
    #[doc = "0: Cache Bypassed. Instruction data is stored in the line fill buffer but is not written to main cache memory array."]
    DIS = 0,
    #[doc = "1: Cache Enabled."]
    EN = 1,
}
impl From<CACHE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CACHE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CACHE_EN` reader - Cache Enable. Controls whether the cache is bypassed or is in use. Changing the state of this bit will cause the instruction cache to be flushed and its contents invalidated."]
pub struct CACHE_EN_R(crate::FieldReader<bool, CACHE_EN_A>);
impl CACHE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHE_EN_A {
        match self.bits {
            false => CACHE_EN_A::DIS,
            true => CACHE_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == CACHE_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == CACHE_EN_A::EN
    }
}
impl core::ops::Deref for CACHE_EN_R {
    type Target = crate::FieldReader<bool, CACHE_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_EN` writer - Cache Enable. Controls whether the cache is bypassed or is in use. Changing the state of this bit will cause the instruction cache to be flushed and its contents invalidated."]
pub struct CACHE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHE_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Cache Bypassed. Instruction data is stored in the line fill buffer but is not written to main cache memory array."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CACHE_EN_A::DIS)
    }
    #[doc = "Cache Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CACHE_EN_A::EN)
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
#[doc = "Cache Ready flag. Cleared by hardware when at any time the cache as a whole is invalidated (including a system reset). When this bit is 0, the cache is effectively in bypass mode (instruction fetches will come from main memory or from the line fill buffer). Set by hardware when the invalidate operation is complete and the cache is ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHE_RDY_A {
    #[doc = "0: Not Ready."]
    NOTREADY = 0,
    #[doc = "1: Ready."]
    READY = 1,
}
impl From<CACHE_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: CACHE_RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CACHE_RDY` reader - Cache Ready flag. Cleared by hardware when at any time the cache as a whole is invalidated (including a system reset). When this bit is 0, the cache is effectively in bypass mode (instruction fetches will come from main memory or from the line fill buffer). Set by hardware when the invalidate operation is complete and the cache is ready."]
pub struct CACHE_RDY_R(crate::FieldReader<bool, CACHE_RDY_A>);
impl CACHE_RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_RDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHE_RDY_A {
        match self.bits {
            false => CACHE_RDY_A::NOTREADY,
            true => CACHE_RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == CACHE_RDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == CACHE_RDY_A::READY
    }
}
impl core::ops::Deref for CACHE_RDY_R {
    type Target = crate::FieldReader<bool, CACHE_RDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Cache Enable. Controls whether the cache is bypassed or is in use. Changing the state of this bit will cause the instruction cache to be flushed and its contents invalidated."]
    #[inline(always)]
    pub fn cache_en(&self) -> CACHE_EN_R {
        CACHE_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Cache Ready flag. Cleared by hardware when at any time the cache as a whole is invalidated (including a system reset). When this bit is 0, the cache is effectively in bypass mode (instruction fetches will come from main memory or from the line fill buffer). Set by hardware when the invalidate operation is complete and the cache is ready."]
    #[inline(always)]
    pub fn cache_rdy(&self) -> CACHE_RDY_R {
        CACHE_RDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache Enable. Controls whether the cache is bypassed or is in use. Changing the state of this bit will cause the instruction cache to be flushed and its contents invalidated."]
    #[inline(always)]
    pub fn cache_en(&mut self) -> CACHE_EN_W {
        CACHE_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Control and Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_ctrl](index.html) module"]
pub struct CACHE_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_ctrl::R](R) reader structure"]
impl crate::Readable for CACHE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_ctrl::W](W) writer structure"]
impl crate::Writable for CACHE_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_CTRL to value 0"]
impl crate::Resettable for CACHE_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
