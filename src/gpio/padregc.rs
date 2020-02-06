#[doc = "Reader of register PADREGC"]
pub type R = crate::R<u32, super::PADREGC>;
#[doc = "Writer for register PADREGC"]
pub type W = crate::W<u32, super::PADREGC>;
#[doc = "Register PADREGC `reset()`'s with value 0x1818_1818"]
impl crate::ResetValue for super::PADREGC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1818_1818
    }
}
#[doc = "Pad 11 lower power switch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD11PWRDN_A {
    #[doc = "0: Power switch disabled"]
    DIS = 0,
    #[doc = "1: Power switch enabled"]
    EN = 1,
}
impl From<PAD11PWRDN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD11PWRDN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD11PWRDN`"]
pub type PAD11PWRDN_R = crate::R<bool, PAD11PWRDN_A>;
impl PAD11PWRDN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD11PWRDN_A {
        match self.bits {
            false => PAD11PWRDN_A::DIS,
            true => PAD11PWRDN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD11PWRDN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD11PWRDN_A::EN
    }
}
#[doc = "Write proxy for field `PAD11PWRDN`"]
pub struct PAD11PWRDN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD11PWRDN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD11PWRDN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power switch disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD11PWRDN_A::DIS)
    }
    #[doc = "Power switch enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD11PWRDN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Pad 11 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD11FNCSEL_A {
    #[doc = "0: Configure as the analog test output signal"]
    ANATST = 0,
    #[doc = "1: Configure as the SPI channel 0 nCE signal from IOMSTR0"]
    M0NCE0 = 1,
    #[doc = "2: Configure as the CLKOUT signal"]
    CLKOUT = 2,
    #[doc = "3: Configure as GPIO11"]
    GPIO11 = 3,
}
impl From<PAD11FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD11FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD11FNCSEL`"]
pub type PAD11FNCSEL_R = crate::R<u8, PAD11FNCSEL_A>;
impl PAD11FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD11FNCSEL_A {
        match self.bits {
            0 => PAD11FNCSEL_A::ANATST,
            1 => PAD11FNCSEL_A::M0NCE0,
            2 => PAD11FNCSEL_A::CLKOUT,
            3 => PAD11FNCSEL_A::GPIO11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ANATST`"]
    #[inline(always)]
    pub fn is_anatst(&self) -> bool {
        *self == PAD11FNCSEL_A::ANATST
    }
    #[doc = "Checks if the value of the field is `M0NCE0`"]
    #[inline(always)]
    pub fn is_m0n_ce0(&self) -> bool {
        *self == PAD11FNCSEL_A::M0NCE0
    }
    #[doc = "Checks if the value of the field is `CLKOUT`"]
    #[inline(always)]
    pub fn is_clkout(&self) -> bool {
        *self == PAD11FNCSEL_A::CLKOUT
    }
    #[doc = "Checks if the value of the field is `GPIO11`"]
    #[inline(always)]
    pub fn is_gpio11(&self) -> bool {
        *self == PAD11FNCSEL_A::GPIO11
    }
}
#[doc = "Write proxy for field `PAD11FNCSEL`"]
pub struct PAD11FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD11FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD11FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the analog test output signal"]
    #[inline(always)]
    pub fn anatst(self) -> &'a mut W {
        self.variant(PAD11FNCSEL_A::ANATST)
    }
    #[doc = "Configure as the SPI channel 0 nCE signal from IOMSTR0"]
    #[inline(always)]
    pub fn m0n_ce0(self) -> &'a mut W {
        self.variant(PAD11FNCSEL_A::M0NCE0)
    }
    #[doc = "Configure as the CLKOUT signal"]
    #[inline(always)]
    pub fn clkout(self) -> &'a mut W {
        self.variant(PAD11FNCSEL_A::CLKOUT)
    }
    #[doc = "Configure as GPIO11"]
    #[inline(always)]
    pub fn gpio11(self) -> &'a mut W {
        self.variant(PAD11FNCSEL_A::GPIO11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Pad 11 drive strentgh\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD11STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD11STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD11STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD11STRNG`"]
pub type PAD11STRNG_R = crate::R<bool, PAD11STRNG_A>;
impl PAD11STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD11STRNG_A {
        match self.bits {
            false => PAD11STRNG_A::LOW,
            true => PAD11STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD11STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD11STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD11STRNG`"]
pub struct PAD11STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD11STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD11STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD11STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD11STRNG_A::HIGH)
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
#[doc = "Pad 11 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD11INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD11INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD11INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD11INPEN`"]
pub type PAD11INPEN_R = crate::R<bool, PAD11INPEN_A>;
impl PAD11INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD11INPEN_A {
        match self.bits {
            false => PAD11INPEN_A::DIS,
            true => PAD11INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD11INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD11INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD11INPEN`"]
pub struct PAD11INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD11INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD11INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD11INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD11INPEN_A::EN)
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
#[doc = "Pad 11 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD11PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD11PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD11PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD11PULL`"]
pub type PAD11PULL_R = crate::R<bool, PAD11PULL_A>;
impl PAD11PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD11PULL_A {
        match self.bits {
            false => PAD11PULL_A::DIS,
            true => PAD11PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD11PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD11PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD11PULL`"]
pub struct PAD11PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD11PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD11PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD11PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD11PULL_A::EN)
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
#[doc = "Pad 10 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD10FNCSEL_A {
    #[doc = "0: Configure as the IOMSTR1 SPI 3-wire MOSI/MISO signal"]
    M1WIR3 = 0,
    #[doc = "1: Configure as the IOMSTR1 SPI MOSI signal"]
    M1MOSI = 1,
    #[doc = "2: Configure as the SPI channel 6 nCE signal from IOMSTR0"]
    M0NCE6 = 2,
    #[doc = "3: Configure as GPIO10"]
    GPIO10 = 3,
    #[doc = "5: Configure as the external HFRC A clock signal"]
    EXTHFA = 5,
    #[doc = "6: Pad disabled"]
    DIS = 6,
    #[doc = "7: Configure as the IOMSTR1 SPI 3-wire MOSI/MISO loopback signal from IOSLAVE"]
    SLWIR3 = 7,
}
impl From<PAD10FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD10FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD10FNCSEL`"]
pub type PAD10FNCSEL_R = crate::R<u8, PAD10FNCSEL_A>;
impl PAD10FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD10FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD10FNCSEL_A::M1WIR3),
            1 => Val(PAD10FNCSEL_A::M1MOSI),
            2 => Val(PAD10FNCSEL_A::M0NCE6),
            3 => Val(PAD10FNCSEL_A::GPIO10),
            5 => Val(PAD10FNCSEL_A::EXTHFA),
            6 => Val(PAD10FNCSEL_A::DIS),
            7 => Val(PAD10FNCSEL_A::SLWIR3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `M1WIR3`"]
    #[inline(always)]
    pub fn is_m1wir3(&self) -> bool {
        *self == PAD10FNCSEL_A::M1WIR3
    }
    #[doc = "Checks if the value of the field is `M1MOSI`"]
    #[inline(always)]
    pub fn is_m1mosi(&self) -> bool {
        *self == PAD10FNCSEL_A::M1MOSI
    }
    #[doc = "Checks if the value of the field is `M0NCE6`"]
    #[inline(always)]
    pub fn is_m0n_ce6(&self) -> bool {
        *self == PAD10FNCSEL_A::M0NCE6
    }
    #[doc = "Checks if the value of the field is `GPIO10`"]
    #[inline(always)]
    pub fn is_gpio10(&self) -> bool {
        *self == PAD10FNCSEL_A::GPIO10
    }
    #[doc = "Checks if the value of the field is `EXTHFA`"]
    #[inline(always)]
    pub fn is_exthfa(&self) -> bool {
        *self == PAD10FNCSEL_A::EXTHFA
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD10FNCSEL_A::DIS
    }
    #[doc = "Checks if the value of the field is `SLWIR3`"]
    #[inline(always)]
    pub fn is_slwir3(&self) -> bool {
        *self == PAD10FNCSEL_A::SLWIR3
    }
}
#[doc = "Write proxy for field `PAD10FNCSEL`"]
pub struct PAD10FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD10FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD10FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the IOMSTR1 SPI 3-wire MOSI/MISO signal"]
    #[inline(always)]
    pub fn m1wir3(self) -> &'a mut W {
        self.variant(PAD10FNCSEL_A::M1WIR3)
    }
    #[doc = "Configure as the IOMSTR1 SPI MOSI signal"]
    #[inline(always)]
    pub fn m1mosi(self) -> &'a mut W {
        self.variant(PAD10FNCSEL_A::M1MOSI)
    }
    #[doc = "Configure as the SPI channel 6 nCE signal from IOMSTR0"]
    #[inline(always)]
    pub fn m0n_ce6(self) -> &'a mut W {
        self.variant(PAD10FNCSEL_A::M0NCE6)
    }
    #[doc = "Configure as GPIO10"]
    #[inline(always)]
    pub fn gpio10(self) -> &'a mut W {
        self.variant(PAD10FNCSEL_A::GPIO10)
    }
    #[doc = "Configure as the external HFRC A clock signal"]
    #[inline(always)]
    pub fn exthfa(self) -> &'a mut W {
        self.variant(PAD10FNCSEL_A::EXTHFA)
    }
    #[doc = "Pad disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD10FNCSEL_A::DIS)
    }
    #[doc = "Configure as the IOMSTR1 SPI 3-wire MOSI/MISO loopback signal from IOSLAVE"]
    #[inline(always)]
    pub fn slwir3(self) -> &'a mut W {
        self.variant(PAD10FNCSEL_A::SLWIR3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad 10 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD10STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD10STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD10STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD10STRNG`"]
pub type PAD10STRNG_R = crate::R<bool, PAD10STRNG_A>;
impl PAD10STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD10STRNG_A {
        match self.bits {
            false => PAD10STRNG_A::LOW,
            true => PAD10STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD10STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD10STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD10STRNG`"]
pub struct PAD10STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD10STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD10STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD10STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD10STRNG_A::HIGH)
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
#[doc = "Pad 10 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD10INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD10INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD10INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD10INPEN`"]
pub type PAD10INPEN_R = crate::R<bool, PAD10INPEN_A>;
impl PAD10INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD10INPEN_A {
        match self.bits {
            false => PAD10INPEN_A::DIS,
            true => PAD10INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD10INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD10INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD10INPEN`"]
pub struct PAD10INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD10INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD10INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD10INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD10INPEN_A::EN)
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
#[doc = "Pad 10 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD10PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD10PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD10PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD10PULL`"]
pub type PAD10PULL_R = crate::R<bool, PAD10PULL_A>;
impl PAD10PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD10PULL_A {
        match self.bits {
            false => PAD10PULL_A::DIS,
            true => PAD10PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD10PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD10PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD10PULL`"]
pub struct PAD10PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD10PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD10PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD10PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD10PULL_A::EN)
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
#[doc = "Pad 9 pullup resistor selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD9RSEL_A {
    #[doc = "0: Pullup is ~1.5 KOhms"]
    PULL1_5K = 0,
    #[doc = "1: Pullup is ~6 KOhms"]
    PULL6K = 1,
    #[doc = "2: Pullup is ~12 KOhms"]
    PULL12K = 2,
    #[doc = "3: Pullup is ~24 KOhms"]
    PULL24K = 3,
}
impl From<PAD9RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD9RSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD9RSEL`"]
pub type PAD9RSEL_R = crate::R<u8, PAD9RSEL_A>;
impl PAD9RSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD9RSEL_A {
        match self.bits {
            0 => PAD9RSEL_A::PULL1_5K,
            1 => PAD9RSEL_A::PULL6K,
            2 => PAD9RSEL_A::PULL12K,
            3 => PAD9RSEL_A::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline(always)]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD9RSEL_A::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline(always)]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD9RSEL_A::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline(always)]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD9RSEL_A::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline(always)]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD9RSEL_A::PULL24K
    }
}
#[doc = "Write proxy for field `PAD9RSEL`"]
pub struct PAD9RSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD9RSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD9RSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms"]
    #[inline(always)]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD9RSEL_A::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms"]
    #[inline(always)]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD9RSEL_A::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms"]
    #[inline(always)]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD9RSEL_A::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms"]
    #[inline(always)]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD9RSEL_A::PULL24K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Pad 9 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD9FNCSEL_A {
    #[doc = "0: Configure as the IOMSTR1 I2C SDA signal"]
    M1SDA = 0,
    #[doc = "1: Configure as the IOMSTR1 SPI MISO signal"]
    M1MISO = 1,
    #[doc = "2: Configure as the SPI channel 5 nCE signal from IOMSTR0"]
    M0NCE5 = 2,
    #[doc = "3: Configure as GPIO9"]
    GPIO9 = 3,
    #[doc = "5: Configure as the IOMSTR1 SPI MISO loopback signal from IOSLAVE"]
    SLMISO = 5,
    #[doc = "6: Pad disabled"]
    DIS = 6,
    #[doc = "7: Configure as the IOMSTR1 I2C SDA loopback signal from IOSLAVE"]
    SLSDA = 7,
}
impl From<PAD9FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD9FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD9FNCSEL`"]
pub type PAD9FNCSEL_R = crate::R<u8, PAD9FNCSEL_A>;
impl PAD9FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD9FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD9FNCSEL_A::M1SDA),
            1 => Val(PAD9FNCSEL_A::M1MISO),
            2 => Val(PAD9FNCSEL_A::M0NCE5),
            3 => Val(PAD9FNCSEL_A::GPIO9),
            5 => Val(PAD9FNCSEL_A::SLMISO),
            6 => Val(PAD9FNCSEL_A::DIS),
            7 => Val(PAD9FNCSEL_A::SLSDA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `M1SDA`"]
    #[inline(always)]
    pub fn is_m1sda(&self) -> bool {
        *self == PAD9FNCSEL_A::M1SDA
    }
    #[doc = "Checks if the value of the field is `M1MISO`"]
    #[inline(always)]
    pub fn is_m1miso(&self) -> bool {
        *self == PAD9FNCSEL_A::M1MISO
    }
    #[doc = "Checks if the value of the field is `M0NCE5`"]
    #[inline(always)]
    pub fn is_m0n_ce5(&self) -> bool {
        *self == PAD9FNCSEL_A::M0NCE5
    }
    #[doc = "Checks if the value of the field is `GPIO9`"]
    #[inline(always)]
    pub fn is_gpio9(&self) -> bool {
        *self == PAD9FNCSEL_A::GPIO9
    }
    #[doc = "Checks if the value of the field is `SLMISO`"]
    #[inline(always)]
    pub fn is_slmiso(&self) -> bool {
        *self == PAD9FNCSEL_A::SLMISO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD9FNCSEL_A::DIS
    }
    #[doc = "Checks if the value of the field is `SLSDA`"]
    #[inline(always)]
    pub fn is_slsda(&self) -> bool {
        *self == PAD9FNCSEL_A::SLSDA
    }
}
#[doc = "Write proxy for field `PAD9FNCSEL`"]
pub struct PAD9FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD9FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD9FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the IOMSTR1 I2C SDA signal"]
    #[inline(always)]
    pub fn m1sda(self) -> &'a mut W {
        self.variant(PAD9FNCSEL_A::M1SDA)
    }
    #[doc = "Configure as the IOMSTR1 SPI MISO signal"]
    #[inline(always)]
    pub fn m1miso(self) -> &'a mut W {
        self.variant(PAD9FNCSEL_A::M1MISO)
    }
    #[doc = "Configure as the SPI channel 5 nCE signal from IOMSTR0"]
    #[inline(always)]
    pub fn m0n_ce5(self) -> &'a mut W {
        self.variant(PAD9FNCSEL_A::M0NCE5)
    }
    #[doc = "Configure as GPIO9"]
    #[inline(always)]
    pub fn gpio9(self) -> &'a mut W {
        self.variant(PAD9FNCSEL_A::GPIO9)
    }
    #[doc = "Configure as the IOMSTR1 SPI MISO loopback signal from IOSLAVE"]
    #[inline(always)]
    pub fn slmiso(self) -> &'a mut W {
        self.variant(PAD9FNCSEL_A::SLMISO)
    }
    #[doc = "Pad disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD9FNCSEL_A::DIS)
    }
    #[doc = "Configure as the IOMSTR1 I2C SDA loopback signal from IOSLAVE"]
    #[inline(always)]
    pub fn slsda(self) -> &'a mut W {
        self.variant(PAD9FNCSEL_A::SLSDA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 9 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD9STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD9STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD9STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD9STRNG`"]
pub type PAD9STRNG_R = crate::R<bool, PAD9STRNG_A>;
impl PAD9STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD9STRNG_A {
        match self.bits {
            false => PAD9STRNG_A::LOW,
            true => PAD9STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD9STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD9STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD9STRNG`"]
pub struct PAD9STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD9STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD9STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD9STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD9STRNG_A::HIGH)
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
#[doc = "Pad 9 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD9INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD9INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD9INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD9INPEN`"]
pub type PAD9INPEN_R = crate::R<bool, PAD9INPEN_A>;
impl PAD9INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD9INPEN_A {
        match self.bits {
            false => PAD9INPEN_A::DIS,
            true => PAD9INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD9INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD9INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD9INPEN`"]
pub struct PAD9INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD9INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD9INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD9INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD9INPEN_A::EN)
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
#[doc = "Pad 9 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD9PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD9PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD9PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD9PULL`"]
pub type PAD9PULL_R = crate::R<bool, PAD9PULL_A>;
impl PAD9PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD9PULL_A {
        match self.bits {
            false => PAD9PULL_A::DIS,
            true => PAD9PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD9PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD9PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD9PULL`"]
pub struct PAD9PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD9PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD9PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD9PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD9PULL_A::EN)
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
#[doc = "Pad 8 pullup resistor selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD8RSEL_A {
    #[doc = "0: Pullup is ~1.5 KOhms"]
    PULL1_5K = 0,
    #[doc = "1: Pullup is ~6 KOhms"]
    PULL6K = 1,
    #[doc = "2: Pullup is ~12 KOhms"]
    PULL12K = 2,
    #[doc = "3: Pullup is ~24 KOhms"]
    PULL24K = 3,
}
impl From<PAD8RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD8RSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD8RSEL`"]
pub type PAD8RSEL_R = crate::R<u8, PAD8RSEL_A>;
impl PAD8RSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD8RSEL_A {
        match self.bits {
            0 => PAD8RSEL_A::PULL1_5K,
            1 => PAD8RSEL_A::PULL6K,
            2 => PAD8RSEL_A::PULL12K,
            3 => PAD8RSEL_A::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline(always)]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD8RSEL_A::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline(always)]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD8RSEL_A::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline(always)]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD8RSEL_A::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline(always)]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD8RSEL_A::PULL24K
    }
}
#[doc = "Write proxy for field `PAD8RSEL`"]
pub struct PAD8RSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD8RSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD8RSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms"]
    #[inline(always)]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD8RSEL_A::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms"]
    #[inline(always)]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD8RSEL_A::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms"]
    #[inline(always)]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD8RSEL_A::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms"]
    #[inline(always)]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD8RSEL_A::PULL24K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Pad 8 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD8FNCSEL_A {
    #[doc = "0: Configure as the IOMSTR1 I2C SCL signal"]
    M1SCL = 0,
    #[doc = "1: Configure as the IOMSTR1 SPI SCK signal"]
    M1SCK = 1,
    #[doc = "2: Configure as the SPI channel 4 nCE signal from IOMSTR0"]
    M0NCE4 = 2,
    #[doc = "3: Configure as GPIO8"]
    GPIO8 = 3,
    #[doc = "5: Configure as the IOMSTR1 SPI SCK loopback signal from IOSLAVE"]
    SLSCK = 5,
    #[doc = "6: Pad disabled"]
    DIS = 6,
    #[doc = "7: Configure as the IOMSTR1 I2C SCL loopback signal from IOSLAVE"]
    SLSCL = 7,
}
impl From<PAD8FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD8FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD8FNCSEL`"]
pub type PAD8FNCSEL_R = crate::R<u8, PAD8FNCSEL_A>;
impl PAD8FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD8FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD8FNCSEL_A::M1SCL),
            1 => Val(PAD8FNCSEL_A::M1SCK),
            2 => Val(PAD8FNCSEL_A::M0NCE4),
            3 => Val(PAD8FNCSEL_A::GPIO8),
            5 => Val(PAD8FNCSEL_A::SLSCK),
            6 => Val(PAD8FNCSEL_A::DIS),
            7 => Val(PAD8FNCSEL_A::SLSCL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `M1SCL`"]
    #[inline(always)]
    pub fn is_m1scl(&self) -> bool {
        *self == PAD8FNCSEL_A::M1SCL
    }
    #[doc = "Checks if the value of the field is `M1SCK`"]
    #[inline(always)]
    pub fn is_m1sck(&self) -> bool {
        *self == PAD8FNCSEL_A::M1SCK
    }
    #[doc = "Checks if the value of the field is `M0NCE4`"]
    #[inline(always)]
    pub fn is_m0n_ce4(&self) -> bool {
        *self == PAD8FNCSEL_A::M0NCE4
    }
    #[doc = "Checks if the value of the field is `GPIO8`"]
    #[inline(always)]
    pub fn is_gpio8(&self) -> bool {
        *self == PAD8FNCSEL_A::GPIO8
    }
    #[doc = "Checks if the value of the field is `SLSCK`"]
    #[inline(always)]
    pub fn is_slsck(&self) -> bool {
        *self == PAD8FNCSEL_A::SLSCK
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD8FNCSEL_A::DIS
    }
    #[doc = "Checks if the value of the field is `SLSCL`"]
    #[inline(always)]
    pub fn is_slscl(&self) -> bool {
        *self == PAD8FNCSEL_A::SLSCL
    }
}
#[doc = "Write proxy for field `PAD8FNCSEL`"]
pub struct PAD8FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD8FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD8FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the IOMSTR1 I2C SCL signal"]
    #[inline(always)]
    pub fn m1scl(self) -> &'a mut W {
        self.variant(PAD8FNCSEL_A::M1SCL)
    }
    #[doc = "Configure as the IOMSTR1 SPI SCK signal"]
    #[inline(always)]
    pub fn m1sck(self) -> &'a mut W {
        self.variant(PAD8FNCSEL_A::M1SCK)
    }
    #[doc = "Configure as the SPI channel 4 nCE signal from IOMSTR0"]
    #[inline(always)]
    pub fn m0n_ce4(self) -> &'a mut W {
        self.variant(PAD8FNCSEL_A::M0NCE4)
    }
    #[doc = "Configure as GPIO8"]
    #[inline(always)]
    pub fn gpio8(self) -> &'a mut W {
        self.variant(PAD8FNCSEL_A::GPIO8)
    }
    #[doc = "Configure as the IOMSTR1 SPI SCK loopback signal from IOSLAVE"]
    #[inline(always)]
    pub fn slsck(self) -> &'a mut W {
        self.variant(PAD8FNCSEL_A::SLSCK)
    }
    #[doc = "Pad disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD8FNCSEL_A::DIS)
    }
    #[doc = "Configure as the IOMSTR1 I2C SCL loopback signal from IOSLAVE"]
    #[inline(always)]
    pub fn slscl(self) -> &'a mut W {
        self.variant(PAD8FNCSEL_A::SLSCL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad 8 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD8STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD8STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD8STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD8STRNG`"]
pub type PAD8STRNG_R = crate::R<bool, PAD8STRNG_A>;
impl PAD8STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD8STRNG_A {
        match self.bits {
            false => PAD8STRNG_A::LOW,
            true => PAD8STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD8STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD8STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD8STRNG`"]
pub struct PAD8STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD8STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD8STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD8STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD8STRNG_A::HIGH)
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
#[doc = "Pad 8 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD8INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD8INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD8INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD8INPEN`"]
pub type PAD8INPEN_R = crate::R<bool, PAD8INPEN_A>;
impl PAD8INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD8INPEN_A {
        match self.bits {
            false => PAD8INPEN_A::DIS,
            true => PAD8INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD8INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD8INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD8INPEN`"]
pub struct PAD8INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD8INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD8INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD8INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD8INPEN_A::EN)
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
#[doc = "Pad 8 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD8PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD8PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD8PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD8PULL`"]
pub type PAD8PULL_R = crate::R<bool, PAD8PULL_A>;
impl PAD8PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD8PULL_A {
        match self.bits {
            false => PAD8PULL_A::DIS,
            true => PAD8PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD8PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD8PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD8PULL`"]
pub struct PAD8PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD8PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD8PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD8PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD8PULL_A::EN)
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
    #[doc = "Bit 30 - Pad 11 lower power switch enable"]
    #[inline(always)]
    pub fn pad11pwrdn(&self) -> PAD11PWRDN_R {
        PAD11PWRDN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - Pad 11 function select"]
    #[inline(always)]
    pub fn pad11fncsel(&self) -> PAD11FNCSEL_R {
        PAD11FNCSEL_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Pad 11 drive strentgh"]
    #[inline(always)]
    pub fn pad11strng(&self) -> PAD11STRNG_R {
        PAD11STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 11 input enable"]
    #[inline(always)]
    pub fn pad11inpen(&self) -> PAD11INPEN_R {
        PAD11INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 11 pullup enable"]
    #[inline(always)]
    pub fn pad11pull(&self) -> PAD11PULL_R {
        PAD11PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - Pad 10 function select"]
    #[inline(always)]
    pub fn pad10fncsel(&self) -> PAD10FNCSEL_R {
        PAD10FNCSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Pad 10 drive strength"]
    #[inline(always)]
    pub fn pad10strng(&self) -> PAD10STRNG_R {
        PAD10STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 10 input enable"]
    #[inline(always)]
    pub fn pad10inpen(&self) -> PAD10INPEN_R {
        PAD10INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 10 pullup enable"]
    #[inline(always)]
    pub fn pad10pull(&self) -> PAD10PULL_R {
        PAD10PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Pad 9 pullup resistor selection"]
    #[inline(always)]
    pub fn pad9rsel(&self) -> PAD9RSEL_R {
        PAD9RSEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 11:13 - Pad 9 function select"]
    #[inline(always)]
    pub fn pad9fncsel(&self) -> PAD9FNCSEL_R {
        PAD9FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 9 drive strength"]
    #[inline(always)]
    pub fn pad9strng(&self) -> PAD9STRNG_R {
        PAD9STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 9 input enable"]
    #[inline(always)]
    pub fn pad9inpen(&self) -> PAD9INPEN_R {
        PAD9INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 9 pullup enable"]
    #[inline(always)]
    pub fn pad9pull(&self) -> PAD9PULL_R {
        PAD9PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Pad 8 pullup resistor selection."]
    #[inline(always)]
    pub fn pad8rsel(&self) -> PAD8RSEL_R {
        PAD8RSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 3:5 - Pad 8 function select"]
    #[inline(always)]
    pub fn pad8fncsel(&self) -> PAD8FNCSEL_R {
        PAD8FNCSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Pad 8 drive strength"]
    #[inline(always)]
    pub fn pad8strng(&self) -> PAD8STRNG_R {
        PAD8STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 8 input enable"]
    #[inline(always)]
    pub fn pad8inpen(&self) -> PAD8INPEN_R {
        PAD8INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 8 pullup enable"]
    #[inline(always)]
    pub fn pad8pull(&self) -> PAD8PULL_R {
        PAD8PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Pad 11 lower power switch enable"]
    #[inline(always)]
    pub fn pad11pwrdn(&mut self) -> PAD11PWRDN_W {
        PAD11PWRDN_W { w: self }
    }
    #[doc = "Bits 27:28 - Pad 11 function select"]
    #[inline(always)]
    pub fn pad11fncsel(&mut self) -> PAD11FNCSEL_W {
        PAD11FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 11 drive strentgh"]
    #[inline(always)]
    pub fn pad11strng(&mut self) -> PAD11STRNG_W {
        PAD11STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 11 input enable"]
    #[inline(always)]
    pub fn pad11inpen(&mut self) -> PAD11INPEN_W {
        PAD11INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 11 pullup enable"]
    #[inline(always)]
    pub fn pad11pull(&mut self) -> PAD11PULL_W {
        PAD11PULL_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad 10 function select"]
    #[inline(always)]
    pub fn pad10fncsel(&mut self) -> PAD10FNCSEL_W {
        PAD10FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 10 drive strength"]
    #[inline(always)]
    pub fn pad10strng(&mut self) -> PAD10STRNG_W {
        PAD10STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 10 input enable"]
    #[inline(always)]
    pub fn pad10inpen(&mut self) -> PAD10INPEN_W {
        PAD10INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 10 pullup enable"]
    #[inline(always)]
    pub fn pad10pull(&mut self) -> PAD10PULL_W {
        PAD10PULL_W { w: self }
    }
    #[doc = "Bits 14:15 - Pad 9 pullup resistor selection"]
    #[inline(always)]
    pub fn pad9rsel(&mut self) -> PAD9RSEL_W {
        PAD9RSEL_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 9 function select"]
    #[inline(always)]
    pub fn pad9fncsel(&mut self) -> PAD9FNCSEL_W {
        PAD9FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 9 drive strength"]
    #[inline(always)]
    pub fn pad9strng(&mut self) -> PAD9STRNG_W {
        PAD9STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 9 input enable"]
    #[inline(always)]
    pub fn pad9inpen(&mut self) -> PAD9INPEN_W {
        PAD9INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 9 pullup enable"]
    #[inline(always)]
    pub fn pad9pull(&mut self) -> PAD9PULL_W {
        PAD9PULL_W { w: self }
    }
    #[doc = "Bits 6:7 - Pad 8 pullup resistor selection."]
    #[inline(always)]
    pub fn pad8rsel(&mut self) -> PAD8RSEL_W {
        PAD8RSEL_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad 8 function select"]
    #[inline(always)]
    pub fn pad8fncsel(&mut self) -> PAD8FNCSEL_W {
        PAD8FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 8 drive strength"]
    #[inline(always)]
    pub fn pad8strng(&mut self) -> PAD8STRNG_W {
        PAD8STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 8 input enable"]
    #[inline(always)]
    pub fn pad8inpen(&mut self) -> PAD8INPEN_W {
        PAD8INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 8 pullup enable"]
    #[inline(always)]
    pub fn pad8pull(&mut self) -> PAD8PULL_W {
        PAD8PULL_W { w: self }
    }
}
