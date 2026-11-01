# NVRAM Hall Of Fame

A lightweight Rust utlity that inspects system boot entries using `efibootmgr`, parsers and cleans up the output, checks the firmware boot mode, and generates a retro-style ASCII art report.

## Preview
![Preview](https://github.com/wxwreak/nvramhof/blob/main/preview.png)

## Features

- **Boot Parsing:** Automatically runs `efibootmgr -v`, extracts registered boot entries, and strips out verbose UEFI device paths (like `VenHw`, `HD`, `BBS`, or `PciRoot`) for a cleaner view.
- **Active Boot Detection:** Identifies the currently active boot entry (`BootCurrent`) and marks it on the leaderboard.
- **Firmware Detection:** Automatically checks whether the system is booted in **UEFI** or **Legacy** mode via `/sys/firmware/uefi`.

## Requirements
- **Linux**
- ['efibootmgr'](https://github.com/rhboot/efibootmgr)
- **Rust**

## Building & Installation

Available Makefile targets:

- `make build`: Compiles the binary into the `dist/` directory.

- `make install`: Copies the binary to `~/.local/bin`.

- `make all`: Runs both build and install steps.

- `make clean`: Removes the `dist/` build directory.

## License
This project is licensed under the MIT License. See the [License](LICENSE) file for details.
