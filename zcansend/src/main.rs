use anyhow::*;

use std::fs;
use std::thread::sleep;

const BASE_GPIO_PATH: &str = "/sys/class/gpio/gpio";

fn main() -> Result<()> {
    println!("Hello, zedcan!");

    let led_nums = 1016..=1023;

    let sleep_time = std::time::Duration::from_millis(200);

    for led in led_nums {
        set_led(led, true)?;
        sleep(sleep_time);
        set_led(led, false)?;
        sleep(sleep_time);
    }

    Ok(())
}

fn set_led(led: u32, on: bool) -> Result<()> {
    let path = format!("{BASE_GPIO_PATH}{led}/value");

    fs::write(&path, (on as u32).to_string()).context(format!("Failed writing to led value at {path}"))
}
