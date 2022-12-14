use anyhow::*;
use lazy_static::lazy_static;

use std::fs;
use std::io::Write;
use std::thread::sleep;

const BASE_GPIO_PATH: &str = "/sys/class/gpio/gpio";
lazy_static! {
    static ref LED_NUMS: Vec<u32> = (1016..=1023).collect();
}

fn main() -> Result<()> {
    println!("Hello, zedcan!");

    let sleep_time = std::time::Duration::from_millis(50);

    // // loop {
    //     for &led in LED_NUMS.iter() {
    //         // set_led(led, true)?;
    //         // sleep(sleep_time);
    //         // set_led(led, false)?;
    //         // sleep(sleep_time);
    //     }
    // // }

    let led_files: Vec<std::fs::File> = LED_NUMS
        .iter()
        .map(|n| std::fs::File::create(format!("{BASE_GPIO_PATH}{n}/value")).unwrap())
        .collect();

    // for mut f in led_files.iter() {
        // f.write(b"0").unwrap();
        // f.write(b"1").unwrap();
    // }

    let mut f = led_files.iter().next().unwrap();
    for _ in 0..1_000_000 {
        f.write_all(b"0").unwrap();
        f.write_all(b"1").unwrap();
    }

    Ok(())
}

fn set_led(led: u32, on: bool) -> Result<()> {
    let path = format!("{BASE_GPIO_PATH}{led}/value");

    fs::write(&path, (on as u32).to_string())
        .context(format!("Failed writing to led value at {path}"));

    Ok(())
}
