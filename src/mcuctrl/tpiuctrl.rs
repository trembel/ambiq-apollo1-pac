#[doc = "Reader of register TPIUCTRL"]
pub type R = crate::R<u32, super::TPIUCTRL>;
#[doc = "Writer for register TPIUCTRL"]
pub type W = crate::W<u32, super::TPIUCTRL>;
#[doc = "Register TPIUCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::TPIUCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "This field selects the frequency of the ARM M4 TPIU port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: Low power state."]
    LOW_PWR = 0,
    #[doc = "1: Selects 6MHz frequency."]
    _6MHZ = 1,
    #[doc = "2: Selects 3MHz frequency."]
    _3MHZ = 2,
    #[doc = "3: Selects 1.5 MHz frequency."]
    _1_5MHZ = 3,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKSEL`"]
pub type CLKSEL_R = crate::R<u8, CLKSEL_A>;
impl CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSEL_A {
        match self.bits {
            0 => CLKSEL_A::LOW_PWR,
            1 => CLKSEL_A::_6MHZ,
            2 => CLKSEL_A::_3MHZ,
            3 => CLKSEL_A::_1_5MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_PWR`"]
    #[inline(always)]
    pub fn is_low_pwr(&self) -> bool {
        *self == CLKSEL_A::LOW_PWR
    }
    #[doc = "Checks if the value of the field is `_6MHZ`"]
    #[inline(always)]
    pub fn is_6mhz(&self) -> bool {
        *self == CLKSEL_A::_6MHZ
    }
    #[doc = "Checks if the value of the field is `_3MHZ`"]
    #[inline(always)]
    pub fn is_3mhz(&self) -> bool {
        *self == CLKSEL_A::_3MHZ
    }
    #[doc = "Checks if the value of the field is `_1_5MHZ`"]
    #[inline(always)]
    pub fn is_1_5mhz(&self) -> bool {
        *self == CLKSEL_A::_1_5MHZ
    }
}
#[doc = "Write proxy for field `CLKSEL`"]
pub struct CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low power state."]
    #[inline(always)]
    pub fn low_pwr(self) -> &'a mut W {
        self.variant(CLKSEL_A::LOW_PWR)
    }
    #[doc = "Selects 6MHz frequency."]
    #[inline(always)]
    pub fn _6mhz(self) -> &'a mut W {
        self.variant(CLKSEL_A::_6MHZ)
    }
    #[doc = "Selects 3MHz frequency."]
    #[inline(always)]
    pub fn _3mhz(self) -> &'a mut W {
        self.variant(CLKSEL_A::_3MHZ)
    }
    #[doc = "Selects 1.5 MHz frequency."]
    #[inline(always)]
    pub fn _1_5mhz(self) -> &'a mut W {
        self.variant(CLKSEL_A::_1_5MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "TPIU Enable field. When set, the ARM M4 TPIU is enabled and data can be streamed out of the MCU's SWO port using the ARM ITM and TPIU modules.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Disable the TPIU."]
    DIS = 0,
    #[doc = "1: Enable the TPIU."]
    EN = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DIS,
            true => ENABLE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ENABLE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ENABLE_A::EN
    }
}
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the TPIU."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ENABLE_A::DIS)
    }
    #[doc = "Enable the TPIU."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ENABLE_A::EN)
    }
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
    #[doc = "Bits 8:9 - This field selects the frequency of the ARM M4 TPIU port."]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 0 - TPIU Enable field. When set, the ARM M4 TPIU is enabled and data can be streamed out of the MCU's SWO port using the ARM ITM and TPIU modules."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:9 - This field selects the frequency of the ARM M4 TPIU port."]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W { w: self }
    }
    #[doc = "Bit 0 - TPIU Enable field. When set, the ARM M4 TPIU is enabled and data can be streamed out of the MCU's SWO port using the ARM ITM and TPIU modules."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
