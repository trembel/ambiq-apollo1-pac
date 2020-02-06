#[doc = "Reader of register ENCA"]
pub type R = crate::R<u32, super::ENCA>;
#[doc = "Writer for register ENCA"]
pub type W = crate::W<u32, super::ENCA>;
#[doc = "Register ENCA `reset()`'s with value 0"]
impl crate::ResetValue for super::ENCA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENCA`"]
pub type ENCA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ENCA`"]
pub struct ENCA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Clear the GPIO31-0 output enables"]
    #[inline(always)]
    pub fn enca(&self) -> ENCA_R {
        ENCA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Clear the GPIO31-0 output enables"]
    #[inline(always)]
    pub fn enca(&mut self) -> ENCA_W {
        ENCA_W { w: self }
    }
}
