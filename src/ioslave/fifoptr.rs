#[doc = "Reader of register FIFOPTR"]
pub type R = crate::R<u32, super::FIFOPTR>;
#[doc = "Writer for register FIFOPTR"]
pub type W = crate::W<u32, super::FIFOPTR>;
#[doc = "Register FIFOPTR `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFOPTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIFOSIZ`"]
pub type FIFOSIZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFOSIZ`"]
pub struct FIFOSIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOSIZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `FIFOPTR`"]
pub type FIFOPTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFOPTR`"]
pub struct FIFOPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - The number of bytes currently in the hardware FIFO."]
    #[inline(always)]
    pub fn fifosiz(&self) -> FIFOSIZ_R {
        FIFOSIZ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Current FIFO pointer."]
    #[inline(always)]
    pub fn fifoptr(&self) -> FIFOPTR_R {
        FIFOPTR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - The number of bytes currently in the hardware FIFO."]
    #[inline(always)]
    pub fn fifosiz(&mut self) -> FIFOSIZ_W {
        FIFOSIZ_W { w: self }
    }
    #[doc = "Bits 0:7 - Current FIFO pointer."]
    #[inline(always)]
    pub fn fifoptr(&mut self) -> FIFOPTR_W {
        FIFOPTR_W { w: self }
    }
}
