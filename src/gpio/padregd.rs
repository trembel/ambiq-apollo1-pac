#[doc = "Reader of register PADREGD"]
pub type R = crate::R<u32, super::PADREGD>;
#[doc = "Writer for register PADREGD"]
pub type W = crate::W<u32, super::PADREGD>;
#[doc = "Register PADREGD `reset()`'s with value 0x1818_1818"]
impl crate::ResetValue for super::PADREGD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1818_1818
    }
}
#[doc = "Pad 15 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD15FNCSEL_A {
    #[doc = "0: Configure as the analog ADC input 3"]
    ADC3 = 0,
    #[doc = "1: Configure as the SPI channel 3 nCE signal from IOMSTR1"]
    M1NCE3 = 1,
    #[doc = "2: Configure as the UART RX signal"]
    UARTRX = 2,
    #[doc = "3: Configure as GPIO15"]
    GPIO15 = 3,
    #[doc = "5: Configure as the external XT clock signal"]
    EXTXT = 5,
    #[doc = "7: Pad disabled"]
    DIS = 7,
}
impl From<PAD15FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD15FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD15FNCSEL`"]
pub type PAD15FNCSEL_R = crate::R<u8, PAD15FNCSEL_A>;
impl PAD15FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD15FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD15FNCSEL_A::ADC3),
            1 => Val(PAD15FNCSEL_A::M1NCE3),
            2 => Val(PAD15FNCSEL_A::UARTRX),
            3 => Val(PAD15FNCSEL_A::GPIO15),
            5 => Val(PAD15FNCSEL_A::EXTXT),
            7 => Val(PAD15FNCSEL_A::DIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC3`"]
    #[inline(always)]
    pub fn is_adc3(&self) -> bool {
        *self == PAD15FNCSEL_A::ADC3
    }
    #[doc = "Checks if the value of the field is `M1NCE3`"]
    #[inline(always)]
    pub fn is_m1n_ce3(&self) -> bool {
        *self == PAD15FNCSEL_A::M1NCE3
    }
    #[doc = "Checks if the value of the field is `UARTRX`"]
    #[inline(always)]
    pub fn is_uartrx(&self) -> bool {
        *self == PAD15FNCSEL_A::UARTRX
    }
    #[doc = "Checks if the value of the field is `GPIO15`"]
    #[inline(always)]
    pub fn is_gpio15(&self) -> bool {
        *self == PAD15FNCSEL_A::GPIO15
    }
    #[doc = "Checks if the value of the field is `EXTXT`"]
    #[inline(always)]
    pub fn is_extxt(&self) -> bool {
        *self == PAD15FNCSEL_A::EXTXT
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD15FNCSEL_A::DIS
    }
}
#[doc = "Write proxy for field `PAD15FNCSEL`"]
pub struct PAD15FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD15FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD15FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the analog ADC input 3"]
    #[inline(always)]
    pub fn adc3(self) -> &'a mut W {
        self.variant(PAD15FNCSEL_A::ADC3)
    }
    #[doc = "Configure as the SPI channel 3 nCE signal from IOMSTR1"]
    #[inline(always)]
    pub fn m1n_ce3(self) -> &'a mut W {
        self.variant(PAD15FNCSEL_A::M1NCE3)
    }
    #[doc = "Configure as the UART RX signal"]
    #[inline(always)]
    pub fn uartrx(self) -> &'a mut W {
        self.variant(PAD15FNCSEL_A::UARTRX)
    }
    #[doc = "Configure as GPIO15"]
    #[inline(always)]
    pub fn gpio15(self) -> &'a mut W {
        self.variant(PAD15FNCSEL_A::GPIO15)
    }
    #[doc = "Configure as the external XT clock signal"]
    #[inline(always)]
    pub fn extxt(self) -> &'a mut W {
        self.variant(PAD15FNCSEL_A::EXTXT)
    }
    #[doc = "Pad disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD15FNCSEL_A::DIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
#[doc = "Pad 15 drive strentgh\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD15STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD15STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD15STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD15STRNG`"]
pub type PAD15STRNG_R = crate::R<bool, PAD15STRNG_A>;
impl PAD15STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD15STRNG_A {
        match self.bits {
            false => PAD15STRNG_A::LOW,
            true => PAD15STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD15STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD15STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD15STRNG`"]
pub struct PAD15STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD15STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD15STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD15STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD15STRNG_A::HIGH)
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
#[doc = "Pad 15 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD15INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD15INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD15INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD15INPEN`"]
pub type PAD15INPEN_R = crate::R<bool, PAD15INPEN_A>;
impl PAD15INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD15INPEN_A {
        match self.bits {
            false => PAD15INPEN_A::DIS,
            true => PAD15INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD15INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD15INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD15INPEN`"]
pub struct PAD15INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD15INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD15INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD15INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD15INPEN_A::EN)
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
#[doc = "Pad 15 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD15PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD15PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD15PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD15PULL`"]
pub type PAD15PULL_R = crate::R<bool, PAD15PULL_A>;
impl PAD15PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD15PULL_A {
        match self.bits {
            false => PAD15PULL_A::DIS,
            true => PAD15PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD15PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD15PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD15PULL`"]
pub struct PAD15PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD15PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD15PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD15PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD15PULL_A::EN)
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
#[doc = "Pad 14 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD14FNCSEL_A {
    #[doc = "0: Configure as the analog ADC input 2"]
    ADC2 = 0,
    #[doc = "1: Configure as the SPI channel 2 nCE signal from IOMSTR1"]
    M1NCE2 = 1,
    #[doc = "2: Configure as the UART TX signal"]
    UARTTX = 2,
    #[doc = "3: Configure as GPIO14"]
    GPIO14 = 3,
    #[doc = "5: Configure as the external HFRC select signal"]
    EXTHFS = 5,
    #[doc = "7: Pad disabled"]
    DIS = 7,
}
impl From<PAD14FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD14FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD14FNCSEL`"]
pub type PAD14FNCSEL_R = crate::R<u8, PAD14FNCSEL_A>;
impl PAD14FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD14FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD14FNCSEL_A::ADC2),
            1 => Val(PAD14FNCSEL_A::M1NCE2),
            2 => Val(PAD14FNCSEL_A::UARTTX),
            3 => Val(PAD14FNCSEL_A::GPIO14),
            5 => Val(PAD14FNCSEL_A::EXTHFS),
            7 => Val(PAD14FNCSEL_A::DIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC2`"]
    #[inline(always)]
    pub fn is_adc2(&self) -> bool {
        *self == PAD14FNCSEL_A::ADC2
    }
    #[doc = "Checks if the value of the field is `M1NCE2`"]
    #[inline(always)]
    pub fn is_m1n_ce2(&self) -> bool {
        *self == PAD14FNCSEL_A::M1NCE2
    }
    #[doc = "Checks if the value of the field is `UARTTX`"]
    #[inline(always)]
    pub fn is_uarttx(&self) -> bool {
        *self == PAD14FNCSEL_A::UARTTX
    }
    #[doc = "Checks if the value of the field is `GPIO14`"]
    #[inline(always)]
    pub fn is_gpio14(&self) -> bool {
        *self == PAD14FNCSEL_A::GPIO14
    }
    #[doc = "Checks if the value of the field is `EXTHFS`"]
    #[inline(always)]
    pub fn is_exthfs(&self) -> bool {
        *self == PAD14FNCSEL_A::EXTHFS
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD14FNCSEL_A::DIS
    }
}
#[doc = "Write proxy for field `PAD14FNCSEL`"]
pub struct PAD14FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD14FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD14FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the analog ADC input 2"]
    #[inline(always)]
    pub fn adc2(self) -> &'a mut W {
        self.variant(PAD14FNCSEL_A::ADC2)
    }
    #[doc = "Configure as the SPI channel 2 nCE signal from IOMSTR1"]
    #[inline(always)]
    pub fn m1n_ce2(self) -> &'a mut W {
        self.variant(PAD14FNCSEL_A::M1NCE2)
    }
    #[doc = "Configure as the UART TX signal"]
    #[inline(always)]
    pub fn uarttx(self) -> &'a mut W {
        self.variant(PAD14FNCSEL_A::UARTTX)
    }
    #[doc = "Configure as GPIO14"]
    #[inline(always)]
    pub fn gpio14(self) -> &'a mut W {
        self.variant(PAD14FNCSEL_A::GPIO14)
    }
    #[doc = "Configure as the external HFRC select signal"]
    #[inline(always)]
    pub fn exthfs(self) -> &'a mut W {
        self.variant(PAD14FNCSEL_A::EXTHFS)
    }
    #[doc = "Pad disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD14FNCSEL_A::DIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad 14 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD14STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD14STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD14STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD14STRNG`"]
pub type PAD14STRNG_R = crate::R<bool, PAD14STRNG_A>;
impl PAD14STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD14STRNG_A {
        match self.bits {
            false => PAD14STRNG_A::LOW,
            true => PAD14STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD14STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD14STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD14STRNG`"]
pub struct PAD14STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD14STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD14STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD14STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD14STRNG_A::HIGH)
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
#[doc = "Pad 14 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD14INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD14INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD14INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD14INPEN`"]
pub type PAD14INPEN_R = crate::R<bool, PAD14INPEN_A>;
impl PAD14INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD14INPEN_A {
        match self.bits {
            false => PAD14INPEN_A::DIS,
            true => PAD14INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD14INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD14INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD14INPEN`"]
pub struct PAD14INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD14INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD14INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD14INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD14INPEN_A::EN)
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
#[doc = "Pad 14 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD14PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD14PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD14PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD14PULL`"]
pub type PAD14PULL_R = crate::R<bool, PAD14PULL_A>;
impl PAD14PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD14PULL_A {
        match self.bits {
            false => PAD14PULL_A::DIS,
            true => PAD14PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD14PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD14PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD14PULL`"]
pub struct PAD14PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD14PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD14PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD14PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD14PULL_A::EN)
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
#[doc = "Pad 13 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD13FNCSEL_A {
    #[doc = "0: Configure as the analog ADC input 1"]
    ADC1 = 0,
    #[doc = "1: Configure as the SPI channel 1 nCE signal from IOMSTR1"]
    M1NCE1 = 1,
    #[doc = "2: Configure as the input/output signal from CTIMER B0"]
    TCTB0 = 2,
    #[doc = "3: Configure as GPIO13"]
    GPIO13 = 3,
    #[doc = "5: Configure as the external HFRC B clock signal"]
    EXTHFA = 5,
    #[doc = "6: Configure as the serial wire debug SWO signal"]
    SWO = 6,
    #[doc = "7: Pad disabled"]
    DIS = 7,
}
impl From<PAD13FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD13FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD13FNCSEL`"]
pub type PAD13FNCSEL_R = crate::R<u8, PAD13FNCSEL_A>;
impl PAD13FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD13FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD13FNCSEL_A::ADC1),
            1 => Val(PAD13FNCSEL_A::M1NCE1),
            2 => Val(PAD13FNCSEL_A::TCTB0),
            3 => Val(PAD13FNCSEL_A::GPIO13),
            5 => Val(PAD13FNCSEL_A::EXTHFA),
            6 => Val(PAD13FNCSEL_A::SWO),
            7 => Val(PAD13FNCSEL_A::DIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC1`"]
    #[inline(always)]
    pub fn is_adc1(&self) -> bool {
        *self == PAD13FNCSEL_A::ADC1
    }
    #[doc = "Checks if the value of the field is `M1NCE1`"]
    #[inline(always)]
    pub fn is_m1n_ce1(&self) -> bool {
        *self == PAD13FNCSEL_A::M1NCE1
    }
    #[doc = "Checks if the value of the field is `TCTB0`"]
    #[inline(always)]
    pub fn is_tctb0(&self) -> bool {
        *self == PAD13FNCSEL_A::TCTB0
    }
    #[doc = "Checks if the value of the field is `GPIO13`"]
    #[inline(always)]
    pub fn is_gpio13(&self) -> bool {
        *self == PAD13FNCSEL_A::GPIO13
    }
    #[doc = "Checks if the value of the field is `EXTHFA`"]
    #[inline(always)]
    pub fn is_exthfa(&self) -> bool {
        *self == PAD13FNCSEL_A::EXTHFA
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline(always)]
    pub fn is_swo(&self) -> bool {
        *self == PAD13FNCSEL_A::SWO
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD13FNCSEL_A::DIS
    }
}
#[doc = "Write proxy for field `PAD13FNCSEL`"]
pub struct PAD13FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD13FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD13FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the analog ADC input 1"]
    #[inline(always)]
    pub fn adc1(self) -> &'a mut W {
        self.variant(PAD13FNCSEL_A::ADC1)
    }
    #[doc = "Configure as the SPI channel 1 nCE signal from IOMSTR1"]
    #[inline(always)]
    pub fn m1n_ce1(self) -> &'a mut W {
        self.variant(PAD13FNCSEL_A::M1NCE1)
    }
    #[doc = "Configure as the input/output signal from CTIMER B0"]
    #[inline(always)]
    pub fn tctb0(self) -> &'a mut W {
        self.variant(PAD13FNCSEL_A::TCTB0)
    }
    #[doc = "Configure as GPIO13"]
    #[inline(always)]
    pub fn gpio13(self) -> &'a mut W {
        self.variant(PAD13FNCSEL_A::GPIO13)
    }
    #[doc = "Configure as the external HFRC B clock signal"]
    #[inline(always)]
    pub fn exthfa(self) -> &'a mut W {
        self.variant(PAD13FNCSEL_A::EXTHFA)
    }
    #[doc = "Configure as the serial wire debug SWO signal"]
    #[inline(always)]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD13FNCSEL_A::SWO)
    }
    #[doc = "Pad disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD13FNCSEL_A::DIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 13 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD13STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD13STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD13STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD13STRNG`"]
pub type PAD13STRNG_R = crate::R<bool, PAD13STRNG_A>;
impl PAD13STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD13STRNG_A {
        match self.bits {
            false => PAD13STRNG_A::LOW,
            true => PAD13STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD13STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD13STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD13STRNG`"]
pub struct PAD13STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD13STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD13STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD13STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD13STRNG_A::HIGH)
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
#[doc = "Pad 13 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD13INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD13INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD13INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD13INPEN`"]
pub type PAD13INPEN_R = crate::R<bool, PAD13INPEN_A>;
impl PAD13INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD13INPEN_A {
        match self.bits {
            false => PAD13INPEN_A::DIS,
            true => PAD13INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD13INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD13INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD13INPEN`"]
pub struct PAD13INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD13INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD13INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD13INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD13INPEN_A::EN)
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
#[doc = "Pad 13 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD13PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD13PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD13PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD13PULL`"]
pub type PAD13PULL_R = crate::R<bool, PAD13PULL_A>;
impl PAD13PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD13PULL_A {
        match self.bits {
            false => PAD13PULL_A::DIS,
            true => PAD13PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD13PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD13PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD13PULL`"]
pub struct PAD13PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD13PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD13PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD13PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD13PULL_A::EN)
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
#[doc = "Pad 12 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD12FNCSEL_A {
    #[doc = "0: Configure as the analog ADC input 0"]
    ADC0 = 0,
    #[doc = "1: Configure as the SPI channel 0 nCE signal from IOMSTR1"]
    M1NCE0 = 1,
    #[doc = "2: Configure as the input/output signal from CTIMER A0"]
    TCTA0 = 2,
    #[doc = "3: Configure as GPIO12"]
    GPIO12 = 3,
}
impl From<PAD12FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD12FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD12FNCSEL`"]
pub type PAD12FNCSEL_R = crate::R<u8, PAD12FNCSEL_A>;
impl PAD12FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD12FNCSEL_A {
        match self.bits {
            0 => PAD12FNCSEL_A::ADC0,
            1 => PAD12FNCSEL_A::M1NCE0,
            2 => PAD12FNCSEL_A::TCTA0,
            3 => PAD12FNCSEL_A::GPIO12,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == PAD12FNCSEL_A::ADC0
    }
    #[doc = "Checks if the value of the field is `M1NCE0`"]
    #[inline(always)]
    pub fn is_m1n_ce0(&self) -> bool {
        *self == PAD12FNCSEL_A::M1NCE0
    }
    #[doc = "Checks if the value of the field is `TCTA0`"]
    #[inline(always)]
    pub fn is_tcta0(&self) -> bool {
        *self == PAD12FNCSEL_A::TCTA0
    }
    #[doc = "Checks if the value of the field is `GPIO12`"]
    #[inline(always)]
    pub fn is_gpio12(&self) -> bool {
        *self == PAD12FNCSEL_A::GPIO12
    }
}
#[doc = "Write proxy for field `PAD12FNCSEL`"]
pub struct PAD12FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD12FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD12FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the analog ADC input 0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut W {
        self.variant(PAD12FNCSEL_A::ADC0)
    }
    #[doc = "Configure as the SPI channel 0 nCE signal from IOMSTR1"]
    #[inline(always)]
    pub fn m1n_ce0(self) -> &'a mut W {
        self.variant(PAD12FNCSEL_A::M1NCE0)
    }
    #[doc = "Configure as the input/output signal from CTIMER A0"]
    #[inline(always)]
    pub fn tcta0(self) -> &'a mut W {
        self.variant(PAD12FNCSEL_A::TCTA0)
    }
    #[doc = "Configure as GPIO12"]
    #[inline(always)]
    pub fn gpio12(self) -> &'a mut W {
        self.variant(PAD12FNCSEL_A::GPIO12)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Pad 12 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD12STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD12STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD12STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD12STRNG`"]
pub type PAD12STRNG_R = crate::R<bool, PAD12STRNG_A>;
impl PAD12STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD12STRNG_A {
        match self.bits {
            false => PAD12STRNG_A::LOW,
            true => PAD12STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD12STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD12STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD12STRNG`"]
pub struct PAD12STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD12STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD12STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD12STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD12STRNG_A::HIGH)
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
#[doc = "Pad 12 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD12INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD12INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD12INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD12INPEN`"]
pub type PAD12INPEN_R = crate::R<bool, PAD12INPEN_A>;
impl PAD12INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD12INPEN_A {
        match self.bits {
            false => PAD12INPEN_A::DIS,
            true => PAD12INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD12INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD12INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD12INPEN`"]
pub struct PAD12INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD12INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD12INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD12INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD12INPEN_A::EN)
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
#[doc = "Pad 12 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD12PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD12PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD12PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD12PULL`"]
pub type PAD12PULL_R = crate::R<bool, PAD12PULL_A>;
impl PAD12PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD12PULL_A {
        match self.bits {
            false => PAD12PULL_A::DIS,
            true => PAD12PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD12PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD12PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD12PULL`"]
pub struct PAD12PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD12PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD12PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD12PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD12PULL_A::EN)
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
    #[doc = "Bits 27:29 - Pad 15 function select"]
    #[inline(always)]
    pub fn pad15fncsel(&self) -> PAD15FNCSEL_R {
        PAD15FNCSEL_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Pad 15 drive strentgh"]
    #[inline(always)]
    pub fn pad15strng(&self) -> PAD15STRNG_R {
        PAD15STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 15 input enable"]
    #[inline(always)]
    pub fn pad15inpen(&self) -> PAD15INPEN_R {
        PAD15INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 15 pullup enable"]
    #[inline(always)]
    pub fn pad15pull(&self) -> PAD15PULL_R {
        PAD15PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - Pad 14 function select"]
    #[inline(always)]
    pub fn pad14fncsel(&self) -> PAD14FNCSEL_R {
        PAD14FNCSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Pad 14 drive strength"]
    #[inline(always)]
    pub fn pad14strng(&self) -> PAD14STRNG_R {
        PAD14STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 14 input enable"]
    #[inline(always)]
    pub fn pad14inpen(&self) -> PAD14INPEN_R {
        PAD14INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 14 pullup enable"]
    #[inline(always)]
    pub fn pad14pull(&self) -> PAD14PULL_R {
        PAD14PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - Pad 13 function select"]
    #[inline(always)]
    pub fn pad13fncsel(&self) -> PAD13FNCSEL_R {
        PAD13FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 13 drive strength"]
    #[inline(always)]
    pub fn pad13strng(&self) -> PAD13STRNG_R {
        PAD13STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 13 input enable"]
    #[inline(always)]
    pub fn pad13inpen(&self) -> PAD13INPEN_R {
        PAD13INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 13 pullup enable"]
    #[inline(always)]
    pub fn pad13pull(&self) -> PAD13PULL_R {
        PAD13PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Pad 12 function select"]
    #[inline(always)]
    pub fn pad12fncsel(&self) -> PAD12FNCSEL_R {
        PAD12FNCSEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2 - Pad 12 drive strength"]
    #[inline(always)]
    pub fn pad12strng(&self) -> PAD12STRNG_R {
        PAD12STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 12 input enable"]
    #[inline(always)]
    pub fn pad12inpen(&self) -> PAD12INPEN_R {
        PAD12INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 12 pullup enable"]
    #[inline(always)]
    pub fn pad12pull(&self) -> PAD12PULL_R {
        PAD12PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:29 - Pad 15 function select"]
    #[inline(always)]
    pub fn pad15fncsel(&mut self) -> PAD15FNCSEL_W {
        PAD15FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 15 drive strentgh"]
    #[inline(always)]
    pub fn pad15strng(&mut self) -> PAD15STRNG_W {
        PAD15STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 15 input enable"]
    #[inline(always)]
    pub fn pad15inpen(&mut self) -> PAD15INPEN_W {
        PAD15INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 15 pullup enable"]
    #[inline(always)]
    pub fn pad15pull(&mut self) -> PAD15PULL_W {
        PAD15PULL_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad 14 function select"]
    #[inline(always)]
    pub fn pad14fncsel(&mut self) -> PAD14FNCSEL_W {
        PAD14FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 14 drive strength"]
    #[inline(always)]
    pub fn pad14strng(&mut self) -> PAD14STRNG_W {
        PAD14STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 14 input enable"]
    #[inline(always)]
    pub fn pad14inpen(&mut self) -> PAD14INPEN_W {
        PAD14INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 14 pullup enable"]
    #[inline(always)]
    pub fn pad14pull(&mut self) -> PAD14PULL_W {
        PAD14PULL_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 13 function select"]
    #[inline(always)]
    pub fn pad13fncsel(&mut self) -> PAD13FNCSEL_W {
        PAD13FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 13 drive strength"]
    #[inline(always)]
    pub fn pad13strng(&mut self) -> PAD13STRNG_W {
        PAD13STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 13 input enable"]
    #[inline(always)]
    pub fn pad13inpen(&mut self) -> PAD13INPEN_W {
        PAD13INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 13 pullup enable"]
    #[inline(always)]
    pub fn pad13pull(&mut self) -> PAD13PULL_W {
        PAD13PULL_W { w: self }
    }
    #[doc = "Bits 3:4 - Pad 12 function select"]
    #[inline(always)]
    pub fn pad12fncsel(&mut self) -> PAD12FNCSEL_W {
        PAD12FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 12 drive strength"]
    #[inline(always)]
    pub fn pad12strng(&mut self) -> PAD12STRNG_W {
        PAD12STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 12 input enable"]
    #[inline(always)]
    pub fn pad12inpen(&mut self) -> PAD12INPEN_W {
        PAD12INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 12 pullup enable"]
    #[inline(always)]
    pub fn pad12pull(&mut self) -> PAD12PULL_W {
        PAD12PULL_W { w: self }
    }
}
