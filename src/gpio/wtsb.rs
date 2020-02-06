#[doc = "Reader of register WTSB"]
pub type R = crate::R<u32, super::WTSB>;
#[doc = "Writer for register WTSB"]
pub type W = crate::W<u32, super::WTSB>;
#[doc = "Register WTSB `reset()`'s with value 0"]
impl crate::ResetValue for super::WTSB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WTSB`"]
pub type WTSB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WTSB`"]
pub struct WTSB_W<'a> {
    w: &'a mut W,
}
impl<'a> WTSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Set the GPIO49-32 write data."]
    #[inline(always)]
    pub fn wtsb(&self) -> WTSB_R {
        WTSB_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Set the GPIO49-32 write data."]
    #[inline(always)]
    pub fn wtsb(&mut self) -> WTSB_W {
        WTSB_W { w: self }
    }
}
