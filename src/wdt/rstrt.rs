#[doc = "Reader of register RSTRT"]
pub type R = crate::R<u32, super::RSTRT>;
#[doc = "Writer for register RSTRT"]
pub type W = crate::W<u32, super::RSTRT>;
#[doc = "Register RSTRT `reset()`'s with value 0"]
impl crate::ResetValue for super::RSTRT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Writing 0xB2 to WDTRSTRT restarts the watchdog timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSTRT_A {
    #[doc = "178: This is the key value to write to WDTRSTRT to restart the WDT."]
    KEYVALUE = 178,
}
impl From<RSTRT_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTRT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSTRT`"]
pub type RSTRT_R = crate::R<u8, RSTRT_A>;
impl RSTRT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RSTRT_A> {
        use crate::Variant::*;
        match self.bits {
            178 => Val(RSTRT_A::KEYVALUE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `KEYVALUE`"]
    #[inline(always)]
    pub fn is_keyvalue(&self) -> bool {
        *self == RSTRT_A::KEYVALUE
    }
}
#[doc = "Write proxy for field `RSTRT`"]
pub struct RSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTRT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTRT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "This is the key value to write to WDTRSTRT to restart the WDT."]
    #[inline(always)]
    pub fn keyvalue(self) -> &'a mut W {
        self.variant(RSTRT_A::KEYVALUE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Writing 0xB2 to WDTRSTRT restarts the watchdog timer."]
    #[inline(always)]
    pub fn rstrt(&self) -> RSTRT_R {
        RSTRT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Writing 0xB2 to WDTRSTRT restarts the watchdog timer."]
    #[inline(always)]
    pub fn rstrt(&mut self) -> RSTRT_W {
        RSTRT_W { w: self }
    }
}
