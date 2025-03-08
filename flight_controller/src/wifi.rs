use esp_hal::{peripherals::{RADIO_CLK, RNG}, rng::Rng, timer::timg::Timer};
use esp_wifi::*;

static mut ESP_WIFI_CONTROLLER: Option<EspWifiController> = None;

pub struct Wifi<'wifi> {
    h: &'wifi ()
}

impl <'wifi> Wifi<'wifi> {
    pub fn init(timer: Timer, rng: RNG, radio_clocks: RADIO_CLK) {
        let wifi: EspWifiController = init(timer, Rng::new(rng), radio_clocks).unwrap();

        unsafe {
            ESP_WIFI_CONTROLLER = Some(wifi)
        }
    }
}