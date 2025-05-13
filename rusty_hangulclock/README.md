# Rusty HangulClock

![rusty_hangulclock](https://homin.dev/asset/blog/img/rusty_hangulclock_00_1024.jpg)

HangulClock written in Rust on ESP32C3 board

Hardware:
- [sch & pcb artwork](../sch/rusty-hangulclock/) - a KiCad project
- [case 3d model](../case/)

## Build and flash

### Pre requirement
Install toolchain (only for one time):
```sh
cargo install espup cargo-espflash ldproxy
espup install
```

### Build and flash:
For dotstar:
```sh
make flash_dotstar
```

For neopixel:
```sh
make flash_neopixel
```

### Factory reset settings:
```sh
make erase_nvs
```

## TODOs
- [x] Display color setting
- [x] Timezone setting
- [x] WPS to connect Wifi
- [x] NTP to sync time
