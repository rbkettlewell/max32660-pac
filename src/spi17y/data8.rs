#[doc = "Register `DATA8[%s]` reader"]
pub struct R(crate::R<DATA8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA8[%s]` writer"]
pub struct W(crate::W<DATA8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA8_SPEC>;
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
impl From<crate::W<DATA8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Read to pull from RX FIFO, write to put into TX FIFO."]
pub struct DATA_R(crate::FieldReader<u8, u8>);
impl DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA` writer - Read to pull from RX FIFO, write to put into TX FIFO."]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Read to pull from RX FIFO, write to put into TX FIFO."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read to pull from RX FIFO, write to put into TX FIFO."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for reading and writing the FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data8](index.html) module"]
pub struct DATA8_SPEC;
impl crate::RegisterSpec for DATA8_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [data8::R](R) reader structure"]
impl crate::Readable for DATA8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data8::W](W) writer structure"]
impl crate::Writable for DATA8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA8[%s]
to value 0"]
impl crate::Resettable for DATA8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
