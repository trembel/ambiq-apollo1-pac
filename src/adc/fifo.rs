#[doc = "Reader of register FIFO"]
pub type R = crate::R<u32, super::FIFO>;
#[doc = "Writer for register FIFO"]
pub type W = crate::W<u32, super::FIFO>;
#[doc = "Register FIFO `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSVD_27`"]
pub type RSVD_27_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSVD_27`"]
pub struct RSVD_27_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_27_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
#[doc = "Reader of field `SLOTNUM`"]
pub type SLOTNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLOTNUM`"]
pub struct SLOTNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOTNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `RSVD_20`"]
pub type RSVD_20_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSVD_20`"]
pub struct RSVD_20_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COUNT`"]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:31 - RESERVED."]
    #[inline(always)]
    pub fn rsvd_27(&self) -> RSVD_27_R {
        RSVD_27_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - Slot number associated with this FIFO data."]
    #[inline(always)]
    pub fn slotnum(&self) -> SLOTNUM_R {
        SLOTNUM_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 20:23 - RESERVED."]
    #[inline(always)]
    pub fn rsvd_20(&self) -> RSVD_20_R {
        RSVD_20_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Number of valid entries in the ADC FIFO."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:15 - Oldest data in the FIFO."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 27:31 - RESERVED."]
    #[inline(always)]
    pub fn rsvd_27(&mut self) -> RSVD_27_W {
        RSVD_27_W { w: self }
    }
    #[doc = "Bits 24:26 - Slot number associated with this FIFO data."]
    #[inline(always)]
    pub fn slotnum(&mut self) -> SLOTNUM_W {
        SLOTNUM_W { w: self }
    }
    #[doc = "Bits 20:23 - RESERVED."]
    #[inline(always)]
    pub fn rsvd_20(&mut self) -> RSVD_20_W {
        RSVD_20_W { w: self }
    }
    #[doc = "Bits 16:19 - Number of valid entries in the ADC FIFO."]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
    #[doc = "Bits 0:15 - Oldest data in the FIFO."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
}
