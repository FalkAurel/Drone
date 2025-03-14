```toml
[package]
edition = "2021"
name    = "flight_controller"
version = "0.1.0"

[[bin]]
name = "flight_controller"
path = "./src/bin/main.rs"

[dependencies]
embedded-io = "0.6.1"
embedded-io-async = "0.6.1"
esp-alloc = { version = "0.6.0" }
esp-backtrace = { version = "0.15.0", features = [
  "esp32",
  "exception-handler",
  "panic-handler",
  "println",
] }
esp-hal = { version = "0.23.1", features = ["esp32", "unstable"] } # defmt
esp-println = { version = "0.13.0", features = ["esp32", "log"] }
esp-wifi = { version = "0.12.0", default-features = false, features = [
  "ble",
  "coex",
  "esp-alloc",
  "esp32",
  #"log",
  #"utils",
  "wifi",
] }

heapless = { version = "0.8.0", default-features = false }
log = { version = "0.4.21"  } # https://docs.esp-rs.org/esp-hal/esp-wifi/0.12.0/esp32/esp_wifi/index.html
smoltcp = { version = "0.12.0", default-features = false, features = [
  "medium-ethernet",
  "multicast",
  "proto-dhcpv4",
  "proto-dns",
  "proto-ipv4",
  "socket-dns",
  "socket-icmp",
  "socket-raw",
  "socket-tcp",
  "socket-udp",
] }
# for more networking protocol support see https://crates.io/crates/edge-net
bleps = { git = "https://github.com/bjoernQ/bleps", package = "bleps", rev = "a5148d8ae679e021b78f53fd33afb8bb35d0b62e", features = [
  "async",
  "macros",
] }

critical-section = "1.2.0"
esp-hal-embassy = { version = "0.6.0", features = ["esp32"] }
static_cell = { version = "2.1.0", features = ["nightly"] }
libm = "0.2.11"
fugit = "0.3.7"


[profile.dev.package.esp-wifi]
opt-level = 3

[profile.dev]
# Rust debug is too slow.
# For debug builds always builds with some optimization
opt-level = "s"

[profile.release]
codegen-units    = 1     # LLVM can perform better optimizations using a single thread
debug            = 2
debug-assertions = false
incremental      = false
lto              = 'fat'
opt-level        = 's'
overflow-checks  = false
```
