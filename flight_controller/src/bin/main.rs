#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::peripherals::{Peripherals, TIMG0};
use esp_hal::{main, Config};
use esp_hal::timer::timg::TimerGroup;
use flight_controller::esc::{ESCControler, RotorStrength};

#[main]
fn main() -> ! {
    // generator version: 0.2.2
    let config: Config = Config::default().with_cpu_clock(CpuClock::max());
    let peripherals: Peripherals = esp_hal::init(config);

    esp_println::logger::init_logger_from_env();

    #[cfg(feature = "wifi")]
    {
        use flight_controller::mem::init_heap;

        init_heap();
    }

    let mut esc_controller: ESCControler = ESCControler::new(
        peripherals.LEDC, peripherals.GPIO27, peripherals.GPIO26, peripherals.GPIO25, peripherals.GPIO23
    ).unwrap();

    esc_controller.init().unwrap();
    esc_controller.update_rotor_frequency(RotorStrength::new(0, 50, 0, 0)).unwrap();

    #[cfg(feature = "wifi")]
    {
        use flight_controller::wifi::Wifi;
        let timg0: TimerGroup<TIMG0> = TimerGroup::new(peripherals.TIMG0);

        Wifi::init(timg0.timer0, peripherals.RNG, peripherals.RADIO_CLK)
    }

    loop {
    }
    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/v0.23.1/examples/src/bin
}
