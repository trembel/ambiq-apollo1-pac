#[doc = "Reader of register SL2CFG"]
pub type R = crate::R<u32, super::SL2CFG>;
#[doc = "Writer for register SL2CFG"]
pub type W = crate::W<u32, super::SL2CFG>;
#[doc = "Register SL2CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SL2CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select the number of measurements to average in the accumulate divide module for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADSEL2_A {
    #[doc = "0: Average in 1 measurement in the accumulate divide module for this slot."]
    AVG_1_MSRMT = 0,
    #[doc = "1: Average in 2 measurements in the accumulate divide module for this slot."]
    AVG_2_MSRMTS = 1,
    #[doc = "2: Average in 4 measurements in the accumulate divide module for this slot."]
    AVG_4_MSRMTS = 2,
    #[doc = "3: Average in 8 measurements in the accumulate divide module for this slot."]
    AVG_8_MSRMT = 3,
    #[doc = "4: Average in 16 measurements in the accumulate divide module for this slot."]
    AVG_16_MSRMTS = 4,
    #[doc = "5: Average in 32 measurements in the accumulate divide module for this slot."]
    AVG_32_MSRMTS = 5,
    #[doc = "6: Average in 64 measurements in the accumulate divide module for this slot."]
    AVG_64_MSRMTS = 6,
    #[doc = "7: Average in 128 measurements in the accumulate divide module for this slot."]
    AVG_128_MSRMTS = 7,
}
impl From<ADSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: ADSEL2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADSEL2`"]
pub type ADSEL2_R = crate::R<u8, ADSEL2_A>;
impl ADSEL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADSEL2_A {
        match self.bits {
            0 => ADSEL2_A::AVG_1_MSRMT,
            1 => ADSEL2_A::AVG_2_MSRMTS,
            2 => ADSEL2_A::AVG_4_MSRMTS,
            3 => ADSEL2_A::AVG_8_MSRMT,
            4 => ADSEL2_A::AVG_16_MSRMTS,
            5 => ADSEL2_A::AVG_32_MSRMTS,
            6 => ADSEL2_A::AVG_64_MSRMTS,
            7 => ADSEL2_A::AVG_128_MSRMTS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AVG_1_MSRMT`"]
    #[inline(always)]
    pub fn is_avg_1_msrmt(&self) -> bool {
        *self == ADSEL2_A::AVG_1_MSRMT
    }
    #[doc = "Checks if the value of the field is `AVG_2_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_2_msrmts(&self) -> bool {
        *self == ADSEL2_A::AVG_2_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_4_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_4_msrmts(&self) -> bool {
        *self == ADSEL2_A::AVG_4_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_8_MSRMT`"]
    #[inline(always)]
    pub fn is_avg_8_msrmt(&self) -> bool {
        *self == ADSEL2_A::AVG_8_MSRMT
    }
    #[doc = "Checks if the value of the field is `AVG_16_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_16_msrmts(&self) -> bool {
        *self == ADSEL2_A::AVG_16_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_32_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_32_msrmts(&self) -> bool {
        *self == ADSEL2_A::AVG_32_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_64_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_64_msrmts(&self) -> bool {
        *self == ADSEL2_A::AVG_64_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_128_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_128_msrmts(&self) -> bool {
        *self == ADSEL2_A::AVG_128_MSRMTS
    }
}
#[doc = "Write proxy for field `ADSEL2`"]
pub struct ADSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADSEL2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_1_msrmt(self) -> &'a mut W {
        self.variant(ADSEL2_A::AVG_1_MSRMT)
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_2_msrmts(self) -> &'a mut W {
        self.variant(ADSEL2_A::AVG_2_MSRMTS)
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_4_msrmts(self) -> &'a mut W {
        self.variant(ADSEL2_A::AVG_4_MSRMTS)
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_8_msrmt(self) -> &'a mut W {
        self.variant(ADSEL2_A::AVG_8_MSRMT)
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_16_msrmts(self) -> &'a mut W {
        self.variant(ADSEL2_A::AVG_16_MSRMTS)
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_32_msrmts(self) -> &'a mut W {
        self.variant(ADSEL2_A::AVG_32_MSRMTS)
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_64_msrmts(self) -> &'a mut W {
        self.variant(ADSEL2_A::AVG_64_MSRMTS)
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_128_msrmts(self) -> &'a mut W {
        self.variant(ADSEL2_A::AVG_128_MSRMTS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Select the track and hold delay for this slot. NOTE: The track and hold delay must be less than 50us for correct operation. When the ADC is configured to use the 1.5Mhz clock, the track and hold delay cannot exceed 64 clocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum THSEL2_A {
    #[doc = "0: 1 ADC clock cycle."]
    _1_ADC_CLK = 0,
    #[doc = "1: 2 ADC clock cycles."]
    _2_ADC_CLKS = 1,
    #[doc = "2: 4 ADC clock cycles."]
    _4_ADC_CLKS = 2,
    #[doc = "3: 8 ADC clock cycles."]
    _8_ADC_CLKS = 3,
    #[doc = "4: 16 ADC clock cycles."]
    _16_ADC_CLKS = 4,
    #[doc = "5: 32 ADC clock cycles."]
    _32_ADC_CLKS = 5,
    #[doc = "6: 64 ADC clock cycles."]
    _64_ADC_CLKS = 6,
    #[doc = "7: 128 ADC clock cycles."]
    _128_ADC_CLKS = 7,
}
impl From<THSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: THSEL2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `THSEL2`"]
pub type THSEL2_R = crate::R<u8, THSEL2_A>;
impl THSEL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THSEL2_A {
        match self.bits {
            0 => THSEL2_A::_1_ADC_CLK,
            1 => THSEL2_A::_2_ADC_CLKS,
            2 => THSEL2_A::_4_ADC_CLKS,
            3 => THSEL2_A::_8_ADC_CLKS,
            4 => THSEL2_A::_16_ADC_CLKS,
            5 => THSEL2_A::_32_ADC_CLKS,
            6 => THSEL2_A::_64_ADC_CLKS,
            7 => THSEL2_A::_128_ADC_CLKS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1_ADC_CLK`"]
    #[inline(always)]
    pub fn is_1_adc_clk(&self) -> bool {
        *self == THSEL2_A::_1_ADC_CLK
    }
    #[doc = "Checks if the value of the field is `_2_ADC_CLKS`"]
    #[inline(always)]
    pub fn is_2_adc_clks(&self) -> bool {
        *self == THSEL2_A::_2_ADC_CLKS
    }
    #[doc = "Checks if the value of the field is `_4_ADC_CLKS`"]
    #[inline(always)]
    pub fn is_4_adc_clks(&self) -> bool {
        *self == THSEL2_A::_4_ADC_CLKS
    }
    #[doc = "Checks if the value of the field is `_8_ADC_CLKS`"]
    #[inline(always)]
    pub fn is_8_adc_clks(&self) -> bool {
        *self == THSEL2_A::_8_ADC_CLKS
    }
    #[doc = "Checks if the value of the field is `_16_ADC_CLKS`"]
    #[inline(always)]
    pub fn is_16_adc_clks(&self) -> bool {
        *self == THSEL2_A::_16_ADC_CLKS
    }
    #[doc = "Checks if the value of the field is `_32_ADC_CLKS`"]
    #[inline(always)]
    pub fn is_32_adc_clks(&self) -> bool {
        *self == THSEL2_A::_32_ADC_CLKS
    }
    #[doc = "Checks if the value of the field is `_64_ADC_CLKS`"]
    #[inline(always)]
    pub fn is_64_adc_clks(&self) -> bool {
        *self == THSEL2_A::_64_ADC_CLKS
    }
    #[doc = "Checks if the value of the field is `_128_ADC_CLKS`"]
    #[inline(always)]
    pub fn is_128_adc_clks(&self) -> bool {
        *self == THSEL2_A::_128_ADC_CLKS
    }
}
#[doc = "Write proxy for field `THSEL2`"]
pub struct THSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> THSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THSEL2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1 ADC clock cycle."]
    #[inline(always)]
    pub fn _1_adc_clk(self) -> &'a mut W {
        self.variant(THSEL2_A::_1_ADC_CLK)
    }
    #[doc = "2 ADC clock cycles."]
    #[inline(always)]
    pub fn _2_adc_clks(self) -> &'a mut W {
        self.variant(THSEL2_A::_2_ADC_CLKS)
    }
    #[doc = "4 ADC clock cycles."]
    #[inline(always)]
    pub fn _4_adc_clks(self) -> &'a mut W {
        self.variant(THSEL2_A::_4_ADC_CLKS)
    }
    #[doc = "8 ADC clock cycles."]
    #[inline(always)]
    pub fn _8_adc_clks(self) -> &'a mut W {
        self.variant(THSEL2_A::_8_ADC_CLKS)
    }
    #[doc = "16 ADC clock cycles."]
    #[inline(always)]
    pub fn _16_adc_clks(self) -> &'a mut W {
        self.variant(THSEL2_A::_16_ADC_CLKS)
    }
    #[doc = "32 ADC clock cycles."]
    #[inline(always)]
    pub fn _32_adc_clks(self) -> &'a mut W {
        self.variant(THSEL2_A::_32_ADC_CLKS)
    }
    #[doc = "64 ADC clock cycles."]
    #[inline(always)]
    pub fn _64_adc_clks(self) -> &'a mut W {
        self.variant(THSEL2_A::_64_ADC_CLKS)
    }
    #[doc = "128 ADC clock cycles."]
    #[inline(always)]
    pub fn _128_adc_clks(self) -> &'a mut W {
        self.variant(THSEL2_A::_128_ADC_CLKS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Select one of the 13 channel inputs for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHSEL2_A {
    #[doc = "0: ADC_EXT0 external GPIO pin connection."]
    EXT0 = 0,
    #[doc = "1: ADC_EXT1 external GPIO pin connection."]
    EXT1 = 1,
    #[doc = "2: ADC_EXT2 external GPIO pin connection."]
    EXT2 = 2,
    #[doc = "3: ADC_EXT3 external GPIO pin connection."]
    EXT3 = 3,
    #[doc = "4: ADC_EXT4 external GPIO pin connection."]
    EXT4 = 4,
    #[doc = "5: ADC_EXT5 external GPIO pin connection."]
    EXT5 = 5,
    #[doc = "6: ADC_EXT6 external GPIO pin connection."]
    EXT6 = 6,
    #[doc = "7: ADC_EXT7 external GPIO pin connection."]
    EXT7 = 7,
    #[doc = "8: ADC_TEMP internal temperature sensor."]
    TEMP = 8,
    #[doc = "9: ADC_VDD internal power rail connection."]
    VDD = 9,
    #[doc = "10: ADC_VSS internal ground connection."]
    VSS = 10,
    #[doc = "12: ADC_VBATT internal voltage divide-by-3 connection to input power rail."]
    VBATT = 12,
}
impl From<CHSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: CHSEL2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CHSEL2`"]
pub type CHSEL2_R = crate::R<u8, CHSEL2_A>;
impl CHSEL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CHSEL2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CHSEL2_A::EXT0),
            1 => Val(CHSEL2_A::EXT1),
            2 => Val(CHSEL2_A::EXT2),
            3 => Val(CHSEL2_A::EXT3),
            4 => Val(CHSEL2_A::EXT4),
            5 => Val(CHSEL2_A::EXT5),
            6 => Val(CHSEL2_A::EXT6),
            7 => Val(CHSEL2_A::EXT7),
            8 => Val(CHSEL2_A::TEMP),
            9 => Val(CHSEL2_A::VDD),
            10 => Val(CHSEL2_A::VSS),
            12 => Val(CHSEL2_A::VBATT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EXT0`"]
    #[inline(always)]
    pub fn is_ext0(&self) -> bool {
        *self == CHSEL2_A::EXT0
    }
    #[doc = "Checks if the value of the field is `EXT1`"]
    #[inline(always)]
    pub fn is_ext1(&self) -> bool {
        *self == CHSEL2_A::EXT1
    }
    #[doc = "Checks if the value of the field is `EXT2`"]
    #[inline(always)]
    pub fn is_ext2(&self) -> bool {
        *self == CHSEL2_A::EXT2
    }
    #[doc = "Checks if the value of the field is `EXT3`"]
    #[inline(always)]
    pub fn is_ext3(&self) -> bool {
        *self == CHSEL2_A::EXT3
    }
    #[doc = "Checks if the value of the field is `EXT4`"]
    #[inline(always)]
    pub fn is_ext4(&self) -> bool {
        *self == CHSEL2_A::EXT4
    }
    #[doc = "Checks if the value of the field is `EXT5`"]
    #[inline(always)]
    pub fn is_ext5(&self) -> bool {
        *self == CHSEL2_A::EXT5
    }
    #[doc = "Checks if the value of the field is `EXT6`"]
    #[inline(always)]
    pub fn is_ext6(&self) -> bool {
        *self == CHSEL2_A::EXT6
    }
    #[doc = "Checks if the value of the field is `EXT7`"]
    #[inline(always)]
    pub fn is_ext7(&self) -> bool {
        *self == CHSEL2_A::EXT7
    }
    #[doc = "Checks if the value of the field is `TEMP`"]
    #[inline(always)]
    pub fn is_temp(&self) -> bool {
        *self == CHSEL2_A::TEMP
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == CHSEL2_A::VDD
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == CHSEL2_A::VSS
    }
    #[doc = "Checks if the value of the field is `VBATT`"]
    #[inline(always)]
    pub fn is_vbatt(&self) -> bool {
        *self == CHSEL2_A::VBATT
    }
}
#[doc = "Write proxy for field `CHSEL2`"]
pub struct CHSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ADC_EXT0 external GPIO pin connection."]
    #[inline(always)]
    pub fn ext0(self) -> &'a mut W {
        self.variant(CHSEL2_A::EXT0)
    }
    #[doc = "ADC_EXT1 external GPIO pin connection."]
    #[inline(always)]
    pub fn ext1(self) -> &'a mut W {
        self.variant(CHSEL2_A::EXT1)
    }
    #[doc = "ADC_EXT2 external GPIO pin connection."]
    #[inline(always)]
    pub fn ext2(self) -> &'a mut W {
        self.variant(CHSEL2_A::EXT2)
    }
    #[doc = "ADC_EXT3 external GPIO pin connection."]
    #[inline(always)]
    pub fn ext3(self) -> &'a mut W {
        self.variant(CHSEL2_A::EXT3)
    }
    #[doc = "ADC_EXT4 external GPIO pin connection."]
    #[inline(always)]
    pub fn ext4(self) -> &'a mut W {
        self.variant(CHSEL2_A::EXT4)
    }
    #[doc = "ADC_EXT5 external GPIO pin connection."]
    #[inline(always)]
    pub fn ext5(self) -> &'a mut W {
        self.variant(CHSEL2_A::EXT5)
    }
    #[doc = "ADC_EXT6 external GPIO pin connection."]
    #[inline(always)]
    pub fn ext6(self) -> &'a mut W {
        self.variant(CHSEL2_A::EXT6)
    }
    #[doc = "ADC_EXT7 external GPIO pin connection."]
    #[inline(always)]
    pub fn ext7(self) -> &'a mut W {
        self.variant(CHSEL2_A::EXT7)
    }
    #[doc = "ADC_TEMP internal temperature sensor."]
    #[inline(always)]
    pub fn temp(self) -> &'a mut W {
        self.variant(CHSEL2_A::TEMP)
    }
    #[doc = "ADC_VDD internal power rail connection."]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut W {
        self.variant(CHSEL2_A::VDD)
    }
    #[doc = "ADC_VSS internal ground connection."]
    #[inline(always)]
    pub fn vss(self) -> &'a mut W {
        self.variant(CHSEL2_A::VSS)
    }
    #[doc = "ADC_VBATT internal voltage divide-by-3 connection to input power rail."]
    #[inline(always)]
    pub fn vbatt(self) -> &'a mut W {
        self.variant(CHSEL2_A::VBATT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "This bit enables the window compare function for slot 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCEN2_A {
    #[doc = "1: Enable the window compare for slot 2."]
    WCEN = 1,
}
impl From<WCEN2_A> for bool {
    #[inline(always)]
    fn from(variant: WCEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WCEN2`"]
pub type WCEN2_R = crate::R<bool, WCEN2_A>;
impl WCEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, WCEN2_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(WCEN2_A::WCEN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WCEN`"]
    #[inline(always)]
    pub fn is_wcen(&self) -> bool {
        *self == WCEN2_A::WCEN
    }
}
#[doc = "Write proxy for field `WCEN2`"]
pub struct WCEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> WCEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WCEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the window compare for slot 2."]
    #[inline(always)]
    pub fn wcen(self) -> &'a mut W {
        self.variant(WCEN2_A::WCEN)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "This bit enables slot 2 for ADC conversions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEN2_A {
    #[doc = "1: Enable slot 2 for ADC conversions."]
    SLEN = 1,
}
impl From<SLEN2_A> for bool {
    #[inline(always)]
    fn from(variant: SLEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLEN2`"]
pub type SLEN2_R = crate::R<bool, SLEN2_A>;
impl SLEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SLEN2_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SLEN2_A::SLEN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLEN`"]
    #[inline(always)]
    pub fn is_slen(&self) -> bool {
        *self == SLEN2_A::SLEN
    }
}
#[doc = "Write proxy for field `SLEN2`"]
pub struct SLEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable slot 2 for ADC conversions."]
    #[inline(always)]
    pub fn slen(self) -> &'a mut W {
        self.variant(SLEN2_A::SLEN)
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
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn adsel2(&self) -> ADSEL2_R {
        ADSEL2_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Select the track and hold delay for this slot. NOTE: The track and hold delay must be less than 50us for correct operation. When the ADC is configured to use the 1.5Mhz clock, the track and hold delay cannot exceed 64 clocks."]
    #[inline(always)]
    pub fn thsel2(&self) -> THSEL2_R {
        THSEL2_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - Select one of the 13 channel inputs for this slot."]
    #[inline(always)]
    pub fn chsel2(&self) -> CHSEL2_R {
        CHSEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 2."]
    #[inline(always)]
    pub fn wcen2(&self) -> WCEN2_R {
        WCEN2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit enables slot 2 for ADC conversions."]
    #[inline(always)]
    pub fn slen2(&self) -> SLEN2_R {
        SLEN2_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn adsel2(&mut self) -> ADSEL2_W {
        ADSEL2_W { w: self }
    }
    #[doc = "Bits 16:18 - Select the track and hold delay for this slot. NOTE: The track and hold delay must be less than 50us for correct operation. When the ADC is configured to use the 1.5Mhz clock, the track and hold delay cannot exceed 64 clocks."]
    #[inline(always)]
    pub fn thsel2(&mut self) -> THSEL2_W {
        THSEL2_W { w: self }
    }
    #[doc = "Bits 8:11 - Select one of the 13 channel inputs for this slot."]
    #[inline(always)]
    pub fn chsel2(&mut self) -> CHSEL2_W {
        CHSEL2_W { w: self }
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 2."]
    #[inline(always)]
    pub fn wcen2(&mut self) -> WCEN2_W {
        WCEN2_W { w: self }
    }
    #[doc = "Bit 0 - This bit enables slot 2 for ADC conversions."]
    #[inline(always)]
    pub fn slen2(&mut self) -> SLEN2_W {
        SLEN2_W { w: self }
    }
}
