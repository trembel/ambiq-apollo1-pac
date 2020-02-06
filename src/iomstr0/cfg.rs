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
#[doc = "This bit enables the IO Master.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFCEN_A {
    #[doc = "0: Disable the IO Master."]
    DIS = 0,
    #[doc = "1: Enable the IO Master."]
    EN = 1,
}
impl From<IFCEN_A> for bool {
    #[inline(always)]
    fn from(variant: IFCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IFCEN`"]
pub type IFCEN_R = crate::R<bool, IFCEN_A>;
impl IFCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFCEN_A {
        match self.bits {
            false => IFCEN_A::DIS,
            true => IFCEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == IFCEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == IFCEN_A::EN
    }
}
#[doc = "Write proxy for field `IFCEN`"]
pub struct IFCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IFCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IFCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the IO Master."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(IFCEN_A::DIS)
    }
    #[doc = "Enable the IO Master."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(IFCEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "This bit selects SPI phase.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPHA_A {
    #[doc = "0: Sample on the leading (first) clock edge."]
    SAMPLE_LEADING_EDGE = 0,
    #[doc = "1: Sample on the trailing (second) clock edge."]
    SAMPLE_TRAILING_EDGE = 1,
}
impl From<SPHA_A> for bool {
    #[inline(always)]
    fn from(variant: SPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPHA`"]
pub type SPHA_R = crate::R<bool, SPHA_A>;
impl SPHA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPHA_A {
        match self.bits {
            false => SPHA_A::SAMPLE_LEADING_EDGE,
            true => SPHA_A::SAMPLE_TRAILING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `SAMPLE_LEADING_EDGE`"]
    #[inline(always)]
    pub fn is_sample_leading_edge(&self) -> bool {
        *self == SPHA_A::SAMPLE_LEADING_EDGE
    }
    #[doc = "Checks if the value of the field is `SAMPLE_TRAILING_EDGE`"]
    #[inline(always)]
    pub fn is_sample_trailing_edge(&self) -> bool {
        *self == SPHA_A::SAMPLE_TRAILING_EDGE
    }
}
#[doc = "Write proxy for field `SPHA`"]
pub struct SPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPHA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sample on the leading (first) clock edge."]
    #[inline(always)]
    pub fn sample_leading_edge(self) -> &'a mut W {
        self.variant(SPHA_A::SAMPLE_LEADING_EDGE)
    }
    #[doc = "Sample on the trailing (second) clock edge."]
    #[inline(always)]
    pub fn sample_trailing_edge(self) -> &'a mut W {
        self.variant(SPHA_A::SAMPLE_TRAILING_EDGE)
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
#[doc = "This bit selects SPI polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOL_A {
    #[doc = "0: The base value of the clock is 0."]
    CLK_BASE_0 = 0,
    #[doc = "1: The base value of the clock is 1."]
    CLK_BASE_1 = 1,
}
impl From<SPOL_A> for bool {
    #[inline(always)]
    fn from(variant: SPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPOL`"]
pub type SPOL_R = crate::R<bool, SPOL_A>;
impl SPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPOL_A {
        match self.bits {
            false => SPOL_A::CLK_BASE_0,
            true => SPOL_A::CLK_BASE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_BASE_0`"]
    #[inline(always)]
    pub fn is_clk_base_0(&self) -> bool {
        *self == SPOL_A::CLK_BASE_0
    }
    #[doc = "Checks if the value of the field is `CLK_BASE_1`"]
    #[inline(always)]
    pub fn is_clk_base_1(&self) -> bool {
        *self == SPOL_A::CLK_BASE_1
    }
}
#[doc = "Write proxy for field `SPOL`"]
pub struct SPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The base value of the clock is 0."]
    #[inline(always)]
    pub fn clk_base_0(self) -> &'a mut W {
        self.variant(SPOL_A::CLK_BASE_0)
    }
    #[doc = "The base value of the clock is 1."]
    #[inline(always)]
    pub fn clk_base_1(self) -> &'a mut W {
        self.variant(SPOL_A::CLK_BASE_1)
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
#[doc = "This bit selects the I/O interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFCSEL_A {
    #[doc = "0: Selects I2C interface for the I/O Master."]
    I2C = 0,
    #[doc = "1: Selects SPI interface for the I/O Master."]
    SPI = 1,
}
impl From<IFCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: IFCSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IFCSEL`"]
pub type IFCSEL_R = crate::R<bool, IFCSEL_A>;
impl IFCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFCSEL_A {
        match self.bits {
            false => IFCSEL_A::I2C,
            true => IFCSEL_A::SPI,
        }
    }
    #[doc = "Checks if the value of the field is `I2C`"]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == IFCSEL_A::I2C
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == IFCSEL_A::SPI
    }
}
#[doc = "Write proxy for field `IFCSEL`"]
pub struct IFCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IFCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IFCSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selects I2C interface for the I/O Master."]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut W {
        self.variant(IFCSEL_A::I2C)
    }
    #[doc = "Selects SPI interface for the I/O Master."]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(IFCSEL_A::SPI)
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
    #[doc = "Bit 31 - This bit enables the IO Master."]
    #[inline(always)]
    pub fn ifcen(&self) -> IFCEN_R {
        IFCEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit selects SPI phase."]
    #[inline(always)]
    pub fn spha(&self) -> SPHA_R {
        SPHA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit selects SPI polarity."]
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit selects the I/O interface."]
    #[inline(always)]
    pub fn ifcsel(&self) -> IFCSEL_R {
        IFCSEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - This bit enables the IO Master."]
    #[inline(always)]
    pub fn ifcen(&mut self) -> IFCEN_W {
        IFCEN_W { w: self }
    }
    #[doc = "Bit 2 - This bit selects SPI phase."]
    #[inline(always)]
    pub fn spha(&mut self) -> SPHA_W {
        SPHA_W { w: self }
    }
    #[doc = "Bit 1 - This bit selects SPI polarity."]
    #[inline(always)]
    pub fn spol(&mut self) -> SPOL_W {
        SPOL_W { w: self }
    }
    #[doc = "Bit 0 - This bit selects the I/O interface."]
    #[inline(always)]
    pub fn ifcsel(&mut self) -> IFCSEL_W {
        IFCSEL_W { w: self }
    }
}
