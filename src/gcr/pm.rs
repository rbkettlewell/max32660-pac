#[doc = "Register `PM` reader"]
pub struct R(crate::R<PM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PM` writer"]
pub struct W(crate::W<PM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PM_SPEC>;
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
impl From<crate::W<PM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Active Mode."]
    ACTIVE = 0,
    #[doc = "3: Shutdown Mode."]
    SHUTDOWN = 3,
    #[doc = "4: Backup Mode."]
    BACKUP = 4,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode."]
pub struct MODE_R(crate::FieldReader<u8, MODE_A>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::ACTIVE),
            3 => Some(MODE_A::SHUTDOWN),
            4 => Some(MODE_A::BACKUP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == MODE_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        **self == MODE_A::SHUTDOWN
    }
    #[doc = "Checks if the value of the field is `BACKUP`"]
    #[inline(always)]
    pub fn is_backup(&self) -> bool {
        **self == MODE_A::BACKUP
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode."]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Active Mode."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(MODE_A::ACTIVE)
    }
    #[doc = "Shutdown Mode."]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(MODE_A::SHUTDOWN)
    }
    #[doc = "Backup Mode."]
    #[inline(always)]
    pub fn backup(self) -> &'a mut W {
        self.variant(MODE_A::BACKUP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOWKEN_A {
    #[doc = "0: Wake Up Disable."]
    DIS = 0,
    #[doc = "1: Wake Up Enable."]
    EN = 1,
}
impl From<GPIOWKEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOWKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOWKEN` reader - GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set."]
pub struct GPIOWKEN_R(crate::FieldReader<bool, GPIOWKEN_A>);
impl GPIOWKEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIOWKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOWKEN_A {
        match self.bits {
            false => GPIOWKEN_A::DIS,
            true => GPIOWKEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIOWKEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == GPIOWKEN_A::EN
    }
}
impl core::ops::Deref for GPIOWKEN_R {
    type Target = crate::FieldReader<bool, GPIOWKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOWKEN` writer - GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set."]
pub struct GPIOWKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOWKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOWKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Wake Up Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIOWKEN_A::DIS)
    }
    #[doc = "Wake Up Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(GPIOWKEN_A::EN)
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
#[doc = "RTC Alarm Wake Up Enable. This bit enables RTC alarm as wakeup source. If enabled, the desired RTC alarm must be configured via the RTC control registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCWKEN_A {
    #[doc = "0: Wake Up Disable."]
    DIS = 0,
    #[doc = "1: Wake Up Enable."]
    EN = 1,
}
impl From<RTCWKEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCWKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCWKEN` reader - RTC Alarm Wake Up Enable. This bit enables RTC alarm as wakeup source. If enabled, the desired RTC alarm must be configured via the RTC control registers."]
pub struct RTCWKEN_R(crate::FieldReader<bool, RTCWKEN_A>);
impl RTCWKEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTCWKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCWKEN_A {
        match self.bits {
            false => RTCWKEN_A::DIS,
            true => RTCWKEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RTCWKEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == RTCWKEN_A::EN
    }
}
impl core::ops::Deref for RTCWKEN_R {
    type Target = crate::FieldReader<bool, RTCWKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCWKEN` writer - RTC Alarm Wake Up Enable. This bit enables RTC alarm as wakeup source. If enabled, the desired RTC alarm must be configured via the RTC control registers."]
pub struct RTCWKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCWKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCWKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Wake Up Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RTCWKEN_A::DIS)
    }
    #[doc = "Wake Up Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RTCWKEN_A::EN)
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
#[doc = "HIRC Power Down. This bit selects HIRC power state in DEEPSLEEP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIRCPD_A {
    #[doc = "0: Mode is Active."]
    ACTIVE = 0,
    #[doc = "1: Powered down in DEEPSLEEP."]
    DEEPSLEEP = 1,
}
impl From<HIRCPD_A> for bool {
    #[inline(always)]
    fn from(variant: HIRCPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIRCPD` reader - HIRC Power Down. This bit selects HIRC power state in DEEPSLEEP mode."]
pub struct HIRCPD_R(crate::FieldReader<bool, HIRCPD_A>);
impl HIRCPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HIRCPD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIRCPD_A {
        match self.bits {
            false => HIRCPD_A::ACTIVE,
            true => HIRCPD_A::DEEPSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == HIRCPD_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        **self == HIRCPD_A::DEEPSLEEP
    }
}
impl core::ops::Deref for HIRCPD_R {
    type Target = crate::FieldReader<bool, HIRCPD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIRCPD` writer - HIRC Power Down. This bit selects HIRC power state in DEEPSLEEP mode."]
pub struct HIRCPD_W<'a> {
    w: &'a mut W,
}
impl<'a> HIRCPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIRCPD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mode is Active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(HIRCPD_A::ACTIVE)
    }
    #[doc = "Powered down in DEEPSLEEP."]
    #[inline(always)]
    pub fn deepsleep(self) -> &'a mut W {
        self.variant(HIRCPD_A::DEEPSLEEP)
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
impl R {
    #[doc = "Bits 0:2 - Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set."]
    #[inline(always)]
    pub fn gpiowken(&self) -> GPIOWKEN_R {
        GPIOWKEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RTC Alarm Wake Up Enable. This bit enables RTC alarm as wakeup source. If enabled, the desired RTC alarm must be configured via the RTC control registers."]
    #[inline(always)]
    pub fn rtcwken(&self) -> RTCWKEN_R {
        RTCWKEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 15 - HIRC Power Down. This bit selects HIRC power state in DEEPSLEEP mode."]
    #[inline(always)]
    pub fn hircpd(&self) -> HIRCPD_R {
        HIRCPD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 4 - GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set."]
    #[inline(always)]
    pub fn gpiowken(&mut self) -> GPIOWKEN_W {
        GPIOWKEN_W { w: self }
    }
    #[doc = "Bit 5 - RTC Alarm Wake Up Enable. This bit enables RTC alarm as wakeup source. If enabled, the desired RTC alarm must be configured via the RTC control registers."]
    #[inline(always)]
    pub fn rtcwken(&mut self) -> RTCWKEN_W {
        RTCWKEN_W { w: self }
    }
    #[doc = "Bit 15 - HIRC Power Down. This bit selects HIRC power state in DEEPSLEEP mode."]
    #[inline(always)]
    pub fn hircpd(&mut self) -> HIRCPD_W {
        HIRCPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Management.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pm](index.html) module"]
pub struct PM_SPEC;
impl crate::RegisterSpec for PM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pm::R](R) reader structure"]
impl crate::Readable for PM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pm::W](W) writer structure"]
impl crate::Writable for PM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PM to value 0"]
impl crate::Resettable for PM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
