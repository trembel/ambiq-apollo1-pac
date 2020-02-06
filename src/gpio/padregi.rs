#[doc = "Reader of register PADREGI"]
pub type R = crate::R<u32, super::PADREGI>;
#[doc = "Writer for register PADREGI"]
pub type W = crate::W<u32, super::PADREGI>;
#[doc = "Register PADREGI `reset()`'s with value 0x1818_1818"]
impl crate::ResetValue for super::PADREGI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1818_1818
    }
}
#[doc = "Pad 35 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD35FNCSEL_A {
    #[doc = "0: Pad disabled"]
    DIS = 0,
    #[doc = "1: Configure as the SPI channel 0 nCE signal from IOMSTR1"]
    M1NCE0 = 1,
    #[doc = "2: Configure as the UART TX signal"]
    UARTTX = 2,
    #[doc = "3: Configure as GPIO35"]
    GPIO35 = 3,
}
impl From<PAD35FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD35FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD35FNCSEL`"]
pub type PAD35FNCSEL_R = crate::R<u8, PAD35FNCSEL_A>;
impl PAD35FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD35FNCSEL_A {
        match self.bits {
            0 => PAD35FNCSEL_A::DIS,
            1 => PAD35FNCSEL_A::M1NCE0,
            2 => PAD35FNCSEL_A::UARTTX,
            3 => PAD35FNCSEL_A::GPIO35,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD35FNCSEL_A::DIS
    }
    #[doc = "Checks if the value of the field is `M1NCE0`"]
    #[inline(always)]
    pub fn is_m1n_ce0(&self) -> bool {
        *self == PAD35FNCSEL_A::M1NCE0
    }
    #[doc = "Checks if the value of the field is `UARTTX`"]
    #[inline(always)]
    pub fn is_uarttx(&self) -> bool {
        *self == PAD35FNCSEL_A::UARTTX
    }
    #[doc = "Checks if the value of the field is `GPIO35`"]
    #[inline(always)]
    pub fn is_gpio35(&self) -> bool {
        *self == PAD35FNCSEL_A::GPIO35
    }
}
#[doc = "Write proxy for field `PAD35FNCSEL`"]
pub struct PAD35FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD35FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD35FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pad disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD35FNCSEL_A::DIS)
    }
    #[doc = "Configure as the SPI channel 0 nCE signal from IOMSTR1"]
    #[inline(always)]
    pub fn m1n_ce0(self) -> &'a mut W {
        self.variant(PAD35FNCSEL_A::M1NCE0)
    }
    #[doc = "Configure as the UART TX signal"]
    #[inline(always)]
    pub fn uarttx(self) -> &'a mut W {
        self.variant(PAD35FNCSEL_A::UARTTX)
    }
    #[doc = "Configure as GPIO35"]
    #[inline(always)]
    pub fn gpio35(self) -> &'a mut W {
        self.variant(PAD35FNCSEL_A::GPIO35)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Pad 35 drive strentgh\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD35STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD35STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD35STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD35STRNG`"]
pub type PAD35STRNG_R = crate::R<bool, PAD35STRNG_A>;
impl PAD35STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD35STRNG_A {
        match self.bits {
            false => PAD35STRNG_A::LOW,
            true => PAD35STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD35STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD35STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD35STRNG`"]
pub struct PAD35STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD35STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD35STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD35STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD35STRNG_A::HIGH)
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
#[doc = "Pad 35 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD35INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD35INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD35INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD35INPEN`"]
pub type PAD35INPEN_R = crate::R<bool, PAD35INPEN_A>;
impl PAD35INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD35INPEN_A {
        match self.bits {
            false => PAD35INPEN_A::DIS,
            true => PAD35INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD35INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD35INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD35INPEN`"]
pub struct PAD35INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD35INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD35INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD35INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD35INPEN_A::EN)
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
#[doc = "Pad 35 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD35PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD35PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD35PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD35PULL`"]
pub type PAD35PULL_R = crate::R<bool, PAD35PULL_A>;
impl PAD35PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD35PULL_A {
        match self.bits {
            false => PAD35PULL_A::DIS,
            true => PAD35PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD35PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD35PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD35PULL`"]
pub struct PAD35PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD35PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD35PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD35PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD35PULL_A::EN)
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
#[doc = "Pad 34 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD34FNCSEL_A {
    #[doc = "0: Configure as the analog comparator reference 2 signal"]
    CMPRF2 = 0,
    #[doc = "1: Configure as the SPI channel 7 nCE signal from IOMSTR0"]
    M0NCE7 = 1,
    #[doc = "2: Pad disabled"]
    DIS = 2,
    #[doc = "3: Configure as GPIO34"]
    GPIO34 = 3,
}
impl From<PAD34FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD34FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD34FNCSEL`"]
pub type PAD34FNCSEL_R = crate::R<u8, PAD34FNCSEL_A>;
impl PAD34FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD34FNCSEL_A {
        match self.bits {
            0 => PAD34FNCSEL_A::CMPRF2,
            1 => PAD34FNCSEL_A::M0NCE7,
            2 => PAD34FNCSEL_A::DIS,
            3 => PAD34FNCSEL_A::GPIO34,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CMPRF2`"]
    #[inline(always)]
    pub fn is_cmprf2(&self) -> bool {
        *self == PAD34FNCSEL_A::CMPRF2
    }
    #[doc = "Checks if the value of the field is `M0NCE7`"]
    #[inline(always)]
    pub fn is_m0n_ce7(&self) -> bool {
        *self == PAD34FNCSEL_A::M0NCE7
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD34FNCSEL_A::DIS
    }
    #[doc = "Checks if the value of the field is `GPIO34`"]
    #[inline(always)]
    pub fn is_gpio34(&self) -> bool {
        *self == PAD34FNCSEL_A::GPIO34
    }
}
#[doc = "Write proxy for field `PAD34FNCSEL`"]
pub struct PAD34FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD34FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD34FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the analog comparator reference 2 signal"]
    #[inline(always)]
    pub fn cmprf2(self) -> &'a mut W {
        self.variant(PAD34FNCSEL_A::CMPRF2)
    }
    #[doc = "Configure as the SPI channel 7 nCE signal from IOMSTR0"]
    #[inline(always)]
    pub fn m0n_ce7(self) -> &'a mut W {
        self.variant(PAD34FNCSEL_A::M0NCE7)
    }
    #[doc = "Pad disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD34FNCSEL_A::DIS)
    }
    #[doc = "Configure as GPIO34"]
    #[inline(always)]
    pub fn gpio34(self) -> &'a mut W {
        self.variant(PAD34FNCSEL_A::GPIO34)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "Pad 34 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD34STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD34STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD34STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD34STRNG`"]
pub type PAD34STRNG_R = crate::R<bool, PAD34STRNG_A>;
impl PAD34STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD34STRNG_A {
        match self.bits {
            false => PAD34STRNG_A::LOW,
            true => PAD34STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD34STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD34STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD34STRNG`"]
pub struct PAD34STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD34STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD34STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD34STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD34STRNG_A::HIGH)
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
#[doc = "Pad 34 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD34INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD34INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD34INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD34INPEN`"]
pub type PAD34INPEN_R = crate::R<bool, PAD34INPEN_A>;
impl PAD34INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD34INPEN_A {
        match self.bits {
            false => PAD34INPEN_A::DIS,
            true => PAD34INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD34INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD34INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD34INPEN`"]
pub struct PAD34INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD34INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD34INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD34INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD34INPEN_A::EN)
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
#[doc = "Pad 34 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD34PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD34PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD34PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD34PULL`"]
pub type PAD34PULL_R = crate::R<bool, PAD34PULL_A>;
impl PAD34PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD34PULL_A {
        match self.bits {
            false => PAD34PULL_A::DIS,
            true => PAD34PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD34PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD34PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD34PULL`"]
pub struct PAD34PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD34PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD34PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD34PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD34PULL_A::EN)
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
#[doc = "Pad 33 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD33FNCSEL_A {
    #[doc = "0: Configure as the analog comparator reference 1 signal"]
    CMPRF1 = 0,
    #[doc = "1: Configure as the SPI channel 6 nCE signal from IOMSTR0"]
    M0NCE6 = 1,
    #[doc = "2: Pad disabled"]
    DIS = 2,
    #[doc = "3: Configure as GPIO33"]
    GPIO33 = 3,
}
impl From<PAD33FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD33FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD33FNCSEL`"]
pub type PAD33FNCSEL_R = crate::R<u8, PAD33FNCSEL_A>;
impl PAD33FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD33FNCSEL_A {
        match self.bits {
            0 => PAD33FNCSEL_A::CMPRF1,
            1 => PAD33FNCSEL_A::M0NCE6,
            2 => PAD33FNCSEL_A::DIS,
            3 => PAD33FNCSEL_A::GPIO33,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CMPRF1`"]
    #[inline(always)]
    pub fn is_cmprf1(&self) -> bool {
        *self == PAD33FNCSEL_A::CMPRF1
    }
    #[doc = "Checks if the value of the field is `M0NCE6`"]
    #[inline(always)]
    pub fn is_m0n_ce6(&self) -> bool {
        *self == PAD33FNCSEL_A::M0NCE6
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD33FNCSEL_A::DIS
    }
    #[doc = "Checks if the value of the field is `GPIO33`"]
    #[inline(always)]
    pub fn is_gpio33(&self) -> bool {
        *self == PAD33FNCSEL_A::GPIO33
    }
}
#[doc = "Write proxy for field `PAD33FNCSEL`"]
pub struct PAD33FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD33FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD33FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the analog comparator reference 1 signal"]
    #[inline(always)]
    pub fn cmprf1(self) -> &'a mut W {
        self.variant(PAD33FNCSEL_A::CMPRF1)
    }
    #[doc = "Configure as the SPI channel 6 nCE signal from IOMSTR0"]
    #[inline(always)]
    pub fn m0n_ce6(self) -> &'a mut W {
        self.variant(PAD33FNCSEL_A::M0NCE6)
    }
    #[doc = "Pad disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD33FNCSEL_A::DIS)
    }
    #[doc = "Configure as GPIO33"]
    #[inline(always)]
    pub fn gpio33(self) -> &'a mut W {
        self.variant(PAD33FNCSEL_A::GPIO33)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Pad 33 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD33STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD33STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD33STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD33STRNG`"]
pub type PAD33STRNG_R = crate::R<bool, PAD33STRNG_A>;
impl PAD33STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD33STRNG_A {
        match self.bits {
            false => PAD33STRNG_A::LOW,
            true => PAD33STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD33STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD33STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD33STRNG`"]
pub struct PAD33STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD33STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD33STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD33STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD33STRNG_A::HIGH)
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
#[doc = "Pad 33 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD33INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD33INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD33INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD33INPEN`"]
pub type PAD33INPEN_R = crate::R<bool, PAD33INPEN_A>;
impl PAD33INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD33INPEN_A {
        match self.bits {
            false => PAD33INPEN_A::DIS,
            true => PAD33INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD33INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD33INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD33INPEN`"]
pub struct PAD33INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD33INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD33INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD33INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD33INPEN_A::EN)
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
#[doc = "Pad 33 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD33PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD33PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD33PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD33PULL`"]
pub type PAD33PULL_R = crate::R<bool, PAD33PULL_A>;
impl PAD33PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD33PULL_A {
        match self.bits {
            false => PAD33PULL_A::DIS,
            true => PAD33PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD33PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD33PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD33PULL`"]
pub struct PAD33PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD33PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD33PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD33PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD33PULL_A::EN)
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
#[doc = "Pad 32 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD32FNCSEL_A {
    #[doc = "0: Configure as the analog ADC input 7 signal"]
    ADC7 = 0,
    #[doc = "1: Configure as the SPI channel 5 nCE signal from IOMSTR0"]
    M0NCE5 = 1,
    #[doc = "2: Configure as the input/output signal from CTIMER B3"]
    TCTB3 = 2,
    #[doc = "3: Configure as GPIO32"]
    GPIO32 = 3,
}
impl From<PAD32FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD32FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD32FNCSEL`"]
pub type PAD32FNCSEL_R = crate::R<u8, PAD32FNCSEL_A>;
impl PAD32FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD32FNCSEL_A {
        match self.bits {
            0 => PAD32FNCSEL_A::ADC7,
            1 => PAD32FNCSEL_A::M0NCE5,
            2 => PAD32FNCSEL_A::TCTB3,
            3 => PAD32FNCSEL_A::GPIO32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC7`"]
    #[inline(always)]
    pub fn is_adc7(&self) -> bool {
        *self == PAD32FNCSEL_A::ADC7
    }
    #[doc = "Checks if the value of the field is `M0NCE5`"]
    #[inline(always)]
    pub fn is_m0n_ce5(&self) -> bool {
        *self == PAD32FNCSEL_A::M0NCE5
    }
    #[doc = "Checks if the value of the field is `TCTB3`"]
    #[inline(always)]
    pub fn is_tctb3(&self) -> bool {
        *self == PAD32FNCSEL_A::TCTB3
    }
    #[doc = "Checks if the value of the field is `GPIO32`"]
    #[inline(always)]
    pub fn is_gpio32(&self) -> bool {
        *self == PAD32FNCSEL_A::GPIO32
    }
}
#[doc = "Write proxy for field `PAD32FNCSEL`"]
pub struct PAD32FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD32FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD32FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the analog ADC input 7 signal"]
    #[inline(always)]
    pub fn adc7(self) -> &'a mut W {
        self.variant(PAD32FNCSEL_A::ADC7)
    }
    #[doc = "Configure as the SPI channel 5 nCE signal from IOMSTR0"]
    #[inline(always)]
    pub fn m0n_ce5(self) -> &'a mut W {
        self.variant(PAD32FNCSEL_A::M0NCE5)
    }
    #[doc = "Configure as the input/output signal from CTIMER B3"]
    #[inline(always)]
    pub fn tctb3(self) -> &'a mut W {
        self.variant(PAD32FNCSEL_A::TCTB3)
    }
    #[doc = "Configure as GPIO32"]
    #[inline(always)]
    pub fn gpio32(self) -> &'a mut W {
        self.variant(PAD32FNCSEL_A::GPIO32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Pad 32 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD32STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD32STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD32STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD32STRNG`"]
pub type PAD32STRNG_R = crate::R<bool, PAD32STRNG_A>;
impl PAD32STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD32STRNG_A {
        match self.bits {
            false => PAD32STRNG_A::LOW,
            true => PAD32STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD32STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD32STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD32STRNG`"]
pub struct PAD32STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD32STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD32STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD32STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD32STRNG_A::HIGH)
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
#[doc = "Pad 32 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD32INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD32INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD32INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD32INPEN`"]
pub type PAD32INPEN_R = crate::R<bool, PAD32INPEN_A>;
impl PAD32INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD32INPEN_A {
        match self.bits {
            false => PAD32INPEN_A::DIS,
            true => PAD32INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD32INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD32INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD32INPEN`"]
pub struct PAD32INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD32INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD32INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD32INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD32INPEN_A::EN)
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
#[doc = "Pad 32 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD32PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD32PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD32PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD32PULL`"]
pub type PAD32PULL_R = crate::R<bool, PAD32PULL_A>;
impl PAD32PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD32PULL_A {
        match self.bits {
            false => PAD32PULL_A::DIS,
            true => PAD32PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD32PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD32PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD32PULL`"]
pub struct PAD32PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD32PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD32PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD32PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD32PULL_A::EN)
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
    #[doc = "Bits 27:28 - Pad 35 function select"]
    #[inline(always)]
    pub fn pad35fncsel(&self) -> PAD35FNCSEL_R {
        PAD35FNCSEL_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Pad 35 drive strentgh"]
    #[inline(always)]
    pub fn pad35strng(&self) -> PAD35STRNG_R {
        PAD35STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 35 input enable"]
    #[inline(always)]
    pub fn pad35inpen(&self) -> PAD35INPEN_R {
        PAD35INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 35 pullup enable"]
    #[inline(always)]
    pub fn pad35pull(&self) -> PAD35PULL_R {
        PAD35PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:20 - Pad 34 function select"]
    #[inline(always)]
    pub fn pad34fncsel(&self) -> PAD34FNCSEL_R {
        PAD34FNCSEL_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Pad 34 drive strength"]
    #[inline(always)]
    pub fn pad34strng(&self) -> PAD34STRNG_R {
        PAD34STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 34 input enable"]
    #[inline(always)]
    pub fn pad34inpen(&self) -> PAD34INPEN_R {
        PAD34INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 34 pullup enable"]
    #[inline(always)]
    pub fn pad34pull(&self) -> PAD34PULL_R {
        PAD34PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - Pad 33 function select"]
    #[inline(always)]
    pub fn pad33fncsel(&self) -> PAD33FNCSEL_R {
        PAD33FNCSEL_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Pad 33 drive strength"]
    #[inline(always)]
    pub fn pad33strng(&self) -> PAD33STRNG_R {
        PAD33STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 33 input enable"]
    #[inline(always)]
    pub fn pad33inpen(&self) -> PAD33INPEN_R {
        PAD33INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 33 pullup enable"]
    #[inline(always)]
    pub fn pad33pull(&self) -> PAD33PULL_R {
        PAD33PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Pad 32 function select"]
    #[inline(always)]
    pub fn pad32fncsel(&self) -> PAD32FNCSEL_R {
        PAD32FNCSEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2 - Pad 32 drive strength"]
    #[inline(always)]
    pub fn pad32strng(&self) -> PAD32STRNG_R {
        PAD32STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 32 input enable"]
    #[inline(always)]
    pub fn pad32inpen(&self) -> PAD32INPEN_R {
        PAD32INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 32 pullup enable"]
    #[inline(always)]
    pub fn pad32pull(&self) -> PAD32PULL_R {
        PAD32PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:28 - Pad 35 function select"]
    #[inline(always)]
    pub fn pad35fncsel(&mut self) -> PAD35FNCSEL_W {
        PAD35FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 35 drive strentgh"]
    #[inline(always)]
    pub fn pad35strng(&mut self) -> PAD35STRNG_W {
        PAD35STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 35 input enable"]
    #[inline(always)]
    pub fn pad35inpen(&mut self) -> PAD35INPEN_W {
        PAD35INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 35 pullup enable"]
    #[inline(always)]
    pub fn pad35pull(&mut self) -> PAD35PULL_W {
        PAD35PULL_W { w: self }
    }
    #[doc = "Bits 19:20 - Pad 34 function select"]
    #[inline(always)]
    pub fn pad34fncsel(&mut self) -> PAD34FNCSEL_W {
        PAD34FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 34 drive strength"]
    #[inline(always)]
    pub fn pad34strng(&mut self) -> PAD34STRNG_W {
        PAD34STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 34 input enable"]
    #[inline(always)]
    pub fn pad34inpen(&mut self) -> PAD34INPEN_W {
        PAD34INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 34 pullup enable"]
    #[inline(always)]
    pub fn pad34pull(&mut self) -> PAD34PULL_W {
        PAD34PULL_W { w: self }
    }
    #[doc = "Bits 11:12 - Pad 33 function select"]
    #[inline(always)]
    pub fn pad33fncsel(&mut self) -> PAD33FNCSEL_W {
        PAD33FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 33 drive strength"]
    #[inline(always)]
    pub fn pad33strng(&mut self) -> PAD33STRNG_W {
        PAD33STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 33 input enable"]
    #[inline(always)]
    pub fn pad33inpen(&mut self) -> PAD33INPEN_W {
        PAD33INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 33 pullup enable"]
    #[inline(always)]
    pub fn pad33pull(&mut self) -> PAD33PULL_W {
        PAD33PULL_W { w: self }
    }
    #[doc = "Bits 3:4 - Pad 32 function select"]
    #[inline(always)]
    pub fn pad32fncsel(&mut self) -> PAD32FNCSEL_W {
        PAD32FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 32 drive strength"]
    #[inline(always)]
    pub fn pad32strng(&mut self) -> PAD32STRNG_W {
        PAD32STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 32 input enable"]
    #[inline(always)]
    pub fn pad32inpen(&mut self) -> PAD32INPEN_W {
        PAD32INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 32 pullup enable"]
    #[inline(always)]
    pub fn pad32pull(&mut self) -> PAD32PULL_W {
        PAD32PULL_W { w: self }
    }
}
