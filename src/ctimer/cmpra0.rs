#[doc = "Reader of register CMPRA0"]
pub type R = crate::R<u32, super::CMPRA0>;
#[doc = "Writer for register CMPRA0"]
pub type W = crate::W<u32, super::CMPRA0>;
#[doc = "Register CMPRA0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPRA0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMPR1A0`"]
pub type CMPR1A0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMPR1A0`"]
pub struct CMPR1A0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR1A0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CMPR0A0`"]
pub type CMPR0A0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMPR0A0`"]
pub struct CMPR0A0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR0A0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer A0 Compare Register 1. Holds the upper limit for timer half A."]
    #[inline(always)]
    pub fn cmpr1a0(&self) -> CMPR1A0_R {
        CMPR1A0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer A0 Compare Register 0. Holds the lower limit for timer half A."]
    #[inline(always)]
    pub fn cmpr0a0(&self) -> CMPR0A0_R {
        CMPR0A0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer A0 Compare Register 1. Holds the upper limit for timer half A."]
    #[inline(always)]
    pub fn cmpr1a0(&mut self) -> CMPR1A0_W {
        CMPR1A0_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer A0 Compare Register 0. Holds the lower limit for timer half A."]
    #[inline(always)]
    pub fn cmpr0a0(&mut self) -> CMPR0A0_W {
        CMPR0A0_W { w: self }
    }
}
