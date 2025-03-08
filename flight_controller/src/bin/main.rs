#![no_std]
#![no_main]
#![feature(allocator_api)]
#![feature(stmt_expr_attributes)]


use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock, ledc::timer, peripherals::{Peripherals, TIMG0}, timer::timg::TimerGroup, Config
};
use flight_controller::esc::{ESCControler, RotorStrength};

#[cfg(feature = "wifi")]
use flight_controller::wifi::Wifi;


#[esp_hal::main]
unsafe fn main() -> ! {
    let config: Config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals: Peripherals = esp_hal::init(config);

    // Setting up GY521
    // let mut gy521: GY521 = GY521::new();
    // gy521.init(MPUConfig::default().set_dlpf(Dlpf::Hz_20)).unwrap();
    // gy521.calibrate(500).unwrap();
    // let mut previous: Angle = compute_angle_acceleration(gy521.read().unwrap().get_accel());
    // let delay: Delay = Delay::new();
    // let delay_us: Duration = Duration::micros(gy521.get_delay().unwrap() as u64);

    // Setting up ESC-Controller
    let mut esc_controller: ESCControler = ESCControler::new(peripherals.LEDC, peripherals.GPIO27, peripherals.GPIO26, peripherals.GPIO25, peripherals.GPIO23).unwrap();
    esc_controller.init().unwrap();
    esc_controller.update_rotor_frequency(RotorStrength::new(50, 50, 50, 50)).unwrap();

    #[cfg(feature = "wifi")]
    {
        let timer_group: TimerGroup<TIMG0> = TimerGroup::new(peripherals.TIMG0);
        Wifi::init(timer_group.timer0, peripherals.RNG, peripherals.RADIO_CLK);
    }

    drop(esc_controller);

    // Setting up Wifi Access Point

    loop {
        // esc_controller.update_rotor_frequency(RotorStrength::new(30, 30, 50, 50)).unwrap()
        // let data: DataFrame = gy521.read().unwrap();
        // let angle: Angle = compute_angle_integration(data.get_gyro(), previous);
        // println!("Messured Angle: {angle:?}");
        // previous = angle;
        // delay.delay(delay_us);
    }
}
