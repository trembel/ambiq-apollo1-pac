#[doc = "Reader of register ALMUP"]
pub type R = crate::R<u32, super::ALMUP>;
#[doc = "Writer for register ALMUP"]
pub type W = crate::W<u32, super::ALMUP>;
#[doc = "Register ALMUP `reset()`'s with value 0"]
impl crate::ResetValue for super::ALMUP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALMWKDY`"]
pub type ALMWKDY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALMWKDY`"]
pub struct ALMWKDY_W<'a> {
    w: &'a mut W,
}
impl<'a> ALMWKDY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `ALMMO`"]
pub type ALMMO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALMMO`"]
pub struct ALMMO_W<'a> {
    w: &'a mut W,
}
impl<'a> ALMMO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ALMDATE`"]
pub type ALMDATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALMDATE`"]
pub struct ALMDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALMDATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:18 - Weekdays Alarm"]
    #[inline(always)]
    pub fn almwkdy(&self) -> ALMWKDY_R {
        ALMWKDY_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 8:12 - Months Alarm"]
    #[inline(always)]
    pub fn almmo(&self) -> ALMMO_R {
        ALMMO_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:5 - Date Alarm"]
    #[inline(always)]
    pub fn almdate(&self) -> ALMDATE_R {
        ALMDATE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - Weekdays Alarm"]
    #[inline(always)]
    pub fn almwkdy(&mut self) -> ALMWKDY_W {
        ALMWKDY_W { w: self }
    }
    #[doc = "Bits 8:12 - Months Alarm"]
    #[inline(always)]
    pub fn almmo(&mut self) -> ALMMO_W {
        ALMMO_W { w: self }
    }
    #[doc = "Bits 0:5 - Date Alarm"]
    #[inline(always)]
    pub fn almdate(&mut self) -> ALMDATE_W {
        ALMDATE_W { w: self }
    }
}
