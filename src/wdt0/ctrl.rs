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
#[doc = "Watchdog Interrupt Period. The watchdog timer will assert an interrupt, if enabled, if the CPU does not write the watchdog reset sequence to the WDT_RST register before the watchdog timer has counted this time period since the last timer reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INT_PERIOD_A {
    #[doc = "0: 2**31 clock cycles."]
    WDT2POW31 = 0,
    #[doc = "1: 2**30 clock cycles."]
    WDT2POW30 = 1,
    #[doc = "2: 2**29 clock cycles."]
    WDT2POW29 = 2,
    #[doc = "3: 2**28 clock cycles."]
    WDT2POW28 = 3,
    #[doc = "4: 2^27 clock cycles."]
    WDT2POW27 = 4,
    #[doc = "5: 2**26 clock cycles."]
    WDT2POW26 = 5,
    #[doc = "6: 2**25 clock cycles."]
    WDT2POW25 = 6,
    #[doc = "7: 2**24 clock cycles."]
    WDT2POW24 = 7,
    #[doc = "8: 2**23 clock cycles."]
    WDT2POW23 = 8,
    #[doc = "9: 2**22 clock cycles."]
    WDT2POW22 = 9,
    #[doc = "10: 2**21 clock cycles."]
    WDT2POW21 = 10,
    #[doc = "11: 2**20 clock cycles."]
    WDT2POW20 = 11,
    #[doc = "12: 2**19 clock cycles."]
    WDT2POW19 = 12,
    #[doc = "13: 2**18 clock cycles."]
    WDT2POW18 = 13,
    #[doc = "14: 2**17 clock cycles."]
    WDT2POW17 = 14,
    #[doc = "15: 2**16 clock cycles."]
    WDT2POW16 = 15,
}
impl From<INT_PERIOD_A> for u8 {
    #[inline(always)]
    fn from(variant: INT_PERIOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INT_PERIOD` reader - Watchdog Interrupt Period. The watchdog timer will assert an interrupt, if enabled, if the CPU does not write the watchdog reset sequence to the WDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
pub struct INT_PERIOD_R(crate::FieldReader<u8, INT_PERIOD_A>);
impl INT_PERIOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INT_PERIOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_PERIOD_A {
        match self.bits {
            0 => INT_PERIOD_A::WDT2POW31,
            1 => INT_PERIOD_A::WDT2POW30,
            2 => INT_PERIOD_A::WDT2POW29,
            3 => INT_PERIOD_A::WDT2POW28,
            4 => INT_PERIOD_A::WDT2POW27,
            5 => INT_PERIOD_A::WDT2POW26,
            6 => INT_PERIOD_A::WDT2POW25,
            7 => INT_PERIOD_A::WDT2POW24,
            8 => INT_PERIOD_A::WDT2POW23,
            9 => INT_PERIOD_A::WDT2POW22,
            10 => INT_PERIOD_A::WDT2POW21,
            11 => INT_PERIOD_A::WDT2POW20,
            12 => INT_PERIOD_A::WDT2POW19,
            13 => INT_PERIOD_A::WDT2POW18,
            14 => INT_PERIOD_A::WDT2POW17,
            15 => INT_PERIOD_A::WDT2POW16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WDT2POW31`"]
    #[inline(always)]
    pub fn is_wdt2pow31(&self) -> bool {
        **self == INT_PERIOD_A::WDT2POW31
    }
    #[doc = "Checks if the value of the field is `WDT2POW30`"]
    #[inline(always)]
    pub fn is_wdt2pow30(&self) -> bool {
        **self == INT_PERIOD_A::WDT2POW30
    }
    #[doc = "Checks if the value of the field is `WDT2POW29`"]
    #[inline(always)]
    pub fn is_wdt2pow29(&self) -> bool {
        **self == INT_PERIOD_A::WDT2POW29
    }
    #[doc = "Checks if the value of the field is `WDT2POW28`"]
    #[inline(always)]
    pub fn is_wdt2pow28(&self) -> bool {
        **self == INT_PERIOD_A::WDT2POW28
    }
    #[doc = "Checks if the value of the field is `WDT2POW27`"]
    #[inline(always)]
    pub fn is_wdt2pow27(&self) -> bool {
        **self == INT_PERIOD_A::WDT2POW27
    }
    #[doc = "Checks if the value of the field is `WDT2POW26`"]
    #[inline(always)]
    pub fn is_wdt2pow26(&self) -> bool {
        **self == INT_PERIOD_A::WDT2POW26
    }
    #[doc = "Checks if the value of the field is `WDT2POW25`"]
    #[inline(always)]
    pub fn is_wdt2pow25(&self) -> bool {
        **self == INT_PERIOD_A::WDT2POW25
    }
    #[doc = "Checks if the value of the field is `WDT2POW24`"]
    #[inline(always)]
    pub fn is_wdt2pow24(&self) -> bool {
        **self == INT_PERIOD_A::WDT2POW24
    }
    #[doc = "Checks if the value of the field is `WDT2POW23`"]
    #[inline(always)]
    pub fn is_wdt2pow23(&self) -> bool {
        **self == INT_PERIOD_A::WDT2POW23
    }
    #[doc = "Checks if the value of the field is `WDT2POW22`"]
    #[inline(always)]
    pub fn is_wdt2pow22(&self) -> bool {
        **self == INT_PERIOD_A::WDT2POW22
    }
    #[doc = "Checks if the value of the field is `WDT2POW21`"]
    #[inline(always)]
    pub fn is_wdt2pow21(&self) -> bool {
        **self == INT_PERIOD_A::WDT2POW21
    }
    #[doc = "Checks if the value of the field is `WDT2POW20`"]
    #[inline(always)]
    pub fn is_wdt2pow20(&self) -> bool {
        **self == INT_PERIOD_A::WDT2POW20
    }
    #[doc = "Checks if the value of the field is `WDT2POW19`"]
    #[inline(always)]
    pub fn is_wdt2pow19(&self) -> bool {
        **self == INT_PERIOD_A::WDT2POW19
    }
    #[doc = "Checks if the value of the field is `WDT2POW18`"]
    #[inline(always)]
    pub fn is_wdt2pow18(&self) -> bool {
        **self == INT_PERIOD_A::WDT2POW18
    }
    #[doc = "Checks if the value of the field is `WDT2POW17`"]
    #[inline(always)]
    pub fn is_wdt2pow17(&self) -> bool {
        **self == INT_PERIOD_A::WDT2POW17
    }
    #[doc = "Checks if the value of the field is `WDT2POW16`"]
    #[inline(always)]
    pub fn is_wdt2pow16(&self) -> bool {
        **self == INT_PERIOD_A::WDT2POW16
    }
}
impl core::ops::Deref for INT_PERIOD_R {
    type Target = crate::FieldReader<u8, INT_PERIOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_PERIOD` writer - Watchdog Interrupt Period. The watchdog timer will assert an interrupt, if enabled, if the CPU does not write the watchdog reset sequence to the WDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
pub struct INT_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_PERIOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT_PERIOD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2**31 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow31(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::WDT2POW31)
    }
    #[doc = "2**30 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow30(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::WDT2POW30)
    }
    #[doc = "2**29 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow29(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::WDT2POW29)
    }
    #[doc = "2**28 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow28(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::WDT2POW28)
    }
    #[doc = "2^27 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow27(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::WDT2POW27)
    }
    #[doc = "2**26 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow26(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::WDT2POW26)
    }
    #[doc = "2**25 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow25(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::WDT2POW25)
    }
    #[doc = "2**24 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow24(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::WDT2POW24)
    }
    #[doc = "2**23 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow23(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::WDT2POW23)
    }
    #[doc = "2**22 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow22(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::WDT2POW22)
    }
    #[doc = "2**21 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow21(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::WDT2POW21)
    }
    #[doc = "2**20 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow20(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::WDT2POW20)
    }
    #[doc = "2**19 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow19(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::WDT2POW19)
    }
    #[doc = "2**18 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow18(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::WDT2POW18)
    }
    #[doc = "2**17 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow17(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::WDT2POW17)
    }
    #[doc = "2**16 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow16(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::WDT2POW16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Watchdog Reset Period. The watchdog timer will assert a reset, if enabled, if the CPU does not write the watchdog reset sequence to the WDT_RST register before the watchdog timer has counted this time period since the last timer reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RST_PERIOD_A {
    #[doc = "0: 2**31 clock cycles."]
    WDT2POW31 = 0,
    #[doc = "1: 2**30 clock cycles."]
    WDT2POW30 = 1,
    #[doc = "2: 2**29 clock cycles."]
    WDT2POW29 = 2,
    #[doc = "3: 2**28 clock cycles."]
    WDT2POW28 = 3,
    #[doc = "4: 2^27 clock cycles."]
    WDT2POW27 = 4,
    #[doc = "5: 2**26 clock cycles."]
    WDT2POW26 = 5,
    #[doc = "6: 2**25 clock cycles."]
    WDT2POW25 = 6,
    #[doc = "7: 2**24 clock cycles."]
    WDT2POW24 = 7,
    #[doc = "8: 2**23 clock cycles."]
    WDT2POW23 = 8,
    #[doc = "9: 2**22 clock cycles."]
    WDT2POW22 = 9,
    #[doc = "10: 2**21 clock cycles."]
    WDT2POW21 = 10,
    #[doc = "11: 2**20 clock cycles."]
    WDT2POW20 = 11,
    #[doc = "12: 2**19 clock cycles."]
    WDT2POW19 = 12,
    #[doc = "13: 2**18 clock cycles."]
    WDT2POW18 = 13,
    #[doc = "14: 2**17 clock cycles."]
    WDT2POW17 = 14,
    #[doc = "15: 2**16 clock cycles."]
    WDT2POW16 = 15,
}
impl From<RST_PERIOD_A> for u8 {
    #[inline(always)]
    fn from(variant: RST_PERIOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RST_PERIOD` reader - Watchdog Reset Period. The watchdog timer will assert a reset, if enabled, if the CPU does not write the watchdog reset sequence to the WDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
pub struct RST_PERIOD_R(crate::FieldReader<u8, RST_PERIOD_A>);
impl RST_PERIOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RST_PERIOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_PERIOD_A {
        match self.bits {
            0 => RST_PERIOD_A::WDT2POW31,
            1 => RST_PERIOD_A::WDT2POW30,
            2 => RST_PERIOD_A::WDT2POW29,
            3 => RST_PERIOD_A::WDT2POW28,
            4 => RST_PERIOD_A::WDT2POW27,
            5 => RST_PERIOD_A::WDT2POW26,
            6 => RST_PERIOD_A::WDT2POW25,
            7 => RST_PERIOD_A::WDT2POW24,
            8 => RST_PERIOD_A::WDT2POW23,
            9 => RST_PERIOD_A::WDT2POW22,
            10 => RST_PERIOD_A::WDT2POW21,
            11 => RST_PERIOD_A::WDT2POW20,
            12 => RST_PERIOD_A::WDT2POW19,
            13 => RST_PERIOD_A::WDT2POW18,
            14 => RST_PERIOD_A::WDT2POW17,
            15 => RST_PERIOD_A::WDT2POW16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WDT2POW31`"]
    #[inline(always)]
    pub fn is_wdt2pow31(&self) -> bool {
        **self == RST_PERIOD_A::WDT2POW31
    }
    #[doc = "Checks if the value of the field is `WDT2POW30`"]
    #[inline(always)]
    pub fn is_wdt2pow30(&self) -> bool {
        **self == RST_PERIOD_A::WDT2POW30
    }
    #[doc = "Checks if the value of the field is `WDT2POW29`"]
    #[inline(always)]
    pub fn is_wdt2pow29(&self) -> bool {
        **self == RST_PERIOD_A::WDT2POW29
    }
    #[doc = "Checks if the value of the field is `WDT2POW28`"]
    #[inline(always)]
    pub fn is_wdt2pow28(&self) -> bool {
        **self == RST_PERIOD_A::WDT2POW28
    }
    #[doc = "Checks if the value of the field is `WDT2POW27`"]
    #[inline(always)]
    pub fn is_wdt2pow27(&self) -> bool {
        **self == RST_PERIOD_A::WDT2POW27
    }
    #[doc = "Checks if the value of the field is `WDT2POW26`"]
    #[inline(always)]
    pub fn is_wdt2pow26(&self) -> bool {
        **self == RST_PERIOD_A::WDT2POW26
    }
    #[doc = "Checks if the value of the field is `WDT2POW25`"]
    #[inline(always)]
    pub fn is_wdt2pow25(&self) -> bool {
        **self == RST_PERIOD_A::WDT2POW25
    }
    #[doc = "Checks if the value of the field is `WDT2POW24`"]
    #[inline(always)]
    pub fn is_wdt2pow24(&self) -> bool {
        **self == RST_PERIOD_A::WDT2POW24
    }
    #[doc = "Checks if the value of the field is `WDT2POW23`"]
    #[inline(always)]
    pub fn is_wdt2pow23(&self) -> bool {
        **self == RST_PERIOD_A::WDT2POW23
    }
    #[doc = "Checks if the value of the field is `WDT2POW22`"]
    #[inline(always)]
    pub fn is_wdt2pow22(&self) -> bool {
        **self == RST_PERIOD_A::WDT2POW22
    }
    #[doc = "Checks if the value of the field is `WDT2POW21`"]
    #[inline(always)]
    pub fn is_wdt2pow21(&self) -> bool {
        **self == RST_PERIOD_A::WDT2POW21
    }
    #[doc = "Checks if the value of the field is `WDT2POW20`"]
    #[inline(always)]
    pub fn is_wdt2pow20(&self) -> bool {
        **self == RST_PERIOD_A::WDT2POW20
    }
    #[doc = "Checks if the value of the field is `WDT2POW19`"]
    #[inline(always)]
    pub fn is_wdt2pow19(&self) -> bool {
        **self == RST_PERIOD_A::WDT2POW19
    }
    #[doc = "Checks if the value of the field is `WDT2POW18`"]
    #[inline(always)]
    pub fn is_wdt2pow18(&self) -> bool {
        **self == RST_PERIOD_A::WDT2POW18
    }
    #[doc = "Checks if the value of the field is `WDT2POW17`"]
    #[inline(always)]
    pub fn is_wdt2pow17(&self) -> bool {
        **self == RST_PERIOD_A::WDT2POW17
    }
    #[doc = "Checks if the value of the field is `WDT2POW16`"]
    #[inline(always)]
    pub fn is_wdt2pow16(&self) -> bool {
        **self == RST_PERIOD_A::WDT2POW16
    }
}
impl core::ops::Deref for RST_PERIOD_R {
    type Target = crate::FieldReader<u8, RST_PERIOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST_PERIOD` writer - Watchdog Reset Period. The watchdog timer will assert a reset, if enabled, if the CPU does not write the watchdog reset sequence to the WDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
pub struct RST_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_PERIOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RST_PERIOD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2**31 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow31(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::WDT2POW31)
    }
    #[doc = "2**30 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow30(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::WDT2POW30)
    }
    #[doc = "2**29 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow29(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::WDT2POW29)
    }
    #[doc = "2**28 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow28(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::WDT2POW28)
    }
    #[doc = "2^27 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow27(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::WDT2POW27)
    }
    #[doc = "2**26 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow26(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::WDT2POW26)
    }
    #[doc = "2**25 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow25(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::WDT2POW25)
    }
    #[doc = "2**24 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow24(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::WDT2POW24)
    }
    #[doc = "2**23 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow23(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::WDT2POW23)
    }
    #[doc = "2**22 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow22(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::WDT2POW22)
    }
    #[doc = "2**21 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow21(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::WDT2POW21)
    }
    #[doc = "2**20 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow20(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::WDT2POW20)
    }
    #[doc = "2**19 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow19(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::WDT2POW19)
    }
    #[doc = "2**18 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow18(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::WDT2POW18)
    }
    #[doc = "2**17 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow17(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::WDT2POW17)
    }
    #[doc = "2**16 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow16(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::WDT2POW16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Watchdog Timer Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT_EN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<WDT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WDT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT_EN` reader - Watchdog Timer Enable."]
pub struct WDT_EN_R(crate::FieldReader<bool, WDT_EN_A>);
impl WDT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT_EN_A {
        match self.bits {
            false => WDT_EN_A::DIS,
            true => WDT_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == WDT_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == WDT_EN_A::EN
    }
}
impl core::ops::Deref for WDT_EN_R {
    type Target = crate::FieldReader<bool, WDT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_EN` writer - Watchdog Timer Enable."]
pub struct WDT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(WDT_EN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(WDT_EN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Watchdog Timer Interrupt Flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_FLAG_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<INT_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: INT_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_FLAG` reader - Watchdog Timer Interrupt Flag."]
pub struct INT_FLAG_R(crate::FieldReader<bool, INT_FLAG_A>);
impl INT_FLAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT_FLAG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_FLAG_A {
        match self.bits {
            false => INT_FLAG_A::INACTIVE,
            true => INT_FLAG_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == INT_FLAG_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == INT_FLAG_A::PENDING
    }
}
impl core::ops::Deref for INT_FLAG_R {
    type Target = crate::FieldReader<bool, INT_FLAG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_FLAG` writer - Watchdog Timer Interrupt Flag."]
pub struct INT_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_FLAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT_FLAG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(INT_FLAG_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(INT_FLAG_A::PENDING)
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
#[doc = "Watchdog Timer Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_EN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EN` reader - Watchdog Timer Interrupt Enable."]
pub struct INT_EN_R(crate::FieldReader<bool, INT_EN_A>);
impl INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN_A {
        match self.bits {
            false => INT_EN_A::DIS,
            true => INT_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == INT_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == INT_EN_A::EN
    }
}
impl core::ops::Deref for INT_EN_R {
    type Target = crate::FieldReader<bool, INT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_EN` writer - Watchdog Timer Interrupt Enable."]
pub struct INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(INT_EN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(INT_EN_A::EN)
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
#[doc = "Watchdog Timer Reset Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RST_EN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<RST_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RST_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST_EN` reader - Watchdog Timer Reset Enable."]
pub struct RST_EN_R(crate::FieldReader<bool, RST_EN_A>);
impl RST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_EN_A {
        match self.bits {
            false => RST_EN_A::DIS,
            true => RST_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RST_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == RST_EN_A::EN
    }
}
impl core::ops::Deref for RST_EN_R {
    type Target = crate::FieldReader<bool, RST_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST_EN` writer - Watchdog Timer Reset Enable."]
pub struct RST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RST_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RST_EN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RST_EN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Watchdog Timer Reset Flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RST_FLAG_A {
    #[doc = "0: The event has not occurred."]
    NOEVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<RST_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: RST_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST_FLAG` reader - Watchdog Timer Reset Flag."]
pub struct RST_FLAG_R(crate::FieldReader<bool, RST_FLAG_A>);
impl RST_FLAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_FLAG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_FLAG_A {
        match self.bits {
            false => RST_FLAG_A::NOEVENT,
            true => RST_FLAG_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == RST_FLAG_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == RST_FLAG_A::OCCURRED
    }
}
impl core::ops::Deref for RST_FLAG_R {
    type Target = crate::FieldReader<bool, RST_FLAG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST_FLAG` writer - Watchdog Timer Reset Flag."]
pub struct RST_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_FLAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RST_FLAG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(RST_FLAG_A::NOEVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(RST_FLAG_A::OCCURRED)
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
    #[doc = "Bits 0:3 - Watchdog Interrupt Period. The watchdog timer will assert an interrupt, if enabled, if the CPU does not write the watchdog reset sequence to the WDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
    #[inline(always)]
    pub fn int_period(&self) -> INT_PERIOD_R {
        INT_PERIOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Watchdog Reset Period. The watchdog timer will assert a reset, if enabled, if the CPU does not write the watchdog reset sequence to the WDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
    #[inline(always)]
    pub fn rst_period(&self) -> RST_PERIOD_R {
        RST_PERIOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Watchdog Timer Enable."]
    #[inline(always)]
    pub fn wdt_en(&self) -> WDT_EN_R {
        WDT_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Watchdog Timer Interrupt Flag."]
    #[inline(always)]
    pub fn int_flag(&self) -> INT_FLAG_R {
        INT_FLAG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Watchdog Timer Interrupt Enable."]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Watchdog Timer Reset Enable."]
    #[inline(always)]
    pub fn rst_en(&self) -> RST_EN_R {
        RST_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Watchdog Timer Reset Flag."]
    #[inline(always)]
    pub fn rst_flag(&self) -> RST_FLAG_R {
        RST_FLAG_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Watchdog Interrupt Period. The watchdog timer will assert an interrupt, if enabled, if the CPU does not write the watchdog reset sequence to the WDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
    #[inline(always)]
    pub fn int_period(&mut self) -> INT_PERIOD_W {
        INT_PERIOD_W { w: self }
    }
    #[doc = "Bits 4:7 - Watchdog Reset Period. The watchdog timer will assert a reset, if enabled, if the CPU does not write the watchdog reset sequence to the WDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
    #[inline(always)]
    pub fn rst_period(&mut self) -> RST_PERIOD_W {
        RST_PERIOD_W { w: self }
    }
    #[doc = "Bit 8 - Watchdog Timer Enable."]
    #[inline(always)]
    pub fn wdt_en(&mut self) -> WDT_EN_W {
        WDT_EN_W { w: self }
    }
    #[doc = "Bit 9 - Watchdog Timer Interrupt Flag."]
    #[inline(always)]
    pub fn int_flag(&mut self) -> INT_FLAG_W {
        INT_FLAG_W { w: self }
    }
    #[doc = "Bit 10 - Watchdog Timer Interrupt Enable."]
    #[inline(always)]
    pub fn int_en(&mut self) -> INT_EN_W {
        INT_EN_W { w: self }
    }
    #[doc = "Bit 11 - Watchdog Timer Reset Enable."]
    #[inline(always)]
    pub fn rst_en(&mut self) -> RST_EN_W {
        RST_EN_W { w: self }
    }
    #[doc = "Bit 31 - Watchdog Timer Reset Flag."]
    #[inline(always)]
    pub fn rst_flag(&mut self) -> RST_FLAG_W {
        RST_FLAG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Timer Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
