#[doc = "Reader of register PWDKEY"]
pub type R = crate::R<u32, super::PWDKEY>;
#[doc = "Writer for register PWDKEY"]
pub type W = crate::W<u32, super::PWDKEY>;
#[doc = "Register PWDKEY `reset()`'s with value 0"]
impl crate::ResetValue for super::PWDKEY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Key register value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PWDKEY_A {
    #[doc = "55: Key"]
    KEY = 55,
}
impl From<PWDKEY_A> for u32 {
    #[inline(always)]
    fn from(variant: PWDKEY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWDKEY`"]
pub type PWDKEY_R = crate::R<u32, PWDKEY_A>;
impl PWDKEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PWDKEY_A> {
        use crate::Variant::*;
        match self.bits {
            55 => Val(PWDKEY_A::KEY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == PWDKEY_A::KEY
    }
}
#[doc = "Write proxy for field `PWDKEY`"]
pub struct PWDKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> PWDKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWDKEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(PWDKEY_A::KEY)
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
    pub fn pwdkey(&self) -> PWDKEY_R {
        PWDKEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key register value."]
    #[inline(always)]
    pub fn pwdkey(&mut self) -> PWDKEY_W {
        PWDKEY_W { w: self }
    }
}
