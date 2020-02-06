#[doc = "Reader of register PRENC"]
pub type R = crate::R<u32, super::PRENC>;
#[doc = "Writer for register PRENC"]
pub type W = crate::W<u32, super::PRENC>;
#[doc = "Register PRENC `reset()`'s with value 0"]
impl crate::ResetValue for super::PRENC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRENC`"]
pub type PRENC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRENC`"]
pub struct PRENC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRENC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - These bits hold the priority encode of the REGACC interrupts."]
    #[inline(always)]
    pub fn prenc(&self) -> PRENC_R {
        PRENC_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - These bits hold the priority encode of the REGACC interrupts."]
    #[inline(always)]
    pub fn prenc(&mut self) -> PRENC_W {
        PRENC_W { w: self }
    }
}
