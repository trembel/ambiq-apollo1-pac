#[doc = "Reader of register HFVAL"]
pub type R = crate::R<u32, super::HFVAL>;
#[doc = "Writer for register HFVAL"]
pub type W = crate::W<u32, super::HFVAL>;
#[doc = "Register HFVAL `reset()`'s with value 0"]
impl crate::ResetValue for super::HFVAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HFTUNERB`"]
pub type HFTUNERB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HFTUNERB`"]
pub struct HFTUNERB_W<'a> {
    w: &'a mut W,
}
impl<'a> HFTUNERB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Current HFTUNE value"]
    #[inline(always)]
    pub fn hftunerb(&self) -> HFTUNERB_R {
        HFTUNERB_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Current HFTUNE value"]
    #[inline(always)]
    pub fn hftunerb(&mut self) -> HFTUNERB_W {
        HFTUNERB_W { w: self }
    }
}
