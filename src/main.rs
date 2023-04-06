use std::{io::Write, thread::sleep, time::Duration};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Baud rate
    #[arg(short, long, default_value = "9600")]
    baud_rate: u32,

    /// Serial port device path
    #[arg(short, long)]
    port_path: String,

    /// Heartbeat interval in seconds
    #[arg(short, long, default_value = "5")]
    update_interval: u64,

    /// Watchdog timeout before reboot in seconds
    #[arg(short, long, default_value="300", value_parser = valid_timeout)]
    timeout: u8,
}
const TIMEOUT_MIN: u32 = 0;
const TIMEOUT_MAX: u32 = 360;
const TIMEOUT_GRANULARITY: u32 = 10;

fn main() {
    let args = Args::parse();

    let mut port =
        serialport::new(args.port_path, args.baud_rate)
            .open_native()
            .expect("Unable to open serial port");

    println!("Proof of Life started");
    loop {
        port
            .write(&[args.timeout])
            .expect("Unable to write heartbeat to watchdog");
        println!("Heartbeat written to watchdog");
        sleep(Duration::from_secs(args.update_interval));
    }
}

fn valid_timeout(s: &str) -> Result<u8, String> {
    let raw_num: u32 = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a timeout number"))?;

    if raw_num < TIMEOUT_MIN || raw_num > TIMEOUT_MAX {
        return Err(format!("Timeout must be in range {}-{}", TIMEOUT_MIN, TIMEOUT_MAX))
    }

    if raw_num % TIMEOUT_GRANULARITY != 0 {
        return Err(format!("Timeout must be a multiple of 10"))
    }

    Ok((raw_num / TIMEOUT_GRANULARITY) as u8)
}
