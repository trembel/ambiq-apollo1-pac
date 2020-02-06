#[doc = "Reader of register CLKKEY"]
pub type R = crate::R<u32, super::CLKKEY>;
#[doc = "Writer for register CLKKEY"]
pub type W = crate::W<u32, super::CLKKEY>;
#[doc = "Register CLKKEY `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKKEY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Key register value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CLKKEY_A {
    #[doc = "71: Key"]
    KEY = 71,
}
impl From<CLKKEY_A> for u32 {
    #[inline(always)]
    fn from(variant: CLKKEY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKKEY`"]
pub type CLKKEY_R = crate::R<u32, CLKKEY_A>;
impl CLKKEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, CLKKEY_A> {
        use crate::Variant::*;
        match self.bits {
            71 => Val(CLKKEY_A::KEY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == CLKKEY_A::KEY
    }
}
#[doc = "Write proxy for field `CLKKEY`"]
pub struct CLKKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKKEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(CLKKEY_A::KEY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Key register value."]
    #[inline(always)]
    pub fn clkkey(&self) -> CLKKEY_R {
        CLKKEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key register value."]
    #[inline(always)]
    pub fn clkkey(&mut self) -> CLKKEY_W {
        CLKKEY_W { w: self }
    }
}
