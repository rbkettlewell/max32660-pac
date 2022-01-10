#[doc = "Register `DMA` reader"]
pub struct R(crate::R<DMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA` writer"]
pub struct W(crate::W<DMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_SPEC>;
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
impl From<crate::W<DMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TX DMA channel enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMA_EN_A {
    #[doc = "0: DMA is disabled"]
    DIS = 0,
    #[doc = "1: DMA is enabled"]
    EN = 1,
}
impl From<TXDMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMA_EN` reader - TX DMA channel enable."]
pub struct TXDMA_EN_R(crate::FieldReader<bool, TXDMA_EN_A>);
impl TXDMA_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXDMA_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDMA_EN_A {
        match self.bits {
            false => TXDMA_EN_A::DIS,
            true => TXDMA_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TXDMA_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TXDMA_EN_A::EN
    }
}
impl core::ops::Deref for TXDMA_EN_R {
    type Target = crate::FieldReader<bool, TXDMA_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDMA_EN` writer - TX DMA channel enable."]
pub struct TXDMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMA_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDMA_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA is disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TXDMA_EN_A::DIS)
    }
    #[doc = "DMA is enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TXDMA_EN_A::EN)
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
#[doc = "RX DMA channel enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMA_EN_A {
    #[doc = "0: DMA is disabled"]
    DIS = 0,
    #[doc = "1: DMA is enabled"]
    EN = 1,
}
impl From<RXDMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMA_EN` reader - RX DMA channel enable."]
pub struct RXDMA_EN_R(crate::FieldReader<bool, RXDMA_EN_A>);
impl RXDMA_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXDMA_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDMA_EN_A {
        match self.bits {
            false => RXDMA_EN_A::DIS,
            true => RXDMA_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RXDMA_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == RXDMA_EN_A::EN
    }
}
impl core::ops::Deref for RXDMA_EN_R {
    type Target = crate::FieldReader<bool, RXDMA_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDMA_EN` writer - RX DMA channel enable."]
pub struct RXDMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMA_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDMA_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA is disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RXDMA_EN_A::DIS)
    }
    #[doc = "DMA is enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RXDMA_EN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TXDMA_LEVEL` reader - TX threshold for DMA transmission."]
pub struct TXDMA_LEVEL_R(crate::FieldReader<u8, u8>);
impl TXDMA_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXDMA_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDMA_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDMA_LEVEL` writer - TX threshold for DMA transmission."]
pub struct TXDMA_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMA_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `RXDMA_LEVEL` reader - RX threshold for DMA transmission."]
pub struct RXDMA_LEVEL_R(crate::FieldReader<u8, u8>);
impl RXDMA_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXDMA_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDMA_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDMA_LEVEL` writer - RX threshold for DMA transmission."]
pub struct RXDMA_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMA_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TX DMA channel enable."]
    #[inline(always)]
    pub fn txdma_en(&self) -> TXDMA_EN_R {
        TXDMA_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX DMA channel enable."]
    #[inline(always)]
    pub fn rxdma_en(&self) -> RXDMA_EN_R {
        RXDMA_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - TX threshold for DMA transmission."]
    #[inline(always)]
    pub fn txdma_level(&self) -> TXDMA_LEVEL_R {
        TXDMA_LEVEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - RX threshold for DMA transmission."]
    #[inline(always)]
    pub fn rxdma_level(&self) -> RXDMA_LEVEL_R {
        RXDMA_LEVEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TX DMA channel enable."]
    #[inline(always)]
    pub fn txdma_en(&mut self) -> TXDMA_EN_W {
        TXDMA_EN_W { w: self }
    }
    #[doc = "Bit 1 - RX DMA channel enable."]
    #[inline(always)]
    pub fn rxdma_en(&mut self) -> RXDMA_EN_W {
        RXDMA_EN_W { w: self }
    }
    #[doc = "Bits 8:13 - TX threshold for DMA transmission."]
    #[inline(always)]
    pub fn txdma_level(&mut self) -> TXDMA_LEVEL_W {
        TXDMA_LEVEL_W { w: self }
    }
    #[doc = "Bits 16:21 - RX threshold for DMA transmission."]
    #[inline(always)]
    pub fn rxdma_level(&mut self) -> RXDMA_LEVEL_W {
        RXDMA_LEVEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma](index.html) module"]
pub struct DMA_SPEC;
impl crate::RegisterSpec for DMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma::R](R) reader structure"]
impl crate::Readable for DMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma::W](W) writer structure"]
impl crate::Writable for DMA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA to value 0"]
impl crate::Resettable for DMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
