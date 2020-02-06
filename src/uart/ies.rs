#[doc = "Reader of register IES"]
pub type R = crate::R<u32, super::IES>;
#[doc = "Writer for register IES"]
pub type W = crate::W<u32, super::IES>;
#[doc = "Register IES `reset()`'s with value 0"]
impl crate::ResetValue for super::IES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OERIS`"]
pub type OERIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OERIS`"]
pub struct OERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OERIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `BERIS`"]
pub type BERIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BERIS`"]
pub struct BERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BERIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `PERIS`"]
pub type PERIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PERIS`"]
pub struct PERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `FERIS`"]
pub type FERIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FERIS`"]
pub struct FERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FERIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RTRIS`"]
pub type RTRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTRIS`"]
pub struct RTRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTRIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TXRIS`"]
pub type TXRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXRIS`"]
pub struct TXRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RXRIS`"]
pub type RXRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXRIS`"]
pub struct RXRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DSRMRIS`"]
pub type DSRMRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSRMRIS`"]
pub struct DSRMRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRMRIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DCDMRIS`"]
pub type DCDMRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDMRIS`"]
pub struct DCDMRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDMRIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CTSMRIS`"]
pub type CTSMRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTSMRIS`"]
pub struct CTSMRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSMRIS_W<'a> {
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
#[doc = "Reader of field `RIMRIS`"]
pub type RIMRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RIMRIS`"]
pub struct RIMRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RIMRIS_W<'a> {
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
    #[doc = "Bit 10 - This bit holds the overflow interrupt status."]
    #[inline(always)]
    pub fn oeris(&self) -> OERIS_R {
        OERIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This bit holds the break error interrupt status."]
    #[inline(always)]
    pub fn beris(&self) -> BERIS_R {
        BERIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This bit holds the parity error interrupt status."]
    #[inline(always)]
    pub fn peris(&self) -> PERIS_R {
        PERIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit holds the framing error interrupt status."]
    #[inline(always)]
    pub fn feris(&self) -> FERIS_R {
        FERIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit holds the receive timeout interrupt status."]
    #[inline(always)]
    pub fn rtris(&self) -> RTRIS_R {
        RTRIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit holds the transmit interrupt status."]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit holds the receive interrupt status."]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit holds the modem DSR interrupt status."]
    #[inline(always)]
    pub fn dsrmris(&self) -> DSRMRIS_R {
        DSRMRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit holds the modem DCD interrupt status."]
    #[inline(always)]
    pub fn dcdmris(&self) -> DCDMRIS_R {
        DCDMRIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit holds the modem CTS interrupt status."]
    #[inline(always)]
    pub fn ctsmris(&self) -> CTSMRIS_R {
        CTSMRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit holds the modem RI interrupt status."]
    #[inline(always)]
    pub fn rimris(&self) -> RIMRIS_R {
        RIMRIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - This bit holds the overflow interrupt status."]
    #[inline(always)]
    pub fn oeris(&mut self) -> OERIS_W {
        OERIS_W { w: self }
    }
    #[doc = "Bit 9 - This bit holds the break error interrupt status."]
    #[inline(always)]
    pub fn beris(&mut self) -> BERIS_W {
        BERIS_W { w: self }
    }
    #[doc = "Bit 8 - This bit holds the parity error interrupt status."]
    #[inline(always)]
    pub fn peris(&mut self) -> PERIS_W {
        PERIS_W { w: self }
    }
    #[doc = "Bit 7 - This bit holds the framing error interrupt status."]
    #[inline(always)]
    pub fn feris(&mut self) -> FERIS_W {
        FERIS_W { w: self }
    }
    #[doc = "Bit 6 - This bit holds the receive timeout interrupt status."]
    #[inline(always)]
    pub fn rtris(&mut self) -> RTRIS_W {
        RTRIS_W { w: self }
    }
    #[doc = "Bit 5 - This bit holds the transmit interrupt status."]
    #[inline(always)]
    pub fn txris(&mut self) -> TXRIS_W {
        TXRIS_W { w: self }
    }
    #[doc = "Bit 4 - This bit holds the receive interrupt status."]
    #[inline(always)]
    pub fn rxris(&mut self) -> RXRIS_W {
        RXRIS_W { w: self }
    }
    #[doc = "Bit 3 - This bit holds the modem DSR interrupt status."]
    #[inline(always)]
    pub fn dsrmris(&mut self) -> DSRMRIS_W {
        DSRMRIS_W { w: self }
    }
    #[doc = "Bit 2 - This bit holds the modem DCD interrupt status."]
    #[inline(always)]
    pub fn dcdmris(&mut self) -> DCDMRIS_W {
        DCDMRIS_W { w: self }
    }
    #[doc = "Bit 1 - This bit holds the modem CTS interrupt status."]
    #[inline(always)]
    pub fn ctsmris(&mut self) -> CTSMRIS_W {
        CTSMRIS_W { w: self }
    }
    #[doc = "Bit 0 - This bit holds the modem RI interrupt status."]
    #[inline(always)]
    pub fn rimris(&mut self) -> RIMRIS_W {
        RIMRIS_W { w: self }
    }
}
