#[doc = "Reader of register CALRC"]
pub type R = crate::R<u32, super::CALRC>;
#[doc = "Writer for register CALRC"]
pub type W = crate::W<u32, super::CALRC>;
#[doc = "Register CALRC `reset()`'s with value 0"]
impl crate::ResetValue for super::CALRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CALRC`"]
pub type CALRC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CALRC`"]
pub struct CALRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CALRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - LFRC Oscillator calibration value"]
    #[inline(always)]
    pub fn calrc(&self) -> CALRC_R {
        CALRC_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - LFRC Oscillator calibration value"]
    #[inline(always)]
    pub fn calrc(&mut self) -> CALRC_W {
        CALRC_W { w: self }
    }
}
