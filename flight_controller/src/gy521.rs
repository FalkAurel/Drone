use esp_hal::{
    delay::Delay, 
    gpio::GpioPin, 
    i2c::master::{Config, Error, I2c}, 
    peripherals::I2C0, 
    Blocking
};

use sensor_data::{CalibrationOffsets, ScalingFactor};

// Re-export
pub use sensor_data::{AccelometerData, GyroscopeData, DataFrame};
pub use mpu_configuration::{Config as MPUConfig, Dlpf, AFullRangeScale, GFullRangeScale};


// Follow chip specification: MPU-6050-Register-Mapping.pdf
const MPU_ADDRESS: u8           =  0x68;
const WAKE_UP_SEQUENCE: [u8; 2] = [0x6B, 0x0];

// These 
const ACCELO_READ_ADDR: u8 = 0x3B;
const GYRO_READ_ADDR: u8   = 0x43; // This address is the start

// Configuration Register Adress
const DLPF_CONFIG_ADDR: u8   = 0x1A;
const GYRO_CONFIG_ADDR: u8   = 0x1B;
const ACCELO_CONFIG_ADDR: u8 = 0x1C;

// This modul serves with configu
mod mpu_configuration {
    use super::sensor_data::ScalingFactor;

    // https://en.wikipedia.org/wiki/Low-pass_filter
    #[repr(u8)]
    #[allow(non_camel_case_types, dead_code)]
    pub enum Dlpf {
        Hz_256 = 0,
        Hz_188 = 1,
        Hz_98  = 2,
        Hz_42  = 3,
        Hz_20  = 4,
        Hz_10  = 5,
        Hz_5   = 6
    }
    
    #[repr(u8)]
    #[allow(non_camel_case_types, dead_code)]
    pub enum GFullRangeScale {
        Sel_250  = 0 << 3,
        Sel_500  = 1 << 3,
        Sel_1000 = 2 << 3,
        Sel_2000 = 3 << 3
    }

    #[repr(u8)]
    #[allow(non_camel_case_types, dead_code)]
    pub enum AFullRangeScale {
        Sel_2g  = 0 << 3,
        Sel_4g  = 1 << 3,
        Sel_8g  = 2 << 3,
        Sel_16g = 3 << 3
    }

    pub struct Config {
        pub(crate) dlpf: Dlpf,        
        pub(crate) a_fs: AFullRangeScale,
        pub(crate) g_fs: GFullRangeScale
    }

    impl Config {
        pub (crate) fn get_scaling_factor(&self) -> ScalingFactor {
            let a: f32 = match self.a_fs {
                AFullRangeScale::Sel_2g => 16384.0,
                AFullRangeScale::Sel_4g => 8192.0,
                AFullRangeScale::Sel_8g => 4096.0,
                AFullRangeScale::Sel_16g => 2048.0
            };

            let g: f32 = match self.g_fs {
                GFullRangeScale::Sel_250 => 131.0,
                GFullRangeScale::Sel_500 => 65.5,
                GFullRangeScale::Sel_1000 => 32.8,
                GFullRangeScale::Sel_2000 => 16.4
            };

            ScalingFactor {a, g}
        }

        pub fn set_afs(mut self, afs: AFullRangeScale) -> Self {
            self.a_fs = afs;
            self
        }

        pub fn set_gfs(mut self, gfs: GFullRangeScale) -> Self {
            self.g_fs = gfs;
            self
        }

        pub fn set_dlpf(mut  self, dlpf: Dlpf) -> Self {
            self.dlpf = dlpf;
            self
        }
    }

    impl Default for Config {
        fn default() -> Self {
            Self { dlpf: Dlpf::Hz_20, a_fs: AFullRangeScale::Sel_8g, g_fs: GFullRangeScale::Sel_1000 }
        }
    }
}


// This module serves data interpretation
mod sensor_data {
    pub (crate) struct CalibrationOffsets {
        pub(crate) ax: i16,
        pub(crate) ay: i16,
        pub(crate) az: i16,
        pub(crate) gx: i16,
        pub(crate) gy: i16,
        pub(crate) gz: i16
    }

    impl Default for CalibrationOffsets {
        fn default() -> Self {
            Self { ax: 0, ay: 0, az: 0, gx: 0, gy: 0, gz: 0 }
        }
    }

    pub (crate) struct ScalingFactor {
        pub(crate) a: f32,
        pub(crate) g: f32,
    }

    #[derive(Debug)]
    pub struct AccelometerData {
        pub x: f32,
        pub y: f32,
        pub z: f32
    }

    #[derive(Debug)]
    pub struct GyroscopeData {
        pub x: f32,
        pub y: f32,
        pub z: f32
    }

    #[derive(Debug)]
    pub struct DataFrame {
        accel: AccelometerData,
        gyro: GyroscopeData
    }

    impl DataFrame {
        pub (crate) fn new(a_bytes: [u8; 6], g_bytes: [u8; 6], scaling_factor: &ScalingFactor, calibration: &CalibrationOffsets) -> Self {
            let &CalibrationOffsets { ax, ay, az, gx, gy, gz } = calibration;
            let (a_x, a_y, a_z) = Self::get_sensor_data(a_bytes, ax, ay, az,scaling_factor.a);
            let (g_x, g_y, g_z) = Self::get_sensor_data(g_bytes, gx, gy, gz, scaling_factor.g);
            
            Self { accel: AccelometerData { x: a_x, y: a_y, z: a_z }, gyro: GyroscopeData { x: g_x, y: g_y, z: g_z } }
        }

        fn get_sensor_data(bytes: [u8; 6], cal_x: i16, cal_y: i16, cal_z: i16, scaling_factor: f32) -> (f32, f32, f32) {
            let x: f32 = i16::checked_sub(
                i16::from_be_bytes([bytes[0], bytes[1]]),
                 cal_x
            ).unwrap_or_else(|| if cal_x > 0 { i16::MIN } else { i16::MAX }) as f32;

            let y: f32 = i16::checked_sub(
                i16::from_be_bytes([bytes[2], bytes[3]]), 
                cal_y
            ).unwrap_or_else(|| if cal_y > 0 { i16::MIN } else { i16::MAX }) as f32; 
            
            let z: f32 = i16::checked_sub(
                i16::from_be_bytes([bytes[4], bytes[5]]), 
                cal_z
            ).unwrap_or_else(|| if cal_z > 0 { i16::MIN } else { i16::MAX }) as f32;

            (x / scaling_factor, y / scaling_factor, z / scaling_factor)
        }

        pub fn get_accel(&self) -> &AccelometerData {
            &self.accel
        }

        pub fn get_gyro(&self) -> &GyroscopeData {
            &self.gyro
        }
    }
}

pub struct GY521<'driver> {
    master: I2c<'driver, Blocking>,
    scaling_factor: Option<ScalingFactor>,
    calibration_offsets: CalibrationOffsets,
}

impl <'driver> GY521 <'driver> {
    pub fn new() -> Self {
        let master: I2c<'driver, Blocking> = I2c::new(
            unsafe { I2C0::steal() }, 
            Config::default()
        ).expect("Creation of Master failed")
        .with_sda(unsafe { GpioPin::<21>::steal() })
        .with_scl(unsafe { GpioPin::<22>::steal() });

        Self { master, scaling_factor: None, calibration_offsets: CalibrationOffsets::default() }
    }

    pub fn init(&mut self, config: MPUConfig) -> Result<(), Error> {
        self.scaling_factor = Some(config.get_scaling_factor());

        self.master.write(MPU_ADDRESS, &WAKE_UP_SEQUENCE)?; // Power on chip
        self.master.write(MPU_ADDRESS, &[DLPF_CONFIG_ADDR, config.dlpf as u8])?; // Set DLPF
        self.master.write(MPU_ADDRESS, &[GYRO_CONFIG_ADDR, config.g_fs as u8])?; // Set Gyro Full-Scale Range
        self.master.write(MPU_ADDRESS, &[ACCELO_CONFIG_ADDR, config.a_fs as u8]) // Set Accelorometer Full-Scale Range
    }

    // This function is to be used AFTER the sensor has been set into a level possition. Iterations defines how many samples
    // it takes before performing calibration. The higher the DLPF, the more iteration you need to acount for the extra noise
    pub fn calibrate(&mut self, iteration: u16) -> Result<(), Error> {
        let mut ax_offset: i32 = 0;
        let mut ay_offset: i32 = 0;
        let mut az_offset: i32 = 0;

        let mut gx_offset: i32 = 0;
        let mut gy_offset: i32 = 0;
        let mut gz_offset: i32 = 0;

        let extract_values: fn([u8; 6]) -> (i16, i16, i16) = |bytes: [u8; 6]| -> (i16, i16, i16) {
            let x: i16 = i16::from_be_bytes([bytes[0], bytes[1]]);
            let y: i16 = i16::from_be_bytes([bytes[2], bytes[3]]); 
            let z: i16 = i16::from_be_bytes([bytes[4], bytes[5]]);

            (x, y, z)
        };

        let delay: Delay = Delay::new();
        let mut registers: [u8; 6] = [0; 6];

        let mut dlpf_register: [u8; 1] = [0];
        self.master.write_read(MPU_ADDRESS, &[DLPF_CONFIG_ADDR], &mut dlpf_register)?;
        let delay_µs: u32 = self.get_delay().unwrap();

        for _ in 0..iteration {
            self.master.write_read(MPU_ADDRESS, &[ACCELO_READ_ADDR], &mut registers)?;
            let accelo_bytes: [u8; 6] = registers; // Make copy of buffer
            
            self.master.write_read(MPU_ADDRESS, &[GYRO_READ_ADDR], &mut  registers)?;
            let gyro_bytes: [u8; 6] = registers;

            let (ax, ay, az) = extract_values(accelo_bytes);
            let (gx, gy, gz) = extract_values(gyro_bytes);
            
            ax_offset += ax as i32;
            ay_offset += ay as i32;
            az_offset += az as i32 - self.scaling_factor.as_ref().expect("Initialize the chip first").a as i32; // To acount for earth gravitational pull

            gx_offset += gx as i32;
            gy_offset += gy as i32;
            gz_offset += gz as i32;

            delay.delay_micros(delay_µs);
        }

        let calibration_offsets: CalibrationOffsets = CalibrationOffsets { 
            ax: (ax_offset as f32 / iteration as f32) as i16, 
            ay: (ay_offset as f32 / iteration as f32) as i16, 
            az: (az_offset as f32 / iteration as f32) as i16,
         
            gx: (gx_offset as f32 / iteration as f32) as i16, 
            gy: (gy_offset as f32 / iteration as f32) as i16, 
            gz: (gz_offset as f32 / iteration as f32) as i16 
        };

        self.calibration_offsets = calibration_offsets;
        Ok(())
    }
    
    pub fn read(&mut self) -> Result<DataFrame, Error> {
        let mut registers: [u8; 6] = [0; 6];

        self.master.write_read(MPU_ADDRESS, &[ACCELO_READ_ADDR], &mut registers)?;
        let accelo_bytes: [u8; 6] = registers; // Make copy of buffer
        
        self.master.write_read(MPU_ADDRESS, &[GYRO_READ_ADDR], &mut  registers)?;
        let gyro_bytes: [u8; 6] = registers;

        Ok(
            DataFrame::new(
                accelo_bytes,
                gyro_bytes, 
                self.scaling_factor.as_ref().expect("Initilize the sensor with init"),
                &self.calibration_offsets
            )
        )
    }

    // Returns the delay in µs
    pub fn get_delay(&mut self) -> Option<u32> {
        let mut dlpf_register: [u8; 1] = [0];
        self.master.write_read(MPU_ADDRESS, &[DLPF_CONFIG_ADDR], &mut dlpf_register).ok()?;
        
        let delay_µs: u32 = match dlpf_register[0] & 7 {
            const { Dlpf::Hz_256 as u8 } => 0,
            const { Dlpf::Hz_188 as u8 } => 2000,
            const { Dlpf::Hz_98 as u8  } => 3000,
            const { Dlpf::Hz_42 as u8  } => 4900,
            const { Dlpf::Hz_20 as u8  } => 8500,
            const { Dlpf::Hz_10 as u8  } => 13800,
            const { Dlpf::Hz_5 as u8   } => 19000,
            _ => panic!("Please intilize the Sensor first")
        };
        Some(delay_µs)
    }
}