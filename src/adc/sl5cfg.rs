#[doc = "Reader of register SL5CFG"]
pub type R = crate::R<u32, super::SL5CFG>;
#[doc = "Writer for register SL5CFG"]
pub type W = crate::W<u32, super::SL5CFG>;
#[doc = "Register SL5CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SL5CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select number of measurements to average in the accumulate divide module for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADSEL5_A {
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
impl From<ADSEL5_A> for u8 {
    #[inline(always)]
    fn from(variant: ADSEL5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADSEL5`"]
pub type ADSEL5_R = crate::R<u8, ADSEL5_A>;
impl ADSEL5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADSEL5_A {
        match self.bits {
            0 => ADSEL5_A::AVG_1_MSRMT,
            1 => ADSEL5_A::AVG_2_MSRMTS,
            2 => ADSEL5_A::AVG_4_MSRMTS,
            3 => ADSEL5_A::AVG_8_MSRMT,
            4 => ADSEL5_A::AVG_16_MSRMTS,
            5 => ADSEL5_A::AVG_32_MSRMTS,
            6 => ADSEL5_A::AVG_64_MSRMTS,
            7 => ADSEL5_A::AVG_128_MSRMTS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AVG_1_MSRMT`"]
    #[inline(always)]
    pub fn is_avg_1_msrmt(&self) -> bool {
        *self == ADSEL5_A::AVG_1_MSRMT
    }
    #[doc = "Checks if the value of the field is `AVG_2_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_2_msrmts(&self) -> bool {
        *self == ADSEL5_A::AVG_2_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_4_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_4_msrmts(&self) -> bool {
        *self == ADSEL5_A::AVG_4_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_8_MSRMT`"]
    #[inline(always)]
    pub fn is_avg_8_msrmt(&self) -> bool {
        *self == ADSEL5_A::AVG_8_MSRMT
    }
    #[doc = "Checks if the value of the field is `AVG_16_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_16_msrmts(&self) -> bool {
        *self == ADSEL5_A::AVG_16_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_32_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_32_msrmts(&self) -> bool {
        *self == ADSEL5_A::AVG_32_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_64_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_64_msrmts(&self) -> bool {
        *self == ADSEL5_A::AVG_64_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_128_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_128_msrmts(&self) -> bool {
        *self == ADSEL5_A::AVG_128_MSRMTS
    }
}
#[doc = "Write proxy for field `ADSEL5`"]
pub struct ADSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADSEL5_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_1_msrmt(self) -> &'a mut W {
        self.variant(ADSEL5_A::AVG_1_MSRMT)
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_2_msrmts(self) -> &'a mut W {
        self.variant(ADSEL5_A::AVG_2_MSRMTS)
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_4_msrmts(self) -> &'a mut W {
        self.variant(ADSEL5_A::AVG_4_MSRMTS)
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_8_msrmt(self) -> &'a mut W {
        self.variant(ADSEL5_A::AVG_8_MSRMT)
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_16_msrmts(self) -> &'a mut W {
        self.variant(ADSEL5_A::AVG_16_MSRMTS)
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_32_msrmts(self) -> &'a mut W {
        self.variant(ADSEL5_A::AVG_32_MSRMTS)
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_64_msrmts(self) -> &'a mut W {
        self.variant(ADSEL5_A::AVG_64_MSRMTS)
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_128_msrmts(self) -> &'a mut W {
        self.variant(ADSEL5_A::AVG_128_MSRMTS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Select track and hold delay for this slot. NOTE: The track and hold delay must be less than 50us for correct operation. When the ADC is configured to use the 1.5Mhz clock, the track and hold delay cannot exceed 64 clocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum THSEL5_A {
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
impl From<THSEL5_A> for u8 {
    #[inline(always)]
    fn from(variant: THSEL5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `THSEL5`"]
pub type THSEL5_R = crate::R<u8, THSEL5_A>;
impl THSEL5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THSEL5_A {
        match self.bits {
            0 => THSEL5_A::_1_ADC_CLK,
            1 => THSEL5_A::_2_ADC_CLKS,
            2 => THSEL5_A::_4_ADC_CLKS,
            3 => THSEL5_A::_8_ADC_CLKS,
            4 => THSEL5_A::_16_ADC_CLKS,
            5 => THSEL5_A::_32_ADC_CLKS,
            6 => THSEL5_A::_64_ADC_CLKS,
            7 => THSEL5_A::_128_ADC_CLKS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1_ADC_CLK`"]
    #[inline(always)]
    pub fn is_1_adc_clk(&self) -> bool {
        *self == THSEL5_A::_1_ADC_CLK
    }
    #[doc = "Checks if the value of the field is `_2_ADC_CLKS`"]
    #[inline(always)]
    pub fn is_2_adc_clks(&self) -> bool {
        *self == THSEL5_A::_2_ADC_CLKS
    }
    #[doc = "Checks if the value of the field is `_4_ADC_CLKS`"]
    #[inline(always)]
    pub fn is_4_adc_clks(&self) -> bool {
        *self == THSEL5_A::_4_ADC_CLKS
    }
    #[doc = "Checks if the value of the field is `_8_ADC_CLKS`"]
    #[inline(always)]
    pub fn is_8_adc_clks(&self) -> bool {
        *self == THSEL5_A::_8_ADC_CLKS
    }
    #[doc = "Checks if the value of the field is `_16_ADC_CLKS`"]
    #[inline(always)]
    pub fn is_16_adc_clks(&self) -> bool {
        *self == THSEL5_A::_16_ADC_CLKS
    }
    #[doc = "Checks if the value of the field is `_32_ADC_CLKS`"]
    #[inline(always)]
    pub fn is_32_adc_clks(&self) -> bool {
        *self == THSEL5_A::_32_ADC_CLKS
    }
    #[doc = "Checks if the value of the field is `_64_ADC_CLKS`"]
    #[inline(always)]
    pub fn is_64_adc_clks(&self) -> bool {
        *self == THSEL5_A::_64_ADC_CLKS
    }
    #[doc = "Checks if the value of the field is `_128_ADC_CLKS`"]
    #[inline(always)]
    pub fn is_128_adc_clks(&self) -> bool {
        *self == THSEL5_A::_128_ADC_CLKS
    }
}
#[doc = "Write proxy for field `THSEL5`"]
pub struct THSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> THSEL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THSEL5_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1 ADC clock cycle."]
    #[inline(always)]
    pub fn _1_adc_clk(self) -> &'a mut W {
        self.variant(THSEL5_A::_1_ADC_CLK)
    }
    #[doc = "2 ADC clock cycles."]
    #[inline(always)]
    pub fn _2_adc_clks(self) -> &'a mut W {
        self.variant(THSEL5_A::_2_ADC_CLKS)
    }
    #[doc = "4 ADC clock cycles."]
    #[inline(always)]
    pub fn _4_adc_clks(self) -> &'a mut W {
        self.variant(THSEL5_A::_4_ADC_CLKS)
    }
    #[doc = "8 ADC clock cycles."]
    #[inline(always)]
    pub fn _8_adc_clks(self) -> &'a mut W {
        self.variant(THSEL5_A::_8_ADC_CLKS)
    }
    #[doc = "16 ADC clock cycles."]
    #[inline(always)]
    pub fn _16_adc_clks(self) -> &'a mut W {
        self.variant(THSEL5_A::_16_ADC_CLKS)
    }
    #[doc = "32 ADC clock cycles."]
    #[inline(always)]
    pub fn _32_adc_clks(self) -> &'a mut W {
        self.variant(THSEL5_A::_32_ADC_CLKS)
    }
    #[doc = "64 ADC clock cycles."]
    #[inline(always)]
    pub fn _64_adc_clks(self) -> &'a mut W {
        self.variant(THSEL5_A::_64_ADC_CLKS)
    }
    #[doc = "128 ADC clock cycles."]
    #[inline(always)]
    pub fn _128_adc_clks(self) -> &'a mut W {
        self.variant(THSEL5_A::_128_ADC_CLKS)
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
pub enum CHSEL5_A {
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
impl From<CHSEL5_A> for u8 {
    #[inline(always)]
    fn from(variant: CHSEL5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CHSEL5`"]
pub type CHSEL5_R = crate::R<u8, CHSEL5_A>;
impl CHSEL5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CHSEL5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CHSEL5_A::EXT0),
            1 => Val(CHSEL5_A::EXT1),
            2 => Val(CHSEL5_A::EXT2),
            3 => Val(CHSEL5_A::EXT3),
            4 => Val(CHSEL5_A::EXT4),
            5 => Val(CHSEL5_A::EXT5),
            6 => Val(CHSEL5_A::EXT6),
            7 => Val(CHSEL5_A::EXT7),
            8 => Val(CHSEL5_A::TEMP),
            9 => Val(CHSEL5_A::VDD),
            10 => Val(CHSEL5_A::VSS),
            12 => Val(CHSEL5_A::VBATT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EXT0`"]
    #[inline(always)]
    pub fn is_ext0(&self) -> bool {
        *self == CHSEL5_A::EXT0
    }
    #[doc = "Checks if the value of the field is `EXT1`"]
    #[inline(always)]
    pub fn is_ext1(&self) -> bool {
        *self == CHSEL5_A::EXT1
    }
    #[doc = "Checks if the value of the field is `EXT2`"]
    #[inline(always)]
    pub fn is_ext2(&self) -> bool {
        *self == CHSEL5_A::EXT2
    }
    #[doc = "Checks if the value of the field is `EXT3`"]
    #[inline(always)]
    pub fn is_ext3(&self) -> bool {
        *self == CHSEL5_A::EXT3
    }
    #[doc = "Checks if the value of the field is `EXT4`"]
    #[inline(always)]
    pub fn is_ext4(&self) -> bool {
        *self == CHSEL5_A::EXT4
    }
    #[doc = "Checks if the value of the field is `EXT5`"]
    #[inline(always)]
    pub fn is_ext5(&self) -> bool {
        *self == CHSEL5_A::EXT5
    }
    #[doc = "Checks if the value of the field is `EXT6`"]
    #[inline(always)]
    pub fn is_ext6(&self) -> bool {
        *self == CHSEL5_A::EXT6
    }
    #[doc = "Checks if the value of the field is `EXT7`"]
    #[inline(always)]
    pub fn is_ext7(&self) -> bool {
        *self == CHSEL5_A::EXT7
    }
    #[doc = "Checks if the value of the field is `TEMP`"]
    #[inline(always)]
    pub fn is_temp(&self) -> bool {
        *self == CHSEL5_A::TEMP
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == CHSEL5_A::VDD
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == CHSEL5_A::VSS
    }
    #[doc = "Checks if the value of the field is `VBATT`"]
    #[inline(always)]
    pub fn is_vbatt(&self) -> bool {
        *self == CHSEL5_A::VBATT
    }
}
#[doc = "Write proxy for field `CHSEL5`"]
pub struct CHSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ADC_EXT0 external GPIO pin connection."]
    #[inline(always)]
    pub fn ext0(self) -> &'a mut W {
        self.variant(CHSEL5_A::EXT0)
    }
    #[doc = "ADC_EXT1 external GPIO pin connection."]
    #[inline(always)]
    pub fn ext1(self) -> &'a mut W {
        self.variant(CHSEL5_A::EXT1)
    }
    #[doc = "ADC_EXT2 external GPIO pin connection."]
    #[inline(always)]
    pub fn ext2(self) -> &'a mut W {
        self.variant(CHSEL5_A::EXT2)
    }
    #[doc = "ADC_EXT3 external GPIO pin connection."]
    #[inline(always)]
    pub fn ext3(self) -> &'a mut W {
        self.variant(CHSEL5_A::EXT3)
    }
    #[doc = "ADC_EXT4 external GPIO pin connection."]
    #[inline(always)]
    pub fn ext4(self) -> &'a mut W {
        self.variant(CHSEL5_A::EXT4)
    }
    #[doc = "ADC_EXT5 external GPIO pin connection."]
    #[inline(always)]
    pub fn ext5(self) -> &'a mut W {
        self.variant(CHSEL5_A::EXT5)
    }
    #[doc = "ADC_EXT6 external GPIO pin connection."]
    #[inline(always)]
    pub fn ext6(self) -> &'a mut W {
        self.variant(CHSEL5_A::EXT6)
    }
    #[doc = "ADC_EXT7 external GPIO pin connection."]
    #[inline(always)]
    pub fn ext7(self) -> &'a mut W {
        self.variant(CHSEL5_A::EXT7)
    }
    #[doc = "ADC_TEMP internal temperature sensor."]
    #[inline(always)]
    pub fn temp(self) -> &'a mut W {
        self.variant(CHSEL5_A::TEMP)
    }
    #[doc = "ADC_VDD internal power rail connection."]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut W {
        self.variant(CHSEL5_A::VDD)
    }
    #[doc = "ADC_VSS internal ground connection."]
    #[inline(always)]
    pub fn vss(self) -> &'a mut W {
        self.variant(CHSEL5_A::VSS)
    }
    #[doc = "ADC_VBATT internal voltage divide-by-3 connection to input power rail."]
    #[inline(always)]
    pub fn vbatt(self) -> &'a mut W {
        self.variant(CHSEL5_A::VBATT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "This bit enables the window compare function for slot 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCEN5_A {
    #[doc = "1: Enable the window compare for slot 5."]
    WCEN = 1,
}
impl From<WCEN5_A> for bool {
    #[inline(always)]
    fn from(variant: WCEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WCEN5`"]
pub type WCEN5_R = crate::R<bool, WCEN5_A>;
impl WCEN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, WCEN5_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(WCEN5_A::WCEN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WCEN`"]
    #[inline(always)]
    pub fn is_wcen(&self) -> bool {
        *self == WCEN5_A::WCEN
    }
}
#[doc = "Write proxy for field `WCEN5`"]
pub struct WCEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> WCEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WCEN5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the window compare for slot 5."]
    #[inline(always)]
    pub fn wcen(self) -> &'a mut W {
        self.variant(WCEN5_A::WCEN)
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
#[doc = "This bit enables slot 5 for ADC conversions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEN5_A {
    #[doc = "1: Enable slot 5 for ADC conversions."]
    SLEN = 1,
}
impl From<SLEN5_A> for bool {
    #[inline(always)]
    fn from(variant: SLEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLEN5`"]
pub type SLEN5_R = crate::R<bool, SLEN5_A>;
impl SLEN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SLEN5_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SLEN5_A::SLEN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLEN`"]
    #[inline(always)]
    pub fn is_slen(&self) -> bool {
        *self == SLEN5_A::SLEN
    }
}
#[doc = "Write proxy for field `SLEN5`"]
pub struct SLEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEN5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable slot 5 for ADC conversions."]
    #[inline(always)]
    pub fn slen(self) -> &'a mut W {
        self.variant(SLEN5_A::SLEN)
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
    #[doc = "Bits 24:26 - Select number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn adsel5(&self) -> ADSEL5_R {
        ADSEL5_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Select track and hold delay for this slot. NOTE: The track and hold delay must be less than 50us for correct operation. When the ADC is configured to use the 1.5Mhz clock, the track and hold delay cannot exceed 64 clocks."]
    #[inline(always)]
    pub fn thsel5(&self) -> THSEL5_R {
        THSEL5_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - Select one of the 13 channel inputs for this slot."]
    #[inline(always)]
    pub fn chsel5(&self) -> CHSEL5_R {
        CHSEL5_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 5."]
    #[inline(always)]
    pub fn wcen5(&self) -> WCEN5_R {
        WCEN5_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit enables slot 5 for ADC conversions."]
    #[inline(always)]
    pub fn slen5(&self) -> SLEN5_R {
        SLEN5_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:26 - Select number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn adsel5(&mut self) -> ADSEL5_W {
        ADSEL5_W { w: self }
    }
    #[doc = "Bits 16:18 - Select track and hold delay for this slot. NOTE: The track and hold delay must be less than 50us for correct operation. When the ADC is configured to use the 1.5Mhz clock, the track and hold delay cannot exceed 64 clocks."]
    #[inline(always)]
    pub fn thsel5(&mut self) -> THSEL5_W {
        THSEL5_W { w: self }
    }
    #[doc = "Bits 8:11 - Select one of the 13 channel inputs for this slot."]
    #[inline(always)]
    pub fn chsel5(&mut self) -> CHSEL5_W {
        CHSEL5_W { w: self }
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 5."]
    #[inline(always)]
    pub fn wcen5(&mut self) -> WCEN5_W {
        WCEN5_W { w: self }
    }
    #[doc = "Bit 0 - This bit enables slot 5 for ADC conversions."]
    #[inline(always)]
    pub fn slen5(&mut self) -> SLEN5_W {
        SLEN5_W { w: self }
    }
}
