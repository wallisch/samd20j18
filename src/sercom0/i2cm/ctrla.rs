#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MODE` reader - Operating Mode"]
pub type MODE_R = crate::FieldReader<MODESELECT_A>;
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODESELECT_A {
    #[doc = "0: USART with external clock"]
    USART_EXT_CLK = 0,
    #[doc = "1: USART with internal clock"]
    USART_INT_CLK = 1,
    #[doc = "2: SPI in slave operation"]
    SPI_SLAVE = 2,
    #[doc = "3: SPI in master operation"]
    SPI_MASTER = 3,
    #[doc = "4: I2C slave operation"]
    I2C_SLAVE = 4,
    #[doc = "5: I2C master operation"]
    I2C_MASTER = 5,
}
impl From<MODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MODESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODESELECT_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODESELECT_A> {
        match self.bits {
            0 => Some(MODESELECT_A::USART_EXT_CLK),
            1 => Some(MODESELECT_A::USART_INT_CLK),
            2 => Some(MODESELECT_A::SPI_SLAVE),
            3 => Some(MODESELECT_A::SPI_MASTER),
            4 => Some(MODESELECT_A::I2C_SLAVE),
            5 => Some(MODESELECT_A::I2C_MASTER),
            _ => None,
        }
    }
    #[doc = "USART with external clock"]
    #[inline(always)]
    pub fn is_usart_ext_clk(&self) -> bool {
        *self == MODESELECT_A::USART_EXT_CLK
    }
    #[doc = "USART with internal clock"]
    #[inline(always)]
    pub fn is_usart_int_clk(&self) -> bool {
        *self == MODESELECT_A::USART_INT_CLK
    }
    #[doc = "SPI in slave operation"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == MODESELECT_A::SPI_SLAVE
    }
    #[doc = "SPI in master operation"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == MODESELECT_A::SPI_MASTER
    }
    #[doc = "I2C slave operation"]
    #[inline(always)]
    pub fn is_i2c_slave(&self) -> bool {
        *self == MODESELECT_A::I2C_SLAVE
    }
    #[doc = "I2C master operation"]
    #[inline(always)]
    pub fn is_i2c_master(&self) -> bool {
        *self == MODESELECT_A::I2C_MASTER
    }
}
#[doc = "Field `MODE` writer - Operating Mode"]
pub type MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, MODESELECT_A>;
impl<'a, REG, const O: u8> MODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USART with external clock"]
    #[inline(always)]
    pub fn usart_ext_clk(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::USART_EXT_CLK)
    }
    #[doc = "USART with internal clock"]
    #[inline(always)]
    pub fn usart_int_clk(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::USART_INT_CLK)
    }
    #[doc = "SPI in slave operation"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::SPI_SLAVE)
    }
    #[doc = "SPI in master operation"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::SPI_MASTER)
    }
    #[doc = "I2C slave operation"]
    #[inline(always)]
    pub fn i2c_slave(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::I2C_SLAVE)
    }
    #[doc = "I2C master operation"]
    #[inline(always)]
    pub fn i2c_master(self) -> &'a mut crate::W<REG> {
        self.variant(MODESELECT_A::I2C_MASTER)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PINOUT` reader - Pin Usage"]
pub type PINOUT_R = crate::BitReader;
#[doc = "Field `PINOUT` writer - Pin Usage"]
pub type PINOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDAHOLD` reader - SDA Hold Time"]
pub type SDAHOLD_R = crate::FieldReader<SDAHOLDSELECT_A>;
#[doc = "SDA Hold Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDAHOLDSELECT_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: 50-100ns hold time"]
    _75NS = 1,
    #[doc = "2: 300-600ns hold time"]
    _450NS = 2,
    #[doc = "3: 400-800ns hold time"]
    _600NS = 3,
}
impl From<SDAHOLDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SDAHOLDSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SDAHOLDSELECT_A {
    type Ux = u8;
}
impl SDAHOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDAHOLDSELECT_A {
        match self.bits {
            0 => SDAHOLDSELECT_A::DISABLE,
            1 => SDAHOLDSELECT_A::_75NS,
            2 => SDAHOLDSELECT_A::_450NS,
            3 => SDAHOLDSELECT_A::_600NS,
            _ => unreachable!(),
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SDAHOLDSELECT_A::DISABLE
    }
    #[doc = "50-100ns hold time"]
    #[inline(always)]
    pub fn is_75ns(&self) -> bool {
        *self == SDAHOLDSELECT_A::_75NS
    }
    #[doc = "300-600ns hold time"]
    #[inline(always)]
    pub fn is_450ns(&self) -> bool {
        *self == SDAHOLDSELECT_A::_450NS
    }
    #[doc = "400-800ns hold time"]
    #[inline(always)]
    pub fn is_600ns(&self) -> bool {
        *self == SDAHOLDSELECT_A::_600NS
    }
}
#[doc = "Field `SDAHOLD` writer - SDA Hold Time"]
pub type SDAHOLD_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, SDAHOLDSELECT_A>;
impl<'a, REG, const O: u8> SDAHOLD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SDAHOLDSELECT_A::DISABLE)
    }
    #[doc = "50-100ns hold time"]
    #[inline(always)]
    pub fn _75ns(self) -> &'a mut crate::W<REG> {
        self.variant(SDAHOLDSELECT_A::_75NS)
    }
    #[doc = "300-600ns hold time"]
    #[inline(always)]
    pub fn _450ns(self) -> &'a mut crate::W<REG> {
        self.variant(SDAHOLDSELECT_A::_450NS)
    }
    #[doc = "400-800ns hold time"]
    #[inline(always)]
    pub fn _600ns(self) -> &'a mut crate::W<REG> {
        self.variant(SDAHOLDSELECT_A::_600NS)
    }
}
#[doc = "Field `INACTOUT` reader - Inactive Time-out"]
pub type INACTOUT_R = crate::FieldReader<INACTOUTSELECT_A>;
#[doc = "Inactive Time-out\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INACTOUTSELECT_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: 5-6 SCL Time-Out(50-60us)"]
    _55US = 1,
    #[doc = "2: 10-11 SCL Time-Out(100-110us)"]
    _105US = 2,
    #[doc = "3: 20-21 SCL Time-Out(200-210us)"]
    _205US = 3,
}
impl From<INACTOUTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: INACTOUTSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INACTOUTSELECT_A {
    type Ux = u8;
}
impl INACTOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INACTOUTSELECT_A {
        match self.bits {
            0 => INACTOUTSELECT_A::DISABLE,
            1 => INACTOUTSELECT_A::_55US,
            2 => INACTOUTSELECT_A::_105US,
            3 => INACTOUTSELECT_A::_205US,
            _ => unreachable!(),
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INACTOUTSELECT_A::DISABLE
    }
    #[doc = "5-6 SCL Time-Out(50-60us)"]
    #[inline(always)]
    pub fn is_55us(&self) -> bool {
        *self == INACTOUTSELECT_A::_55US
    }
    #[doc = "10-11 SCL Time-Out(100-110us)"]
    #[inline(always)]
    pub fn is_105us(&self) -> bool {
        *self == INACTOUTSELECT_A::_105US
    }
    #[doc = "20-21 SCL Time-Out(200-210us)"]
    #[inline(always)]
    pub fn is_205us(&self) -> bool {
        *self == INACTOUTSELECT_A::_205US
    }
}
#[doc = "Field `INACTOUT` writer - Inactive Time-out"]
pub type INACTOUT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, INACTOUTSELECT_A>;
impl<'a, REG, const O: u8> INACTOUT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(INACTOUTSELECT_A::DISABLE)
    }
    #[doc = "5-6 SCL Time-Out(50-60us)"]
    #[inline(always)]
    pub fn _55us(self) -> &'a mut crate::W<REG> {
        self.variant(INACTOUTSELECT_A::_55US)
    }
    #[doc = "10-11 SCL Time-Out(100-110us)"]
    #[inline(always)]
    pub fn _105us(self) -> &'a mut crate::W<REG> {
        self.variant(INACTOUTSELECT_A::_105US)
    }
    #[doc = "20-21 SCL Time-Out(200-210us)"]
    #[inline(always)]
    pub fn _205us(self) -> &'a mut crate::W<REG> {
        self.variant(INACTOUTSELECT_A::_205US)
    }
}
#[doc = "Field `LOWTOUT` reader - SCL Low Time-out"]
pub type LOWTOUT_R = crate::BitReader;
#[doc = "Field `LOWTOUT` writer - SCL Low Time-out"]
pub type LOWTOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 7 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Pin Usage"]
    #[inline(always)]
    pub fn pinout(&self) -> PINOUT_R {
        PINOUT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - SDA Hold Time"]
    #[inline(always)]
    pub fn sdahold(&self) -> SDAHOLD_R {
        SDAHOLD_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Inactive Time-out"]
    #[inline(always)]
    pub fn inactout(&self) -> INACTOUT_R {
        INACTOUT_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - SCL Low Time-out"]
    #[inline(always)]
    pub fn lowtout(&self) -> LOWTOUT_R {
        LOWTOUT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<CTRLA_SPEC, 0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CTRLA_SPEC, 1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CTRLA_SPEC, 2> {
        MODE_W::new(self)
    }
    #[doc = "Bit 7 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<CTRLA_SPEC, 7> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 16 - Pin Usage"]
    #[inline(always)]
    #[must_use]
    pub fn pinout(&mut self) -> PINOUT_W<CTRLA_SPEC, 16> {
        PINOUT_W::new(self)
    }
    #[doc = "Bits 20:21 - SDA Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn sdahold(&mut self) -> SDAHOLD_W<CTRLA_SPEC, 20> {
        SDAHOLD_W::new(self)
    }
    #[doc = "Bits 28:29 - Inactive Time-out"]
    #[inline(always)]
    #[must_use]
    pub fn inactout(&mut self) -> INACTOUT_W<CTRLA_SPEC, 28> {
        INACTOUT_W::new(self)
    }
    #[doc = "Bit 30 - SCL Low Time-out"]
    #[inline(always)]
    #[must_use]
    pub fn lowtout(&mut self) -> LOWTOUT_W<CTRLA_SPEC, 30> {
        LOWTOUT_W::new(self)
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
#[doc = "I2CM Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
