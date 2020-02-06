#[doc = "Reader of register CMPRA3"]
pub type R = crate::R<u32, super::CMPRA3>;
#[doc = "Writer for register CMPRA3"]
pub type W = crate::W<u32, super::CMPRA3>;
#[doc = "Register CMPRA3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPRA3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMPR1A3`"]
pub type CMPR1A3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMPR1A3`"]
pub struct CMPR1A3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR1A3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CMPR0A3`"]
pub type CMPR0A3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMPR0A3`"]
pub struct CMPR0A3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR0A3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer A3 Compare Register 1."]
    #[inline(always)]
    pub fn cmpr1a3(&self) -> CMPR1A3_R {
        CMPR1A3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer A3 Compare Register 0."]
    #[inline(always)]
    pub fn cmpr0a3(&self) -> CMPR0A3_R {
        CMPR0A3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer A3 Compare Register 1."]
    #[inline(always)]
    pub fn cmpr1a3(&mut self) -> CMPR1A3_W {
        CMPR1A3_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer A3 Compare Register 0."]
    #[inline(always)]
    pub fn cmpr0a3(&mut self) -> CMPR0A3_W {
        CMPR0A3_W { w: self }
    }
}
