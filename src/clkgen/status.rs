#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OSCF`"]
pub type OSCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSCF`"]
pub struct OSCF_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `OMODE`"]
pub type OMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OMODE`"]
pub struct OMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - XT Oscillator is enabled but not oscillating"]
    #[inline(always)]
    pub fn oscf(&self) -> OSCF_R {
        OSCF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Current RTC oscillator (1 => LFRC, 0 => XT)"]
    #[inline(always)]
    pub fn omode(&self) -> OMODE_R {
        OMODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - XT Oscillator is enabled but not oscillating"]
    #[inline(always)]
    pub fn oscf(&mut self) -> OSCF_W {
        OSCF_W { w: self }
    }
    #[doc = "Bit 0 - Current RTC oscillator (1 => LFRC, 0 => XT)"]
    #[inline(always)]
    pub fn omode(&mut self) -> OMODE_W {
        OMODE_W { w: self }
    }
}
