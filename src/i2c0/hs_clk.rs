#[doc = "Register `HS_CLK` reader"]
pub struct R(crate::R<HS_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HS_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HS_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HS_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HS_CLK` writer"]
pub struct W(crate::W<HS_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HS_CLK_SPEC>;
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
impl From<crate::W<HS_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HS_CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HS_CLK_LO` reader - Slave Address."]
pub struct HS_CLK_LO_R(crate::FieldReader<u8, u8>);
impl HS_CLK_LO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HS_CLK_LO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HS_CLK_LO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HS_CLK_LO` writer - Slave Address."]
pub struct HS_CLK_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_CLK_LO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `HS_CLK_HI` reader - Slave Address."]
pub struct HS_CLK_HI_R(crate::FieldReader<u8, u8>);
impl HS_CLK_HI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HS_CLK_HI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HS_CLK_HI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HS_CLK_HI` writer - Slave Address."]
pub struct HS_CLK_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_CLK_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Slave Address."]
    #[inline(always)]
    pub fn hs_clk_lo(&self) -> HS_CLK_LO_R {
        HS_CLK_LO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Slave Address."]
    #[inline(always)]
    pub fn hs_clk_hi(&self) -> HS_CLK_HI_R {
        HS_CLK_HI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slave Address."]
    #[inline(always)]
    pub fn hs_clk_lo(&mut self) -> HS_CLK_LO_W {
        HS_CLK_LO_W { w: self }
    }
    #[doc = "Bits 8:15 - Slave Address."]
    #[inline(always)]
    pub fn hs_clk_hi(&mut self) -> HS_CLK_HI_W {
        HS_CLK_HI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HS-Mode Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hs_clk](index.html) module"]
pub struct HS_CLK_SPEC;
impl crate::RegisterSpec for HS_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hs_clk::R](R) reader structure"]
impl crate::Readable for HS_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hs_clk::W](W) writer structure"]
impl crate::Writable for HS_CLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HS_CLK to value 0"]
impl crate::Resettable for HS_CLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
