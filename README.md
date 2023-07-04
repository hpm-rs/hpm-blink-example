# hpm-blink-example

A blink example running on the HPM6750EVKMINI board. Written in Rust.

# Build

```shell
# debug in ram
cargo build
# release optimized, flash xip
cargo build --release
```

# Download

```shell
cargo objcopy --release  -- -O binary firmware.bin
# if you haven't installed hpm_isp before
cargo install hpm_isp
# program firmware.bin into flash
hpm_isp flash 0 write 0x0 ./firmware.bin
```
