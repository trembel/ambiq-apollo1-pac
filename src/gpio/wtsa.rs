#[doc = "Reader of register WTSA"]
pub type R = crate::R<u32, super::WTSA>;
#[doc = "Writer for register WTSA"]
pub type W = crate::W<u32, super::WTSA>;
#[doc = "Register WTSA `reset()`'s with value 0"]
impl crate::ResetValue for super::WTSA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WTSA`"]
pub type WTSA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WTSA`"]
pub struct WTSA_W<'a> {
    w: &'a mut W,
}
impl<'a> WTSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Set the GPIO31-0 write data."]
    #[inline(always)]
    pub fn wtsa(&self) -> WTSA_R {
        WTSA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Set the GPIO31-0 write data."]
    #[inline(always)]
    pub fn wtsa(&mut self) -> WTSA_W {
        WTSA_W { w: self }
    }
}
