#[doc = "Reader of register ALMLOW"]
pub type R = crate::R<u32, super::ALMLOW>;
#[doc = "Writer for register ALMLOW"]
pub type W = crate::W<u32, super::ALMLOW>;
#[doc = "Register ALMLOW `reset()`'s with value 0"]
impl crate::ResetValue for super::ALMLOW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALMHR`"]
pub type ALMHR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALMHR`"]
pub struct ALMHR_W<'a> {
    w: &'a mut W,
}
impl<'a> ALMHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `ALMMIN`"]
pub type ALMMIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALMMIN`"]
pub struct ALMMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALMMIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `ALMSEC`"]
pub type ALMSEC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALMSEC`"]
pub struct ALMSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> ALMSEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ALM100`"]
pub type ALM100_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALM100`"]
pub struct ALM100_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM100_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29 - Hours Alarm"]
    #[inline(always)]
    pub fn almhr(&self) -> ALMHR_R {
        ALMHR_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:22 - Minutes Alarm"]
    #[inline(always)]
    pub fn almmin(&self) -> ALMMIN_R {
        ALMMIN_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Seconds Alarm"]
    #[inline(always)]
    pub fn almsec(&self) -> ALMSEC_R {
        ALMSEC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:7 - 100ths of a second Alarm"]
    #[inline(always)]
    pub fn alm100(&self) -> ALM100_R {
        ALM100_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - Hours Alarm"]
    #[inline(always)]
    pub fn almhr(&mut self) -> ALMHR_W {
        ALMHR_W { w: self }
    }
    #[doc = "Bits 16:22 - Minutes Alarm"]
    #[inline(always)]
    pub fn almmin(&mut self) -> ALMMIN_W {
        ALMMIN_W { w: self }
    }
    #[doc = "Bits 8:14 - Seconds Alarm"]
    #[inline(always)]
    pub fn almsec(&mut self) -> ALMSEC_W {
        ALMSEC_W { w: self }
    }
    #[doc = "Bits 0:7 - 100ths of a second Alarm"]
    #[inline(always)]
    pub fn alm100(&mut self) -> ALM100_W {
        ALM100_W { w: self }
    }
}
