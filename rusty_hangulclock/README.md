# Rusty HangulClock

![rusty_hangulclock](https://homin.dev/asset/blog/img/rusty_hangulclock_00_1024.jpg)

HangulClock written in Rust on ESP32C3 board

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