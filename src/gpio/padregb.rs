#[doc = "Reader of register PADREGB"]
pub type R = crate::R<u32, super::PADREGB>;
#[doc = "Writer for register PADREGB"]
pub type W = crate::W<u32, super::PADREGB>;
#[doc = "Register PADREGB `reset()`'s with value 0x1818_1818"]
impl crate::ResetValue for super::PADREGB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1818_1818
    }
}
#[doc = "Pad 7 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD7FNCSEL_A {
    #[doc = "0: Configure as the IOMSTR0 SPI 3-wire MOSI/MISO signal"]
    M0WIR3 = 0,
    #[doc = "1: Configure as the IOMSTR0 SPI MOSI signal"]
    M0MOSI = 1,
    #[doc = "2: Configure as the CLKOUT signal"]
    CLKOUT = 2,
    #[doc = "3: Configure as GPIO7"]
    GPIO7 = 3,
    #[doc = "6: Configure as the IOMSTR0 SPI 3-wire MOSI/MISO loopback signal from IOSLAVE"]
    SLWIR3 = 6,
    #[doc = "7: Pad disabled"]
    DIS = 7,
}
impl From<PAD7FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD7FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD7FNCSEL`"]
pub type PAD7FNCSEL_R = crate::R<u8, PAD7FNCSEL_A>;
impl PAD7FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD7FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD7FNCSEL_A::M0WIR3),
            1 => Val(PAD7FNCSEL_A::M0MOSI),
            2 => Val(PAD7FNCSEL_A::CLKOUT),
            3 => Val(PAD7FNCSEL_A::GPIO7),
            6 => Val(PAD7FNCSEL_A::SLWIR3),
            7 => Val(PAD7FNCSEL_A::DIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `M0WIR3`"]
    #[inline(always)]
    pub fn is_m0wir3(&self) -> bool {
        *self == PAD7FNCSEL_A::M0WIR3
    }
    #[doc = "Checks if the value of the field is `M0MOSI`"]
    #[inline(always)]
    pub fn is_m0mosi(&self) -> bool {
        *self == PAD7FNCSEL_A::M0MOSI
    }
    #[doc = "Checks if the value of the field is `CLKOUT`"]
    #[inline(always)]
    pub fn is_clkout(&self) -> bool {
        *self == PAD7FNCSEL_A::CLKOUT
    }
    #[doc = "Checks if the value of the field is `GPIO7`"]
    #[inline(always)]
    pub fn is_gpio7(&self) -> bool {
        *self == PAD7FNCSEL_A::GPIO7
    }
    #[doc = "Checks if the value of the field is `SLWIR3`"]
    #[inline(always)]
    pub fn is_slwir3(&self) -> bool {
        *self == PAD7FNCSEL_A::SLWIR3
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD7FNCSEL_A::DIS
    }
}
#[doc = "Write proxy for field `PAD7FNCSEL`"]
pub struct PAD7FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD7FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD7FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the IOMSTR0 SPI 3-wire MOSI/MISO signal"]
    #[inline(always)]
    pub fn m0wir3(self) -> &'a mut W {
        self.variant(PAD7FNCSEL_A::M0WIR3)
    }
    #[doc = "Configure as the IOMSTR0 SPI MOSI signal"]
    #[inline(always)]
    pub fn m0mosi(self) -> &'a mut W {
        self.variant(PAD7FNCSEL_A::M0MOSI)
    }
    #[doc = "Configure as the CLKOUT signal"]
    #[inline(always)]
    pub fn clkout(self) -> &'a mut W {
        self.variant(PAD7FNCSEL_A::CLKOUT)
    }
    #[doc = "Configure as GPIO7"]
    #[inline(always)]
    pub fn gpio7(self) -> &'a mut W {
        self.variant(PAD7FNCSEL_A::GPIO7)
    }
    #[doc = "Configure as the IOMSTR0 SPI 3-wire MOSI/MISO loopback signal from IOSLAVE"]
    #[inline(always)]
    pub fn slwir3(self) -> &'a mut W {
        self.variant(PAD7FNCSEL_A::SLWIR3)
    }
    #[doc = "Pad disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD7FNCSEL_A::DIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
#[doc = "Pad 7 drive strentgh\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD7STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD7STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD7STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD7STRNG`"]
pub type PAD7STRNG_R = crate::R<bool, PAD7STRNG_A>;
impl PAD7STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD7STRNG_A {
        match self.bits {
            false => PAD7STRNG_A::LOW,
            true => PAD7STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD7STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD7STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD7STRNG`"]
pub struct PAD7STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD7STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD7STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD7STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD7STRNG_A::HIGH)
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
#[doc = "Pad 7 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD7INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD7INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD7INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD7INPEN`"]
pub type PAD7INPEN_R = crate::R<bool, PAD7INPEN_A>;
impl PAD7INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD7INPEN_A {
        match self.bits {
            false => PAD7INPEN_A::DIS,
            true => PAD7INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD7INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD7INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD7INPEN`"]
pub struct PAD7INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD7INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD7INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD7INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD7INPEN_A::EN)
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
#[doc = "Pad 7 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD7PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD7PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD7PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD7PULL`"]
pub type PAD7PULL_R = crate::R<bool, PAD7PULL_A>;
impl PAD7PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD7PULL_A {
        match self.bits {
            false => PAD7PULL_A::DIS,
            true => PAD7PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD7PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD7PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD7PULL`"]
pub struct PAD7PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD7PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD7PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD7PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD7PULL_A::EN)
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
#[doc = "Pad 6 pullup resistor selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD6RSEL_A {
    #[doc = "0: Pullup is ~1.5 KOhms"]
    PULL1_5K = 0,
    #[doc = "1: Pullup is ~6 KOhms"]
    PULL6K = 1,
    #[doc = "2: Pullup is ~12 KOhms"]
    PULL12K = 2,
    #[doc = "3: Pullup is ~24 KOhms"]
    PULL24K = 3,
}
impl From<PAD6RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD6RSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD6RSEL`"]
pub type PAD6RSEL_R = crate::R<u8, PAD6RSEL_A>;
impl PAD6RSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD6RSEL_A {
        match self.bits {
            0 => PAD6RSEL_A::PULL1_5K,
            1 => PAD6RSEL_A::PULL6K,
            2 => PAD6RSEL_A::PULL12K,
            3 => PAD6RSEL_A::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline(always)]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD6RSEL_A::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline(always)]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD6RSEL_A::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline(always)]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD6RSEL_A::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline(always)]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD6RSEL_A::PULL24K
    }
}
#[doc = "Write proxy for field `PAD6RSEL`"]
pub struct PAD6RSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD6RSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD6RSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms"]
    #[inline(always)]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD6RSEL_A::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms"]
    #[inline(always)]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD6RSEL_A::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms"]
    #[inline(always)]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD6RSEL_A::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms"]
    #[inline(always)]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD6RSEL_A::PULL24K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Pad 6 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD6FNCSEL_A {
    #[doc = "0: Configure as the IOMSTR0 I2C SDA signal"]
    M0SDA = 0,
    #[doc = "1: Configure as the IOMSTR0 SPI MISO signal"]
    M0MISO = 1,
    #[doc = "2: Configure as the UART CTS signal"]
    UACTS = 2,
    #[doc = "3: Configure as GPIO6"]
    GPIO6 = 3,
    #[doc = "4: Configure as the IOMSTR0 SPI MISO loopback signal from IOSLAVE"]
    SLMISO = 4,
    #[doc = "6: Configure as the IOMSTR0 I2C SDA loopback signal from IOSLAVE"]
    SLSDA = 6,
    #[doc = "7: Pad disabled"]
    DIS = 7,
}
impl From<PAD6FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD6FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD6FNCSEL`"]
pub type PAD6FNCSEL_R = crate::R<u8, PAD6FNCSEL_A>;
impl PAD6FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD6FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD6FNCSEL_A::M0SDA),
            1 => Val(PAD6FNCSEL_A::M0MISO),
            2 => Val(PAD6FNCSEL_A::UACTS),
            3 => Val(PAD6FNCSEL_A::GPIO6),
            4 => Val(PAD6FNCSEL_A::SLMISO),
            6 => Val(PAD6FNCSEL_A::SLSDA),
            7 => Val(PAD6FNCSEL_A::DIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `M0SDA`"]
    #[inline(always)]
    pub fn is_m0sda(&self) -> bool {
        *self == PAD6FNCSEL_A::M0SDA
    }
    #[doc = "Checks if the value of the field is `M0MISO`"]
    #[inline(always)]
    pub fn is_m0miso(&self) -> bool {
        *self == PAD6FNCSEL_A::M0MISO
    }
    #[doc = "Checks if the value of the field is `UACTS`"]
    #[inline(always)]
    pub fn is_uacts(&self) -> bool {
        *self == PAD6FNCSEL_A::UACTS
    }
    #[doc = "Checks if the value of the field is `GPIO6`"]
    #[inline(always)]
    pub fn is_gpio6(&self) -> bool {
        *self == PAD6FNCSEL_A::GPIO6
    }
    #[doc = "Checks if the value of the field is `SLMISO`"]
    #[inline(always)]
    pub fn is_slmiso(&self) -> bool {
        *self == PAD6FNCSEL_A::SLMISO
    }
    #[doc = "Checks if the value of the field is `SLSDA`"]
    #[inline(always)]
    pub fn is_slsda(&self) -> bool {
        *self == PAD6FNCSEL_A::SLSDA
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD6FNCSEL_A::DIS
    }
}
#[doc = "Write proxy for field `PAD6FNCSEL`"]
pub struct PAD6FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD6FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD6FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the IOMSTR0 I2C SDA signal"]
    #[inline(always)]
    pub fn m0sda(self) -> &'a mut W {
        self.variant(PAD6FNCSEL_A::M0SDA)
    }
    #[doc = "Configure as the IOMSTR0 SPI MISO signal"]
    #[inline(always)]
    pub fn m0miso(self) -> &'a mut W {
        self.variant(PAD6FNCSEL_A::M0MISO)
    }
    #[doc = "Configure as the UART CTS signal"]
    #[inline(always)]
    pub fn uacts(self) -> &'a mut W {
        self.variant(PAD6FNCSEL_A::UACTS)
    }
    #[doc = "Configure as GPIO6"]
    #[inline(always)]
    pub fn gpio6(self) -> &'a mut W {
        self.variant(PAD6FNCSEL_A::GPIO6)
    }
    #[doc = "Configure as the IOMSTR0 SPI MISO loopback signal from IOSLAVE"]
    #[inline(always)]
    pub fn slmiso(self) -> &'a mut W {
        self.variant(PAD6FNCSEL_A::SLMISO)
    }
    #[doc = "Configure as the IOMSTR0 I2C SDA loopback signal from IOSLAVE"]
    #[inline(always)]
    pub fn slsda(self) -> &'a mut W {
        self.variant(PAD6FNCSEL_A::SLSDA)
    }
    #[doc = "Pad disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD6FNCSEL_A::DIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad 6 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD6STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD6STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD6STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD6STRNG`"]
pub type PAD6STRNG_R = crate::R<bool, PAD6STRNG_A>;
impl PAD6STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD6STRNG_A {
        match self.bits {
            false => PAD6STRNG_A::LOW,
            true => PAD6STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD6STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD6STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD6STRNG`"]
pub struct PAD6STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD6STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD6STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD6STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD6STRNG_A::HIGH)
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
#[doc = "Pad 6 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD6INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD6INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD6INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD6INPEN`"]
pub type PAD6INPEN_R = crate::R<bool, PAD6INPEN_A>;
impl PAD6INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD6INPEN_A {
        match self.bits {
            false => PAD6INPEN_A::DIS,
            true => PAD6INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD6INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD6INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD6INPEN`"]
pub struct PAD6INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD6INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD6INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD6INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD6INPEN_A::EN)
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
#[doc = "Pad 6 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD6PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD6PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD6PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD6PULL`"]
pub type PAD6PULL_R = crate::R<bool, PAD6PULL_A>;
impl PAD6PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD6PULL_A {
        match self.bits {
            false => PAD6PULL_A::DIS,
            true => PAD6PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD6PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD6PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD6PULL`"]
pub struct PAD6PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD6PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD6PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD6PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD6PULL_A::EN)
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
#[doc = "Pad 5 pullup resistor selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD5RSEL_A {
    #[doc = "0: Pullup is ~1.5 KOhms"]
    PULL1_5K = 0,
    #[doc = "1: Pullup is ~6 KOhms"]
    PULL6K = 1,
    #[doc = "2: Pullup is ~12 KOhms"]
    PULL12K = 2,
    #[doc = "3: Pullup is ~24 KOhms"]
    PULL24K = 3,
}
impl From<PAD5RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD5RSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD5RSEL`"]
pub type PAD5RSEL_R = crate::R<u8, PAD5RSEL_A>;
impl PAD5RSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD5RSEL_A {
        match self.bits {
            0 => PAD5RSEL_A::PULL1_5K,
            1 => PAD5RSEL_A::PULL6K,
            2 => PAD5RSEL_A::PULL12K,
            3 => PAD5RSEL_A::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline(always)]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD5RSEL_A::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline(always)]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD5RSEL_A::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline(always)]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD5RSEL_A::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline(always)]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD5RSEL_A::PULL24K
    }
}
#[doc = "Write proxy for field `PAD5RSEL`"]
pub struct PAD5RSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD5RSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD5RSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms"]
    #[inline(always)]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD5RSEL_A::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms"]
    #[inline(always)]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD5RSEL_A::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms"]
    #[inline(always)]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD5RSEL_A::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms"]
    #[inline(always)]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD5RSEL_A::PULL24K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Pad 5 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD5FNCSEL_A {
    #[doc = "0: Configure as the IOMSTR0 I2C SCL signal"]
    M0SCL = 0,
    #[doc = "1: Configure as the IOMSTR0 SPI SCK signal"]
    M0SCK = 1,
    #[doc = "2: Configure as the UART RTS signal"]
    UARTS = 2,
    #[doc = "3: Configure as GPIO5"]
    GPIO5 = 3,
    #[doc = "4: Configure as the IOMSTR0 SPI SCK loopback signal from IOSLAVE"]
    SLSCK = 4,
    #[doc = "6: Configure as the IOMSTR0 I2C SCL loopback signal from IOSLAVE"]
    SLSCL = 6,
    #[doc = "7: Pad disabled"]
    DIS = 7,
}
impl From<PAD5FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD5FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD5FNCSEL`"]
pub type PAD5FNCSEL_R = crate::R<u8, PAD5FNCSEL_A>;
impl PAD5FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD5FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD5FNCSEL_A::M0SCL),
            1 => Val(PAD5FNCSEL_A::M0SCK),
            2 => Val(PAD5FNCSEL_A::UARTS),
            3 => Val(PAD5FNCSEL_A::GPIO5),
            4 => Val(PAD5FNCSEL_A::SLSCK),
            6 => Val(PAD5FNCSEL_A::SLSCL),
            7 => Val(PAD5FNCSEL_A::DIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `M0SCL`"]
    #[inline(always)]
    pub fn is_m0scl(&self) -> bool {
        *self == PAD5FNCSEL_A::M0SCL
    }
    #[doc = "Checks if the value of the field is `M0SCK`"]
    #[inline(always)]
    pub fn is_m0sck(&self) -> bool {
        *self == PAD5FNCSEL_A::M0SCK
    }
    #[doc = "Checks if the value of the field is `UARTS`"]
    #[inline(always)]
    pub fn is_uarts(&self) -> bool {
        *self == PAD5FNCSEL_A::UARTS
    }
    #[doc = "Checks if the value of the field is `GPIO5`"]
    #[inline(always)]
    pub fn is_gpio5(&self) -> bool {
        *self == PAD5FNCSEL_A::GPIO5
    }
    #[doc = "Checks if the value of the field is `SLSCK`"]
    #[inline(always)]
    pub fn is_slsck(&self) -> bool {
        *self == PAD5FNCSEL_A::SLSCK
    }
    #[doc = "Checks if the value of the field is `SLSCL`"]
    #[inline(always)]
    pub fn is_slscl(&self) -> bool {
        *self == PAD5FNCSEL_A::SLSCL
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD5FNCSEL_A::DIS
    }
}
#[doc = "Write proxy for field `PAD5FNCSEL`"]
pub struct PAD5FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD5FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD5FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the IOMSTR0 I2C SCL signal"]
    #[inline(always)]
    pub fn m0scl(self) -> &'a mut W {
        self.variant(PAD5FNCSEL_A::M0SCL)
    }
    #[doc = "Configure as the IOMSTR0 SPI SCK signal"]
    #[inline(always)]
    pub fn m0sck(self) -> &'a mut W {
        self.variant(PAD5FNCSEL_A::M0SCK)
    }
    #[doc = "Configure as the UART RTS signal"]
    #[inline(always)]
    pub fn uarts(self) -> &'a mut W {
        self.variant(PAD5FNCSEL_A::UARTS)
    }
    #[doc = "Configure as GPIO5"]
    #[inline(always)]
    pub fn gpio5(self) -> &'a mut W {
        self.variant(PAD5FNCSEL_A::GPIO5)
    }
    #[doc = "Configure as the IOMSTR0 SPI SCK loopback signal from IOSLAVE"]
    #[inline(always)]
    pub fn slsck(self) -> &'a mut W {
        self.variant(PAD5FNCSEL_A::SLSCK)
    }
    #[doc = "Configure as the IOMSTR0 I2C SCL loopback signal from IOSLAVE"]
    #[inline(always)]
    pub fn slscl(self) -> &'a mut W {
        self.variant(PAD5FNCSEL_A::SLSCL)
    }
    #[doc = "Pad disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD5FNCSEL_A::DIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 5 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD5STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD5STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD5STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD5STRNG`"]
pub type PAD5STRNG_R = crate::R<bool, PAD5STRNG_A>;
impl PAD5STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD5STRNG_A {
        match self.bits {
            false => PAD5STRNG_A::LOW,
            true => PAD5STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD5STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD5STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD5STRNG`"]
pub struct PAD5STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD5STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD5STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD5STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD5STRNG_A::HIGH)
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
#[doc = "Pad 5 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD5INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD5INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD5INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD5INPEN`"]
pub type PAD5INPEN_R = crate::R<bool, PAD5INPEN_A>;
impl PAD5INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD5INPEN_A {
        match self.bits {
            false => PAD5INPEN_A::DIS,
            true => PAD5INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD5INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD5INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD5INPEN`"]
pub struct PAD5INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD5INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD5INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD5INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD5INPEN_A::EN)
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
#[doc = "Pad 5 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD5PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD5PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD5PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD5PULL`"]
pub type PAD5PULL_R = crate::R<bool, PAD5PULL_A>;
impl PAD5PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD5PULL_A {
        match self.bits {
            false => PAD5PULL_A::DIS,
            true => PAD5PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD5PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD5PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD5PULL`"]
pub struct PAD5PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD5PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD5PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD5PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD5PULL_A::EN)
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
#[doc = "Pad 4 upper power switch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD4PWRUP_A {
    #[doc = "0: Power switch disabled"]
    DIS = 0,
    #[doc = "1: Power switch enabled"]
    EN = 1,
}
impl From<PAD4PWRUP_A> for bool {
    #[inline(always)]
    fn from(variant: PAD4PWRUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD4PWRUP`"]
pub type PAD4PWRUP_R = crate::R<bool, PAD4PWRUP_A>;
impl PAD4PWRUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD4PWRUP_A {
        match self.bits {
            false => PAD4PWRUP_A::DIS,
            true => PAD4PWRUP_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD4PWRUP_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD4PWRUP_A::EN
    }
}
#[doc = "Write proxy for field `PAD4PWRUP`"]
pub struct PAD4PWRUP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD4PWRUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD4PWRUP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power switch disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD4PWRUP_A::DIS)
    }
    #[doc = "Power switch enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD4PWRUP_A::EN)
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
#[doc = "Pad 4 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD4FNCSEL_A {
    #[doc = "0: Configure as the ADC Trigger 1 signal"]
    TRIG1 = 0,
    #[doc = "1: Configure as the IOSLAVE interrupt out signal"]
    SLINT = 1,
    #[doc = "2: Configure as the SPI channel 5 nCE signal from IOMSTR0"]
    M0NCE5 = 2,
    #[doc = "3: Configure as GPIO4"]
    GPIO4 = 3,
    #[doc = "4: Configure as the IOSLAVE interrupt loopback signal to GPIO4"]
    SLINTGP = 4,
    #[doc = "5: Configure as the serial wire debug SWO signal"]
    SWO = 5,
    #[doc = "6: Configure as the CLKOUT signal"]
    CLKOUT = 6,
    #[doc = "7: Pad disabled"]
    DIS = 7,
}
impl From<PAD4FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD4FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD4FNCSEL`"]
pub type PAD4FNCSEL_R = crate::R<u8, PAD4FNCSEL_A>;
impl PAD4FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD4FNCSEL_A {
        match self.bits {
            0 => PAD4FNCSEL_A::TRIG1,
            1 => PAD4FNCSEL_A::SLINT,
            2 => PAD4FNCSEL_A::M0NCE5,
            3 => PAD4FNCSEL_A::GPIO4,
            4 => PAD4FNCSEL_A::SLINTGP,
            5 => PAD4FNCSEL_A::SWO,
            6 => PAD4FNCSEL_A::CLKOUT,
            7 => PAD4FNCSEL_A::DIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRIG1`"]
    #[inline(always)]
    pub fn is_trig1(&self) -> bool {
        *self == PAD4FNCSEL_A::TRIG1
    }
    #[doc = "Checks if the value of the field is `SLINT`"]
    #[inline(always)]
    pub fn is_slint(&self) -> bool {
        *self == PAD4FNCSEL_A::SLINT
    }
    #[doc = "Checks if the value of the field is `M0NCE5`"]
    #[inline(always)]
    pub fn is_m0n_ce5(&self) -> bool {
        *self == PAD4FNCSEL_A::M0NCE5
    }
    #[doc = "Checks if the value of the field is `GPIO4`"]
    #[inline(always)]
    pub fn is_gpio4(&self) -> bool {
        *self == PAD4FNCSEL_A::GPIO4
    }
    #[doc = "Checks if the value of the field is `SLINTGP`"]
    #[inline(always)]
    pub fn is_slintgp(&self) -> bool {
        *self == PAD4FNCSEL_A::SLINTGP
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline(always)]
    pub fn is_swo(&self) -> bool {
        *self == PAD4FNCSEL_A::SWO
    }
    #[doc = "Checks if the value of the field is `CLKOUT`"]
    #[inline(always)]
    pub fn is_clkout(&self) -> bool {
        *self == PAD4FNCSEL_A::CLKOUT
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD4FNCSEL_A::DIS
    }
}
#[doc = "Write proxy for field `PAD4FNCSEL`"]
pub struct PAD4FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD4FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD4FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the ADC Trigger 1 signal"]
    #[inline(always)]
    pub fn trig1(self) -> &'a mut W {
        self.variant(PAD4FNCSEL_A::TRIG1)
    }
    #[doc = "Configure as the IOSLAVE interrupt out signal"]
    #[inline(always)]
    pub fn slint(self) -> &'a mut W {
        self.variant(PAD4FNCSEL_A::SLINT)
    }
    #[doc = "Configure as the SPI channel 5 nCE signal from IOMSTR0"]
    #[inline(always)]
    pub fn m0n_ce5(self) -> &'a mut W {
        self.variant(PAD4FNCSEL_A::M0NCE5)
    }
    #[doc = "Configure as GPIO4"]
    #[inline(always)]
    pub fn gpio4(self) -> &'a mut W {
        self.variant(PAD4FNCSEL_A::GPIO4)
    }
    #[doc = "Configure as the IOSLAVE interrupt loopback signal to GPIO4"]
    #[inline(always)]
    pub fn slintgp(self) -> &'a mut W {
        self.variant(PAD4FNCSEL_A::SLINTGP)
    }
    #[doc = "Configure as the serial wire debug SWO signal"]
    #[inline(always)]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD4FNCSEL_A::SWO)
    }
    #[doc = "Configure as the CLKOUT signal"]
    #[inline(always)]
    pub fn clkout(self) -> &'a mut W {
        self.variant(PAD4FNCSEL_A::CLKOUT)
    }
    #[doc = "Pad disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD4FNCSEL_A::DIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad 4 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD4STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD4STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD4STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD4STRNG`"]
pub type PAD4STRNG_R = crate::R<bool, PAD4STRNG_A>;
impl PAD4STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD4STRNG_A {
        match self.bits {
            false => PAD4STRNG_A::LOW,
            true => PAD4STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD4STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD4STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD4STRNG`"]
pub struct PAD4STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD4STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD4STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD4STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD4STRNG_A::HIGH)
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
#[doc = "Pad 4 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD4INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD4INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD4INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD4INPEN`"]
pub type PAD4INPEN_R = crate::R<bool, PAD4INPEN_A>;
impl PAD4INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD4INPEN_A {
        match self.bits {
            false => PAD4INPEN_A::DIS,
            true => PAD4INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD4INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD4INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD4INPEN`"]
pub struct PAD4INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD4INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD4INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD4INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD4INPEN_A::EN)
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
#[doc = "Pad 4 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD4PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD4PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD4PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD4PULL`"]
pub type PAD4PULL_R = crate::R<bool, PAD4PULL_A>;
impl PAD4PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD4PULL_A {
        match self.bits {
            false => PAD4PULL_A::DIS,
            true => PAD4PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD4PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD4PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD4PULL`"]
pub struct PAD4PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD4PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD4PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD4PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD4PULL_A::EN)
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
    #[doc = "Bits 27:29 - Pad 7 function select"]
    #[inline(always)]
    pub fn pad7fncsel(&self) -> PAD7FNCSEL_R {
        PAD7FNCSEL_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Pad 7 drive strentgh"]
    #[inline(always)]
    pub fn pad7strng(&self) -> PAD7STRNG_R {
        PAD7STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 7 input enable"]
    #[inline(always)]
    pub fn pad7inpen(&self) -> PAD7INPEN_R {
        PAD7INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 7 pullup enable"]
    #[inline(always)]
    pub fn pad7pull(&self) -> PAD7PULL_R {
        PAD7PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - Pad 6 pullup resistor selection."]
    #[inline(always)]
    pub fn pad6rsel(&self) -> PAD6RSEL_R {
        PAD6RSEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 19:21 - Pad 6 function select"]
    #[inline(always)]
    pub fn pad6fncsel(&self) -> PAD6FNCSEL_R {
        PAD6FNCSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Pad 6 drive strength"]
    #[inline(always)]
    pub fn pad6strng(&self) -> PAD6STRNG_R {
        PAD6STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 6 input enable"]
    #[inline(always)]
    pub fn pad6inpen(&self) -> PAD6INPEN_R {
        PAD6INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 6 pullup enable"]
    #[inline(always)]
    pub fn pad6pull(&self) -> PAD6PULL_R {
        PAD6PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Pad 5 pullup resistor selection."]
    #[inline(always)]
    pub fn pad5rsel(&self) -> PAD5RSEL_R {
        PAD5RSEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 11:13 - Pad 5 function select"]
    #[inline(always)]
    pub fn pad5fncsel(&self) -> PAD5FNCSEL_R {
        PAD5FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 5 drive strength"]
    #[inline(always)]
    pub fn pad5strng(&self) -> PAD5STRNG_R {
        PAD5STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 5 input enable"]
    #[inline(always)]
    pub fn pad5inpen(&self) -> PAD5INPEN_R {
        PAD5INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 5 pullup enable"]
    #[inline(always)]
    pub fn pad5pull(&self) -> PAD5PULL_R {
        PAD5PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Pad 4 upper power switch enable"]
    #[inline(always)]
    pub fn pad4pwrup(&self) -> PAD4PWRUP_R {
        PAD4PWRUP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Pad 4 function select"]
    #[inline(always)]
    pub fn pad4fncsel(&self) -> PAD4FNCSEL_R {
        PAD4FNCSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Pad 4 drive strength"]
    #[inline(always)]
    pub fn pad4strng(&self) -> PAD4STRNG_R {
        PAD4STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 4 input enable"]
    #[inline(always)]
    pub fn pad4inpen(&self) -> PAD4INPEN_R {
        PAD4INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 4 pullup enable"]
    #[inline(always)]
    pub fn pad4pull(&self) -> PAD4PULL_R {
        PAD4PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:29 - Pad 7 function select"]
    #[inline(always)]
    pub fn pad7fncsel(&mut self) -> PAD7FNCSEL_W {
        PAD7FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 7 drive strentgh"]
    #[inline(always)]
    pub fn pad7strng(&mut self) -> PAD7STRNG_W {
        PAD7STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 7 input enable"]
    #[inline(always)]
    pub fn pad7inpen(&mut self) -> PAD7INPEN_W {
        PAD7INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 7 pullup enable"]
    #[inline(always)]
    pub fn pad7pull(&mut self) -> PAD7PULL_W {
        PAD7PULL_W { w: self }
    }
    #[doc = "Bits 22:23 - Pad 6 pullup resistor selection."]
    #[inline(always)]
    pub fn pad6rsel(&mut self) -> PAD6RSEL_W {
        PAD6RSEL_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad 6 function select"]
    #[inline(always)]
    pub fn pad6fncsel(&mut self) -> PAD6FNCSEL_W {
        PAD6FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 6 drive strength"]
    #[inline(always)]
    pub fn pad6strng(&mut self) -> PAD6STRNG_W {
        PAD6STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 6 input enable"]
    #[inline(always)]
    pub fn pad6inpen(&mut self) -> PAD6INPEN_W {
        PAD6INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 6 pullup enable"]
    #[inline(always)]
    pub fn pad6pull(&mut self) -> PAD6PULL_W {
        PAD6PULL_W { w: self }
    }
    #[doc = "Bits 14:15 - Pad 5 pullup resistor selection."]
    #[inline(always)]
    pub fn pad5rsel(&mut self) -> PAD5RSEL_W {
        PAD5RSEL_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 5 function select"]
    #[inline(always)]
    pub fn pad5fncsel(&mut self) -> PAD5FNCSEL_W {
        PAD5FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 5 drive strength"]
    #[inline(always)]
    pub fn pad5strng(&mut self) -> PAD5STRNG_W {
        PAD5STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 5 input enable"]
    #[inline(always)]
    pub fn pad5inpen(&mut self) -> PAD5INPEN_W {
        PAD5INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 5 pullup enable"]
    #[inline(always)]
    pub fn pad5pull(&mut self) -> PAD5PULL_W {
        PAD5PULL_W { w: self }
    }
    #[doc = "Bit 7 - Pad 4 upper power switch enable"]
    #[inline(always)]
    pub fn pad4pwrup(&mut self) -> PAD4PWRUP_W {
        PAD4PWRUP_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad 4 function select"]
    #[inline(always)]
    pub fn pad4fncsel(&mut self) -> PAD4FNCSEL_W {
        PAD4FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 4 drive strength"]
    #[inline(always)]
    pub fn pad4strng(&mut self) -> PAD4STRNG_W {
        PAD4STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 4 input enable"]
    #[inline(always)]
    pub fn pad4inpen(&mut self) -> PAD4INPEN_W {
        PAD4INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 4 pullup enable"]
    #[inline(always)]
    pub fn pad4pull(&mut self) -> PAD4PULL_W {
        PAD4PULL_W { w: self }
    }
}
