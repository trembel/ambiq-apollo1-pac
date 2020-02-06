#[doc = "Reader of register PADREGE"]
pub type R = crate::R<u32, super::PADREGE>;
#[doc = "Writer for register PADREGE"]
pub type W = crate::W<u32, super::PADREGE>;
#[doc = "Register PADREGE `reset()`'s with value 0x1818_1818"]
impl crate::ResetValue for super::PADREGE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1818_1818
    }
}
#[doc = "Pad 19 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD19FNCSEL_A {
    #[doc = "0: Configure as the analog comparator reference signal"]
    CMPRF = 0,
    #[doc = "1: Configure as the SPI channel 3 nCE signal from IOMSTR0"]
    M0NCE3 = 1,
    #[doc = "2: Configure as the input/output signal from CTIMER B1"]
    TCTB1 = 2,
    #[doc = "3: Configure as GPIO19"]
    GPIO19 = 3,
}
impl From<PAD19FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD19FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD19FNCSEL`"]
pub type PAD19FNCSEL_R = crate::R<u8, PAD19FNCSEL_A>;
impl PAD19FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD19FNCSEL_A {
        match self.bits {
            0 => PAD19FNCSEL_A::CMPRF,
            1 => PAD19FNCSEL_A::M0NCE3,
            2 => PAD19FNCSEL_A::TCTB1,
            3 => PAD19FNCSEL_A::GPIO19,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CMPRF`"]
    #[inline(always)]
    pub fn is_cmprf(&self) -> bool {
        *self == PAD19FNCSEL_A::CMPRF
    }
    #[doc = "Checks if the value of the field is `M0NCE3`"]
    #[inline(always)]
    pub fn is_m0n_ce3(&self) -> bool {
        *self == PAD19FNCSEL_A::M0NCE3
    }
    #[doc = "Checks if the value of the field is `TCTB1`"]
    #[inline(always)]
    pub fn is_tctb1(&self) -> bool {
        *self == PAD19FNCSEL_A::TCTB1
    }
    #[doc = "Checks if the value of the field is `GPIO19`"]
    #[inline(always)]
    pub fn is_gpio19(&self) -> bool {
        *self == PAD19FNCSEL_A::GPIO19
    }
}
#[doc = "Write proxy for field `PAD19FNCSEL`"]
pub struct PAD19FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD19FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD19FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the analog comparator reference signal"]
    #[inline(always)]
    pub fn cmprf(self) -> &'a mut W {
        self.variant(PAD19FNCSEL_A::CMPRF)
    }
    #[doc = "Configure as the SPI channel 3 nCE signal from IOMSTR0"]
    #[inline(always)]
    pub fn m0n_ce3(self) -> &'a mut W {
        self.variant(PAD19FNCSEL_A::M0NCE3)
    }
    #[doc = "Configure as the input/output signal from CTIMER B1"]
    #[inline(always)]
    pub fn tctb1(self) -> &'a mut W {
        self.variant(PAD19FNCSEL_A::TCTB1)
    }
    #[doc = "Configure as GPIO19"]
    #[inline(always)]
    pub fn gpio19(self) -> &'a mut W {
        self.variant(PAD19FNCSEL_A::GPIO19)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Pad 19 drive strentgh\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD19STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD19STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD19STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD19STRNG`"]
pub type PAD19STRNG_R = crate::R<bool, PAD19STRNG_A>;
impl PAD19STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD19STRNG_A {
        match self.bits {
            false => PAD19STRNG_A::LOW,
            true => PAD19STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD19STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD19STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD19STRNG`"]
pub struct PAD19STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD19STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD19STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD19STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD19STRNG_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Pad 19 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD19INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD19INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD19INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD19INPEN`"]
pub type PAD19INPEN_R = crate::R<bool, PAD19INPEN_A>;
impl PAD19INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD19INPEN_A {
        match self.bits {
            false => PAD19INPEN_A::DIS,
            true => PAD19INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD19INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD19INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD19INPEN`"]
pub struct PAD19INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD19INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD19INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD19INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD19INPEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Pad 19 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD19PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD19PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD19PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD19PULL`"]
pub type PAD19PULL_R = crate::R<bool, PAD19PULL_A>;
impl PAD19PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD19PULL_A {
        match self.bits {
            false => PAD19PULL_A::DIS,
            true => PAD19PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD19PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD19PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD19PULL`"]
pub struct PAD19PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD19PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD19PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD19PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD19PULL_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Pad 18 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD18FNCSEL_A {
    #[doc = "0: Configure as the analog comparator input 1 signal"]
    CMPIN1 = 0,
    #[doc = "1: Configure as the SPI channel 2 nCE signal from IOMSTR0"]
    M0NCE2 = 1,
    #[doc = "2: Configure as the input/output signal from CTIMER A1"]
    TCTA1 = 2,
    #[doc = "3: Configure as GPIO18"]
    GPIO18 = 3,
}
impl From<PAD18FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD18FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD18FNCSEL`"]
pub type PAD18FNCSEL_R = crate::R<u8, PAD18FNCSEL_A>;
impl PAD18FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD18FNCSEL_A {
        match self.bits {
            0 => PAD18FNCSEL_A::CMPIN1,
            1 => PAD18FNCSEL_A::M0NCE2,
            2 => PAD18FNCSEL_A::TCTA1,
            3 => PAD18FNCSEL_A::GPIO18,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CMPIN1`"]
    #[inline(always)]
    pub fn is_cmpin1(&self) -> bool {
        *self == PAD18FNCSEL_A::CMPIN1
    }
    #[doc = "Checks if the value of the field is `M0NCE2`"]
    #[inline(always)]
    pub fn is_m0n_ce2(&self) -> bool {
        *self == PAD18FNCSEL_A::M0NCE2
    }
    #[doc = "Checks if the value of the field is `TCTA1`"]
    #[inline(always)]
    pub fn is_tcta1(&self) -> bool {
        *self == PAD18FNCSEL_A::TCTA1
    }
    #[doc = "Checks if the value of the field is `GPIO18`"]
    #[inline(always)]
    pub fn is_gpio18(&self) -> bool {
        *self == PAD18FNCSEL_A::GPIO18
    }
}
#[doc = "Write proxy for field `PAD18FNCSEL`"]
pub struct PAD18FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD18FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD18FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the analog comparator input 1 signal"]
    #[inline(always)]
    pub fn cmpin1(self) -> &'a mut W {
        self.variant(PAD18FNCSEL_A::CMPIN1)
    }
    #[doc = "Configure as the SPI channel 2 nCE signal from IOMSTR0"]
    #[inline(always)]
    pub fn m0n_ce2(self) -> &'a mut W {
        self.variant(PAD18FNCSEL_A::M0NCE2)
    }
    #[doc = "Configure as the input/output signal from CTIMER A1"]
    #[inline(always)]
    pub fn tcta1(self) -> &'a mut W {
        self.variant(PAD18FNCSEL_A::TCTA1)
    }
    #[doc = "Configure as GPIO18"]
    #[inline(always)]
    pub fn gpio18(self) -> &'a mut W {
        self.variant(PAD18FNCSEL_A::GPIO18)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "Pad 18 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD18STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD18STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD18STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD18STRNG`"]
pub type PAD18STRNG_R = crate::R<bool, PAD18STRNG_A>;
impl PAD18STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD18STRNG_A {
        match self.bits {
            false => PAD18STRNG_A::LOW,
            true => PAD18STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD18STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD18STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD18STRNG`"]
pub struct PAD18STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD18STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD18STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD18STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD18STRNG_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Pad 18 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD18INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD18INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD18INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD18INPEN`"]
pub type PAD18INPEN_R = crate::R<bool, PAD18INPEN_A>;
impl PAD18INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD18INPEN_A {
        match self.bits {
            false => PAD18INPEN_A::DIS,
            true => PAD18INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD18INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD18INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD18INPEN`"]
pub struct PAD18INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD18INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD18INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD18INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD18INPEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Pad 18 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD18PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD18PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD18PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD18PULL`"]
pub type PAD18PULL_R = crate::R<bool, PAD18PULL_A>;
impl PAD18PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD18PULL_A {
        match self.bits {
            false => PAD18PULL_A::DIS,
            true => PAD18PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD18PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD18PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD18PULL`"]
pub struct PAD18PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD18PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD18PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD18PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD18PULL_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Pad 17 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD17FNCSEL_A {
    #[doc = "0: Configure as the analog comparator input 0 signal"]
    CMPIN0 = 0,
    #[doc = "1: Configure as the SPI channel 1 nCE signal from IOMSTR0"]
    M0NCE1 = 1,
    #[doc = "2: Configure as the ADC Trigger 3 signal"]
    TRIG3 = 2,
    #[doc = "3: Configure as GPIO17"]
    GPIO17 = 3,
    #[doc = "5: Configure as the external LFRC clock signal"]
    EXTLF = 5,
    #[doc = "7: Pad disabled"]
    DIS = 7,
}
impl From<PAD17FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD17FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD17FNCSEL`"]
pub type PAD17FNCSEL_R = crate::R<u8, PAD17FNCSEL_A>;
impl PAD17FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD17FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD17FNCSEL_A::CMPIN0),
            1 => Val(PAD17FNCSEL_A::M0NCE1),
            2 => Val(PAD17FNCSEL_A::TRIG3),
            3 => Val(PAD17FNCSEL_A::GPIO17),
            5 => Val(PAD17FNCSEL_A::EXTLF),
            7 => Val(PAD17FNCSEL_A::DIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CMPIN0`"]
    #[inline(always)]
    pub fn is_cmpin0(&self) -> bool {
        *self == PAD17FNCSEL_A::CMPIN0
    }
    #[doc = "Checks if the value of the field is `M0NCE1`"]
    #[inline(always)]
    pub fn is_m0n_ce1(&self) -> bool {
        *self == PAD17FNCSEL_A::M0NCE1
    }
    #[doc = "Checks if the value of the field is `TRIG3`"]
    #[inline(always)]
    pub fn is_trig3(&self) -> bool {
        *self == PAD17FNCSEL_A::TRIG3
    }
    #[doc = "Checks if the value of the field is `GPIO17`"]
    #[inline(always)]
    pub fn is_gpio17(&self) -> bool {
        *self == PAD17FNCSEL_A::GPIO17
    }
    #[doc = "Checks if the value of the field is `EXTLF`"]
    #[inline(always)]
    pub fn is_extlf(&self) -> bool {
        *self == PAD17FNCSEL_A::EXTLF
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD17FNCSEL_A::DIS
    }
}
#[doc = "Write proxy for field `PAD17FNCSEL`"]
pub struct PAD17FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD17FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD17FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the analog comparator input 0 signal"]
    #[inline(always)]
    pub fn cmpin0(self) -> &'a mut W {
        self.variant(PAD17FNCSEL_A::CMPIN0)
    }
    #[doc = "Configure as the SPI channel 1 nCE signal from IOMSTR0"]
    #[inline(always)]
    pub fn m0n_ce1(self) -> &'a mut W {
        self.variant(PAD17FNCSEL_A::M0NCE1)
    }
    #[doc = "Configure as the ADC Trigger 3 signal"]
    #[inline(always)]
    pub fn trig3(self) -> &'a mut W {
        self.variant(PAD17FNCSEL_A::TRIG3)
    }
    #[doc = "Configure as GPIO17"]
    #[inline(always)]
    pub fn gpio17(self) -> &'a mut W {
        self.variant(PAD17FNCSEL_A::GPIO17)
    }
    #[doc = "Configure as the external LFRC clock signal"]
    #[inline(always)]
    pub fn extlf(self) -> &'a mut W {
        self.variant(PAD17FNCSEL_A::EXTLF)
    }
    #[doc = "Pad disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD17FNCSEL_A::DIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 17 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD17STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD17STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD17STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD17STRNG`"]
pub type PAD17STRNG_R = crate::R<bool, PAD17STRNG_A>;
impl PAD17STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD17STRNG_A {
        match self.bits {
            false => PAD17STRNG_A::LOW,
            true => PAD17STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD17STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD17STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD17STRNG`"]
pub struct PAD17STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD17STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD17STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD17STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD17STRNG_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Pad 17 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD17INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD17INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD17INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD17INPEN`"]
pub type PAD17INPEN_R = crate::R<bool, PAD17INPEN_A>;
impl PAD17INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD17INPEN_A {
        match self.bits {
            false => PAD17INPEN_A::DIS,
            true => PAD17INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD17INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD17INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD17INPEN`"]
pub struct PAD17INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD17INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD17INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD17INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD17INPEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Pad 17 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD17PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD17PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD17PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD17PULL`"]
pub type PAD17PULL_R = crate::R<bool, PAD17PULL_A>;
impl PAD17PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD17PULL_A {
        match self.bits {
            false => PAD17PULL_A::DIS,
            true => PAD17PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD17PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD17PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD17PULL`"]
pub struct PAD17PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD17PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD17PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD17PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD17PULL_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Pad 16 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD16FNCSEL_A {
    #[doc = "0: Configure as the analog ADC reference input signal"]
    ADCREF = 0,
    #[doc = "1: Configure as the SPI channel 4 nCE signal from IOMSTR0"]
    M0NCE4 = 1,
    #[doc = "2: Configure as the ADC Trigger 2 signal"]
    TRIG2 = 2,
    #[doc = "3: Configure as GPIO16"]
    GPIO16 = 3,
}
impl From<PAD16FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD16FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD16FNCSEL`"]
pub type PAD16FNCSEL_R = crate::R<u8, PAD16FNCSEL_A>;
impl PAD16FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD16FNCSEL_A {
        match self.bits {
            0 => PAD16FNCSEL_A::ADCREF,
            1 => PAD16FNCSEL_A::M0NCE4,
            2 => PAD16FNCSEL_A::TRIG2,
            3 => PAD16FNCSEL_A::GPIO16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCREF`"]
    #[inline(always)]
    pub fn is_adcref(&self) -> bool {
        *self == PAD16FNCSEL_A::ADCREF
    }
    #[doc = "Checks if the value of the field is `M0NCE4`"]
    #[inline(always)]
    pub fn is_m0n_ce4(&self) -> bool {
        *self == PAD16FNCSEL_A::M0NCE4
    }
    #[doc = "Checks if the value of the field is `TRIG2`"]
    #[inline(always)]
    pub fn is_trig2(&self) -> bool {
        *self == PAD16FNCSEL_A::TRIG2
    }
    #[doc = "Checks if the value of the field is `GPIO16`"]
    #[inline(always)]
    pub fn is_gpio16(&self) -> bool {
        *self == PAD16FNCSEL_A::GPIO16
    }
}
#[doc = "Write proxy for field `PAD16FNCSEL`"]
pub struct PAD16FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD16FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD16FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the analog ADC reference input signal"]
    #[inline(always)]
    pub fn adcref(self) -> &'a mut W {
        self.variant(PAD16FNCSEL_A::ADCREF)
    }
    #[doc = "Configure as the SPI channel 4 nCE signal from IOMSTR0"]
    #[inline(always)]
    pub fn m0n_ce4(self) -> &'a mut W {
        self.variant(PAD16FNCSEL_A::M0NCE4)
    }
    #[doc = "Configure as the ADC Trigger 2 signal"]
    #[inline(always)]
    pub fn trig2(self) -> &'a mut W {
        self.variant(PAD16FNCSEL_A::TRIG2)
    }
    #[doc = "Configure as GPIO16"]
    #[inline(always)]
    pub fn gpio16(self) -> &'a mut W {
        self.variant(PAD16FNCSEL_A::GPIO16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Pad 16 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD16STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD16STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD16STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD16STRNG`"]
pub type PAD16STRNG_R = crate::R<bool, PAD16STRNG_A>;
impl PAD16STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD16STRNG_A {
        match self.bits {
            false => PAD16STRNG_A::LOW,
            true => PAD16STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD16STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD16STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD16STRNG`"]
pub struct PAD16STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD16STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD16STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD16STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD16STRNG_A::HIGH)
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
#[doc = "Pad 16 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD16INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD16INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD16INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD16INPEN`"]
pub type PAD16INPEN_R = crate::R<bool, PAD16INPEN_A>;
impl PAD16INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD16INPEN_A {
        match self.bits {
            false => PAD16INPEN_A::DIS,
            true => PAD16INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD16INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD16INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD16INPEN`"]
pub struct PAD16INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD16INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD16INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD16INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD16INPEN_A::EN)
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
#[doc = "Pad 16 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD16PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD16PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD16PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD16PULL`"]
pub type PAD16PULL_R = crate::R<bool, PAD16PULL_A>;
impl PAD16PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD16PULL_A {
        match self.bits {
            false => PAD16PULL_A::DIS,
            true => PAD16PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD16PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD16PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD16PULL`"]
pub struct PAD16PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD16PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD16PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD16PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD16PULL_A::EN)
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
    #[doc = "Bits 27:28 - Pad 19 function select"]
    #[inline(always)]
    pub fn pad19fncsel(&self) -> PAD19FNCSEL_R {
        PAD19FNCSEL_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Pad 19 drive strentgh"]
    #[inline(always)]
    pub fn pad19strng(&self) -> PAD19STRNG_R {
        PAD19STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 19 input enable"]
    #[inline(always)]
    pub fn pad19inpen(&self) -> PAD19INPEN_R {
        PAD19INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 19 pullup enable"]
    #[inline(always)]
    pub fn pad19pull(&self) -> PAD19PULL_R {
        PAD19PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:20 - Pad 18 function select"]
    #[inline(always)]
    pub fn pad18fncsel(&self) -> PAD18FNCSEL_R {
        PAD18FNCSEL_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Pad 18 drive strength"]
    #[inline(always)]
    pub fn pad18strng(&self) -> PAD18STRNG_R {
        PAD18STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 18 input enable"]
    #[inline(always)]
    pub fn pad18inpen(&self) -> PAD18INPEN_R {
        PAD18INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 18 pullup enable"]
    #[inline(always)]
    pub fn pad18pull(&self) -> PAD18PULL_R {
        PAD18PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - Pad 17 function select"]
    #[inline(always)]
    pub fn pad17fncsel(&self) -> PAD17FNCSEL_R {
        PAD17FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 17 drive strength"]
    #[inline(always)]
    pub fn pad17strng(&self) -> PAD17STRNG_R {
        PAD17STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 17 input enable"]
    #[inline(always)]
    pub fn pad17inpen(&self) -> PAD17INPEN_R {
        PAD17INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 17 pullup enable"]
    #[inline(always)]
    pub fn pad17pull(&self) -> PAD17PULL_R {
        PAD17PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Pad 16 function select"]
    #[inline(always)]
    pub fn pad16fncsel(&self) -> PAD16FNCSEL_R {
        PAD16FNCSEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2 - Pad 16 drive strength"]
    #[inline(always)]
    pub fn pad16strng(&self) -> PAD16STRNG_R {
        PAD16STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 16 input enable"]
    #[inline(always)]
    pub fn pad16inpen(&self) -> PAD16INPEN_R {
        PAD16INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 16 pullup enable"]
    #[inline(always)]
    pub fn pad16pull(&self) -> PAD16PULL_R {
        PAD16PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:28 - Pad 19 function select"]
    #[inline(always)]
    pub fn pad19fncsel(&mut self) -> PAD19FNCSEL_W {
        PAD19FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 19 drive strentgh"]
    #[inline(always)]
    pub fn pad19strng(&mut self) -> PAD19STRNG_W {
        PAD19STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 19 input enable"]
    #[inline(always)]
    pub fn pad19inpen(&mut self) -> PAD19INPEN_W {
        PAD19INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 19 pullup enable"]
    #[inline(always)]
    pub fn pad19pull(&mut self) -> PAD19PULL_W {
        PAD19PULL_W { w: self }
    }
    #[doc = "Bits 19:20 - Pad 18 function select"]
    #[inline(always)]
    pub fn pad18fncsel(&mut self) -> PAD18FNCSEL_W {
        PAD18FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 18 drive strength"]
    #[inline(always)]
    pub fn pad18strng(&mut self) -> PAD18STRNG_W {
        PAD18STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 18 input enable"]
    #[inline(always)]
    pub fn pad18inpen(&mut self) -> PAD18INPEN_W {
        PAD18INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 18 pullup enable"]
    #[inline(always)]
    pub fn pad18pull(&mut self) -> PAD18PULL_W {
        PAD18PULL_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 17 function select"]
    #[inline(always)]
    pub fn pad17fncsel(&mut self) -> PAD17FNCSEL_W {
        PAD17FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 17 drive strength"]
    #[inline(always)]
    pub fn pad17strng(&mut self) -> PAD17STRNG_W {
        PAD17STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 17 input enable"]
    #[inline(always)]
    pub fn pad17inpen(&mut self) -> PAD17INPEN_W {
        PAD17INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 17 pullup enable"]
    #[inline(always)]
    pub fn pad17pull(&mut self) -> PAD17PULL_W {
        PAD17PULL_W { w: self }
    }
    #[doc = "Bits 3:4 - Pad 16 function select"]
    #[inline(always)]
    pub fn pad16fncsel(&mut self) -> PAD16FNCSEL_W {
        PAD16FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 16 drive strength"]
    #[inline(always)]
    pub fn pad16strng(&mut self) -> PAD16STRNG_W {
        PAD16STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 16 input enable"]
    #[inline(always)]
    pub fn pad16inpen(&mut self) -> PAD16INPEN_W {
        PAD16INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 16 pullup enable"]
    #[inline(always)]
    pub fn pad16pull(&mut self) -> PAD16PULL_W {
        PAD16PULL_W { w: self }
    }
}
