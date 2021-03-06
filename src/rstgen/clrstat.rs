#[doc = "Reader of register CLRSTAT"]
pub type R = crate::R<u32, super::CLRSTAT>;
#[doc = "Writer for register CLRSTAT"]
pub type W = crate::W<u32, super::CLRSTAT>;
#[doc = "Register CLRSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::CLRSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLRSTAT`"]
pub type CLRSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRSTAT`"]
pub struct CLRSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRSTAT_W<'a> {
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
    #[doc = "Bit 0 - Writing a 1 to this bit clears all bits in the RST_STAT."]
    #[inline(always)]
    pub fn clrstat(&self) -> CLRSTAT_R {
        CLRSTAT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing a 1 to this bit clears all bits in the RST_STAT."]
    #[inline(always)]
    pub fn clrstat(&mut self) -> CLRSTAT_W {
        CLRSTAT_W { w: self }
    }
}
