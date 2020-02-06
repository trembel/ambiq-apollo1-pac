#[doc = "Reader of register CTRLOW"]
pub type R = crate::R<u32, super::CTRLOW>;
#[doc = "Writer for register CTRLOW"]
pub type W = crate::W<u32, super::CTRLOW>;
#[doc = "Register CTRLOW `reset()`'s with value 0x0100_0000"]
impl crate::ResetValue for super::CTRLOW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100_0000
    }
}
#[doc = "Reader of field `CTRHR`"]
pub type CTRHR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTRHR`"]
pub struct CTRHR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `CTRMIN`"]
pub type CTRMIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTRMIN`"]
pub struct CTRMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRMIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CTRSEC`"]
pub type CTRSEC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTRSEC`"]
pub struct CTRSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRSEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CTR100`"]
pub type CTR100_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTR100`"]
pub struct CTR100_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR100_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29 - Hours Counter"]
    #[inline(always)]
    pub fn ctrhr(&self) -> CTRHR_R {
        CTRHR_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:22 - Minutes Counter"]
    #[inline(always)]
    pub fn ctrmin(&self) -> CTRMIN_R {
        CTRMIN_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Seconds Counter"]
    #[inline(always)]
    pub fn ctrsec(&self) -> CTRSEC_R {
        CTRSEC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:7 - 100ths of a second Counter"]
    #[inline(always)]
    pub fn ctr100(&self) -> CTR100_R {
        CTR100_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - Hours Counter"]
    #[inline(always)]
    pub fn ctrhr(&mut self) -> CTRHR_W {
        CTRHR_W { w: self }
    }
    #[doc = "Bits 16:22 - Minutes Counter"]
    #[inline(always)]
    pub fn ctrmin(&mut self) -> CTRMIN_W {
        CTRMIN_W { w: self }
    }
    #[doc = "Bits 8:14 - Seconds Counter"]
    #[inline(always)]
    pub fn ctrsec(&mut self) -> CTRSEC_W {
        CTRSEC_W { w: self }
    }
    #[doc = "Bits 0:7 - 100ths of a second Counter"]
    #[inline(always)]
    pub fn ctr100(&mut self) -> CTR100_W {
        CTR100_W { w: self }
    }
}
