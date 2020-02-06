#[doc = "Reader of register TMR0"]
pub type R = crate::R<u32, super::TMR0>;
#[doc = "Writer for register TMR0"]
pub type W = crate::W<u32, super::TMR0>;
#[doc = "Register TMR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::TMR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTTMRB0`"]
pub type CTTMRB0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTTMRB0`"]
pub struct CTTMRB0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRB0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CTTMRA0`"]
pub type CTTMRA0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTTMRA0`"]
pub struct CTTMRA0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRA0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer B0."]
    #[inline(always)]
    pub fn cttmrb0(&self) -> CTTMRB0_R {
        CTTMRB0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer A0."]
    #[inline(always)]
    pub fn cttmra0(&self) -> CTTMRA0_R {
        CTTMRA0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer B0."]
    #[inline(always)]
    pub fn cttmrb0(&mut self) -> CTTMRB0_W {
        CTTMRB0_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer A0."]
    #[inline(always)]
    pub fn cttmra0(&mut self) -> CTTMRA0_W {
        CTTMRA0_W { w: self }
    }
}
