#[doc = "Reader of register ENCB"]
pub type R = crate::R<u32, super::ENCB>;
#[doc = "Writer for register ENCB"]
pub type W = crate::W<u32, super::ENCB>;
#[doc = "Register ENCB `reset()`'s with value 0"]
impl crate::ResetValue for super::ENCB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENCB`"]
pub type ENCB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ENCB`"]
pub struct ENCB_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Clear the GPIO49-32 output enables"]
    #[inline(always)]
    pub fn encb(&self) -> ENCB_R {
        ENCB_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Clear the GPIO49-32 output enables"]
    #[inline(always)]
    pub fn encb(&mut self) -> ENCB_W {
        ENCB_W { w: self }
    }
}
