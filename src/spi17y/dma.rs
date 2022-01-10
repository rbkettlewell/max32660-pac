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
#[doc = "Field `TX_FIFO_LEVEL` reader - Transmit FIFO level that will trigger a DMA request, also level for threshold status. When TX FIFO has fewer than this many bytes, the associated events and conditions are triggered."]
pub struct TX_FIFO_LEVEL_R(crate::FieldReader<u8, u8>);
impl TX_FIFO_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_FIFO_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FIFO_LEVEL` writer - Transmit FIFO level that will trigger a DMA request, also level for threshold status. When TX FIFO has fewer than this many bytes, the associated events and conditions are triggered."]
pub struct TX_FIFO_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Transmit FIFO enabled for SPI transactions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_FIFO_EN_A {
    #[doc = "0: Transmit FIFO is not enabled."]
    DIS = 0,
    #[doc = "1: Transmit FIFO is enabled."]
    EN = 1,
}
impl From<TX_FIFO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_FIFO_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_FIFO_EN` reader - Transmit FIFO enabled for SPI transactions."]
pub struct TX_FIFO_EN_R(crate::FieldReader<bool, TX_FIFO_EN_A>);
impl TX_FIFO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_FIFO_EN_A {
        match self.bits {
            false => TX_FIFO_EN_A::DIS,
            true => TX_FIFO_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TX_FIFO_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TX_FIFO_EN_A::EN
    }
}
impl core::ops::Deref for TX_FIFO_EN_R {
    type Target = crate::FieldReader<bool, TX_FIFO_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FIFO_EN` writer - Transmit FIFO enabled for SPI transactions."]
pub struct TX_FIFO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_FIFO_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit FIFO is not enabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_FIFO_EN_A::DIS)
    }
    #[doc = "Transmit FIFO is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_FIFO_EN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Clear TX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side. .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_FIFO_CLEAR_A {
    #[doc = "1: Clear the Transmit FIFO, clears any pending TX FIFO status."]
    CLEAR = 1,
}
impl From<TX_FIFO_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: TX_FIFO_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_FIFO_CLEAR` reader - Clear TX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side. ."]
pub struct TX_FIFO_CLEAR_R(crate::FieldReader<bool, TX_FIFO_CLEAR_A>);
impl TX_FIFO_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TX_FIFO_CLEAR_A> {
        match self.bits {
            true => Some(TX_FIFO_CLEAR_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == TX_FIFO_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for TX_FIFO_CLEAR_R {
    type Target = crate::FieldReader<bool, TX_FIFO_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FIFO_CLEAR` writer - Clear TX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side. ."]
pub struct TX_FIFO_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_FIFO_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the Transmit FIFO, clears any pending TX FIFO status."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TX_FIFO_CLEAR_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `TX_FIFO_CNT` reader - Count of entries in TX FIFO."]
pub struct TX_FIFO_CNT_R(crate::FieldReader<u8, u8>);
impl TX_FIFO_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_FIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "TX DMA Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_DMA_EN_A {
    #[doc = "0: TX DMA requests are disabled, andy pending DMA requests are cleared."]
    DIS = 0,
    #[doc = "1: TX DMA requests are enabled."]
    EN = 1,
}
impl From<TX_DMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_DMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_DMA_EN` reader - TX DMA Enable."]
pub struct TX_DMA_EN_R(crate::FieldReader<bool, TX_DMA_EN_A>);
impl TX_DMA_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_DMA_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_DMA_EN_A {
        match self.bits {
            false => TX_DMA_EN_A::DIS,
            true => TX_DMA_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TX_DMA_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TX_DMA_EN_A::EN
    }
}
impl core::ops::Deref for TX_DMA_EN_R {
    type Target = crate::FieldReader<bool, TX_DMA_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DMA_EN` writer - TX DMA Enable."]
pub struct TX_DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DMA_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_DMA_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TX DMA requests are disabled, andy pending DMA requests are cleared."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_DMA_EN_A::DIS)
    }
    #[doc = "TX DMA requests are enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_DMA_EN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `RX_FIFO_LEVEL` reader - Receive FIFO level that will trigger a DMA request, also level for threshold status. When RX FIFO has more than this many bytes, the associated events and conditions are triggered."]
pub struct RX_FIFO_LEVEL_R(crate::FieldReader<u8, u8>);
impl RX_FIFO_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_FIFO_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FIFO_LEVEL` writer - Receive FIFO level that will trigger a DMA request, also level for threshold status. When RX FIFO has more than this many bytes, the associated events and conditions are triggered."]
pub struct RX_FIFO_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Receive FIFO enabled for SPI transactions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_EN_A {
    #[doc = "0: Receive FIFO is not enabled."]
    DIS = 0,
    #[doc = "1: Receive FIFO is enabled."]
    EN = 1,
}
impl From<RX_FIFO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFO_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_FIFO_EN` reader - Receive FIFO enabled for SPI transactions."]
pub struct RX_FIFO_EN_R(crate::FieldReader<bool, RX_FIFO_EN_A>);
impl RX_FIFO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FIFO_EN_A {
        match self.bits {
            false => RX_FIFO_EN_A::DIS,
            true => RX_FIFO_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RX_FIFO_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == RX_FIFO_EN_A::EN
    }
}
impl core::ops::Deref for RX_FIFO_EN_R {
    type Target = crate::FieldReader<bool, RX_FIFO_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FIFO_EN` writer - Receive FIFO enabled for SPI transactions."]
pub struct RX_FIFO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_FIFO_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive FIFO is not enabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RX_FIFO_EN_A::DIS)
    }
    #[doc = "Receive FIFO is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RX_FIFO_EN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Clear RX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_CLEAR_A {
    #[doc = "1: Clear the Receive FIFO, clears any pending RX FIFO status."]
    CLEAR = 1,
}
impl From<RX_FIFO_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFO_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_FIFO_CLEAR` reader - Clear RX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side."]
pub struct RX_FIFO_CLEAR_R(crate::FieldReader<bool, RX_FIFO_CLEAR_A>);
impl RX_FIFO_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RX_FIFO_CLEAR_A> {
        match self.bits {
            true => Some(RX_FIFO_CLEAR_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == RX_FIFO_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for RX_FIFO_CLEAR_R {
    type Target = crate::FieldReader<bool, RX_FIFO_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FIFO_CLEAR` writer - Clear RX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side."]
pub struct RX_FIFO_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_FIFO_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the Receive FIFO, clears any pending RX FIFO status."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RX_FIFO_CLEAR_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `RX_FIFO_CNT` reader - Count of entries in RX FIFO."]
pub struct RX_FIFO_CNT_R(crate::FieldReader<u8, u8>);
impl RX_FIFO_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_FIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RX DMA Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DMA_EN_A {
    #[doc = "0: RX DMA requests are disabled, any pending DMA requests are cleared."]
    DIS = 0,
    #[doc = "1: RX DMA requests are enabled."]
    EN = 1,
}
impl From<RX_DMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_DMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_DMA_EN` reader - RX DMA Enable."]
pub struct RX_DMA_EN_R(crate::FieldReader<bool, RX_DMA_EN_A>);
impl RX_DMA_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_DMA_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_DMA_EN_A {
        match self.bits {
            false => RX_DMA_EN_A::DIS,
            true => RX_DMA_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RX_DMA_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == RX_DMA_EN_A::EN
    }
}
impl core::ops::Deref for RX_DMA_EN_R {
    type Target = crate::FieldReader<bool, RX_DMA_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_DMA_EN` writer - RX DMA Enable."]
pub struct RX_DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DMA_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_DMA_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RX DMA requests are disabled, any pending DMA requests are cleared."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RX_DMA_EN_A::DIS)
    }
    #[doc = "RX DMA requests are enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RX_DMA_EN_A::EN)
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
    #[doc = "Bits 0:4 - Transmit FIFO level that will trigger a DMA request, also level for threshold status. When TX FIFO has fewer than this many bytes, the associated events and conditions are triggered."]
    #[inline(always)]
    pub fn tx_fifo_level(&self) -> TX_FIFO_LEVEL_R {
        TX_FIFO_LEVEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Transmit FIFO enabled for SPI transactions."]
    #[inline(always)]
    pub fn tx_fifo_en(&self) -> TX_FIFO_EN_R {
        TX_FIFO_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Clear TX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side. ."]
    #[inline(always)]
    pub fn tx_fifo_clear(&self) -> TX_FIFO_CLEAR_R {
        TX_FIFO_CLEAR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - Count of entries in TX FIFO."]
    #[inline(always)]
    pub fn tx_fifo_cnt(&self) -> TX_FIFO_CNT_R {
        TX_FIFO_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - TX DMA Enable."]
    #[inline(always)]
    pub fn tx_dma_en(&self) -> TX_DMA_EN_R {
        TX_DMA_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Receive FIFO level that will trigger a DMA request, also level for threshold status. When RX FIFO has more than this many bytes, the associated events and conditions are triggered."]
    #[inline(always)]
    pub fn rx_fifo_level(&self) -> RX_FIFO_LEVEL_R {
        RX_FIFO_LEVEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - Receive FIFO enabled for SPI transactions."]
    #[inline(always)]
    pub fn rx_fifo_en(&self) -> RX_FIFO_EN_R {
        RX_FIFO_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Clear RX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side."]
    #[inline(always)]
    pub fn rx_fifo_clear(&self) -> RX_FIFO_CLEAR_R {
        RX_FIFO_CLEAR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:29 - Count of entries in RX FIFO."]
    #[inline(always)]
    pub fn rx_fifo_cnt(&self) -> RX_FIFO_CNT_R {
        RX_FIFO_CNT_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - RX DMA Enable."]
    #[inline(always)]
    pub fn rx_dma_en(&self) -> RX_DMA_EN_R {
        RX_DMA_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Transmit FIFO level that will trigger a DMA request, also level for threshold status. When TX FIFO has fewer than this many bytes, the associated events and conditions are triggered."]
    #[inline(always)]
    pub fn tx_fifo_level(&mut self) -> TX_FIFO_LEVEL_W {
        TX_FIFO_LEVEL_W { w: self }
    }
    #[doc = "Bit 6 - Transmit FIFO enabled for SPI transactions."]
    #[inline(always)]
    pub fn tx_fifo_en(&mut self) -> TX_FIFO_EN_W {
        TX_FIFO_EN_W { w: self }
    }
    #[doc = "Bit 7 - Clear TX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side. ."]
    #[inline(always)]
    pub fn tx_fifo_clear(&mut self) -> TX_FIFO_CLEAR_W {
        TX_FIFO_CLEAR_W { w: self }
    }
    #[doc = "Bit 15 - TX DMA Enable."]
    #[inline(always)]
    pub fn tx_dma_en(&mut self) -> TX_DMA_EN_W {
        TX_DMA_EN_W { w: self }
    }
    #[doc = "Bits 16:20 - Receive FIFO level that will trigger a DMA request, also level for threshold status. When RX FIFO has more than this many bytes, the associated events and conditions are triggered."]
    #[inline(always)]
    pub fn rx_fifo_level(&mut self) -> RX_FIFO_LEVEL_W {
        RX_FIFO_LEVEL_W { w: self }
    }
    #[doc = "Bit 22 - Receive FIFO enabled for SPI transactions."]
    #[inline(always)]
    pub fn rx_fifo_en(&mut self) -> RX_FIFO_EN_W {
        RX_FIFO_EN_W { w: self }
    }
    #[doc = "Bit 23 - Clear RX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side."]
    #[inline(always)]
    pub fn rx_fifo_clear(&mut self) -> RX_FIFO_CLEAR_W {
        RX_FIFO_CLEAR_W { w: self }
    }
    #[doc = "Bit 31 - RX DMA Enable."]
    #[inline(always)]
    pub fn rx_dma_en(&mut self) -> RX_DMA_EN_W {
        RX_DMA_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for controlling DMA.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma](index.html) module"]
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
