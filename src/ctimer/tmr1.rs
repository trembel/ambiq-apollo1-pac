#[doc = "Reader of register TMR1"]
pub type R = crate::R<u32, super::TMR1>;
#[doc = "Writer for register TMR1"]
pub type W = crate::W<u32, super::TMR1>;
#[doc = "Register TMR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TMR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTTMRB1`"]
pub type CTTMRB1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTTMRB1`"]
pub struct CTTMRB1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRB1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CTTMRA1`"]
pub type CTTMRA1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTTMRA1`"]
pub struct CTTMRA1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer B1."]
    #[inline(always)]
    pub fn cttmrb1(&self) -> CTTMRB1_R {
        CTTMRB1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer A1."]
    #[inline(always)]
    pub fn cttmra1(&self) -> CTTMRA1_R {
        CTTMRA1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer B1."]
    #[inline(always)]
    pub fn cttmrb1(&mut self) -> CTTMRB1_W {
        CTTMRB1_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer A1."]
    #[inline(always)]
    pub fn cttmra1(&mut self) -> CTTMRA1_W {
        CTTMRA1_W { w: self }
    }
}
