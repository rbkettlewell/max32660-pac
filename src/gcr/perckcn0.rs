#[doc = "Register `PERCKCN0` reader"]
pub struct R(crate::R<PERCKCN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERCKCN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERCKCN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERCKCN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERCKCN0` writer"]
pub struct W(crate::W<PERCKCN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERCKCN0_SPEC>;
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
impl From<crate::W<PERCKCN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERCKCN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIO0 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO0D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<GPIO0D_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO0D` reader - GPIO0 Disable."]
pub struct GPIO0D_R(crate::FieldReader<bool, GPIO0D_A>);
impl GPIO0D_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO0D_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0D_A {
        match self.bits {
            false => GPIO0D_A::EN,
            true => GPIO0D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == GPIO0D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO0D_A::DIS
    }
}
impl core::ops::Deref for GPIO0D_R {
    type Target = crate::FieldReader<bool, GPIO0D_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO0D` writer - GPIO0 Disable."]
pub struct GPIO0D_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO0D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO0D_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(GPIO0D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO0D_A::DIS)
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
#[doc = "DMA Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAD_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<DMAD_A> for bool {
    #[inline(always)]
    fn from(variant: DMAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAD` reader - DMA Disable."]
pub struct DMAD_R(crate::FieldReader<bool, DMAD_A>);
impl DMAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAD_A {
        match self.bits {
            false => DMAD_A::EN,
            true => DMAD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == DMAD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == DMAD_A::DIS
    }
}
impl core::ops::Deref for DMAD_R {
    type Target = crate::FieldReader<bool, DMAD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAD` writer - DMA Disable."]
pub struct DMAD_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DMAD_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DMAD_A::DIS)
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
#[doc = "SPI 0 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<SPI0D_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI0D` reader - SPI 0 Disable."]
pub struct SPI0D_R(crate::FieldReader<bool, SPI0D_A>);
impl SPI0D_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI0D_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0D_A {
        match self.bits {
            false => SPI0D_A::EN,
            true => SPI0D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == SPI0D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == SPI0D_A::DIS
    }
}
impl core::ops::Deref for SPI0D_R {
    type Target = crate::FieldReader<bool, SPI0D_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI0D` writer - SPI 0 Disable."]
pub struct SPI0D_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0D_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SPI0D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SPI0D_A::DIS)
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
#[doc = "SPI 1 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<SPI1D_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI1D` reader - SPI 1 Disable."]
pub struct SPI1D_R(crate::FieldReader<bool, SPI1D_A>);
impl SPI1D_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI1D_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1D_A {
        match self.bits {
            false => SPI1D_A::EN,
            true => SPI1D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == SPI1D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == SPI1D_A::DIS
    }
}
impl core::ops::Deref for SPI1D_R {
    type Target = crate::FieldReader<bool, SPI1D_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1D` writer - SPI 1 Disable."]
pub struct SPI1D_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1D_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SPI1D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SPI1D_A::DIS)
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
#[doc = "UART 0 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<UART0D_A> for bool {
    #[inline(always)]
    fn from(variant: UART0D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART0D` reader - UART 0 Disable."]
pub struct UART0D_R(crate::FieldReader<bool, UART0D_A>);
impl UART0D_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART0D_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0D_A {
        match self.bits {
            false => UART0D_A::EN,
            true => UART0D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == UART0D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == UART0D_A::DIS
    }
}
impl core::ops::Deref for UART0D_R {
    type Target = crate::FieldReader<bool, UART0D_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART0D` writer - UART 0 Disable."]
pub struct UART0D_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0D_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(UART0D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(UART0D_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "UART 1 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<UART1D_A> for bool {
    #[inline(always)]
    fn from(variant: UART1D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART1D` reader - UART 1 Disable."]
pub struct UART1D_R(crate::FieldReader<bool, UART1D_A>);
impl UART1D_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART1D_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1D_A {
        match self.bits {
            false => UART1D_A::EN,
            true => UART1D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == UART1D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == UART1D_A::DIS
    }
}
impl core::ops::Deref for UART1D_R {
    type Target = crate::FieldReader<bool, UART1D_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART1D` writer - UART 1 Disable."]
pub struct UART1D_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART1D_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(UART1D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(UART1D_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "I2C 0 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<I2C0D_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C0D` reader - I2C 0 Disable."]
pub struct I2C0D_R(crate::FieldReader<bool, I2C0D_A>);
impl I2C0D_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C0D_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0D_A {
        match self.bits {
            false => I2C0D_A::EN,
            true => I2C0D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == I2C0D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == I2C0D_A::DIS
    }
}
impl core::ops::Deref for I2C0D_R {
    type Target = crate::FieldReader<bool, I2C0D_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C0D` writer - I2C 0 Disable."]
pub struct I2C0D_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0D_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2C0D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2C0D_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Timer 0 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T0D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<T0D_A> for bool {
    #[inline(always)]
    fn from(variant: T0D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `T0D` reader - Timer 0 Disable."]
pub struct T0D_R(crate::FieldReader<bool, T0D_A>);
impl T0D_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        T0D_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T0D_A {
        match self.bits {
            false => T0D_A::EN,
            true => T0D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == T0D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == T0D_A::DIS
    }
}
impl core::ops::Deref for T0D_R {
    type Target = crate::FieldReader<bool, T0D_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0D` writer - Timer 0 Disable."]
pub struct T0D_W<'a> {
    w: &'a mut W,
}
impl<'a> T0D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: T0D_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(T0D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(T0D_A::DIS)
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
#[doc = "Timer 1 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T1D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<T1D_A> for bool {
    #[inline(always)]
    fn from(variant: T1D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `T1D` reader - Timer 1 Disable."]
pub struct T1D_R(crate::FieldReader<bool, T1D_A>);
impl T1D_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        T1D_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T1D_A {
        match self.bits {
            false => T1D_A::EN,
            true => T1D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == T1D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == T1D_A::DIS
    }
}
impl core::ops::Deref for T1D_R {
    type Target = crate::FieldReader<bool, T1D_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T1D` writer - Timer 1 Disable."]
pub struct T1D_W<'a> {
    w: &'a mut W,
}
impl<'a> T1D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: T1D_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(T1D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(T1D_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Timer 2 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T2D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<T2D_A> for bool {
    #[inline(always)]
    fn from(variant: T2D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `T2D` reader - Timer 2 Disable."]
pub struct T2D_R(crate::FieldReader<bool, T2D_A>);
impl T2D_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        T2D_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T2D_A {
        match self.bits {
            false => T2D_A::EN,
            true => T2D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == T2D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == T2D_A::DIS
    }
}
impl core::ops::Deref for T2D_R {
    type Target = crate::FieldReader<bool, T2D_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T2D` writer - Timer 2 Disable."]
pub struct T2D_W<'a> {
    w: &'a mut W,
}
impl<'a> T2D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: T2D_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(T2D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(T2D_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "I2C 1 Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1D_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<I2C1D_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1D` reader - I2C 1 Disable."]
pub struct I2C1D_R(crate::FieldReader<bool, I2C1D_A>);
impl I2C1D_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C1D_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1D_A {
        match self.bits {
            false => I2C1D_A::EN,
            true => I2C1D_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == I2C1D_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == I2C1D_A::DIS
    }
}
impl core::ops::Deref for I2C1D_R {
    type Target = crate::FieldReader<bool, I2C1D_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1D` writer - I2C 1 Disable."]
pub struct I2C1D_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1D_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2C1D_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2C1D_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - GPIO0 Disable."]
    #[inline(always)]
    pub fn gpio0d(&self) -> GPIO0D_R {
        GPIO0D_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5 - DMA Disable."]
    #[inline(always)]
    pub fn dmad(&self) -> DMAD_R {
        DMAD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SPI 0 Disable."]
    #[inline(always)]
    pub fn spi0d(&self) -> SPI0D_R {
        SPI0D_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SPI 1 Disable."]
    #[inline(always)]
    pub fn spi1d(&self) -> SPI1D_R {
        SPI1D_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UART 0 Disable."]
    #[inline(always)]
    pub fn uart0d(&self) -> UART0D_R {
        UART0D_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UART 1 Disable."]
    #[inline(always)]
    pub fn uart1d(&self) -> UART1D_R {
        UART1D_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - I2C 0 Disable."]
    #[inline(always)]
    pub fn i2c0d(&self) -> I2C0D_R {
        I2C0D_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Timer 0 Disable."]
    #[inline(always)]
    pub fn t0d(&self) -> T0D_R {
        T0D_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Timer 1 Disable."]
    #[inline(always)]
    pub fn t1d(&self) -> T1D_R {
        T1D_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Timer 2 Disable."]
    #[inline(always)]
    pub fn t2d(&self) -> T2D_R {
        T2D_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 28 - I2C 1 Disable."]
    #[inline(always)]
    pub fn i2c1d(&self) -> I2C1D_R {
        I2C1D_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO0 Disable."]
    #[inline(always)]
    pub fn gpio0d(&mut self) -> GPIO0D_W {
        GPIO0D_W { w: self }
    }
    #[doc = "Bit 5 - DMA Disable."]
    #[inline(always)]
    pub fn dmad(&mut self) -> DMAD_W {
        DMAD_W { w: self }
    }
    #[doc = "Bit 6 - SPI 0 Disable."]
    #[inline(always)]
    pub fn spi0d(&mut self) -> SPI0D_W {
        SPI0D_W { w: self }
    }
    #[doc = "Bit 7 - SPI 1 Disable."]
    #[inline(always)]
    pub fn spi1d(&mut self) -> SPI1D_W {
        SPI1D_W { w: self }
    }
    #[doc = "Bit 9 - UART 0 Disable."]
    #[inline(always)]
    pub fn uart0d(&mut self) -> UART0D_W {
        UART0D_W { w: self }
    }
    #[doc = "Bit 10 - UART 1 Disable."]
    #[inline(always)]
    pub fn uart1d(&mut self) -> UART1D_W {
        UART1D_W { w: self }
    }
    #[doc = "Bit 13 - I2C 0 Disable."]
    #[inline(always)]
    pub fn i2c0d(&mut self) -> I2C0D_W {
        I2C0D_W { w: self }
    }
    #[doc = "Bit 15 - Timer 0 Disable."]
    #[inline(always)]
    pub fn t0d(&mut self) -> T0D_W {
        T0D_W { w: self }
    }
    #[doc = "Bit 16 - Timer 1 Disable."]
    #[inline(always)]
    pub fn t1d(&mut self) -> T1D_W {
        T1D_W { w: self }
    }
    #[doc = "Bit 17 - Timer 2 Disable."]
    #[inline(always)]
    pub fn t2d(&mut self) -> T2D_W {
        T2D_W { w: self }
    }
    #[doc = "Bit 28 - I2C 1 Disable."]
    #[inline(always)]
    pub fn i2c1d(&mut self) -> I2C1D_W {
        I2C1D_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Clock Disable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perckcn0](index.html) module"]
pub struct PERCKCN0_SPEC;
impl crate::RegisterSpec for PERCKCN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perckcn0::R](R) reader structure"]
impl crate::Readable for PERCKCN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perckcn0::W](W) writer structure"]
impl crate::Writable for PERCKCN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERCKCN0 to value 0"]
impl crate::Resettable for PERCKCN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
