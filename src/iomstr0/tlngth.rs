#[doc = "Reader of register TLNGTH"]
pub type R = crate::R<u32, super::TLNGTH>;
#[doc = "Writer for register TLNGTH"]
pub type W = crate::W<u32, super::TLNGTH>;
#[doc = "Register TLNGTH `reset()`'s with value 0"]
impl crate::ResetValue for super::TLNGTH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TLNGTH`"]
pub type TLNGTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TLNGTH`"]
pub struct TLNGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> TLNGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Remaining transfer length."]
    #[inline(always)]
    pub fn tlngth(&self) -> TLNGTH_R {
        TLNGTH_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Remaining transfer length."]
    #[inline(always)]
    pub fn tlngth(&mut self) -> TLNGTH_W {
        TLNGTH_W { w: self }
    }
}
