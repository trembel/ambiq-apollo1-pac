#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select the source and frequency for the ADC clock. All values not enumerated below are undefined.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: Low Power Mode."]
    OFF = 0,
    #[doc = "1: 12 MHz ADC clock."]
    _12MHZ = 1,
    #[doc = "2: 6 MHz ADC clock."]
    _6MHZ = 2,
    #[doc = "3: 12 MHz ADC clock."]
    _3MHZ = 3,
    #[doc = "4: 1.5 MHz ADC clock."]
    _1_5MHZ = 4,
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
    pub fn variant(&self) -> crate::Variant<u8, CLKSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKSEL_A::OFF),
            1 => Val(CLKSEL_A::_12MHZ),
            2 => Val(CLKSEL_A::_6MHZ),
            3 => Val(CLKSEL_A::_3MHZ),
            4 => Val(CLKSEL_A::_1_5MHZ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CLKSEL_A::OFF
    }
    #[doc = "Checks if the value of the field is `_12MHZ`"]
    #[inline(always)]
    pub fn is_12mhz(&self) -> bool {
        *self == CLKSEL_A::_12MHZ
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
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Low Power Mode."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CLKSEL_A::OFF)
    }
    #[doc = "12 MHz ADC clock."]
    #[inline(always)]
    pub fn _12mhz(self) -> &'a mut W {
        self.variant(CLKSEL_A::_12MHZ)
    }
    #[doc = "6 MHz ADC clock."]
    #[inline(always)]
    pub fn _6mhz(self) -> &'a mut W {
        self.variant(CLKSEL_A::_6MHZ)
    }
    #[doc = "12 MHz ADC clock."]
    #[inline(always)]
    pub fn _3mhz(self) -> &'a mut W {
        self.variant(CLKSEL_A::_3MHZ)
    }
    #[doc = "1.5 MHz ADC clock."]
    #[inline(always)]
    pub fn _1_5mhz(self) -> &'a mut W {
        self.variant(CLKSEL_A::_1_5MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "This bit selects the ADC trigger polarity for external off chip triggers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGPOL_A {
    #[doc = "0: Trigger on rising edge."]
    RISING_EDGE = 0,
    #[doc = "1: Trigger on falling edge."]
    FALLING_EDGE = 1,
}
impl From<TRIGPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIGPOL`"]
pub type TRIGPOL_R = crate::R<bool, TRIGPOL_A>;
impl TRIGPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGPOL_A {
        match self.bits {
            false => TRIGPOL_A::RISING_EDGE,
            true => TRIGPOL_A::FALLING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == TRIGPOL_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == TRIGPOL_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `TRIGPOL`"]
pub struct TRIGPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger on rising edge."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(TRIGPOL_A::RISING_EDGE)
    }
    #[doc = "Trigger on falling edge."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(TRIGPOL_A::FALLING_EDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Select the ADC trigger source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGSEL_A {
    #[doc = "0: Off chip External Trigger0 (ADC_ET0)"]
    EXT0 = 0,
    #[doc = "1: Off chip External Trigger1 (ADC_ET1)"]
    EXT1 = 1,
    #[doc = "2: Off chip External Trigger2 (ADC_ET2)"]
    EXT2 = 2,
    #[doc = "3: Off chip External Trigger3 (ADC_ET3)"]
    EXT3 = 3,
    #[doc = "4: Off chip External Trigger4 (ADC_ET4)"]
    EXT4 = 4,
    #[doc = "5: Off chip External Trigger5 (ADC_ET5)"]
    EXT5 = 5,
    #[doc = "6: Off chip External Trigger6 (ADC_ET6)"]
    EXT6 = 6,
    #[doc = "7: Off chip External Trigger7 (ADC_ET7)"]
    EXT7 = 7,
    #[doc = "8: Software Trigger"]
    SWT = 8,
}
impl From<TRIGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRIGSEL`"]
pub type TRIGSEL_R = crate::R<u8, TRIGSEL_A>;
impl TRIGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRIGSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRIGSEL_A::EXT0),
            1 => Val(TRIGSEL_A::EXT1),
            2 => Val(TRIGSEL_A::EXT2),
            3 => Val(TRIGSEL_A::EXT3),
            4 => Val(TRIGSEL_A::EXT4),
            5 => Val(TRIGSEL_A::EXT5),
            6 => Val(TRIGSEL_A::EXT6),
            7 => Val(TRIGSEL_A::EXT7),
            8 => Val(TRIGSEL_A::SWT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EXT0`"]
    #[inline(always)]
    pub fn is_ext0(&self) -> bool {
        *self == TRIGSEL_A::EXT0
    }
    #[doc = "Checks if the value of the field is `EXT1`"]
    #[inline(always)]
    pub fn is_ext1(&self) -> bool {
        *self == TRIGSEL_A::EXT1
    }
    #[doc = "Checks if the value of the field is `EXT2`"]
    #[inline(always)]
    pub fn is_ext2(&self) -> bool {
        *self == TRIGSEL_A::EXT2
    }
    #[doc = "Checks if the value of the field is `EXT3`"]
    #[inline(always)]
    pub fn is_ext3(&self) -> bool {
        *self == TRIGSEL_A::EXT3
    }
    #[doc = "Checks if the value of the field is `EXT4`"]
    #[inline(always)]
    pub fn is_ext4(&self) -> bool {
        *self == TRIGSEL_A::EXT4
    }
    #[doc = "Checks if the value of the field is `EXT5`"]
    #[inline(always)]
    pub fn is_ext5(&self) -> bool {
        *self == TRIGSEL_A::EXT5
    }
    #[doc = "Checks if the value of the field is `EXT6`"]
    #[inline(always)]
    pub fn is_ext6(&self) -> bool {
        *self == TRIGSEL_A::EXT6
    }
    #[doc = "Checks if the value of the field is `EXT7`"]
    #[inline(always)]
    pub fn is_ext7(&self) -> bool {
        *self == TRIGSEL_A::EXT7
    }
    #[doc = "Checks if the value of the field is `SWT`"]
    #[inline(always)]
    pub fn is_swt(&self) -> bool {
        *self == TRIGSEL_A::SWT
    }
}
#[doc = "Write proxy for field `TRIGSEL`"]
pub struct TRIGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Off chip External Trigger0 (ADC_ET0)"]
    #[inline(always)]
    pub fn ext0(self) -> &'a mut W {
        self.variant(TRIGSEL_A::EXT0)
    }
    #[doc = "Off chip External Trigger1 (ADC_ET1)"]
    #[inline(always)]
    pub fn ext1(self) -> &'a mut W {
        self.variant(TRIGSEL_A::EXT1)
    }
    #[doc = "Off chip External Trigger2 (ADC_ET2)"]
    #[inline(always)]
    pub fn ext2(self) -> &'a mut W {
        self.variant(TRIGSEL_A::EXT2)
    }
    #[doc = "Off chip External Trigger3 (ADC_ET3)"]
    #[inline(always)]
    pub fn ext3(self) -> &'a mut W {
        self.variant(TRIGSEL_A::EXT3)
    }
    #[doc = "Off chip External Trigger4 (ADC_ET4)"]
    #[inline(always)]
    pub fn ext4(self) -> &'a mut W {
        self.variant(TRIGSEL_A::EXT4)
    }
    #[doc = "Off chip External Trigger5 (ADC_ET5)"]
    #[inline(always)]
    pub fn ext5(self) -> &'a mut W {
        self.variant(TRIGSEL_A::EXT5)
    }
    #[doc = "Off chip External Trigger6 (ADC_ET6)"]
    #[inline(always)]
    pub fn ext6(self) -> &'a mut W {
        self.variant(TRIGSEL_A::EXT6)
    }
    #[doc = "Off chip External Trigger7 (ADC_ET7)"]
    #[inline(always)]
    pub fn ext7(self) -> &'a mut W {
        self.variant(TRIGSEL_A::EXT7)
    }
    #[doc = "Software Trigger"]
    #[inline(always)]
    pub fn swt(self) -> &'a mut W {
        self.variant(TRIGSEL_A::SWT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Select the ADC reference voltage.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Internal Bandgap Reference Voltage"]
    INTERNAL = 0,
    #[doc = "1: Select VDD as the ADEC reference voltage."]
    VDD = 1,
    #[doc = "2: Off Chip Reference (ADC_REF)"]
    ADCREF = 2,
    #[doc = "3: Reserved"]
    UNDEFINED = 3,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REFSEL`"]
pub type REFSEL_R = crate::R<u8, REFSEL_A>;
impl REFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFSEL_A {
        match self.bits {
            0 => REFSEL_A::INTERNAL,
            1 => REFSEL_A::VDD,
            2 => REFSEL_A::ADCREF,
            3 => REFSEL_A::UNDEFINED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == REFSEL_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == REFSEL_A::VDD
    }
    #[doc = "Checks if the value of the field is `ADCREF`"]
    #[inline(always)]
    pub fn is_adcref(&self) -> bool {
        *self == REFSEL_A::ADCREF
    }
    #[doc = "Checks if the value of the field is `UNDEFINED`"]
    #[inline(always)]
    pub fn is_undefined(&self) -> bool {
        *self == REFSEL_A::UNDEFINED
    }
}
#[doc = "Write proxy for field `REFSEL`"]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Internal Bandgap Reference Voltage"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(REFSEL_A::INTERNAL)
    }
    #[doc = "Select VDD as the ADEC reference voltage."]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::VDD)
    }
    #[doc = "Off Chip Reference (ADC_REF)"]
    #[inline(always)]
    pub fn adcref(self) -> &'a mut W {
        self.variant(REFSEL_A::ADCREF)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn undefined(self) -> &'a mut W {
        self.variant(REFSEL_A::UNDEFINED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Control 500 Ohm battery load resistor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BATTLOAD_A {
    #[doc = "0: Disable battery load."]
    DIS = 0,
    #[doc = "1: Enable battery load."]
    EN = 1,
}
impl From<BATTLOAD_A> for bool {
    #[inline(always)]
    fn from(variant: BATTLOAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BATTLOAD`"]
pub type BATTLOAD_R = crate::R<bool, BATTLOAD_A>;
impl BATTLOAD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BATTLOAD_A {
        match self.bits {
            false => BATTLOAD_A::DIS,
            true => BATTLOAD_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BATTLOAD_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == BATTLOAD_A::EN
    }
}
#[doc = "Write proxy for field `BATTLOAD`"]
pub struct BATTLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> BATTLOAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BATTLOAD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable battery load."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BATTLOAD_A::DIS)
    }
    #[doc = "Enable battery load."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BATTLOAD_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Select the sample rate mode. It adjusts the current in the ADC for higher sample rates. A 12MHz ADC clock can result in a sample rate up to 1Msps depending on the trigger or repeating mode rate. A 1.5MHz ADC clock can result in a sample rate up 125K sps. NOTE: All other values not specified below are undefined.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPMODE_A {
    #[doc = "0: Sample Rate <= 125K sps"]
    SAMPLE_RATE_LE_125KSPS = 0,
    #[doc = "2: Sample Rate 125K to 1M sps"]
    SAMPLE_RATE_125K_1MSPS = 2,
}
impl From<OPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OPMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPMODE`"]
pub type OPMODE_R = crate::R<u8, OPMODE_A>;
impl OPMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPMODE_A::SAMPLE_RATE_LE_125KSPS),
            2 => Val(OPMODE_A::SAMPLE_RATE_125K_1MSPS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAMPLE_RATE_LE_125KSPS`"]
    #[inline(always)]
    pub fn is_sample_rate_le_125ksps(&self) -> bool {
        *self == OPMODE_A::SAMPLE_RATE_LE_125KSPS
    }
    #[doc = "Checks if the value of the field is `SAMPLE_RATE_125K_1MSPS`"]
    #[inline(always)]
    pub fn is_sample_rate_125k_1msps(&self) -> bool {
        *self == OPMODE_A::SAMPLE_RATE_125K_1MSPS
    }
}
#[doc = "Write proxy for field `OPMODE`"]
pub struct OPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Sample Rate <= 125K sps"]
    #[inline(always)]
    pub fn sample_rate_le_125ksps(self) -> &'a mut W {
        self.variant(OPMODE_A::SAMPLE_RATE_LE_125KSPS)
    }
    #[doc = "Sample Rate 125K to 1M sps"]
    #[inline(always)]
    pub fn sample_rate_125k_1msps(self) -> &'a mut W {
        self.variant(OPMODE_A::SAMPLE_RATE_125K_1MSPS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Select power mode to enter between active scans.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPMODE_A {
    #[doc = "0: Low Power Mode 0 (2'b00).  Leaves the ADC fully powered between scans with no latency between a trigger event and sample data collection."]
    MODE0 = 0,
    #[doc = "1: Low Power Mode 1 (2'b01).  Enables a low power mode for the ADC between scans requiring 50us initialization time (latency) between a trigger event and the scan (assuming the HFRC remains running and the MCU is not in deepsleep mode in which case additional startup latency for HFRC startup is required)."]
    MODE1 = 1,
    #[doc = "2: Low Power Mode 2 (2'b10).  Disconnects power and clocks to the ADC effectively eliminating all active power associated with the ADC between scans. This mode requires 150us initialization (again, assuming the HFRC remains running and the MCU is not in deepsleep mode in which case additional startup latency for HFRC startup is required)."]
    MODE2 = 2,
    #[doc = "3: Undefined Mode (2'b11)"]
    MODE_UNDEFINED = 3,
}
impl From<LPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LPMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPMODE`"]
pub type LPMODE_R = crate::R<u8, LPMODE_A>;
impl LPMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPMODE_A {
        match self.bits {
            0 => LPMODE_A::MODE0,
            1 => LPMODE_A::MODE1,
            2 => LPMODE_A::MODE2,
            3 => LPMODE_A::MODE_UNDEFINED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == LPMODE_A::MODE0
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == LPMODE_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == LPMODE_A::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE_UNDEFINED`"]
    #[inline(always)]
    pub fn is_mode_undefined(&self) -> bool {
        *self == LPMODE_A::MODE_UNDEFINED
    }
}
#[doc = "Write proxy for field `LPMODE`"]
pub struct LPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low Power Mode 0 (2'b00). Leaves the ADC fully powered between scans with no latency between a trigger event and sample data collection."]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(LPMODE_A::MODE0)
    }
    #[doc = "Low Power Mode 1 (2'b01). Enables a low power mode for the ADC between scans requiring 50us initialization time (latency) between a trigger event and the scan (assuming the HFRC remains running and the MCU is not in deepsleep mode in which case additional startup latency for HFRC startup is required)."]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(LPMODE_A::MODE1)
    }
    #[doc = "Low Power Mode 2 (2'b10). Disconnects power and clocks to the ADC effectively eliminating all active power associated with the ADC between scans. This mode requires 150us initialization (again, assuming the HFRC remains running and the MCU is not in deepsleep mode in which case additional startup latency for HFRC startup is required)."]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(LPMODE_A::MODE2)
    }
    #[doc = "Undefined Mode (2'b11)"]
    #[inline(always)]
    pub fn mode_undefined(self) -> &'a mut W {
        self.variant(LPMODE_A::MODE_UNDEFINED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "This bit enables Repeating Scan Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPTEN_A {
    #[doc = "0: In Single Scan Mode, the ADC will complete a single scan upon each trigger event."]
    SINGLE_SCAN = 0,
    #[doc = "1: In Repeating Scan Mode, the ADC will complete it's first scan upon the initial trigger event and all subsequent scans will occur at regular intervals defined by the configuration programmed for the CTTMRA3 internal timer until the timer is disabled or the ADC is disabled."]
    REPEATING_SCAN = 1,
}
impl From<RPTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RPTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RPTEN`"]
pub type RPTEN_R = crate::R<bool, RPTEN_A>;
impl RPTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPTEN_A {
        match self.bits {
            false => RPTEN_A::SINGLE_SCAN,
            true => RPTEN_A::REPEATING_SCAN,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_SCAN`"]
    #[inline(always)]
    pub fn is_single_scan(&self) -> bool {
        *self == RPTEN_A::SINGLE_SCAN
    }
    #[doc = "Checks if the value of the field is `REPEATING_SCAN`"]
    #[inline(always)]
    pub fn is_repeating_scan(&self) -> bool {
        *self == RPTEN_A::REPEATING_SCAN
    }
}
#[doc = "Write proxy for field `RPTEN`"]
pub struct RPTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RPTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "In Single Scan Mode, the ADC will complete a single scan upon each trigger event."]
    #[inline(always)]
    pub fn single_scan(self) -> &'a mut W {
        self.variant(RPTEN_A::SINGLE_SCAN)
    }
    #[doc = "In Repeating Scan Mode, the ADC will complete it's first scan upon the initial trigger event and all subsequent scans will occur at regular intervals defined by the configuration programmed for the CTTMRA3 internal timer until the timer is disabled or the ADC is disabled."]
    #[inline(always)]
    pub fn repeating_scan(self) -> &'a mut W {
        self.variant(RPTEN_A::REPEATING_SCAN)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "This enables power to the temperature sensor module. After setting this bit, the temperature sensor will remain powered down while the ADC is power is disconnected (i.e, when the ADC PWDSTAT is 2'b10).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMPSPWR_A {
    #[doc = "0: Power down the temperature sensor."]
    DIS = 0,
    #[doc = "1: Enable the temperature sensor when the ADC is in it's active state."]
    EN = 1,
}
impl From<TMPSPWR_A> for bool {
    #[inline(always)]
    fn from(variant: TMPSPWR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMPSPWR`"]
pub type TMPSPWR_R = crate::R<bool, TMPSPWR_A>;
impl TMPSPWR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMPSPWR_A {
        match self.bits {
            false => TMPSPWR_A::DIS,
            true => TMPSPWR_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMPSPWR_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMPSPWR_A::EN
    }
}
#[doc = "Write proxy for field `TMPSPWR`"]
pub struct TMPSPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMPSPWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMPSPWR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power down the temperature sensor."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMPSPWR_A::DIS)
    }
    #[doc = "Enable the temperature sensor when the ADC is in it's active state."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMPSPWR_A::EN)
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
#[doc = "This bit enables the ADC module. While the ADC is enabled, the ADCCFG and SLOT Configuration regsiter settings must remain stable and unchanged.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCEN_A {
    #[doc = "0: Disable the ADC module."]
    DIS = 0,
    #[doc = "1: Enable the ADC module."]
    EN = 1,
}
impl From<ADCEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCEN`"]
pub type ADCEN_R = crate::R<bool, ADCEN_A>;
impl ADCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCEN_A {
        match self.bits {
            false => ADCEN_A::DIS,
            true => ADCEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ADCEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ADCEN_A::EN
    }
}
#[doc = "Write proxy for field `ADCEN`"]
pub struct ADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the ADC module."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ADCEN_A::DIS)
    }
    #[doc = "Enable the ADC module."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ADCEN_A::EN)
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
    #[doc = "Bits 24:26 - Select the source and frequency for the ADC clock. All values not enumerated below are undefined."]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 20 - This bit selects the ADC trigger polarity for external off chip triggers."]
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Select the ADC trigger source."]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Select the ADC reference voltage."]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Control 500 Ohm battery load resistor."]
    #[inline(always)]
    pub fn battload(&self) -> BATTLOAD_R {
        BATTLOAD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Select the sample rate mode. It adjusts the current in the ADC for higher sample rates. A 12MHz ADC clock can result in a sample rate up to 1Msps depending on the trigger or repeating mode rate. A 1.5MHz ADC clock can result in a sample rate up 125K sps. NOTE: All other values not specified below are undefined."]
    #[inline(always)]
    pub fn opmode(&self) -> OPMODE_R {
        OPMODE_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - Select power mode to enter between active scans."]
    #[inline(always)]
    pub fn lpmode(&self) -> LPMODE_R {
        LPMODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2 - This bit enables Repeating Scan Mode."]
    #[inline(always)]
    pub fn rpten(&self) -> RPTEN_R {
        RPTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This enables power to the temperature sensor module. After setting this bit, the temperature sensor will remain powered down while the ADC is power is disconnected (i.e, when the ADC PWDSTAT is 2'b10)."]
    #[inline(always)]
    pub fn tmpspwr(&self) -> TMPSPWR_R {
        TMPSPWR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit enables the ADC module. While the ADC is enabled, the ADCCFG and SLOT Configuration regsiter settings must remain stable and unchanged."]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:26 - Select the source and frequency for the ADC clock. All values not enumerated below are undefined."]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W { w: self }
    }
    #[doc = "Bit 20 - This bit selects the ADC trigger polarity for external off chip triggers."]
    #[inline(always)]
    pub fn trigpol(&mut self) -> TRIGPOL_W {
        TRIGPOL_W { w: self }
    }
    #[doc = "Bits 16:19 - Select the ADC trigger source."]
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W {
        TRIGSEL_W { w: self }
    }
    #[doc = "Bits 8:9 - Select the ADC reference voltage."]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
    #[doc = "Bit 7 - Control 500 Ohm battery load resistor."]
    #[inline(always)]
    pub fn battload(&mut self) -> BATTLOAD_W {
        BATTLOAD_W { w: self }
    }
    #[doc = "Bits 5:6 - Select the sample rate mode. It adjusts the current in the ADC for higher sample rates. A 12MHz ADC clock can result in a sample rate up to 1Msps depending on the trigger or repeating mode rate. A 1.5MHz ADC clock can result in a sample rate up 125K sps. NOTE: All other values not specified below are undefined."]
    #[inline(always)]
    pub fn opmode(&mut self) -> OPMODE_W {
        OPMODE_W { w: self }
    }
    #[doc = "Bits 3:4 - Select power mode to enter between active scans."]
    #[inline(always)]
    pub fn lpmode(&mut self) -> LPMODE_W {
        LPMODE_W { w: self }
    }
    #[doc = "Bit 2 - This bit enables Repeating Scan Mode."]
    #[inline(always)]
    pub fn rpten(&mut self) -> RPTEN_W {
        RPTEN_W { w: self }
    }
    #[doc = "Bit 1 - This enables power to the temperature sensor module. After setting this bit, the temperature sensor will remain powered down while the ADC is power is disconnected (i.e, when the ADC PWDSTAT is 2'b10)."]
    #[inline(always)]
    pub fn tmpspwr(&mut self) -> TMPSPWR_W {
        TMPSPWR_W { w: self }
    }
    #[doc = "Bit 0 - This bit enables the ADC module. While the ADC is enabled, the ADCCFG and SLOT Configuration regsiter settings must remain stable and unchanged."]
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W {
        ADCEN_W { w: self }
    }
}
