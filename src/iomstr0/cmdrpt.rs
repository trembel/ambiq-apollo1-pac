#[doc = "Reader of register CMDRPT"]
pub type R = crate::R<u32, super::CMDRPT>;
#[doc = "Writer for register CMDRPT"]
pub type W = crate::W<u32, super::CMDRPT>;
#[doc = "Register CMDRPT `reset()`'s with value 0"]
impl crate::ResetValue for super::CMDRPT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMDRPT`"]
pub type CMDRPT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMDRPT`"]
pub struct CMDRPT_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDRPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - These bits hold the Command repeat count."]
    #[inline(always)]
    pub fn cmdrpt(&self) -> CMDRPT_R {
        CMDRPT_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - These bits hold the Command repeat count."]
    #[inline(always)]
    pub fn cmdrpt(&mut self) -> CMDRPT_W {
        CMDRPT_W { w: self }
    }
}
