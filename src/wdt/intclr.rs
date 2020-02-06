#[doc = "Reader of register INTCLR"]
pub type R = crate::R<u32, super::INTCLR>;
#[doc = "Writer for register INTCLR"]
pub type W = crate::W<u32, super::INTCLR>;
#[doc = "Register INTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDTINT`"]
pub type WDTINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTINT`"]
pub struct WDTINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTINT_W<'a> {
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
    #[doc = "Bit 0 - Watchdog Timer Interrupt."]
    #[inline(always)]
    pub fn wdtint(&self) -> WDTINT_R {
        WDTINT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Timer Interrupt."]
    #[inline(always)]
    pub fn wdtint(&mut self) -> WDTINT_W {
        WDTINT_W { w: self }
    }
}
