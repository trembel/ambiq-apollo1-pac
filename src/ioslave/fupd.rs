#[doc = "Reader of register FUPD"]
pub type R = crate::R<u32, super::FUPD>;
#[doc = "Writer for register FUPD"]
pub type W = crate::W<u32, super::FUPD>;
#[doc = "Register FUPD `reset()`'s with value 0"]
impl crate::ResetValue for super::FUPD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IOREAD`"]
pub type IOREAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOREAD`"]
pub struct IOREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> IOREAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `FIFOUPD`"]
pub type FIFOUPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFOUPD`"]
pub struct FIFOUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOUPD_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - This bitfield indicates an IO read is active."]
    #[inline(always)]
    pub fn ioread(&self) -> IOREAD_R {
        IOREAD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit indicates that a FIFO update is underway."]
    #[inline(always)]
    pub fn fifoupd(&self) -> FIFOUPD_R {
        FIFOUPD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - This bitfield indicates an IO read is active."]
    #[inline(always)]
    pub fn ioread(&mut self) -> IOREAD_W {
        IOREAD_W { w: self }
    }
    #[doc = "Bit 0 - This bit indicates that a FIFO update is underway."]
    #[inline(always)]
    pub fn fifoupd(&mut self) -> FIFOUPD_W {
        FIFOUPD_W { w: self }
    }
}
