#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EVCTRL_SPEC>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EVCTRL_SPEC>;
#[doc = "Field `EVACT` reader - Event Action"]
pub type EVACT_R = crate::FieldReader<EVACTSELECT_A>;
#[doc = "Event Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EVACTSELECT_A {
    #[doc = "0: Event action disabled"]
    OFF = 0,
    #[doc = "1: Start, restart or retrigger TC on event"]
    RETRIGGER = 1,
    #[doc = "2: Count on event"]
    COUNT = 2,
    #[doc = "3: Start TC on event"]
    START = 3,
    #[doc = "5: Period captured into CC0 Pulse Width in CC1"]
    PPW = 5,
    #[doc = "6: Period captured into CC1 Pulse Width on CC0"]
    PWP = 6,
}
impl From<EVACTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: EVACTSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EVACTSELECT_A {
    type Ux = u8;
}
impl EVACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EVACTSELECT_A> {
        match self.bits {
            0 => Some(EVACTSELECT_A::OFF),
            1 => Some(EVACTSELECT_A::RETRIGGER),
            2 => Some(EVACTSELECT_A::COUNT),
            3 => Some(EVACTSELECT_A::START),
            5 => Some(EVACTSELECT_A::PPW),
            6 => Some(EVACTSELECT_A::PWP),
            _ => None,
        }
    }
    #[doc = "Event action disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == EVACTSELECT_A::OFF
    }
    #[doc = "Start, restart or retrigger TC on event"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        *self == EVACTSELECT_A::RETRIGGER
    }
    #[doc = "Count on event"]
    #[inline(always)]
    pub fn is_count(&self) -> bool {
        *self == EVACTSELECT_A::COUNT
    }
    #[doc = "Start TC on event"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == EVACTSELECT_A::START
    }
    #[doc = "Period captured into CC0 Pulse Width in CC1"]
    #[inline(always)]
    pub fn is_ppw(&self) -> bool {
        *self == EVACTSELECT_A::PPW
    }
    #[doc = "Period captured into CC1 Pulse Width on CC0"]
    #[inline(always)]
    pub fn is_pwp(&self) -> bool {
        *self == EVACTSELECT_A::PWP
    }
}
#[doc = "Field `EVACT` writer - Event Action"]
pub type EVACT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, EVACTSELECT_A>;
impl<'a, REG, const O: u8> EVACT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Event action disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::OFF)
    }
    #[doc = "Start, restart or retrigger TC on event"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::RETRIGGER)
    }
    #[doc = "Count on event"]
    #[inline(always)]
    pub fn count(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::COUNT)
    }
    #[doc = "Start TC on event"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::START)
    }
    #[doc = "Period captured into CC0 Pulse Width in CC1"]
    #[inline(always)]
    pub fn ppw(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::PPW)
    }
    #[doc = "Period captured into CC1 Pulse Width on CC0"]
    #[inline(always)]
    pub fn pwp(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::PWP)
    }
}
#[doc = "Field `TCINV` reader - TC Inverted Event Input"]
pub type TCINV_R = crate::BitReader;
#[doc = "Field `TCINV` writer - TC Inverted Event Input"]
pub type TCINV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCEI` reader - TC Event Input"]
pub type TCEI_R = crate::BitReader;
#[doc = "Field `TCEI` writer - TC Event Input"]
pub type TCEI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVFEO` reader - Overflow/Underflow Event Output Enable"]
pub type OVFEO_R = crate::BitReader;
#[doc = "Field `OVFEO` writer - Overflow/Underflow Event Output Enable"]
pub type OVFEO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MCEO0` reader - Match or Capture Channel 0 Event Output Enable"]
pub type MCEO0_R = crate::BitReader;
#[doc = "Field `MCEO0` writer - Match or Capture Channel 0 Event Output Enable"]
pub type MCEO0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MCEO1` reader - Match or Capture Channel 1 Event Output Enable"]
pub type MCEO1_R = crate::BitReader;
#[doc = "Field `MCEO1` writer - Match or Capture Channel 1 Event Output Enable"]
pub type MCEO1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - Event Action"]
    #[inline(always)]
    pub fn evact(&self) -> EVACT_R {
        EVACT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - TC Inverted Event Input"]
    #[inline(always)]
    pub fn tcinv(&self) -> TCINV_R {
        TCINV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TC Event Input"]
    #[inline(always)]
    pub fn tcei(&self) -> TCEI_R {
        TCEI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Overflow/Underflow Event Output Enable"]
    #[inline(always)]
    pub fn ovfeo(&self) -> OVFEO_R {
        OVFEO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Match or Capture Channel 0 Event Output Enable"]
    #[inline(always)]
    pub fn mceo0(&self) -> MCEO0_R {
        MCEO0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Match or Capture Channel 1 Event Output Enable"]
    #[inline(always)]
    pub fn mceo1(&self) -> MCEO1_R {
        MCEO1_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Event Action"]
    #[inline(always)]
    #[must_use]
    pub fn evact(&mut self) -> EVACT_W<EVCTRL_SPEC, 0> {
        EVACT_W::new(self)
    }
    #[doc = "Bit 4 - TC Inverted Event Input"]
    #[inline(always)]
    #[must_use]
    pub fn tcinv(&mut self) -> TCINV_W<EVCTRL_SPEC, 4> {
        TCINV_W::new(self)
    }
    #[doc = "Bit 5 - TC Event Input"]
    #[inline(always)]
    #[must_use]
    pub fn tcei(&mut self) -> TCEI_W<EVCTRL_SPEC, 5> {
        TCEI_W::new(self)
    }
    #[doc = "Bit 8 - Overflow/Underflow Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovfeo(&mut self) -> OVFEO_W<EVCTRL_SPEC, 8> {
        OVFEO_W::new(self)
    }
    #[doc = "Bit 12 - Match or Capture Channel 0 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mceo0(&mut self) -> MCEO0_W<EVCTRL_SPEC, 12> {
        MCEO0_W::new(self)
    }
    #[doc = "Bit 13 - Match or Capture Channel 1 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mceo1(&mut self) -> MCEO1_W<EVCTRL_SPEC, 13> {
        MCEO1_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Event Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVCTRL_SPEC;
impl crate::RegisterSpec for EVCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EVCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EVCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EVCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
