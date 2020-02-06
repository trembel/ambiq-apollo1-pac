#[doc = "Reader of register PADREGK"]
pub type R = crate::R<u32, super::PADREGK>;
#[doc = "Writer for register PADREGK"]
pub type W = crate::W<u32, super::PADREGK>;
#[doc = "Register PADREGK `reset()`'s with value 0x1818_1818"]
impl crate::ResetValue for super::PADREGK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1818_1818
    }
}
#[doc = "Pad 43 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD43FNCSEL_A {
    #[doc = "0: Configure as the ADC Trigger 6 signal"]
    TRIG6 = 0,
    #[doc = "1: Configure as the SPI channel 1 nCE signal from IOMSTR0"]
    M0NCE1 = 1,
    #[doc = "2: Configure as the input/output signal from CTIMER B0"]
    TCTB0 = 2,
    #[doc = "3: Configure as GPIO43"]
    GPIO43 = 3,
}
impl From<PAD43FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD43FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD43FNCSEL`"]
pub type PAD43FNCSEL_R = crate::R<u8, PAD43FNCSEL_A>;
impl PAD43FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD43FNCSEL_A {
        match self.bits {
            0 => PAD43FNCSEL_A::TRIG6,
            1 => PAD43FNCSEL_A::M0NCE1,
            2 => PAD43FNCSEL_A::TCTB0,
            3 => PAD43FNCSEL_A::GPIO43,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRIG6`"]
    #[inline(always)]
    pub fn is_trig6(&self) -> bool {
        *self == PAD43FNCSEL_A::TRIG6
    }
    #[doc = "Checks if the value of the field is `M0NCE1`"]
    #[inline(always)]
    pub fn is_m0n_ce1(&self) -> bool {
        *self == PAD43FNCSEL_A::M0NCE1
    }
    #[doc = "Checks if the value of the field is `TCTB0`"]
    #[inline(always)]
    pub fn is_tctb0(&self) -> bool {
        *self == PAD43FNCSEL_A::TCTB0
    }
    #[doc = "Checks if the value of the field is `GPIO43`"]
    #[inline(always)]
    pub fn is_gpio43(&self) -> bool {
        *self == PAD43FNCSEL_A::GPIO43
    }
}
#[doc = "Write proxy for field `PAD43FNCSEL`"]
pub struct PAD43FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD43FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD43FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the ADC Trigger 6 signal"]
    #[inline(always)]
    pub fn trig6(self) -> &'a mut W {
        self.variant(PAD43FNCSEL_A::TRIG6)
    }
    #[doc = "Configure as the SPI channel 1 nCE signal from IOMSTR0"]
    #[inline(always)]
    pub fn m0n_ce1(self) -> &'a mut W {
        self.variant(PAD43FNCSEL_A::M0NCE1)
    }
    #[doc = "Configure as the input/output signal from CTIMER B0"]
    #[inline(always)]
    pub fn tctb0(self) -> &'a mut W {
        self.variant(PAD43FNCSEL_A::TCTB0)
    }
    #[doc = "Configure as GPIO43"]
    #[inline(always)]
    pub fn gpio43(self) -> &'a mut W {
        self.variant(PAD43FNCSEL_A::GPIO43)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Pad 43 drive strentgh\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD43STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD43STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD43STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD43STRNG`"]
pub type PAD43STRNG_R = crate::R<bool, PAD43STRNG_A>;
impl PAD43STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD43STRNG_A {
        match self.bits {
            false => PAD43STRNG_A::LOW,
            true => PAD43STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD43STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD43STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD43STRNG`"]
pub struct PAD43STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD43STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD43STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD43STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD43STRNG_A::HIGH)
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
#[doc = "Pad 43 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD43INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD43INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD43INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD43INPEN`"]
pub type PAD43INPEN_R = crate::R<bool, PAD43INPEN_A>;
impl PAD43INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD43INPEN_A {
        match self.bits {
            false => PAD43INPEN_A::DIS,
            true => PAD43INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD43INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD43INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD43INPEN`"]
pub struct PAD43INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD43INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD43INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD43INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD43INPEN_A::EN)
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
#[doc = "Pad 43 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD43PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD43PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD43PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD43PULL`"]
pub type PAD43PULL_R = crate::R<bool, PAD43PULL_A>;
impl PAD43PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD43PULL_A {
        match self.bits {
            false => PAD43PULL_A::DIS,
            true => PAD43PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD43PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD43PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD43PULL`"]
pub struct PAD43PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD43PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD43PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD43PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD43PULL_A::EN)
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
#[doc = "Pad 42 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD42FNCSEL_A {
    #[doc = "0: Configure as the ADC Trigger 5 signal"]
    TRIG5 = 0,
    #[doc = "1: Configure as the SPI channel 0 nCE signal from IOMSTR0"]
    M0NCE0 = 1,
    #[doc = "2: Configure as the input/output signal from CTIMER A0"]
    TCTA0 = 2,
    #[doc = "3: Configure as GPIO42"]
    GPIO42 = 3,
}
impl From<PAD42FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD42FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD42FNCSEL`"]
pub type PAD42FNCSEL_R = crate::R<u8, PAD42FNCSEL_A>;
impl PAD42FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD42FNCSEL_A {
        match self.bits {
            0 => PAD42FNCSEL_A::TRIG5,
            1 => PAD42FNCSEL_A::M0NCE0,
            2 => PAD42FNCSEL_A::TCTA0,
            3 => PAD42FNCSEL_A::GPIO42,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRIG5`"]
    #[inline(always)]
    pub fn is_trig5(&self) -> bool {
        *self == PAD42FNCSEL_A::TRIG5
    }
    #[doc = "Checks if the value of the field is `M0NCE0`"]
    #[inline(always)]
    pub fn is_m0n_ce0(&self) -> bool {
        *self == PAD42FNCSEL_A::M0NCE0
    }
    #[doc = "Checks if the value of the field is `TCTA0`"]
    #[inline(always)]
    pub fn is_tcta0(&self) -> bool {
        *self == PAD42FNCSEL_A::TCTA0
    }
    #[doc = "Checks if the value of the field is `GPIO42`"]
    #[inline(always)]
    pub fn is_gpio42(&self) -> bool {
        *self == PAD42FNCSEL_A::GPIO42
    }
}
#[doc = "Write proxy for field `PAD42FNCSEL`"]
pub struct PAD42FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD42FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD42FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the ADC Trigger 5 signal"]
    #[inline(always)]
    pub fn trig5(self) -> &'a mut W {
        self.variant(PAD42FNCSEL_A::TRIG5)
    }
    #[doc = "Configure as the SPI channel 0 nCE signal from IOMSTR0"]
    #[inline(always)]
    pub fn m0n_ce0(self) -> &'a mut W {
        self.variant(PAD42FNCSEL_A::M0NCE0)
    }
    #[doc = "Configure as the input/output signal from CTIMER A0"]
    #[inline(always)]
    pub fn tcta0(self) -> &'a mut W {
        self.variant(PAD42FNCSEL_A::TCTA0)
    }
    #[doc = "Configure as GPIO42"]
    #[inline(always)]
    pub fn gpio42(self) -> &'a mut W {
        self.variant(PAD42FNCSEL_A::GPIO42)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "Pad 42 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD42STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD42STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD42STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD42STRNG`"]
pub type PAD42STRNG_R = crate::R<bool, PAD42STRNG_A>;
impl PAD42STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD42STRNG_A {
        match self.bits {
            false => PAD42STRNG_A::LOW,
            true => PAD42STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD42STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD42STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD42STRNG`"]
pub struct PAD42STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD42STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD42STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD42STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD42STRNG_A::HIGH)
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
#[doc = "Pad 42 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD42INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD42INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD42INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD42INPEN`"]
pub type PAD42INPEN_R = crate::R<bool, PAD42INPEN_A>;
impl PAD42INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD42INPEN_A {
        match self.bits {
            false => PAD42INPEN_A::DIS,
            true => PAD42INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD42INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD42INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD42INPEN`"]
pub struct PAD42INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD42INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD42INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD42INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD42INPEN_A::EN)
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
#[doc = "Pad 42 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD42PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD42PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD42PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD42PULL`"]
pub type PAD42PULL_R = crate::R<bool, PAD42PULL_A>;
impl PAD42PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD42PULL_A {
        match self.bits {
            false => PAD42PULL_A::DIS,
            true => PAD42PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD42PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD42PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD42PULL`"]
pub struct PAD42PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD42PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD42PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD42PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD42PULL_A::EN)
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
#[doc = "Pad 41 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD41FNCSEL_A {
    #[doc = "0: Configure as the ADC Trigger 4 signal"]
    TRIG4 = 0,
    #[doc = "1: Pad disabled"]
    DIS = 1,
    #[doc = "2: Configure as the serial wire debug SWO signal"]
    SWO = 2,
    #[doc = "3: Configure as GPIO41"]
    GPIO41 = 3,
}
impl From<PAD41FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD41FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD41FNCSEL`"]
pub type PAD41FNCSEL_R = crate::R<u8, PAD41FNCSEL_A>;
impl PAD41FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD41FNCSEL_A {
        match self.bits {
            0 => PAD41FNCSEL_A::TRIG4,
            1 => PAD41FNCSEL_A::DIS,
            2 => PAD41FNCSEL_A::SWO,
            3 => PAD41FNCSEL_A::GPIO41,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRIG4`"]
    #[inline(always)]
    pub fn is_trig4(&self) -> bool {
        *self == PAD41FNCSEL_A::TRIG4
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD41FNCSEL_A::DIS
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline(always)]
    pub fn is_swo(&self) -> bool {
        *self == PAD41FNCSEL_A::SWO
    }
    #[doc = "Checks if the value of the field is `GPIO41`"]
    #[inline(always)]
    pub fn is_gpio41(&self) -> bool {
        *self == PAD41FNCSEL_A::GPIO41
    }
}
#[doc = "Write proxy for field `PAD41FNCSEL`"]
pub struct PAD41FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD41FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD41FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the ADC Trigger 4 signal"]
    #[inline(always)]
    pub fn trig4(self) -> &'a mut W {
        self.variant(PAD41FNCSEL_A::TRIG4)
    }
    #[doc = "Pad disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD41FNCSEL_A::DIS)
    }
    #[doc = "Configure as the serial wire debug SWO signal"]
    #[inline(always)]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD41FNCSEL_A::SWO)
    }
    #[doc = "Configure as GPIO41"]
    #[inline(always)]
    pub fn gpio41(self) -> &'a mut W {
        self.variant(PAD41FNCSEL_A::GPIO41)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Pad 41 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD41STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD41STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD41STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD41STRNG`"]
pub type PAD41STRNG_R = crate::R<bool, PAD41STRNG_A>;
impl PAD41STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD41STRNG_A {
        match self.bits {
            false => PAD41STRNG_A::LOW,
            true => PAD41STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD41STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD41STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD41STRNG`"]
pub struct PAD41STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD41STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD41STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD41STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD41STRNG_A::HIGH)
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
#[doc = "Pad 41 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD41INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD41INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD41INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD41INPEN`"]
pub type PAD41INPEN_R = crate::R<bool, PAD41INPEN_A>;
impl PAD41INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD41INPEN_A {
        match self.bits {
            false => PAD41INPEN_A::DIS,
            true => PAD41INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD41INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD41INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD41INPEN`"]
pub struct PAD41INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD41INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD41INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD41INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD41INPEN_A::EN)
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
#[doc = "Pad 41 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD41PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD41PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD41PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD41PULL`"]
pub type PAD41PULL_R = crate::R<bool, PAD41PULL_A>;
impl PAD41PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD41PULL_A {
        match self.bits {
            false => PAD41PULL_A::DIS,
            true => PAD41PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD41PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD41PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD41PULL`"]
pub struct PAD41PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD41PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD41PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD41PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD41PULL_A::EN)
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
#[doc = "Pad 40 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD40FNCSEL_A {
    #[doc = "0: Configure as the ADC Trigger 3 signal"]
    TRIG3 = 0,
    #[doc = "1: Configure as the UART RX signal"]
    UARTRX = 1,
    #[doc = "2: Pad disabled"]
    DIS = 2,
    #[doc = "3: Configure as GPIO40"]
    GPIO40 = 3,
}
impl From<PAD40FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD40FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD40FNCSEL`"]
pub type PAD40FNCSEL_R = crate::R<u8, PAD40FNCSEL_A>;
impl PAD40FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD40FNCSEL_A {
        match self.bits {
            0 => PAD40FNCSEL_A::TRIG3,
            1 => PAD40FNCSEL_A::UARTRX,
            2 => PAD40FNCSEL_A::DIS,
            3 => PAD40FNCSEL_A::GPIO40,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRIG3`"]
    #[inline(always)]
    pub fn is_trig3(&self) -> bool {
        *self == PAD40FNCSEL_A::TRIG3
    }
    #[doc = "Checks if the value of the field is `UARTRX`"]
    #[inline(always)]
    pub fn is_uartrx(&self) -> bool {
        *self == PAD40FNCSEL_A::UARTRX
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD40FNCSEL_A::DIS
    }
    #[doc = "Checks if the value of the field is `GPIO40`"]
    #[inline(always)]
    pub fn is_gpio40(&self) -> bool {
        *self == PAD40FNCSEL_A::GPIO40
    }
}
#[doc = "Write proxy for field `PAD40FNCSEL`"]
pub struct PAD40FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD40FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD40FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the ADC Trigger 3 signal"]
    #[inline(always)]
    pub fn trig3(self) -> &'a mut W {
        self.variant(PAD40FNCSEL_A::TRIG3)
    }
    #[doc = "Configure as the UART RX signal"]
    #[inline(always)]
    pub fn uartrx(self) -> &'a mut W {
        self.variant(PAD40FNCSEL_A::UARTRX)
    }
    #[doc = "Pad disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD40FNCSEL_A::DIS)
    }
    #[doc = "Configure as GPIO40"]
    #[inline(always)]
    pub fn gpio40(self) -> &'a mut W {
        self.variant(PAD40FNCSEL_A::GPIO40)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Pad 40 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD40STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD40STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD40STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD40STRNG`"]
pub type PAD40STRNG_R = crate::R<bool, PAD40STRNG_A>;
impl PAD40STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD40STRNG_A {
        match self.bits {
            false => PAD40STRNG_A::LOW,
            true => PAD40STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD40STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD40STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD40STRNG`"]
pub struct PAD40STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD40STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD40STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD40STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD40STRNG_A::HIGH)
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
#[doc = "Pad 40 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD40INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD40INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD40INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD40INPEN`"]
pub type PAD40INPEN_R = crate::R<bool, PAD40INPEN_A>;
impl PAD40INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD40INPEN_A {
        match self.bits {
            false => PAD40INPEN_A::DIS,
            true => PAD40INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD40INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD40INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD40INPEN`"]
pub struct PAD40INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD40INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD40INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD40INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD40INPEN_A::EN)
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
#[doc = "Pad 40 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD40PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD40PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD40PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD40PULL`"]
pub type PAD40PULL_R = crate::R<bool, PAD40PULL_A>;
impl PAD40PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD40PULL_A {
        match self.bits {
            false => PAD40PULL_A::DIS,
            true => PAD40PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD40PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD40PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD40PULL`"]
pub struct PAD40PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD40PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD40PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD40PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD40PULL_A::EN)
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
    #[doc = "Bits 27:28 - Pad 43 function select"]
    #[inline(always)]
    pub fn pad43fncsel(&self) -> PAD43FNCSEL_R {
        PAD43FNCSEL_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Pad 43 drive strentgh"]
    #[inline(always)]
    pub fn pad43strng(&self) -> PAD43STRNG_R {
        PAD43STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 43 input enable"]
    #[inline(always)]
    pub fn pad43inpen(&self) -> PAD43INPEN_R {
        PAD43INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 43 pullup enable"]
    #[inline(always)]
    pub fn pad43pull(&self) -> PAD43PULL_R {
        PAD43PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:20 - Pad 42 function select"]
    #[inline(always)]
    pub fn pad42fncsel(&self) -> PAD42FNCSEL_R {
        PAD42FNCSEL_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Pad 42 drive strength"]
    #[inline(always)]
    pub fn pad42strng(&self) -> PAD42STRNG_R {
        PAD42STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 42 input enable"]
    #[inline(always)]
    pub fn pad42inpen(&self) -> PAD42INPEN_R {
        PAD42INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 42 pullup enable"]
    #[inline(always)]
    pub fn pad42pull(&self) -> PAD42PULL_R {
        PAD42PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - Pad 41 function select"]
    #[inline(always)]
    pub fn pad41fncsel(&self) -> PAD41FNCSEL_R {
        PAD41FNCSEL_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Pad 41 drive strength"]
    #[inline(always)]
    pub fn pad41strng(&self) -> PAD41STRNG_R {
        PAD41STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 41 input enable"]
    #[inline(always)]
    pub fn pad41inpen(&self) -> PAD41INPEN_R {
        PAD41INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 41 pullup enable"]
    #[inline(always)]
    pub fn pad41pull(&self) -> PAD41PULL_R {
        PAD41PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Pad 40 function select"]
    #[inline(always)]
    pub fn pad40fncsel(&self) -> PAD40FNCSEL_R {
        PAD40FNCSEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2 - Pad 40 drive strength"]
    #[inline(always)]
    pub fn pad40strng(&self) -> PAD40STRNG_R {
        PAD40STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 40 input enable"]
    #[inline(always)]
    pub fn pad40inpen(&self) -> PAD40INPEN_R {
        PAD40INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 40 pullup enable"]
    #[inline(always)]
    pub fn pad40pull(&self) -> PAD40PULL_R {
        PAD40PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:28 - Pad 43 function select"]
    #[inline(always)]
    pub fn pad43fncsel(&mut self) -> PAD43FNCSEL_W {
        PAD43FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 43 drive strentgh"]
    #[inline(always)]
    pub fn pad43strng(&mut self) -> PAD43STRNG_W {
        PAD43STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 43 input enable"]
    #[inline(always)]
    pub fn pad43inpen(&mut self) -> PAD43INPEN_W {
        PAD43INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 43 pullup enable"]
    #[inline(always)]
    pub fn pad43pull(&mut self) -> PAD43PULL_W {
        PAD43PULL_W { w: self }
    }
    #[doc = "Bits 19:20 - Pad 42 function select"]
    #[inline(always)]
    pub fn pad42fncsel(&mut self) -> PAD42FNCSEL_W {
        PAD42FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 42 drive strength"]
    #[inline(always)]
    pub fn pad42strng(&mut self) -> PAD42STRNG_W {
        PAD42STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 42 input enable"]
    #[inline(always)]
    pub fn pad42inpen(&mut self) -> PAD42INPEN_W {
        PAD42INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 42 pullup enable"]
    #[inline(always)]
    pub fn pad42pull(&mut self) -> PAD42PULL_W {
        PAD42PULL_W { w: self }
    }
    #[doc = "Bits 11:12 - Pad 41 function select"]
    #[inline(always)]
    pub fn pad41fncsel(&mut self) -> PAD41FNCSEL_W {
        PAD41FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 41 drive strength"]
    #[inline(always)]
    pub fn pad41strng(&mut self) -> PAD41STRNG_W {
        PAD41STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 41 input enable"]
    #[inline(always)]
    pub fn pad41inpen(&mut self) -> PAD41INPEN_W {
        PAD41INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 41 pullup enable"]
    #[inline(always)]
    pub fn pad41pull(&mut self) -> PAD41PULL_W {
        PAD41PULL_W { w: self }
    }
    #[doc = "Bits 3:4 - Pad 40 function select"]
    #[inline(always)]
    pub fn pad40fncsel(&mut self) -> PAD40FNCSEL_W {
        PAD40FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 40 drive strength"]
    #[inline(always)]
    pub fn pad40strng(&mut self) -> PAD40STRNG_W {
        PAD40STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 40 input enable"]
    #[inline(always)]
    pub fn pad40inpen(&mut self) -> PAD40INPEN_W {
        PAD40INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 40 pullup enable"]
    #[inline(always)]
    pub fn pad40pull(&mut self) -> PAD40PULL_W {
        PAD40PULL_W { w: self }
    }
}
