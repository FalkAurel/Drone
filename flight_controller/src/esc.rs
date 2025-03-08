use alloc::boxed::Box;

static mut TIMER: Option<Timer<'static, LowSpeed>> = None;
const ESC_HZ_FREQUENCY: u32 = 50;

use esp_hal::{
    gpio::GpioPin, 
    ledc::{
        channel::{
            config::{Config as ChannelConfig, PinConfig}, Channel, ChannelIFace, Number as ChannelNumber
        }, 
        timer::{
            config::{Config as TimerConfig, Duty}, LSClockSource, 
            Number as TimerNumber, Timer, TimerIFace
        },
        LSGlobalClkSource, Ledc, LowSpeed
    },
    peripherals::LEDC, time::{self, Rate}
};

pub use error_handling::ESCError;

use crate::{mem::{BumpAllocator, ALLOCATOR}, sync::Mutex};


mod error_handling {
    use core::fmt::Debug;
    use esp_hal::ledc::channel::Error;

    pub enum ESCError {
        TimerConfigError,
        ChannelConfigError(u8, Error),
        DutyError(u8, Error)
    }

    impl Debug for ESCError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                ESCError::ChannelConfigError(channel, err) => write!(f, "Configuration of Channel {channel} failed with: {err:#?}")?,
                ESCError::DutyError(channel, err) => write!(f, "Setting duty for Channel {channel} failed with: {err:#?}")?,
                ESCError::TimerConfigError => write!(f, "Configuring Timer failed: Please refer to: https://docs.esp-rs.org/esp-hal/esp-hal/0.23.1/esp32/esp_hal/ledc/timer/enum.Error.html")?
            }
            Ok(())
        }
    }
}

/// Represents Strength in percentages (0 - 100) no fractions are permitted
pub struct RotorStrength {
    m1: u8,
    m2: u8,
    m3: u8,
    m4: u8
}

impl RotorStrength {
    pub const fn new(m1: u8, m2: u8, m3: u8, m4: u8) -> Self {
        assert!(m1 < 101 && m2 < 101 && m3 < 101 && m4 < 101, "Strength cannot exceed 100%");
        Self { m1, m2, m3, m4 }
    }
}


/// The ESC 30A operates at 50-60hz
pub struct ESCControler<'controller> {
    ledc: Ledc<'controller>,
    #[cfg(feature = "wifi")]
    channels: Box<[Channel<'controller, LowSpeed>; 4], &'controller Mutex<BumpAllocator>>,
    #[cfg(not(feature = "wifi"))]
    channels: Box<[Channel<'controller, LowSpeed>; 4]>,
}

impl <'controller> ESCControler<'controller> {
    /// Creates a new ESCController. After creation you must call `init`, otherwise all subsequent calls will fail
    pub fn new(ledc: LEDC, pin27: GpioPin<27>, pin26: GpioPin<26>, pin25: GpioPin<25>, pin23: GpioPin<23>) -> Result<Self, ESCError> {
        let mut ledc: Ledc = Ledc::new(ledc);
        ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

        let mut timer: Timer<'static, LowSpeed> = ledc.timer(TimerNumber::Timer0);

        timer.configure(
            TimerConfig {
                duty: Duty::Duty8Bit,
                clock_source: LSClockSource::APBClk,
                frequency: time::Rate::from_hz(ESC_HZ_FREQUENCY)
            }
        ).map_err(|_| ESCError::TimerConfigError)?;

        unsafe {
            TIMER = Some(timer)
        }

        #[cfg(feature = "wifi")]
        let channels: Box<[Channel<'controller, LowSpeed>; 4], &Mutex<BumpAllocator>> = Box::new_in([
            ledc.channel(ChannelNumber::Channel0, pin27),
            ledc.channel(ChannelNumber::Channel1, pin26),
            ledc.channel(ChannelNumber::Channel2, pin25),
            ledc.channel(ChannelNumber::Channel3, pin23),
        ], &ALLOCATOR);

        #[cfg(not(feature = "wifi"))]
        let channels: Box<[Channel<'controller, LowSpeed>; 4]> = Box::new([
            ledc.channel(ChannelNumber::Channel0, pin27),
            ledc.channel(ChannelNumber::Channel1, pin26),
            ledc.channel(ChannelNumber::Channel2, pin25),
            ledc.channel(ChannelNumber::Channel3, pin23),
        ]);


        Ok(Self { ledc, channels })
    }

    /// Due to the borrow checker we must split the initialization of the channels from the channel creation.
    /// If you don't call this function before you use the motors, it will fail.
    #[allow(static_mut_refs)]
    pub fn init(&mut self) -> Result<(), ESCError> {
        self.channels[0].configure(ChannelConfig { timer: unsafe { TIMER.as_ref().unwrap() }, duty_pct: 0, pin_config: PinConfig::PushPull }).map_err(|err| ESCError::ChannelConfigError(0, err))?;
        self.channels[1].configure(ChannelConfig { timer: unsafe { TIMER.as_ref().unwrap() }, duty_pct: 0, pin_config: PinConfig::PushPull }).map_err(|err| ESCError::ChannelConfigError(1, err))?;
        self.channels[2].configure(ChannelConfig { timer: unsafe { TIMER.as_ref().unwrap() }, duty_pct: 0, pin_config: PinConfig::PushPull }).map_err(|err| ESCError::ChannelConfigError(2, err))?;
        self.channels[3].configure(ChannelConfig { timer: unsafe { TIMER.as_ref().unwrap() }, duty_pct: 0, pin_config: PinConfig::PushPull }).map_err(|err| ESCError::ChannelConfigError(3, err))?;

        Ok(())
    }

    pub fn update_rotor_frequency(&mut self, rotor_strength: RotorStrength) -> Result<(), ESCError> {
        let RotorStrength { m1, m2, m3, m4 } = rotor_strength;

        self.channels[0].set_duty(m1).map_err(|err| ESCError::DutyError(0, err))?;
        self.channels[1].set_duty(m2).map_err(|err| ESCError::DutyError(1, err))?;
        self.channels[2].set_duty(m3).map_err(|err| ESCError::DutyError(2, err))?;
        self.channels[3].set_duty(m4).map_err(|err| ESCError::DutyError(3, err))?;

        Ok(())
    }


    pub fn create_timer(&self) -> Result<Timer<'controller, LowSpeed>, ESCError> {
        let mut timer: Timer<'controller, LowSpeed> = self.ledc.timer(TimerNumber::Timer0);

        timer.configure(
            TimerConfig {
                duty: Duty::Duty8Bit,
                clock_source: LSClockSource::APBClk,
                frequency: Rate::from_hz(ESC_HZ_FREQUENCY)
            }
        ).map_err(|_| ESCError::TimerConfigError)?;
        Ok(timer)
    }
}