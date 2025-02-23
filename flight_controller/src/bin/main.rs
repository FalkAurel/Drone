#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock, delay::Delay, peripherals::Peripherals, time::{now, Duration, Instant}, Config
};

use esp_println::println;
use flight_controller::{
    esc::{ESCControler, RotorStrength}, 
    gy521::{DataFrame, Dlpf, MPUConfig, GY521}, 
    math::{compute_angle_acceleration, compute_angle_integration, Angle}, mem::init_heap
};

#[esp_hal::main]
unsafe fn main() -> ! {
    let config: Config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals: Peripherals = esp_hal::init(config);

    init_heap();

    let mut gyro: GY521 = GY521::new();
    gyro.init(MPUConfig::default().set_dlpf(Dlpf::Hz_98)).expect("Initialization failed");
    gyro.calibrate(1000).expect("Calibration failed");
    let _delay: Delay = Delay::new();
    let _delay_us: Duration = Duration::micros(gyro.get_delay().unwrap() as u64);

    let mut motor_control: ESCControler = ESCControler::new(peripherals.LEDC, peripherals.GPIO27, peripherals.GPIO26, peripherals.GPIO25, peripherals.GPIO23).unwrap();

    motor_control.init().unwrap();
    motor_control.update_rotor_frequency(RotorStrength::new(50, 50, 50, 50)).unwrap();

    let mut previous: Angle = compute_angle_acceleration(gyro.read().unwrap().get_accel());

    let mut sum: u64 = 0;
    println!("Starting event loop");
    for _ in 0..10000 {
        let start: Instant = now();
        let dataframe: DataFrame = gyro.read().unwrap();

        let angle: Angle = compute_angle_integration(dataframe.get_gyro(), previous);
        previous = angle;
        motor_control.update_rotor_frequency(RotorStrength::new(70, 40, 40, 70)).unwrap();
        let end: Instant = now();

        sum += (end - start).to_micros();
    }


    println!("{}", sum as f64 / 10000.0);

    loop {   
    }
}
