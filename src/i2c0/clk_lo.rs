#[doc = "Register `CLK_LO` reader"]
pub struct R(crate::R<CLK_LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_LO` writer"]
pub struct W(crate::W<CLK_LO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_LO_SPEC>;
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
impl From<crate::W<CLK_LO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_LO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_LO` reader - Clock low. In master mode, these bits define the SCL low period. In slave mode, these bits define the time SCL will be held low after data is outputted."]
pub struct CLK_LO_R(crate::FieldReader<u16, u16>);
impl CLK_LO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CLK_LO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_LO_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_LO` writer - Clock low. In master mode, these bits define the SCL low period. In slave mode, these bits define the time SCL will be held low after data is outputted."]
pub struct CLK_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_LO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Clock low. In master mode, these bits define the SCL low period. In slave mode, these bits define the time SCL will be held low after data is outputted."]
    #[inline(always)]
    pub fn clk_lo(&self) -> CLK_LO_R {
        CLK_LO_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Clock low. In master mode, these bits define the SCL low period. In slave mode, these bits define the time SCL will be held low after data is outputted."]
    #[inline(always)]
    pub fn clk_lo(&mut self) -> CLK_LO_W {
        CLK_LO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Low Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_lo](index.html) module"]
pub struct CLK_LO_SPEC;
impl crate::RegisterSpec for CLK_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_lo::R](R) reader structure"]
impl crate::Readable for CLK_LO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_lo::W](W) writer structure"]
impl crate::Writable for CLK_LO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_LO to value 0"]
impl crate::Resettable for CLK_LO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
