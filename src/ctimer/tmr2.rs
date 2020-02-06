#[doc = "Reader of register TMR2"]
pub type R = crate::R<u32, super::TMR2>;
#[doc = "Writer for register TMR2"]
pub type W = crate::W<u32, super::TMR2>;
#[doc = "Register TMR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::TMR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTTMRB2`"]
pub type CTTMRB2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTTMRB2`"]
pub struct CTTMRB2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRB2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CTTMRA2`"]
pub type CTTMRA2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTTMRA2`"]
pub struct CTTMRA2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRA2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer B2."]
    #[inline(always)]
    pub fn cttmrb2(&self) -> CTTMRB2_R {
        CTTMRB2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer A2."]
    #[inline(always)]
    pub fn cttmra2(&self) -> CTTMRA2_R {
        CTTMRA2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer B2."]
    #[inline(always)]
    pub fn cttmrb2(&mut self) -> CTTMRB2_W {
        CTTMRB2_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer A2."]
    #[inline(always)]
    pub fn cttmra2(&mut self) -> CTTMRA2_W {
        CTTMRA2_W { w: self }
    }
}
