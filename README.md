# Overview

Proof of Life is a simple command line application for controlling a USB Watchdog device on a Linux server.

This application is inspired by a [similar Python application](https://github.com/zatarra/usb-watchdog) written by [David Gouveia](https://github.com/zatarra). His [original blogpost](https://www.davidgouveia.net/2018/02/how-to-create-your-own-script-for-a-usb-watchdog/) provides a more detailed background on these devices and how they work.

In my case I was looking for a solution packaged as an all-in-one binary that avoided the hassle of installing and configuring Python and application dependencies.

# Installation

Download the `proof-of-life` binary from the latest [GitHub release](https://github.com/brettch/proof-of-life/releases) or build your own (see below).

Copy the `proof-of-life` binary to your preferred location on the target server. As an all-in-one binary there are no other files required. No configuration file is required because all parameters are specified on the command line.

To run it via systemd, create a file called `/etc/systemd/system/proof-of-life.service` with the following contents. Update paths and parameters to suit your needs.

```
[Unit]
Description=Proof of Life Watchdog service

[Service]
Type=simple
ExecStart=/path/to/proof-of-life --port-path /dev/ttyUSB0 --timeout 300
Restart=always

[Install]
WantedBy=multi-user.target

```

Start the service and enable it to start on boot.
```bash
systemctl start proof-of-life
systemctl enable proof-of-life
```

# Development

## Dependencies

In order to build this application, the following dependencies must be installed.

* A working Rust toolchain. See [rustup](https://rustup.rs/) for details. At the time of writing the latest Rust version is 1.77.0.
* libudev library. On Fedora this is part of the `systemd-devel` package.

## Useful commands

Build a fully optimised binary to `./target/release/proof_of_life`.

```bash
cargo build --release
```

Build and run a debug binary.

```bash
cargo run -- <args>
```
