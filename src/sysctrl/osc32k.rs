#[doc = "Register `OSC32K` reader"]
pub type R = crate::R<OSC32K_SPEC>;
#[doc = "Register `OSC32K` writer"]
pub type W = crate::W<OSC32K_SPEC>;
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN32K` reader - 32kHz Output Enable"]
pub type EN32K_R = crate::BitReader;
#[doc = "Field `EN32K` writer - 32kHz Output Enable"]
pub type EN32K_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN1K` reader - 1kHz Output Enable"]
pub type EN1K_R = crate::BitReader;
#[doc = "Field `EN1K` writer - 1kHz Output Enable"]
pub type EN1K_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type ONDEMAND_R = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type ONDEMAND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STARTUP` reader - Oscillator Start-Up Time"]
pub type STARTUP_R = crate::FieldReader<STARTUPSELECT_A>;
#[doc = "Oscillator Start-Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STARTUPSELECT_A {
    #[doc = "0: 0.092 ms"]
    CYCLE3 = 0,
    #[doc = "1: 0.122 ms"]
    CYCLE4 = 1,
    #[doc = "2: 0.183 ms"]
    CYCLE6 = 2,
    #[doc = "3: 0.305 ms"]
    CYCLE10 = 3,
    #[doc = "4: 0.549 ms"]
    CYCLE18 = 4,
    #[doc = "5: 1.038 ms"]
    CYCLE34 = 5,
    #[doc = "6: 2.014 ms"]
    CYCLE66 = 6,
    #[doc = "7: 3.967 ms"]
    CYCLE130 = 7,
}
impl From<STARTUPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: STARTUPSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STARTUPSELECT_A {
    type Ux = u8;
}
impl STARTUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STARTUPSELECT_A {
        match self.bits {
            0 => STARTUPSELECT_A::CYCLE3,
            1 => STARTUPSELECT_A::CYCLE4,
            2 => STARTUPSELECT_A::CYCLE6,
            3 => STARTUPSELECT_A::CYCLE10,
            4 => STARTUPSELECT_A::CYCLE18,
            5 => STARTUPSELECT_A::CYCLE34,
            6 => STARTUPSELECT_A::CYCLE66,
            7 => STARTUPSELECT_A::CYCLE130,
            _ => unreachable!(),
        }
    }
    #[doc = "0.092 ms"]
    #[inline(always)]
    pub fn is_cycle3(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE3
    }
    #[doc = "0.122 ms"]
    #[inline(always)]
    pub fn is_cycle4(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE4
    }
    #[doc = "0.183 ms"]
    #[inline(always)]
    pub fn is_cycle6(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE6
    }
    #[doc = "0.305 ms"]
    #[inline(always)]
    pub fn is_cycle10(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE10
    }
    #[doc = "0.549 ms"]
    #[inline(always)]
    pub fn is_cycle18(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE18
    }
    #[doc = "1.038 ms"]
    #[inline(always)]
    pub fn is_cycle34(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE34
    }
    #[doc = "2.014 ms"]
    #[inline(always)]
    pub fn is_cycle66(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE66
    }
    #[doc = "3.967 ms"]
    #[inline(always)]
    pub fn is_cycle130(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE130
    }
}
#[doc = "Field `STARTUP` writer - Oscillator Start-Up Time"]
pub type STARTUP_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, STARTUPSELECT_A>;
impl<'a, REG, const O: u8> STARTUP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.092 ms"]
    #[inline(always)]
    pub fn cycle3(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE3)
    }
    #[doc = "0.122 ms"]
    #[inline(always)]
    pub fn cycle4(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE4)
    }
    #[doc = "0.183 ms"]
    #[inline(always)]
    pub fn cycle6(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE6)
    }
    #[doc = "0.305 ms"]
    #[inline(always)]
    pub fn cycle10(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE10)
    }
    #[doc = "0.549 ms"]
    #[inline(always)]
    pub fn cycle18(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE18)
    }
    #[doc = "1.038 ms"]
    #[inline(always)]
    pub fn cycle34(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE34)
    }
    #[doc = "2.014 ms"]
    #[inline(always)]
    pub fn cycle66(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE66)
    }
    #[doc = "3.967 ms"]
    #[inline(always)]
    pub fn cycle130(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE130)
    }
}
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WRTLOCK_R = crate::BitReader;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WRTLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CALIB` reader - Oscillator Calibration"]
pub type CALIB_R = crate::FieldReader;
#[doc = "Field `CALIB` writer - Oscillator Calibration"]
pub type CALIB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 32kHz Output Enable"]
    #[inline(always)]
    pub fn en32k(&self) -> EN32K_R {
        EN32K_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1kHz Output Enable"]
    #[inline(always)]
    pub fn en1k(&self) -> EN1K_R {
        EN1K_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WRTLOCK_R {
        WRTLOCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Oscillator Calibration"]
    #[inline(always)]
    pub fn calib(&self) -> CALIB_R {
        CALIB_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<OSC32K_SPEC, 1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - 32kHz Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en32k(&mut self) -> EN32K_W<OSC32K_SPEC, 2> {
        EN32K_W::new(self)
    }
    #[doc = "Bit 3 - 1kHz Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en1k(&mut self) -> EN1K_W<OSC32K_SPEC, 3> {
        EN1K_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<OSC32K_SPEC, 6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> ONDEMAND_W<OSC32K_SPEC, 7> {
        ONDEMAND_W::new(self)
    }
    #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> STARTUP_W<OSC32K_SPEC, 8> {
        STARTUP_W::new(self)
    }
    #[doc = "Bit 12 - Write Lock"]
    #[inline(always)]
    #[must_use]
    pub fn wrtlock(&mut self) -> WRTLOCK_W<OSC32K_SPEC, 12> {
        WRTLOCK_W::new(self)
    }
    #[doc = "Bits 16:22 - Oscillator Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn calib(&mut self) -> CALIB_W<OSC32K_SPEC, 16> {
        CALIB_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "32kHz Internal Oscillator (OSC32K) Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc32k::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc32k::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSC32K_SPEC;
impl crate::RegisterSpec for OSC32K_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osc32k::R`](R) reader structure"]
impl crate::Readable for OSC32K_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osc32k::W`](W) writer structure"]
impl crate::Writable for OSC32K_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSC32K to value 0x003f_0080"]
impl crate::Resettable for OSC32K_SPEC {
    const RESET_VALUE: Self::Ux = 0x003f_0080;
}
