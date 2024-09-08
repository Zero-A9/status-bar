# Rust Status Bar

A lightweight and customizable status bar written in Rust, designed for Linux users. This status bar displays useful system information such as CPU usage, battery status, Wi-Fi connectivity, network speed, and the current date and time. It also dynamically updates the colors and icons based on certain thresholds, like battery percentage or Wi-Fi connection status.

## Features

- **CPU Usage**: Displays the current CPU load with customizable colors.
- **Battery Status**: Displays the battery percentage and changes the icon and color based on battery levels:
  - 0-20%: Red background with the lowest battery icon (``)
  - 21-40%: Orange background with a low battery icon (``)
  - 41-60%: Yellow background with a medium battery icon (``)
  - 61-80%: Green background with a high battery icon (``)
  - 81-100%: Blue background with a full battery icon (``)
- **Wi-Fi Connectivity**: Displays the Wi-Fi status (Connected/Disconnected) and changes the background color to red when disconnected.
- **Network Speed**: Displays the download and upload speed, converting to MB/s when the speed exceeds 10 KB/s.
- **Date and Time**: Displays the current date (day and month) and time in `HH:MM AM/PM` format.

## Dependencies

The status bar requires the following dependencies:

1. **Rust**: Install the Rust toolchain using `rustup`.
2. **`chrono` Crate**: Used for date and time handling.
3. **`sysinfo` Crate**: Used for system information such as CPU usage.
4. **`ifstat`**: A command-line utility to monitor network bandwidth.
5. **`xsetroot`**: root window parameter setting utility for X

### Installing Dependencies

To install Rust and Cargo, run the following commands:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

To add the necessary dependencies to your Cargo.toml, include the following:

```bash
[dependencies]
chrono = "0.4"
sysinfo = "0.23"
```

Install ifstat for monitoring network speed:

```bash
sudo apt-get install ifstat
```

### Installation

1. Clone the repository:

```bash
git clone https://github.com/Zero-A9/status-bar.git
cd status-bar
```

2. Set up your environment:

Make sure you have Rust installed and have added the required dependencies in Cargo.toml.

Build the project using Cargo:

```bash
cargo build --release
```
3. Run the compiled binary:

```bash
./target/release/status-bar
```
You can add this to your .xinitrc or .xsession file to automatically start the status bar when you start your X session.
