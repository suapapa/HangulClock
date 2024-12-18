# Rusty HangulClock

HangulClock written in Rust on ESP32S2 board

## Build and flash

Install toolchain (only for one time):

```sh
cargo install espup
espup install
```

Build:
```sh
source $HOME/export-esp.sh
export WIFI_SSID="YOUR_WIFI_SSID"
export WIFI_PASS="YOUR_WIFI_PASS"
cargo build --release
```

Flash:
```sh
export WIFI_SSID="YOUR_WIFI_SSID"
export WIFI_PASS="YOUR_WIFI_PASS"
cargo espflash flash --release
```