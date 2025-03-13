#![no_std]
#![no_main]
#![feature(allocator_api)]

use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::peripherals::{Peripherals, TIMG0};
use esp_hal::Config;
use esp_hal::timer::timg::TimerGroup;
use flight_controller::esc::{ESCControler, RotorStrength};

#[esp_hal::main]
fn main() -> ! {
    // generator version: 0.2.2
    let config: Config = Config::default().with_cpu_clock(CpuClock::max());
    let peripherals: Peripherals = esp_hal::init(config);

    esp_println::logger::init_logger_from_env();

    if cfg!(feature = "wifi") {
        wifi_main(peripherals)
    } else {
        #[cfg(not(feature = "wifi"))]
        regular_main(peripherals);
        loop {}
    }
    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/v0.23.1/examples/src/bin
}


#[cfg(feature = "wifi")]
fn wifi_main(peripherals: Peripherals) -> ! {
    use flight_controller::{wifi::*, mem::init_heap};
    use core::net::Ipv4Addr;

    init_heap();
    let timer_group: TimerGroup<TIMG0> = TimerGroup::new(peripherals.TIMG0);
    let wifi: Wifi<Init> = Wifi::new(timer_group.timer0, peripherals.RNG, peripherals.RADIO_CLK)
    .unwrap()
    .init(peripherals.WIFI, "Flightcontroller", "Password123", 255)
    .unwrap();

    // 192.168.2.1
    setup_udp_socket(wifi, Ipv4Addr::new(192, 168, 2, 1));
}

#[cfg(not(feature = "wifi"))]
fn regular_main(peripherals: Peripherals) {
    let mut esc_controller: ESCControler = ESCControler::new(
        peripherals.LEDC, peripherals.GPIO27, peripherals.GPIO26, peripherals.GPIO25, peripherals.GPIO23
    ).unwrap();

    esc_controller.init().unwrap();
    esc_controller.update_rotor_frequency(RotorStrength::new(0, 50, 0, 0)).unwrap();
}
