#[doc = "Reader of register FIFOTHR"]
pub type R = crate::R<u32, super::FIFOTHR>;
#[doc = "Writer for register FIFOTHR"]
pub type W = crate::W<u32, super::FIFOTHR>;
#[doc = "Register FIFOTHR `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFOTHR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIFOTHR`"]
pub type FIFOTHR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFOTHR`"]
pub struct FIFOTHR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOTHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - FIFO size interrupt threshold."]
    #[inline(always)]
    pub fn fifothr(&self) -> FIFOTHR_R {
        FIFOTHR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FIFO size interrupt threshold."]
    #[inline(always)]
    pub fn fifothr(&mut self) -> FIFOTHR_W {
        FIFOTHR_W { w: self }
    }
}
