#[doc = "Reader of register IOINTCTL"]
pub type R = crate::R<u32, super::IOINTCTL>;
#[doc = "Writer for register IOINTCTL"]
pub type W = crate::W<u32, super::IOINTCTL>;
#[doc = "Register IOINTCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::IOINTCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IOINTSET`"]
pub type IOINTSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOINTSET`"]
pub struct IOINTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> IOINTSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `IOINTCLR`"]
pub type IOINTCLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOINTCLR`"]
pub struct IOINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IOINTCLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `IOINT`"]
pub type IOINT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOINT`"]
pub struct IOINT_W<'a> {
    w: &'a mut W,
}
impl<'a> IOINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `IOINTEN`"]
pub type IOINTEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOINTEN`"]
pub struct IOINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOINTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - These bits set the IOINT interrupts when written with a 1."]
    #[inline(always)]
    pub fn iointset(&self) -> IOINTSET_R {
        IOINTSET_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 16 - This bit clears all of the IOINT interrupts when written with a 1."]
    #[inline(always)]
    pub fn iointclr(&self) -> IOINTCLR_R {
        IOINTCLR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - These bits read the IOINT interrupts."]
    #[inline(always)]
    pub fn ioint(&self) -> IOINT_R {
        IOINT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - These bits setread the IOINT interrupt enables."]
    #[inline(always)]
    pub fn iointen(&self) -> IOINTEN_R {
        IOINTEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - These bits set the IOINT interrupts when written with a 1."]
    #[inline(always)]
    pub fn iointset(&mut self) -> IOINTSET_W {
        IOINTSET_W { w: self }
    }
    #[doc = "Bit 16 - This bit clears all of the IOINT interrupts when written with a 1."]
    #[inline(always)]
    pub fn iointclr(&mut self) -> IOINTCLR_W {
        IOINTCLR_W { w: self }
    }
    #[doc = "Bits 8:15 - These bits read the IOINT interrupts."]
    #[inline(always)]
    pub fn ioint(&mut self) -> IOINT_W {
        IOINT_W { w: self }
    }
    #[doc = "Bits 0:7 - These bits setread the IOINT interrupt enables."]
    #[inline(always)]
    pub fn iointen(&mut self) -> IOINTEN_W {
        IOINTEN_W { w: self }
    }
}
