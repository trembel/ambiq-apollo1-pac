#[doc = "Reader of register FIFOCFG"]
pub type R = crate::R<u32, super::FIFOCFG>;
#[doc = "Writer for register FIFOCFG"]
pub type W = crate::W<u32, super::FIFOCFG>;
#[doc = "Register FIFOCFG `reset()`'s with value 0x2000_0000"]
impl crate::ResetValue for super::FIFOCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000_0000
    }
}
#[doc = "Reader of field `ROBASE`"]
pub type ROBASE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ROBASE`"]
pub struct ROBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROBASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `FIFOMAX`"]
pub type FIFOMAX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFOMAX`"]
pub struct FIFOMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOMAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `FIFOBASE`"]
pub type FIFOBASE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFOBASE`"]
pub struct FIFOBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOBASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29 - Defines the read-only area. The IO Slave read-only area is situated in LRAM at (ROBASE*8) to (FIFOOBASE*8-1)"]
    #[inline(always)]
    pub fn robase(&self) -> ROBASE_R {
        ROBASE_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - These bits hold the maximum FIFO address in 8 byte segments. It is also the beginning of the RAM area of the LRAM. Note that no RAM area is configured if FIFOMAX is set to 0x1F."]
    #[inline(always)]
    pub fn fifomax(&self) -> FIFOMAX_R {
        FIFOMAX_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:4 - These bits hold the base address of the I/O FIFO in 8 byte segments. The IO Slave FIFO is situated in LRAM at (FIFOBASE*8) to (FIFOMAX*8-1)."]
    #[inline(always)]
    pub fn fifobase(&self) -> FIFOBASE_R {
        FIFOBASE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - Defines the read-only area. The IO Slave read-only area is situated in LRAM at (ROBASE*8) to (FIFOOBASE*8-1)"]
    #[inline(always)]
    pub fn robase(&mut self) -> ROBASE_W {
        ROBASE_W { w: self }
    }
    #[doc = "Bits 8:13 - These bits hold the maximum FIFO address in 8 byte segments. It is also the beginning of the RAM area of the LRAM. Note that no RAM area is configured if FIFOMAX is set to 0x1F."]
    #[inline(always)]
    pub fn fifomax(&mut self) -> FIFOMAX_W {
        FIFOMAX_W { w: self }
    }
    #[doc = "Bits 0:4 - These bits hold the base address of the I/O FIFO in 8 byte segments. The IO Slave FIFO is situated in LRAM at (FIFOBASE*8) to (FIFOMAX*8-1)."]
    #[inline(always)]
    pub fn fifobase(&mut self) -> FIFOBASE_W {
        FIFOBASE_W { w: self }
    }
}
