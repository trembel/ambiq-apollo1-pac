#[doc = "Reader of register ENSA"]
pub type R = crate::R<u32, super::ENSA>;
#[doc = "Writer for register ENSA"]
pub type W = crate::W<u32, super::ENSA>;
#[doc = "Register ENSA `reset()`'s with value 0"]
impl crate::ResetValue for super::ENSA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENSA`"]
pub type ENSA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ENSA`"]
pub struct ENSA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Set the GPIO31-0 output enables"]
    #[inline(always)]
    pub fn ensa(&self) -> ENSA_R {
        ENSA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Set the GPIO31-0 output enables"]
    #[inline(always)]
    pub fn ensa(&mut self) -> ENSA_W {
        ENSA_W { w: self }
    }
}
