#[doc = "Reader of register FIFOINC"]
pub type R = crate::R<u32, super::FIFOINC>;
#[doc = "Writer for register FIFOINC"]
pub type W = crate::W<u32, super::FIFOINC>;
#[doc = "Register FIFOINC `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFOINC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIFOINC`"]
pub type FIFOINC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FIFOINC`"]
pub struct FIFOINC_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOINC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Increment the Overall FIFO Counter by this value on a write"]
    #[inline(always)]
    pub fn fifoinc(&self) -> FIFOINC_R {
        FIFOINC_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Increment the Overall FIFO Counter by this value on a write"]
    #[inline(always)]
    pub fn fifoinc(&mut self) -> FIFOINC_W {
        FIFOINC_W { w: self }
    }
}
