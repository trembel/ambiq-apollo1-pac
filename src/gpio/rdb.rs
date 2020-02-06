#[doc = "Reader of register RDB"]
pub type R = crate::R<u32, super::RDB>;
#[doc = "Writer for register RDB"]
pub type W = crate::W<u32, super::RDB>;
#[doc = "Register RDB `reset()`'s with value 0"]
impl crate::ResetValue for super::RDB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDB`"]
pub type RDB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RDB`"]
pub struct RDB_W<'a> {
    w: &'a mut W,
}
impl<'a> RDB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - GPIO49-32 read data."]
    #[inline(always)]
    pub fn rdb(&self) -> RDB_R {
        RDB_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - GPIO49-32 read data."]
    #[inline(always)]
    pub fn rdb(&mut self) -> RDB_W {
        RDB_W { w: self }
    }
}
