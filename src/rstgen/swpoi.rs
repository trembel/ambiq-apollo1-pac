#[doc = "Reader of register SWPOI"]
pub type R = crate::R<u32, super::SWPOI>;
#[doc = "Writer for register SWPOI"]
pub type W = crate::W<u32, super::SWPOI>;
#[doc = "Register SWPOI `reset()`'s with value 0"]
impl crate::ResetValue for super::SWPOI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0x1B generates a software POI reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWPOIKEY_A {
    #[doc = "27: Writing 0x1B key value generates a software POI reset."]
    KEYVALUE = 27,
}
impl From<SWPOIKEY_A> for u8 {
    #[inline(always)]
    fn from(variant: SWPOIKEY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SWPOIKEY`"]
pub type SWPOIKEY_R = crate::R<u8, SWPOIKEY_A>;
impl SWPOIKEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SWPOIKEY_A> {
        use crate::Variant::*;
        match self.bits {
            27 => Val(SWPOIKEY_A::KEYVALUE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `KEYVALUE`"]
    #[inline(always)]
    pub fn is_keyvalue(&self) -> bool {
        *self == SWPOIKEY_A::KEYVALUE
    }
}
#[doc = "Write proxy for field `SWPOIKEY`"]
pub struct SWPOIKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPOIKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWPOIKEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Writing 0x1B key value generates a software POI reset."]
    #[inline(always)]
    pub fn keyvalue(self) -> &'a mut W {
        self.variant(SWPOIKEY_A::KEYVALUE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 0x1B generates a software POI reset."]
    #[inline(always)]
    pub fn swpoikey(&self) -> SWPOIKEY_R {
        SWPOIKEY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 0x1B generates a software POI reset."]
    #[inline(always)]
    pub fn swpoikey(&mut self) -> SWPOIKEY_W {
        SWPOIKEY_W { w: self }
    }
}
