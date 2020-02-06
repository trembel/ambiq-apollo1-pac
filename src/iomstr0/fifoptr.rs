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
#[doc = "Reader of field `FIFOREM`"]
pub type FIFOREM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFOREM`"]
pub struct FIFOREM_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOREM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
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
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:22 - The number of bytes remaining in the FIFO (i.e. 64-FIFOSIZ)."]
    #[inline(always)]
    pub fn fiforem(&self) -> FIFOREM_R {
        FIFOREM_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 0:6 - The number of bytes currently in the FIFO."]
    #[inline(always)]
    pub fn fifosiz(&self) -> FIFOSIZ_R {
        FIFOSIZ_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:22 - The number of bytes remaining in the FIFO (i.e. 64-FIFOSIZ)."]
    #[inline(always)]
    pub fn fiforem(&mut self) -> FIFOREM_W {
        FIFOREM_W { w: self }
    }
    #[doc = "Bits 0:6 - The number of bytes currently in the FIFO."]
    #[inline(always)]
    pub fn fifosiz(&mut self) -> FIFOSIZ_W {
        FIFOSIZ_W { w: self }
    }
}
