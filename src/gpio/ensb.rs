#[doc = "Reader of register ENSB"]
pub type R = crate::R<u32, super::ENSB>;
#[doc = "Writer for register ENSB"]
pub type W = crate::W<u32, super::ENSB>;
#[doc = "Register ENSB `reset()`'s with value 0"]
impl crate::ResetValue for super::ENSB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENSB`"]
pub type ENSB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ENSB`"]
pub struct ENSB_W<'a> {
    w: &'a mut W,
}
impl<'a> ENSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Set the GPIO49-32 output enables"]
    #[inline(always)]
    pub fn ensb(&self) -> ENSB_R {
        ENSB_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Set the GPIO49-32 output enables"]
    #[inline(always)]
    pub fn ensb(&mut self) -> ENSB_W {
        ENSB_W { w: self }
    }
}
