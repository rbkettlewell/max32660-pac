#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SPI Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIEN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<SPIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIEN` reader - SPI Enable."]
pub struct SPIEN_R(crate::FieldReader<bool, SPIEN_A>);
impl SPIEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIEN_A {
        match self.bits {
            false => SPIEN_A::DISABLE,
            true => SPIEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SPIEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SPIEN_A::ENABLE
    }
}
impl core::ops::Deref for SPIEN_R {
    type Target = crate::FieldReader<bool, SPIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPIEN` writer - SPI Enable."]
pub struct SPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SPIEN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SPIEN_A::ENABLE)
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
#[doc = "SPI Master Mode Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMEN_A {
    #[doc = "0: `0`"]
    SLAVE = 0,
    #[doc = "1: `1`"]
    MASTER = 1,
}
impl From<MMEN_A> for bool {
    #[inline(always)]
    fn from(variant: MMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMEN` reader - SPI Master Mode Enable."]
pub struct MMEN_R(crate::FieldReader<bool, MMEN_A>);
impl MMEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MMEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMEN_A {
        match self.bits {
            false => MMEN_A::SLAVE,
            true => MMEN_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        **self == MMEN_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        **self == MMEN_A::MASTER
    }
}
impl core::ops::Deref for MMEN_R {
    type Target = crate::FieldReader<bool, MMEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMEN` writer - SPI Master Mode Enable."]
pub struct MMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MMEN_A::SLAVE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MMEN_A::MASTER)
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
#[doc = "Wired OR (open drain) Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WOR_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<WOR_A> for bool {
    #[inline(always)]
    fn from(variant: WOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WOR` reader - Wired OR (open drain) Enable."]
pub struct WOR_R(crate::FieldReader<bool, WOR_A>);
impl WOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WOR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WOR_A {
        match self.bits {
            false => WOR_A::DISABLE,
            true => WOR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WOR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WOR_A::ENABLE
    }
}
impl core::ops::Deref for WOR_R {
    type Target = crate::FieldReader<bool, WOR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WOR` writer - Wired OR (open drain) Enable."]
pub struct WOR_W<'a> {
    w: &'a mut W,
}
impl<'a> WOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WOR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WOR_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WOR_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Clock Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKPOL_A {
    #[doc = "0: SCLK idles Low (0) after character transmission/reception."]
    IDLELO = 0,
    #[doc = "1: SCLK idles High (1) after character transmission/reception."]
    IDLEHI = 1,
}
impl From<CLKPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKPOL` reader - Clock Polarity."]
pub struct CLKPOL_R(crate::FieldReader<bool, CLKPOL_A>);
impl CLKPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKPOL_A {
        match self.bits {
            false => CLKPOL_A::IDLELO,
            true => CLKPOL_A::IDLEHI,
        }
    }
    #[doc = "Checks if the value of the field is `IDLELO`"]
    #[inline(always)]
    pub fn is_idle_lo(&self) -> bool {
        **self == CLKPOL_A::IDLELO
    }
    #[doc = "Checks if the value of the field is `IDLEHI`"]
    #[inline(always)]
    pub fn is_idle_hi(&self) -> bool {
        **self == CLKPOL_A::IDLEHI
    }
}
impl core::ops::Deref for CLKPOL_R {
    type Target = crate::FieldReader<bool, CLKPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKPOL` writer - Clock Polarity."]
pub struct CLKPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SCLK idles Low (0) after character transmission/reception."]
    #[inline(always)]
    pub fn idle_lo(self) -> &'a mut W {
        self.variant(CLKPOL_A::IDLELO)
    }
    #[doc = "SCLK idles High (1) after character transmission/reception."]
    #[inline(always)]
    pub fn idle_hi(self) -> &'a mut W {
        self.variant(CLKPOL_A::IDLEHI)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Phase Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHASE_A {
    #[doc = "0: Transmit on active edge of SCLK."]
    ACTIVEEDGE = 0,
    #[doc = "1: Transmit on inactive edge of SCLK."]
    INACTIVEEDGE = 1,
}
impl From<PHASE_A> for bool {
    #[inline(always)]
    fn from(variant: PHASE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHASE` reader - Phase Select."]
pub struct PHASE_R(crate::FieldReader<bool, PHASE_A>);
impl PHASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PHASE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHASE_A {
        match self.bits {
            false => PHASE_A::ACTIVEEDGE,
            true => PHASE_A::INACTIVEEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVEEDGE`"]
    #[inline(always)]
    pub fn is_active_edge(&self) -> bool {
        **self == PHASE_A::ACTIVEEDGE
    }
    #[doc = "Checks if the value of the field is `INACTIVEEDGE`"]
    #[inline(always)]
    pub fn is_inactive_edge(&self) -> bool {
        **self == PHASE_A::INACTIVEEDGE
    }
}
impl core::ops::Deref for PHASE_R {
    type Target = crate::FieldReader<bool, PHASE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHASE` writer - Phase Select."]
pub struct PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHASE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit on active edge of SCLK."]
    #[inline(always)]
    pub fn active_edge(self) -> &'a mut W {
        self.variant(PHASE_A::ACTIVEEDGE)
    }
    #[doc = "Transmit on inactive edge of SCLK."]
    #[inline(always)]
    pub fn inactive_edge(self) -> &'a mut W {
        self.variant(PHASE_A::INACTIVEEDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Baud Rate Generator Timer Interrupt Request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIRQ_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<BIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: BIRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIRQ` reader - Baud Rate Generator Timer Interrupt Request."]
pub struct BIRQ_R(crate::FieldReader<bool, BIRQ_A>);
impl BIRQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIRQ_A {
        match self.bits {
            false => BIRQ_A::DISABLE,
            true => BIRQ_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == BIRQ_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == BIRQ_A::ENABLE
    }
}
impl core::ops::Deref for BIRQ_R {
    type Target = crate::FieldReader<bool, BIRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIRQ` writer - Baud Rate Generator Timer Interrupt Request."]
pub struct BIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> BIRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BIRQ_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BIRQ_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Start SPI Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STR_A {
    #[doc = "0: No operation/complete."]
    COMPLETE = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<STR_A> for bool {
    #[inline(always)]
    fn from(variant: STR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STR` reader - Start SPI Interrupt."]
pub struct STR_R(crate::FieldReader<bool, STR_A>);
impl STR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STR_A {
        match self.bits {
            false => STR_A::COMPLETE,
            true => STR_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        **self == STR_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == STR_A::START
    }
}
impl core::ops::Deref for STR_R {
    type Target = crate::FieldReader<bool, STR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STR` writer - Start SPI Interrupt."]
pub struct STR_W<'a> {
    w: &'a mut W,
}
impl<'a> STR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(STR_A::COMPLETE)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(STR_A::START)
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
#[doc = "Interrupt Request Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<IRQE_A> for bool {
    #[inline(always)]
    fn from(variant: IRQE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQE` reader - Interrupt Request Enable."]
pub struct IRQE_R(crate::FieldReader<bool, IRQE_A>);
impl IRQE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRQE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQE_A {
        match self.bits {
            false => IRQE_A::DISABLE,
            true => IRQE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == IRQE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == IRQE_A::ENABLE
    }
}
impl core::ops::Deref for IRQE_R {
    type Target = crate::FieldReader<bool, IRQE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQE` writer - Interrupt Request Enable."]
pub struct IRQE_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IRQE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IRQE_A::ENABLE)
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
impl R {
    #[doc = "Bit 0 - SPI Enable."]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SPI Master Mode Enable."]
    #[inline(always)]
    pub fn mmen(&self) -> MMEN_R {
        MMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wired OR (open drain) Enable."]
    #[inline(always)]
    pub fn wor(&self) -> WOR_R {
        WOR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clock Polarity."]
    #[inline(always)]
    pub fn clkpol(&self) -> CLKPOL_R {
        CLKPOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Phase Select."]
    #[inline(always)]
    pub fn phase(&self) -> PHASE_R {
        PHASE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Baud Rate Generator Timer Interrupt Request."]
    #[inline(always)]
    pub fn birq(&self) -> BIRQ_R {
        BIRQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Start SPI Interrupt."]
    #[inline(always)]
    pub fn str(&self) -> STR_R {
        STR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt Request Enable."]
    #[inline(always)]
    pub fn irqe(&self) -> IRQE_R {
        IRQE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Enable."]
    #[inline(always)]
    pub fn spien(&mut self) -> SPIEN_W {
        SPIEN_W { w: self }
    }
    #[doc = "Bit 1 - SPI Master Mode Enable."]
    #[inline(always)]
    pub fn mmen(&mut self) -> MMEN_W {
        MMEN_W { w: self }
    }
    #[doc = "Bit 2 - Wired OR (open drain) Enable."]
    #[inline(always)]
    pub fn wor(&mut self) -> WOR_W {
        WOR_W { w: self }
    }
    #[doc = "Bit 3 - Clock Polarity."]
    #[inline(always)]
    pub fn clkpol(&mut self) -> CLKPOL_W {
        CLKPOL_W { w: self }
    }
    #[doc = "Bit 4 - Phase Select."]
    #[inline(always)]
    pub fn phase(&mut self) -> PHASE_W {
        PHASE_W { w: self }
    }
    #[doc = "Bit 5 - Baud Rate Generator Timer Interrupt Request."]
    #[inline(always)]
    pub fn birq(&mut self) -> BIRQ_W {
        BIRQ_W { w: self }
    }
    #[doc = "Bit 6 - Start SPI Interrupt."]
    #[inline(always)]
    pub fn str(&mut self) -> STR_W {
        STR_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt Request Enable."]
    #[inline(always)]
    pub fn irqe(&mut self) -> IRQE_W {
        IRQE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
