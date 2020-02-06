#[doc = "Reader of register IEC"]
pub type R = crate::R<u32, super::IEC>;
#[doc = "Writer for register IEC"]
pub type W = crate::W<u32, super::IEC>;
#[doc = "Register IEC `reset()`'s with value 0"]
impl crate::ResetValue for super::IEC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OEIC`"]
pub type OEIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OEIC`"]
pub struct OEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> OEIC_W<'a> {
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
#[doc = "Reader of field `BEIC`"]
pub type BEIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEIC`"]
pub struct BEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIC_W<'a> {
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
#[doc = "Reader of field `PEIC`"]
pub type PEIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEIC`"]
pub struct PEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> PEIC_W<'a> {
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
#[doc = "Reader of field `FEIC`"]
pub type FEIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEIC`"]
pub struct FEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIC_W<'a> {
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
#[doc = "Reader of field `RTIC`"]
pub type RTIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTIC`"]
pub struct RTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIC_W<'a> {
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
#[doc = "Reader of field `TXIC`"]
pub type TXIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXIC`"]
pub struct TXIC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIC_W<'a> {
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
#[doc = "Reader of field `RXIC`"]
pub type RXIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIC`"]
pub struct RXIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIC_W<'a> {
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
#[doc = "Reader of field `DSRMIC`"]
pub type DSRMIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSRMIC`"]
pub struct DSRMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRMIC_W<'a> {
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
#[doc = "Reader of field `DCDMIC`"]
pub type DCDMIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDMIC`"]
pub struct DCDMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDMIC_W<'a> {
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
#[doc = "Reader of field `CTSMIC`"]
pub type CTSMIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTSMIC`"]
pub struct CTSMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSMIC_W<'a> {
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
#[doc = "Reader of field `RIMIC`"]
pub type RIMIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RIMIC`"]
pub struct RIMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RIMIC_W<'a> {
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
    #[doc = "Bit 10 - This bit holds the overflow interrupt clear."]
    #[inline(always)]
    pub fn oeic(&self) -> OEIC_R {
        OEIC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This bit holds the break error interrupt clear."]
    #[inline(always)]
    pub fn beic(&self) -> BEIC_R {
        BEIC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This bit holds the parity error interrupt clear."]
    #[inline(always)]
    pub fn peic(&self) -> PEIC_R {
        PEIC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit holds the framing error interrupt clear."]
    #[inline(always)]
    pub fn feic(&self) -> FEIC_R {
        FEIC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit holds the receive timeout interrupt clear."]
    #[inline(always)]
    pub fn rtic(&self) -> RTIC_R {
        RTIC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit holds the transmit interrupt clear."]
    #[inline(always)]
    pub fn txic(&self) -> TXIC_R {
        TXIC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit holds the receive interrupt clear."]
    #[inline(always)]
    pub fn rxic(&self) -> RXIC_R {
        RXIC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit holds the modem DSR interrupt clear."]
    #[inline(always)]
    pub fn dsrmic(&self) -> DSRMIC_R {
        DSRMIC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit holds the modem DCD interrupt clear."]
    #[inline(always)]
    pub fn dcdmic(&self) -> DCDMIC_R {
        DCDMIC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit holds the modem CTS interrupt clear."]
    #[inline(always)]
    pub fn ctsmic(&self) -> CTSMIC_R {
        CTSMIC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit holds the modem RI interrupt clear."]
    #[inline(always)]
    pub fn rimic(&self) -> RIMIC_R {
        RIMIC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - This bit holds the overflow interrupt clear."]
    #[inline(always)]
    pub fn oeic(&mut self) -> OEIC_W {
        OEIC_W { w: self }
    }
    #[doc = "Bit 9 - This bit holds the break error interrupt clear."]
    #[inline(always)]
    pub fn beic(&mut self) -> BEIC_W {
        BEIC_W { w: self }
    }
    #[doc = "Bit 8 - This bit holds the parity error interrupt clear."]
    #[inline(always)]
    pub fn peic(&mut self) -> PEIC_W {
        PEIC_W { w: self }
    }
    #[doc = "Bit 7 - This bit holds the framing error interrupt clear."]
    #[inline(always)]
    pub fn feic(&mut self) -> FEIC_W {
        FEIC_W { w: self }
    }
    #[doc = "Bit 6 - This bit holds the receive timeout interrupt clear."]
    #[inline(always)]
    pub fn rtic(&mut self) -> RTIC_W {
        RTIC_W { w: self }
    }
    #[doc = "Bit 5 - This bit holds the transmit interrupt clear."]
    #[inline(always)]
    pub fn txic(&mut self) -> TXIC_W {
        TXIC_W { w: self }
    }
    #[doc = "Bit 4 - This bit holds the receive interrupt clear."]
    #[inline(always)]
    pub fn rxic(&mut self) -> RXIC_W {
        RXIC_W { w: self }
    }
    #[doc = "Bit 3 - This bit holds the modem DSR interrupt clear."]
    #[inline(always)]
    pub fn dsrmic(&mut self) -> DSRMIC_W {
        DSRMIC_W { w: self }
    }
    #[doc = "Bit 2 - This bit holds the modem DCD interrupt clear."]
    #[inline(always)]
    pub fn dcdmic(&mut self) -> DCDMIC_W {
        DCDMIC_W { w: self }
    }
    #[doc = "Bit 1 - This bit holds the modem CTS interrupt clear."]
    #[inline(always)]
    pub fn ctsmic(&mut self) -> CTSMIC_W {
        CTSMIC_W { w: self }
    }
    #[doc = "Bit 0 - This bit holds the modem RI interrupt clear."]
    #[inline(always)]
    pub fn rimic(&mut self) -> RIMIC_W {
        RIMIC_W { w: self }
    }
}
