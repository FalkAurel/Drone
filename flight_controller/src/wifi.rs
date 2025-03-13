use core::{marker::PhantomData, mem::MaybeUninit};

use esp_hal::{delay::Delay, peripherals::{RADIO_CLK, RNG, WIFI}, rng::Rng, time::now, timer::timg::Timer};
use esp_println::println;
use esp_wifi::{wifi::{new_with_config, AccessPointConfiguration, AuthMethod, WifiApDevice, WifiController, WifiDevice}, *};

use alloc::{boxed::Box, vec::Vec};
use crate::{mem::{BumpAllocator, ALLOCATOR}, sync::{Mutex, OnceLock}};
static ESP_WIFI_CONTROLLER: OnceLock<EspWifiController> = OnceLock::new();

pub use error::Error;

mod error {
    use core::fmt::Debug;
    use esp_wifi::{InitializationError, wifi::WifiError};
    pub enum Error {
        WifiRadioInitialization(InitializationError),
        UTF8Parsing,
        AccessPointConfig(WifiError),
        StartAP(WifiError)
    }

    impl Debug for Error {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::AccessPointConfig(err) => write!(f, "Access Point Configuration failed with: {err:?}"),
                Self::StartAP(err) => write!(f, "Starting Access Point failed with: {err:?}"),
                Self::WifiRadioInitialization(err) => write!(f, "Wifiradio initialization failed: {err:?}"),
                Self::UTF8Parsing => write!(f, "UTF8 Parsing failed")
            }
        }
    }
}

pub struct Uninit;
pub struct Init;

pub struct Wifi<'wifi: 'static, State> {
    device: MaybeUninit<WifiDevice<'wifi, WifiApDevice>>,
    controller: MaybeUninit<Box<WifiController<'wifi>, &'wifi Mutex<BumpAllocator>>>,
    state: PhantomData<State>
}


impl <'wifi> Wifi<'wifi, Uninit> {
    pub fn new(timer: Timer, rng: RNG, radio_clocks: RADIO_CLK) -> Result<Self, Error> {
        let wifi: EspWifiController = init(timer, Rng::new(rng), radio_clocks).map_err(|err| Error::WifiRadioInitialization(err))?;
        ESP_WIFI_CONTROLLER.set(wifi);

        Ok(Self { device: MaybeUninit::uninit(), controller: MaybeUninit::uninit(), state: PhantomData })
    }

    // 0xFF_FF_00_00
    // IP = Network part + Device Part
    // Network: 192.168.2
    // Device ESP32: 192.168.2.1
    // Gateway Addr: 192.168.2.1 
    // Resources: https://github.com/esp-rs/esp-hal/blob/cdcd3bee4d52dd992bd1a690639947dbf6f01d99/examples/src/bin/wifi_access_point.rs
    pub fn init(mut self, wifi: WIFI, ssid: &str, password: &str, max_connections: u16) -> Result<Wifi<'wifi, Init>, Error> {
        let config: AccessPointConfiguration = AccessPointConfiguration {
            ssid: ssid.try_into().map_err(|_| Error::UTF8Parsing)?,
            password: password.try_into().map_err(|_| Error::UTF8Parsing)?,
            auth_method: AuthMethod::WPA2Personal,
            max_connections,
            ..Default::default()
        };

        let (device, mut controller) = new_with_config(ESP_WIFI_CONTROLLER.get().unwrap(), wifi, config)
        .map_err(|err| Error::AccessPointConfig(err))?;

        controller.start().map_err(|err| Error::StartAP(err))?;
        self.device = MaybeUninit::new(device);

        self.controller = MaybeUninit::new(Box::new_in(controller, &ALLOCATOR));
        let inited_wifi: Wifi<Init> = Wifi { device: self.device, controller: self.controller, state: PhantomData };
        Ok(inited_wifi)
    }
}

impl <'wifi> Wifi<'wifi, Init> {
    pub fn test(&self) {
        println!("test")
    }

    pub fn hardware_address(&self) -> [u8; 6] {
        unsafe {
            self.device.assume_init_ref().mac_address()
        }
    }
}

use smoltcp::{
    iface::{Config, Interface, PollResult, SocketHandle, SocketSet, SocketStorage}, 
    socket::udp::{PacketMetadata, Socket, UdpMetadata}, 
    storage::PacketBuffer, time::Instant, 
    wire::{EthernetAddress, HardwareAddress, IpAddress, IpCidr, IpEndpoint, Ipv4Cidr}
};
use core::net::Ipv4Addr;

pub fn setup_udp_socket(mut wifi: Wifi<'static, Init>, ipv4: Ipv4Addr) -> !{
    // Set up hardware interface

    let now = || {
        use esp_hal::time::{now, Instant as EspInstant};
        Instant::from_micros_const(EspInstant::duration_since_epoch(now()).to_micros() as i64)
    };

    let config: Config = Config::new(HardwareAddress::Ethernet(EthernetAddress::from_bytes(&wifi.hardware_address())));
    let mut iface: Interface = Interface::new(config, unsafe { wifi.device.assume_init_mut() }, now());

    iface.update_ip_addrs(|ipaddr| {
        ipaddr.push(IpCidr::Ipv4(Ipv4Cidr::new(Ipv4Addr::new(196, 168, 2, 2), 24))).expect("Adding Interface IP Address")
    });

    println!("IP: {:?}", iface.ip_addrs());

    if let Some(ip_addr) = iface.context().ipv4_addr() {
        println!("IP: {ip_addr:?}")
    }

    let mut rx_ms: [PacketMetadata; 1] = [PacketMetadata::EMPTY; 1];
    let mut tx_ms: [PacketMetadata; 0] = [PacketMetadata::EMPTY; 0];

    let mut rx_payload: [u8; 1024] = [0; 1024];
    let mut tx_payload: [u8; 0] = [0; 0];

    let rx_buffer: PacketBuffer<UdpMetadata> = PacketBuffer::new(&mut rx_ms[..], &mut rx_payload[..]);
    let tx_buffer: PacketBuffer<UdpMetadata> = PacketBuffer::new(&mut tx_ms[..], &mut tx_payload[..]);
    let mut udp_socket: Socket = Socket::new(rx_buffer, tx_buffer);

    if udp_socket.is_open() {
        udp_socket.bind(IpEndpoint::new(
            IpAddress::Ipv4(Ipv4Addr::from_bits(iface.ipv4_addr().expect("Interface doesn't have an IP set").to_bits())), 5000
        )
        ).unwrap()
    }

    if udp_socket.can_recv() {
        loop {
            if let Ok((data, _metadata)) = udp_socket.recv() {

            }
        }
    } else {
        loop {
            
        }
    }


}