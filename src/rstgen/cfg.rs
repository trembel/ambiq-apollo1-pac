#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDREN`"]
pub type WDREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDREN`"]
pub struct WDREN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDREN_W<'a> {
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
#[doc = "Reader of field `BODHREN`"]
pub type BODHREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BODHREN`"]
pub struct BODHREN_W<'a> {
    w: &'a mut W,
}
impl<'a> BODHREN_W<'a> {
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
    #[doc = "Bit 1 - Watchdog Timer Reset Enable. NOTE: The WDT module must also be configured for WDT reset."]
    #[inline(always)]
    pub fn wdren(&self) -> WDREN_R {
        WDREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Brown out high (2.1v) reset enable."]
    #[inline(always)]
    pub fn bodhren(&self) -> BODHREN_R {
        BODHREN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Watchdog Timer Reset Enable. NOTE: The WDT module must also be configured for WDT reset."]
    #[inline(always)]
    pub fn wdren(&mut self) -> WDREN_W {
        WDREN_W { w: self }
    }
    #[doc = "Bit 0 - Brown out high (2.1v) reset enable."]
    #[inline(always)]
    pub fn bodhren(&mut self) -> BODHREN_W {
        BODHREN_W { w: self }
    }
}
