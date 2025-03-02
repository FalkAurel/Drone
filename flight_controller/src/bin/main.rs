#![no_std]
#![no_main]
#![feature(allocator_api)]

use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock, peripherals::Peripherals, Config
};

use esp_println::println;
use flight_controller::{
    boxed::Box, esc::ESCControler, mem::{get_mem_stats, init_heap}
};

#[esp_hal::main]
unsafe fn main() -> ! {
    let config: Config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals: Peripherals = esp_hal::init(config);

    init_heap();
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


    use flight_controller::alloc::vec::Vec;
    let mut vec: Vec<usize,_> = Vec::with_capacity(2000);

    for i in 0..vec.capacity() {
        vec.push(i);
    }

    let sum: usize = vec.iter().fold(0, |acc, current| -> usize {
        acc + *current
    });

    assert_eq!(sum, (vec.len() * (vec.len() - 1) / 2));
    drop(vec);
    let klein: Box<u8> = Box::new(1);
    println!("{}", klein);

    get_mem_stats();

    loop {
    }
}
