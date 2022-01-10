#[doc = "Register `SCON` reader"]
pub struct R(crate::R<SCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCON` writer"]
pub struct W(crate::W<SCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCON_SPEC>;
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
impl From<crate::W<SCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "System bus abritration scheme. These bits are used to select between Fixed-burst abritration and Round-Robin scheme. The Round-Robin scheme is selected by default. These bits are reset by the system reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SBUSARB_A {
    #[doc = "0: Fixed Burst abritration."]
    FIX = 0,
    #[doc = "1: Round-robin scheme."]
    ROUND = 1,
}
impl From<SBUSARB_A> for u8 {
    #[inline(always)]
    fn from(variant: SBUSARB_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SBUSARB` reader - System bus abritration scheme. These bits are used to select between Fixed-burst abritration and Round-Robin scheme. The Round-Robin scheme is selected by default. These bits are reset by the system reset."]
pub struct SBUSARB_R(crate::FieldReader<u8, SBUSARB_A>);
impl SBUSARB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SBUSARB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SBUSARB_A> {
        match self.bits {
            0 => Some(SBUSARB_A::FIX),
            1 => Some(SBUSARB_A::ROUND),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FIX`"]
    #[inline(always)]
    pub fn is_fix(&self) -> bool {
        **self == SBUSARB_A::FIX
    }
    #[doc = "Checks if the value of the field is `ROUND`"]
    #[inline(always)]
    pub fn is_round(&self) -> bool {
        **self == SBUSARB_A::ROUND
    }
}
impl core::ops::Deref for SBUSARB_R {
    type Target = crate::FieldReader<u8, SBUSARB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBUSARB` writer - System bus abritration scheme. These bits are used to select between Fixed-burst abritration and Round-Robin scheme. The Round-Robin scheme is selected by default. These bits are reset by the system reset."]
pub struct SBUSARB_W<'a> {
    w: &'a mut W,
}
impl<'a> SBUSARB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBUSARB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Fixed Burst abritration."]
    #[inline(always)]
    pub fn fix(self) -> &'a mut W {
        self.variant(SBUSARB_A::FIX)
    }
    #[doc = "Round-robin scheme."]
    #[inline(always)]
    pub fn round(self) -> &'a mut W {
        self.variant(SBUSARB_A::ROUND)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_PAGE_FLIP_A {
    #[doc = "0: Physical layout matches logical layout."]
    NORMAL = 0,
    #[doc = "1: Bottom half mapped to logical top half and vice versa."]
    SWAPPED = 1,
}
impl From<FLASH_PAGE_FLIP_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_PAGE_FLIP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASH_PAGE_FLIP` reader - Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer."]
pub struct FLASH_PAGE_FLIP_R(crate::FieldReader<bool, FLASH_PAGE_FLIP_A>);
impl FLASH_PAGE_FLIP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_PAGE_FLIP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_PAGE_FLIP_A {
        match self.bits {
            false => FLASH_PAGE_FLIP_A::NORMAL,
            true => FLASH_PAGE_FLIP_A::SWAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == FLASH_PAGE_FLIP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SWAPPED`"]
    #[inline(always)]
    pub fn is_swapped(&self) -> bool {
        **self == FLASH_PAGE_FLIP_A::SWAPPED
    }
}
impl core::ops::Deref for FLASH_PAGE_FLIP_R {
    type Target = crate::FieldReader<bool, FLASH_PAGE_FLIP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_PAGE_FLIP` writer - Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer."]
pub struct FLASH_PAGE_FLIP_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PAGE_FLIP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_PAGE_FLIP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Physical layout matches logical layout."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(FLASH_PAGE_FLIP_A::NORMAL)
    }
    #[doc = "Bottom half mapped to logical top half and vice versa."]
    #[inline(always)]
    pub fn swapped(self) -> &'a mut W {
        self.variant(FLASH_PAGE_FLIP_A::SWAPPED)
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
#[doc = "Floating Point Unit Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPU_DIS_A {
    #[doc = "0: enable Floating point unit"]
    ENABLE = 0,
    #[doc = "1: disable floating point unit"]
    DISABLE = 1,
}
impl From<FPU_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPU_DIS` reader - Floating Point Unit Disable"]
pub struct FPU_DIS_R(crate::FieldReader<bool, FPU_DIS_A>);
impl FPU_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPU_DIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPU_DIS_A {
        match self.bits {
            false => FPU_DIS_A::ENABLE,
            true => FPU_DIS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FPU_DIS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FPU_DIS_A::DISABLE
    }
}
impl core::ops::Deref for FPU_DIS_R {
    type Target = crate::FieldReader<bool, FPU_DIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPU_DIS` writer - Floating Point Unit Disable"]
pub struct FPU_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FPU_DIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPU_DIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "enable Floating point unit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FPU_DIS_A::ENABLE)
    }
    #[doc = "disable floating point unit"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FPU_DIS_A::DISABLE)
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
#[doc = "Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCACHE_FLUSH_A {
    #[doc = "0: Normal Code Cache Operation"]
    NORMAL = 0,
    #[doc = "1: Code Caches and CPU instruction buffer are flushed"]
    FLUSH = 1,
}
impl From<CCACHE_FLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: CCACHE_FLUSH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCACHE_FLUSH` reader - Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4."]
pub struct CCACHE_FLUSH_R(crate::FieldReader<bool, CCACHE_FLUSH_A>);
impl CCACHE_FLUSH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCACHE_FLUSH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCACHE_FLUSH_A {
        match self.bits {
            false => CCACHE_FLUSH_A::NORMAL,
            true => CCACHE_FLUSH_A::FLUSH,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == CCACHE_FLUSH_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `FLUSH`"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        **self == CCACHE_FLUSH_A::FLUSH
    }
}
impl core::ops::Deref for CCACHE_FLUSH_R {
    type Target = crate::FieldReader<bool, CCACHE_FLUSH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCACHE_FLUSH` writer - Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4."]
pub struct CCACHE_FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> CCACHE_FLUSH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCACHE_FLUSH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal Code Cache Operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CCACHE_FLUSH_A::NORMAL)
    }
    #[doc = "Code Caches and CPU instruction buffer are flushed"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(CCACHE_FLUSH_A::FLUSH)
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
#[doc = "Serial Wire Debug Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWD_DIS_A {
    #[doc = "0: Enable JTAG SWD"]
    ENABLE = 0,
    #[doc = "1: Disable JTAG SWD"]
    DISABLE = 1,
}
impl From<SWD_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: SWD_DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWD_DIS` reader - Serial Wire Debug Disable"]
pub struct SWD_DIS_R(crate::FieldReader<bool, SWD_DIS_A>);
impl SWD_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWD_DIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWD_DIS_A {
        match self.bits {
            false => SWD_DIS_A::ENABLE,
            true => SWD_DIS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SWD_DIS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SWD_DIS_A::DISABLE
    }
}
impl core::ops::Deref for SWD_DIS_R {
    type Target = crate::FieldReader<bool, SWD_DIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWD_DIS` writer - Serial Wire Debug Disable"]
pub struct SWD_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SWD_DIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWD_DIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable JTAG SWD"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SWD_DIS_A::ENABLE)
    }
    #[doc = "Disable JTAG SWD"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SWD_DIS_A::DISABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:2 - System bus abritration scheme. These bits are used to select between Fixed-burst abritration and Round-Robin scheme. The Round-Robin scheme is selected by default. These bits are reset by the system reset."]
    #[inline(always)]
    pub fn sbusarb(&self) -> SBUSARB_R {
        SBUSARB_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer."]
    #[inline(always)]
    pub fn flash_page_flip(&self) -> FLASH_PAGE_FLIP_R {
        FLASH_PAGE_FLIP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Floating Point Unit Disable"]
    #[inline(always)]
    pub fn fpu_dis(&self) -> FPU_DIS_R {
        FPU_DIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4."]
    #[inline(always)]
    pub fn ccache_flush(&self) -> CCACHE_FLUSH_R {
        CCACHE_FLUSH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Serial Wire Debug Disable"]
    #[inline(always)]
    pub fn swd_dis(&self) -> SWD_DIS_R {
        SWD_DIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:2 - System bus abritration scheme. These bits are used to select between Fixed-burst abritration and Round-Robin scheme. The Round-Robin scheme is selected by default. These bits are reset by the system reset."]
    #[inline(always)]
    pub fn sbusarb(&mut self) -> SBUSARB_W {
        SBUSARB_W { w: self }
    }
    #[doc = "Bit 4 - Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer."]
    #[inline(always)]
    pub fn flash_page_flip(&mut self) -> FLASH_PAGE_FLIP_W {
        FLASH_PAGE_FLIP_W { w: self }
    }
    #[doc = "Bit 5 - Floating Point Unit Disable"]
    #[inline(always)]
    pub fn fpu_dis(&mut self) -> FPU_DIS_W {
        FPU_DIS_W { w: self }
    }
    #[doc = "Bit 6 - Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4."]
    #[inline(always)]
    pub fn ccache_flush(&mut self) -> CCACHE_FLUSH_W {
        CCACHE_FLUSH_W { w: self }
    }
    #[doc = "Bit 14 - Serial Wire Debug Disable"]
    #[inline(always)]
    pub fn swd_dis(&mut self) -> SWD_DIS_W {
        SWD_DIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scon](index.html) module"]
pub struct SCON_SPEC;
impl crate::RegisterSpec for SCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scon::R](R) reader structure"]
impl crate::Readable for SCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scon::W](W) writer structure"]
impl crate::Writable for SCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCON to value 0"]
impl crate::Resettable for SCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
