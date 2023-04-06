# Overview

Proof of Life is a simple command line application for controlling a USB Watchdog device on a Linux server.

This application is inspired by a [similar Python application](https://github.com/zatarra/usb-watchdog) written by [David Gouveia](https://github.com/zatarra). His [original blogpost](https://www.davidgouveia.net/2018/02/how-to-create-your-own-script-for-a-usb-watchdog/) provides a more detailed background on these devices and how they work.

In my case I was looking for a solution packaged as an all-in-one binary that avoided the hassle of installing and configuring Python and application dependencies.

# Development

## Dependencies

In order to build this application, the following dependencies must be installed.

* A working Rust toolchain. See [rustup](https://rustup.rs/) for details. At the time of writing the latest Rust version is 1.68.2.
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
