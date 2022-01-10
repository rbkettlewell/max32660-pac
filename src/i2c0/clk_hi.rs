#[doc = "Register `CLK_HI` reader"]
pub struct R(crate::R<CLK_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_HI` writer"]
pub struct W(crate::W<CLK_HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_HI_SPEC>;
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
impl From<crate::W<CLK_HI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_HI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKH` reader - Clock High. In master mode, these bits define the SCL high period."]
pub struct CKH_R(crate::FieldReader<u16, u16>);
impl CKH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CKH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKH` writer - Clock High. In master mode, these bits define the SCL high period."]
pub struct CKH_W<'a> {
    w: &'a mut W,
}
impl<'a> CKH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Clock High. In master mode, these bits define the SCL high period."]
    #[inline(always)]
    pub fn ckh(&self) -> CKH_R {
        CKH_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Clock High. In master mode, these bits define the SCL high period."]
    #[inline(always)]
    pub fn ckh(&mut self) -> CKH_W {
        CKH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock high Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_hi](index.html) module"]
pub struct CLK_HI_SPEC;
impl crate::RegisterSpec for CLK_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_hi::R](R) reader structure"]
impl crate::Readable for CLK_HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_hi::W](W) writer structure"]
impl crate::Writable for CLK_HI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_HI to value 0"]
impl crate::Resettable for CLK_HI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
