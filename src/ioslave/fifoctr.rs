#[doc = "Reader of register FIFOCTR"]
pub type R = crate::R<u32, super::FIFOCTR>;
#[doc = "Writer for register FIFOCTR"]
pub type W = crate::W<u32, super::FIFOCTR>;
#[doc = "Register FIFOCTR `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFOCTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIFOCTR`"]
pub type FIFOCTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FIFOCTR`"]
pub struct FIFOCTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOCTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Virtual FIFO byte count"]
    #[inline(always)]
    pub fn fifoctr(&self) -> FIFOCTR_R {
        FIFOCTR_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Virtual FIFO byte count"]
    #[inline(always)]
    pub fn fifoctr(&mut self) -> FIFOCTR_W {
        FIFOCTR_W { w: self }
    }
}
