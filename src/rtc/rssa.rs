#[doc = "Register `RSSA` reader"]
pub struct R(crate::R<RSSA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSSA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSSA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSSA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSSA` writer"]
pub struct W(crate::W<RSSA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSSA_SPEC>;
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
impl From<crate::W<RSSA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSSA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSSA` reader - This register contains the reload value for the sub-second alarm."]
pub struct RSSA_R(crate::FieldReader<u32, u32>);
impl RSSA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RSSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSSA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSSA` writer - This register contains the reload value for the sub-second alarm."]
pub struct RSSA_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - This register contains the reload value for the sub-second alarm."]
    #[inline(always)]
    pub fn rssa(&self) -> RSSA_R {
        RSSA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register contains the reload value for the sub-second alarm."]
    #[inline(always)]
    pub fn rssa(&mut self) -> RSSA_W {
        RSSA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC sub-second alarm. This register contains the reload value for the sub-second alarm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rssa](index.html) module"]
pub struct RSSA_SPEC;
impl crate::RegisterSpec for RSSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rssa::R](R) reader structure"]
impl crate::Readable for RSSA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rssa::W](W) writer structure"]
impl crate::Writable for RSSA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSSA to value 0"]
impl crate::Resettable for RSSA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
