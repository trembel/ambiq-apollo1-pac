#[doc = "Reader of register ENB"]
pub type R = crate::R<u32, super::ENB>;
#[doc = "Writer for register ENB"]
pub type W = crate::W<u32, super::ENB>;
#[doc = "Register ENB `reset()`'s with value 0"]
impl crate::ResetValue for super::ENB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENB`"]
pub type ENB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ENB`"]
pub struct ENB_W<'a> {
    w: &'a mut W,
}
impl<'a> ENB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - GPIO49-32 output enables"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - GPIO49-32 output enables"]
    #[inline(always)]
    pub fn enb(&mut self) -> ENB_W {
        ENB_W { w: self }
    }
}
