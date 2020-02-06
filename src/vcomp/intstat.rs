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
#[doc = "Reader of field `OUTHI`"]
pub type OUTHI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTHI`"]
pub struct OUTHI_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTHI_W<'a> {
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
#[doc = "Reader of field `OUTLOW`"]
pub type OUTLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTLOW`"]
pub struct OUTLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLOW_W<'a> {
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
    #[doc = "Bit 1 - This bit is the vcompout high interrupt."]
    #[inline(always)]
    pub fn outhi(&self) -> OUTHI_R {
        OUTHI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit is the vcompout low interrupt."]
    #[inline(always)]
    pub fn outlow(&self) -> OUTLOW_R {
        OUTLOW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - This bit is the vcompout high interrupt."]
    #[inline(always)]
    pub fn outhi(&mut self) -> OUTHI_W {
        OUTHI_W { w: self }
    }
    #[doc = "Bit 0 - This bit is the vcompout low interrupt."]
    #[inline(always)]
    pub fn outlow(&mut self) -> OUTLOW_W {
        OUTLOW_W { w: self }
    }
}
