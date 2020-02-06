#[doc = "Reader of register WTB"]
pub type R = crate::R<u32, super::WTB>;
#[doc = "Writer for register WTB"]
pub type W = crate::W<u32, super::WTB>;
#[doc = "Register WTB `reset()`'s with value 0"]
impl crate::ResetValue for super::WTB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WTB`"]
pub type WTB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WTB`"]
pub struct WTB_W<'a> {
    w: &'a mut W,
}
impl<'a> WTB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - GPIO49-32 write data."]
    #[inline(always)]
    pub fn wtb(&self) -> WTB_R {
        WTB_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - GPIO49-32 write data."]
    #[inline(always)]
    pub fn wtb(&mut self) -> WTB_W {
        WTB_W { w: self }
    }
}
