#[doc = "Register `SRC_RLD` reader"]
pub struct R(crate::R<SRC_RLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRC_RLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRC_RLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRC_RLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRC_RLD` writer"]
pub struct W(crate::W<SRC_RLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRC_RLD_SPEC>;
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
impl From<crate::W<SRC_RLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRC_RLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRC_RLD` reader - Source Address Reload Value."]
pub struct SRC_RLD_R(crate::FieldReader<u32, u32>);
impl SRC_RLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SRC_RLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC_RLD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC_RLD` writer - Source Address Reload Value."]
pub struct SRC_RLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_RLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | (value as u32 & 0x7fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:30 - Source Address Reload Value."]
    #[inline(always)]
    pub fn src_rld(&self) -> SRC_RLD_R {
        SRC_RLD_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:30 - Source Address Reload Value."]
    #[inline(always)]
    pub fn src_rld(&mut self) -> SRC_RLD_W {
        SRC_RLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source Address Reload Value. The value of this register is loaded into DMA0_SRC upon a count-to-zero condition.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src_rld](index.html) module"]
pub struct SRC_RLD_SPEC;
impl crate::RegisterSpec for SRC_RLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [src_rld::R](R) reader structure"]
impl crate::Readable for SRC_RLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [src_rld::W](W) writer structure"]
impl crate::Writable for SRC_RLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRC_RLD to value 0"]
impl crate::Resettable for SRC_RLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
