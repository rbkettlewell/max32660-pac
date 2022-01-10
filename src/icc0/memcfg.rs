#[doc = "Register `MEMCFG` reader"]
pub struct R(crate::R<MEMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CCHSZ` reader - Cache Size. Indicates total size in Kbytes of cache."]
pub struct CCHSZ_R(crate::FieldReader<u16, u16>);
impl CCHSZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CCHSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCHSZ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMSZ` reader - Main Memory Size. Indicates the total size, in units of 128 Kbytes, of code memory accessible to the cache controller."]
pub struct MEMSZ_R(crate::FieldReader<u16, u16>);
impl MEMSZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MEMSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMSZ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Cache Size. Indicates total size in Kbytes of cache."]
    #[inline(always)]
    pub fn cchsz(&self) -> CCHSZ_R {
        CCHSZ_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Main Memory Size. Indicates the total size, in units of 128 Kbytes, of code memory accessible to the cache controller."]
    #[inline(always)]
    pub fn memsz(&self) -> MEMSZ_R {
        MEMSZ_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Memory Configuration Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memcfg](index.html) module"]
pub struct MEMCFG_SPEC;
impl crate::RegisterSpec for MEMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [memcfg::R](R) reader structure"]
impl crate::Readable for MEMCFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MEMCFG to value 0x0008_0008"]
impl crate::Resettable for MEMCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0008_0008
    }
}
