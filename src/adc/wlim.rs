#[doc = "Reader of register WLIM"]
pub type R = crate::R<u32, super::WLIM>;
#[doc = "Writer for register WLIM"]
pub type W = crate::W<u32, super::WLIM>;
#[doc = "Register WLIM `reset()`'s with value 0"]
impl crate::ResetValue for super::WLIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ULIM`"]
pub type ULIM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ULIM`"]
pub struct ULIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ULIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `LLIM`"]
pub type LLIM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LLIM`"]
pub struct LLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LLIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Sets the upper limit for the wondow comparator."]
    #[inline(always)]
    pub fn ulim(&self) -> ULIM_R {
        ULIM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Sets the lower limit for the wondow comparator."]
    #[inline(always)]
    pub fn llim(&self) -> LLIM_R {
        LLIM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Sets the upper limit for the wondow comparator."]
    #[inline(always)]
    pub fn ulim(&mut self) -> ULIM_W {
        ULIM_W { w: self }
    }
    #[doc = "Bits 0:15 - Sets the lower limit for the wondow comparator."]
    #[inline(always)]
    pub fn llim(&mut self) -> LLIM_W {
        LLIM_W { w: self }
    }
}
