#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Writer for register INTEN"]
pub type W = crate::W<u32, super::INTEN>;
#[doc = "Register INTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BODH`"]
pub type BODH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BODH`"]
pub struct BODH_W<'a> {
    w: &'a mut W,
}
impl<'a> BODH_W<'a> {
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
    #[doc = "Bit 0 - Enables an interrupt that triggers when VCC is below BODH level."]
    #[inline(always)]
    pub fn bodh(&self) -> BODH_R {
        BODH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables an interrupt that triggers when VCC is below BODH level."]
    #[inline(always)]
    pub fn bodh(&mut self) -> BODH_W {
        BODH_W { w: self }
    }
}
