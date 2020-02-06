#[doc = "Reader of register WTCB"]
pub type R = crate::R<u32, super::WTCB>;
#[doc = "Writer for register WTCB"]
pub type W = crate::W<u32, super::WTCB>;
#[doc = "Register WTCB `reset()`'s with value 0"]
impl crate::ResetValue for super::WTCB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WTCB`"]
pub type WTCB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WTCB`"]
pub struct WTCB_W<'a> {
    w: &'a mut W,
}
impl<'a> WTCB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Clear the GPIO49-32 write data."]
    #[inline(always)]
    pub fn wtcb(&self) -> WTCB_R {
        WTCB_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Clear the GPIO49-32 write data."]
    #[inline(always)]
    pub fn wtcb(&mut self) -> WTCB_W {
        WTCB_W { w: self }
    }
}
