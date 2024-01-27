# steelseries-apex-oled
> A driver for the OLED screen on the SteelSeries Apex line of keyboards.

## Overview
Several keyboards in the [Apex](https://steelseries.com/apex) line of SteelSeries keyboards come with an OLED display on the top-right of the keyboard; on full-size keyboards this is fitted above the numpad, whereas on TKL boards it takes the place of the Print Screen, Scroll Lock and Pause/Break keys. The manufacturer's intent is that you use their software, [SteelSeries GG](https://steelseries.com/gg), to make this keyboard do anything meaningful, but this software: only runs on Windows and macOS; is becoming increasingly bloated; and doesn't actually allow you to do all that much.

This project's aim is to provide a driver in Rust that is compatible with the [embedded-graphics](https://crates.io/crates/embedded-graphics) crate, allowing more open source development for this platform.

## Supported hardware

### Keyboards
| Product | Supported |
| --- | --- |
| Apex 5 | Unknown |
| Apex 7 | Unknown |
| Apex 7 TKL | Unknown |
| Apex Pro | Yes |
| Apex Pro TKL | Unknown |
| Apex Pro TKL (2023) | Unknown |

### Mice
| Product | Supported |
| --- | --- |
| Rival 700 | Unknown |
| Rival 710 | Unknown |
