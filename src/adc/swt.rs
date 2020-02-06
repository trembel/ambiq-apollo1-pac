#[doc = "Reader of register SWT"]
pub type R = crate::R<u32, super::SWT>;
#[doc = "Writer for register SWT"]
pub type W = crate::W<u32, super::SWT>;
#[doc = "Register SWT `reset()`'s with value 0"]
impl crate::ResetValue for super::SWT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Writing 0x37 to this register generates a software trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWT_A {
    #[doc = "55: Writing this value generates a software trigger."]
    GEN_SW_TRIGGER = 55,
}
impl From<SWT_A> for u8 {
    #[inline(always)]
    fn from(variant: SWT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SWT`"]
pub type SWT_R = crate::R<u8, SWT_A>;
impl SWT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SWT_A> {
        use crate::Variant::*;
        match self.bits {
            55 => Val(SWT_A::GEN_SW_TRIGGER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GEN_SW_TRIGGER`"]
    #[inline(always)]
    pub fn is_gen_sw_trigger(&self) -> bool {
        *self == SWT_A::GEN_SW_TRIGGER
    }
}
#[doc = "Write proxy for field `SWT`"]
pub struct SWT_W<'a> {
    w: &'a mut W,
}
impl<'a> SWT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Writing this value generates a software trigger."]
    #[inline(always)]
    pub fn gen_sw_trigger(self) -> &'a mut W {
        self.variant(SWT_A::GEN_SW_TRIGGER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Writing 0x37 to this register generates a software trigger."]
    #[inline(always)]
    pub fn swt(&self) -> SWT_R {
        SWT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Writing 0x37 to this register generates a software trigger."]
    #[inline(always)]
    pub fn swt(&mut self) -> SWT_W {
        SWT_W { w: self }
    }
}
