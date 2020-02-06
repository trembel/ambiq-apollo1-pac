#[doc = "Reader of register SWPOR"]
pub type R = crate::R<u32, super::SWPOR>;
#[doc = "Writer for register SWPOR"]
pub type W = crate::W<u32, super::SWPOR>;
#[doc = "Register SWPOR `reset()`'s with value 0"]
impl crate::ResetValue for super::SWPOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0xD4 generates a software POR reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWPORKEY_A {
    #[doc = "212: Writing 0xD4 key value generates a software POR reset."]
    KEYVALUE = 212,
}
impl From<SWPORKEY_A> for u8 {
    #[inline(always)]
    fn from(variant: SWPORKEY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SWPORKEY`"]
pub type SWPORKEY_R = crate::R<u8, SWPORKEY_A>;
impl SWPORKEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SWPORKEY_A> {
        use crate::Variant::*;
        match self.bits {
            212 => Val(SWPORKEY_A::KEYVALUE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `KEYVALUE`"]
    #[inline(always)]
    pub fn is_keyvalue(&self) -> bool {
        *self == SWPORKEY_A::KEYVALUE
    }
}
#[doc = "Write proxy for field `SWPORKEY`"]
pub struct SWPORKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPORKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWPORKEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Writing 0xD4 key value generates a software POR reset."]
    #[inline(always)]
    pub fn keyvalue(self) -> &'a mut W {
        self.variant(SWPORKEY_A::KEYVALUE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 0xD4 generates a software POR reset."]
    #[inline(always)]
    pub fn swporkey(&self) -> SWPORKEY_R {
        SWPORKEY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 0xD4 generates a software POR reset."]
    #[inline(always)]
    pub fn swporkey(&mut self) -> SWPORKEY_W {
        SWPORKEY_W { w: self }
    }
}
