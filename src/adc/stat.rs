#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Writer for register STAT"]
pub type W = crate::W<u32, super::STAT>;
#[doc = "Register STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Indicates the power-status of the ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWDSTAT_A {
    #[doc = "0: Powered on."]
    ON = 0,
    #[doc = "1: Power switch on, ADC Low Power Mode 1."]
    SWITCH_ON_SAR_OFF = 1,
    #[doc = "2: Power switch off, ADC disabled."]
    POWER_SWITCH_OFF = 2,
}
impl From<PWDSTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: PWDSTAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWDSTAT`"]
pub type PWDSTAT_R = crate::R<u8, PWDSTAT_A>;
impl PWDSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PWDSTAT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PWDSTAT_A::ON),
            1 => Val(PWDSTAT_A::SWITCH_ON_SAR_OFF),
            2 => Val(PWDSTAT_A::POWER_SWITCH_OFF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == PWDSTAT_A::ON
    }
    #[doc = "Checks if the value of the field is `SWITCH_ON_SAR_OFF`"]
    #[inline(always)]
    pub fn is_switch_on_sar_off(&self) -> bool {
        *self == PWDSTAT_A::SWITCH_ON_SAR_OFF
    }
    #[doc = "Checks if the value of the field is `POWER_SWITCH_OFF`"]
    #[inline(always)]
    pub fn is_power_switch_off(&self) -> bool {
        *self == PWDSTAT_A::POWER_SWITCH_OFF
    }
}
#[doc = "Write proxy for field `PWDSTAT`"]
pub struct PWDSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWDSTAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWDSTAT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Powered on."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(PWDSTAT_A::ON)
    }
    #[doc = "Power switch on, ADC Low Power Mode 1."]
    #[inline(always)]
    pub fn switch_on_sar_off(self) -> &'a mut W {
        self.variant(PWDSTAT_A::SWITCH_ON_SAR_OFF)
    }
    #[doc = "Power switch off, ADC disabled."]
    #[inline(always)]
    pub fn power_switch_off(self) -> &'a mut W {
        self.variant(PWDSTAT_A::POWER_SWITCH_OFF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Indicates the power-status of the ADC."]
    #[inline(always)]
    pub fn pwdstat(&self) -> PWDSTAT_R {
        PWDSTAT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Indicates the power-status of the ADC."]
    #[inline(always)]
    pub fn pwdstat(&mut self) -> PWDSTAT_W {
        PWDSTAT_W { w: self }
    }
}
