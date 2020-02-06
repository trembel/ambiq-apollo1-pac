#[doc = "Reader of register CHIPREV"]
pub type R = crate::R<u32, super::CHIPREV>;
#[doc = "Writer for register CHIPREV"]
pub type W = crate::W<u32, super::CHIPREV>;
#[doc = "Register CHIPREV `reset()`'s with value 0"]
impl crate::ResetValue for super::CHIPREV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Chip Revision Number.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REVISION_A {
    #[doc = "0: Apollo CHIPREV."]
    APOLLO = 0,
}
impl From<REVISION_A> for u8 {
    #[inline(always)]
    fn from(variant: REVISION_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REVISION`"]
pub type REVISION_R = crate::R<u8, REVISION_A>;
impl REVISION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REVISION_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REVISION_A::APOLLO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `APOLLO`"]
    #[inline(always)]
    pub fn is_apollo(&self) -> bool {
        *self == REVISION_A::APOLLO
    }
}
#[doc = "Write proxy for field `REVISION`"]
pub struct REVISION_W<'a> {
    w: &'a mut W,
}
impl<'a> REVISION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REVISION_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Apollo CHIPREV."]
    #[inline(always)]
    pub fn apollo(self) -> &'a mut W {
        self.variant(REVISION_A::APOLLO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Chip Revision Number."]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Chip Revision Number."]
    #[inline(always)]
    pub fn revision(&mut self) -> REVISION_W {
        REVISION_W { w: self }
    }
}
