#[doc = "Reader of register PADKEY"]
pub type R = crate::R<u32, super::PADKEY>;
#[doc = "Writer for register PADKEY"]
pub type W = crate::W<u32, super::PADKEY>;
#[doc = "Register PADKEY `reset()`'s with value 0"]
impl crate::ResetValue for super::PADKEY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Key register value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PADKEY_A {
    #[doc = "115: Key"]
    KEY = 115,
}
impl From<PADKEY_A> for u32 {
    #[inline(always)]
    fn from(variant: PADKEY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PADKEY`"]
pub type PADKEY_R = crate::R<u32, PADKEY_A>;
impl PADKEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PADKEY_A> {
        use crate::Variant::*;
        match self.bits {
            115 => Val(PADKEY_A::KEY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == PADKEY_A::KEY
    }
}
#[doc = "Write proxy for field `PADKEY`"]
pub struct PADKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> PADKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PADKEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(PADKEY_A::KEY)
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
    pub fn padkey(&self) -> PADKEY_R {
        PADKEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key register value."]
    #[inline(always)]
    pub fn padkey(&mut self) -> PADKEY_W {
        PADKEY_W { w: self }
    }
}
