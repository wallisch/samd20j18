#[doc = "Register `XOSC32K` reader"]
pub type R = crate::R<XOSC32K_SPEC>;
#[doc = "Register `XOSC32K` writer"]
pub type W = crate::W<XOSC32K_SPEC>;
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XTALEN` reader - Crystal Oscillator Enable"]
pub type XTALEN_R = crate::BitReader;
#[doc = "Field `XTALEN` writer - Crystal Oscillator Enable"]
pub type XTALEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN32K` reader - 32kHz Output Enable"]
pub type EN32K_R = crate::BitReader;
#[doc = "Field `EN32K` writer - 32kHz Output Enable"]
pub type EN32K_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN1K` reader - 1kHz Output Enable"]
pub type EN1K_R = crate::BitReader;
#[doc = "Field `EN1K` writer - 1kHz Output Enable"]
pub type EN1K_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AAMPEN` reader - Automatic Amplitude Control Enable"]
pub type AAMPEN_R = crate::BitReader;
#[doc = "Field `AAMPEN` writer - Automatic Amplitude Control Enable"]
pub type AAMPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "0: 0.122 ms"]
    CYCLE1 = 0,
    #[doc = "1: 1.068 ms"]
    CYCLE32 = 1,
    #[doc = "2: 62.592 ms"]
    CYCLE2048 = 2,
    #[doc = "3: 125.092 ms"]
    CYCLE4096 = 3,
    #[doc = "4: 500.092 ms"]
    CYCLE16384 = 4,
    #[doc = "5: 1000.092 ms"]
    CYCLE32768 = 5,
    #[doc = "6: 2000.092 ms"]
    CYCLE65536 = 6,
    #[doc = "7: 4000.092 ms"]
    CYCLE131072 = 7,
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
            0 => STARTUPSELECT_A::CYCLE1,
            1 => STARTUPSELECT_A::CYCLE32,
            2 => STARTUPSELECT_A::CYCLE2048,
            3 => STARTUPSELECT_A::CYCLE4096,
            4 => STARTUPSELECT_A::CYCLE16384,
            5 => STARTUPSELECT_A::CYCLE32768,
            6 => STARTUPSELECT_A::CYCLE65536,
            7 => STARTUPSELECT_A::CYCLE131072,
            _ => unreachable!(),
        }
    }
    #[doc = "0.122 ms"]
    #[inline(always)]
    pub fn is_cycle1(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE1
    }
    #[doc = "1.068 ms"]
    #[inline(always)]
    pub fn is_cycle32(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE32
    }
    #[doc = "62.592 ms"]
    #[inline(always)]
    pub fn is_cycle2048(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE2048
    }
    #[doc = "125.092 ms"]
    #[inline(always)]
    pub fn is_cycle4096(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE4096
    }
    #[doc = "500.092 ms"]
    #[inline(always)]
    pub fn is_cycle16384(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE16384
    }
    #[doc = "1000.092 ms"]
    #[inline(always)]
    pub fn is_cycle32768(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE32768
    }
    #[doc = "2000.092 ms"]
    #[inline(always)]
    pub fn is_cycle65536(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE65536
    }
    #[doc = "4000.092 ms"]
    #[inline(always)]
    pub fn is_cycle131072(&self) -> bool {
        *self == STARTUPSELECT_A::CYCLE131072
    }
}
#[doc = "Field `STARTUP` writer - Oscillator Start-Up Time"]
pub type STARTUP_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, STARTUPSELECT_A>;
impl<'a, REG, const O: u8> STARTUP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.122 ms"]
    #[inline(always)]
    pub fn cycle1(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE1)
    }
    #[doc = "1.068 ms"]
    #[inline(always)]
    pub fn cycle32(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE32)
    }
    #[doc = "62.592 ms"]
    #[inline(always)]
    pub fn cycle2048(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE2048)
    }
    #[doc = "125.092 ms"]
    #[inline(always)]
    pub fn cycle4096(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE4096)
    }
    #[doc = "500.092 ms"]
    #[inline(always)]
    pub fn cycle16384(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE16384)
    }
    #[doc = "1000.092 ms"]
    #[inline(always)]
    pub fn cycle32768(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE32768)
    }
    #[doc = "2000.092 ms"]
    #[inline(always)]
    pub fn cycle65536(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE65536)
    }
    #[doc = "4000.092 ms"]
    #[inline(always)]
    pub fn cycle131072(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::CYCLE131072)
    }
}
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WRTLOCK_R = crate::BitReader;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WRTLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn xtalen(&self) -> XTALEN_R {
        XTALEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 32kHz Output Enable"]
    #[inline(always)]
    pub fn en32k(&self) -> EN32K_R {
        EN32K_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1kHz Output Enable"]
    #[inline(always)]
    pub fn en1k(&self) -> EN1K_R {
        EN1K_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic Amplitude Control Enable"]
    #[inline(always)]
    pub fn aampen(&self) -> AAMPEN_R {
        AAMPEN_R::new(((self.bits >> 5) & 1) != 0)
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
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<XOSC32K_SPEC, 1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xtalen(&mut self) -> XTALEN_W<XOSC32K_SPEC, 2> {
        XTALEN_W::new(self)
    }
    #[doc = "Bit 3 - 32kHz Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en32k(&mut self) -> EN32K_W<XOSC32K_SPEC, 3> {
        EN32K_W::new(self)
    }
    #[doc = "Bit 4 - 1kHz Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en1k(&mut self) -> EN1K_W<XOSC32K_SPEC, 4> {
        EN1K_W::new(self)
    }
    #[doc = "Bit 5 - Automatic Amplitude Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aampen(&mut self) -> AAMPEN_W<XOSC32K_SPEC, 5> {
        AAMPEN_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<XOSC32K_SPEC, 6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> ONDEMAND_W<XOSC32K_SPEC, 7> {
        ONDEMAND_W::new(self)
    }
    #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> STARTUP_W<XOSC32K_SPEC, 8> {
        STARTUP_W::new(self)
    }
    #[doc = "Bit 12 - Write Lock"]
    #[inline(always)]
    #[must_use]
    pub fn wrtlock(&mut self) -> WRTLOCK_W<XOSC32K_SPEC, 12> {
        WRTLOCK_W::new(self)
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
#[doc = "32kHz External Crystal Oscillator (XOSC32K) Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xosc32k::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xosc32k::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XOSC32K_SPEC;
impl crate::RegisterSpec for XOSC32K_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`xosc32k::R`](R) reader structure"]
impl crate::Readable for XOSC32K_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xosc32k::W`](W) writer structure"]
impl crate::Writable for XOSC32K_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XOSC32K to value 0x80"]
impl crate::Resettable for XOSC32K_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
