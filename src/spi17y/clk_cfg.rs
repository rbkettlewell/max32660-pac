#[doc = "Register `CLK_CFG` reader"]
pub struct R(crate::R<CLK_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CFG` writer"]
pub struct W(crate::W<CLK_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CFG_SPEC>;
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
impl From<crate::W<CLK_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Low duty cycle control. In timer mode, reload\\[7:0\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LO_A {
    #[doc = "0: Duty cycle control of serial clock generation is disabled."]
    DIS = 0,
}
impl From<LO_A> for u8 {
    #[inline(always)]
    fn from(variant: LO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LO` reader - Low duty cycle control. In timer mode, reload\\[7:0\\]."]
pub struct LO_R(crate::FieldReader<u8, LO_A>);
impl LO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LO_A> {
        match self.bits {
            0 => Some(LO_A::DIS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == LO_A::DIS
    }
}
impl core::ops::Deref for LO_R {
    type Target = crate::FieldReader<u8, LO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LO` writer - Low duty cycle control. In timer mode, reload\\[7:0\\]."]
pub struct LO_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Duty cycle control of serial clock generation is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(LO_A::DIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "High duty cycle control. In timer mode, reload\\[15:8\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HI_A {
    #[doc = "0: Duty cycle control of serial clock generation is disabled."]
    DIS = 0,
}
impl From<HI_A> for u8 {
    #[inline(always)]
    fn from(variant: HI_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HI` reader - High duty cycle control. In timer mode, reload\\[15:8\\]."]
pub struct HI_R(crate::FieldReader<u8, HI_A>);
impl HI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HI_A> {
        match self.bits {
            0 => Some(HI_A::DIS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == HI_A::DIS
    }
}
impl core::ops::Deref for HI_R {
    type Target = crate::FieldReader<u8, HI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HI` writer - High duty cycle control. In timer mode, reload\\[15:8\\]."]
pub struct HI_W<'a> {
    w: &'a mut W,
}
impl<'a> HI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HI_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Duty cycle control of serial clock generation is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(HI_A::DIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SCALE` reader - System Clock scale factor. Scales the AMBA clock by 2^SCALE before generating serial clock."]
pub struct SCALE_R(crate::FieldReader<u8, u8>);
impl SCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCALE` writer - System Clock scale factor. Scales the AMBA clock by 2^SCALE before generating serial clock."]
pub struct SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Low duty cycle control. In timer mode, reload\\[7:0\\]."]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - High duty cycle control. In timer mode, reload\\[15:8\\]."]
    #[inline(always)]
    pub fn hi(&self) -> HI_R {
        HI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - System Clock scale factor. Scales the AMBA clock by 2^SCALE before generating serial clock."]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low duty cycle control. In timer mode, reload\\[7:0\\]."]
    #[inline(always)]
    pub fn lo(&mut self) -> LO_W {
        LO_W { w: self }
    }
    #[doc = "Bits 8:15 - High duty cycle control. In timer mode, reload\\[15:8\\]."]
    #[inline(always)]
    pub fn hi(&mut self) -> HI_W {
        HI_W { w: self }
    }
    #[doc = "Bits 16:19 - System Clock scale factor. Scales the AMBA clock by 2^SCALE before generating serial clock."]
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W {
        SCALE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for controlling SPI clock rate.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cfg](index.html) module"]
pub struct CLK_CFG_SPEC;
impl crate::RegisterSpec for CLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_cfg::R](R) reader structure"]
impl crate::Readable for CLK_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_cfg::W](W) writer structure"]
impl crate::Writable for CLK_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CFG to value 0"]
impl crate::Resettable for CLK_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
