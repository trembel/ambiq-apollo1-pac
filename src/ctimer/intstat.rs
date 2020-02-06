#[doc = "Reader of register INTSTAT"]
pub type R = crate::R<u32, super::INTSTAT>;
#[doc = "Writer for register INTSTAT"]
pub type W = crate::W<u32, super::INTSTAT>;
#[doc = "Register INTSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::INTSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTMRB3INT`"]
pub type CTMRB3INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRB3INT`"]
pub struct CTMRB3INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRB3INT_W<'a> {
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
#[doc = "Reader of field `CTMRA3INT`"]
pub type CTMRA3INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRA3INT`"]
pub struct CTMRA3INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRA3INT_W<'a> {
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
#[doc = "Reader of field `CTMRB2INT`"]
pub type CTMRB2INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRB2INT`"]
pub struct CTMRB2INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRB2INT_W<'a> {
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
#[doc = "Reader of field `CTMRA2INT`"]
pub type CTMRA2INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRA2INT`"]
pub struct CTMRA2INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRA2INT_W<'a> {
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
#[doc = "Reader of field `CTMRB1INT`"]
pub type CTMRB1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRB1INT`"]
pub struct CTMRB1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRB1INT_W<'a> {
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
#[doc = "Reader of field `CTMRA1INT`"]
pub type CTMRA1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRA1INT`"]
pub struct CTMRA1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRA1INT_W<'a> {
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
#[doc = "Reader of field `CTMRB0INT`"]
pub type CTMRB0INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRB0INT`"]
pub struct CTMRB0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRB0INT_W<'a> {
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
#[doc = "Reader of field `CTMRA0INT`"]
pub type CTMRA0INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMRA0INT`"]
pub struct CTMRA0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMRA0INT_W<'a> {
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
    #[doc = "Bit 7 - Counter/Timer B3 interrupt."]
    #[inline(always)]
    pub fn ctmrb3int(&self) -> CTMRB3INT_R {
        CTMRB3INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Counter/Timer A3 interrupt."]
    #[inline(always)]
    pub fn ctmra3int(&self) -> CTMRA3INT_R {
        CTMRA3INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Counter/Timer B2 interrupt."]
    #[inline(always)]
    pub fn ctmrb2int(&self) -> CTMRB2INT_R {
        CTMRB2INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Counter/Timer A2 interrupt."]
    #[inline(always)]
    pub fn ctmra2int(&self) -> CTMRA2INT_R {
        CTMRA2INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Counter/Timer B1 interrupt."]
    #[inline(always)]
    pub fn ctmrb1int(&self) -> CTMRB1INT_R {
        CTMRB1INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Counter/Timer A1 interrupt."]
    #[inline(always)]
    pub fn ctmra1int(&self) -> CTMRA1INT_R {
        CTMRA1INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counter/Timer B0 interrupt."]
    #[inline(always)]
    pub fn ctmrb0int(&self) -> CTMRB0INT_R {
        CTMRB0INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Counter/Timer A0 interrupt."]
    #[inline(always)]
    pub fn ctmra0int(&self) -> CTMRA0INT_R {
        CTMRA0INT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Counter/Timer B3 interrupt."]
    #[inline(always)]
    pub fn ctmrb3int(&mut self) -> CTMRB3INT_W {
        CTMRB3INT_W { w: self }
    }
    #[doc = "Bit 6 - Counter/Timer A3 interrupt."]
    #[inline(always)]
    pub fn ctmra3int(&mut self) -> CTMRA3INT_W {
        CTMRA3INT_W { w: self }
    }
    #[doc = "Bit 5 - Counter/Timer B2 interrupt."]
    #[inline(always)]
    pub fn ctmrb2int(&mut self) -> CTMRB2INT_W {
        CTMRB2INT_W { w: self }
    }
    #[doc = "Bit 4 - Counter/Timer A2 interrupt."]
    #[inline(always)]
    pub fn ctmra2int(&mut self) -> CTMRA2INT_W {
        CTMRA2INT_W { w: self }
    }
    #[doc = "Bit 3 - Counter/Timer B1 interrupt."]
    #[inline(always)]
    pub fn ctmrb1int(&mut self) -> CTMRB1INT_W {
        CTMRB1INT_W { w: self }
    }
    #[doc = "Bit 2 - Counter/Timer A1 interrupt."]
    #[inline(always)]
    pub fn ctmra1int(&mut self) -> CTMRA1INT_W {
        CTMRA1INT_W { w: self }
    }
    #[doc = "Bit 1 - Counter/Timer B0 interrupt."]
    #[inline(always)]
    pub fn ctmrb0int(&mut self) -> CTMRB0INT_W {
        CTMRB0INT_W { w: self }
    }
    #[doc = "Bit 0 - Counter/Timer A0 interrupt."]
    #[inline(always)]
    pub fn ctmra0int(&mut self) -> CTMRA0INT_W {
        CTMRA0INT_W { w: self }
    }
}
