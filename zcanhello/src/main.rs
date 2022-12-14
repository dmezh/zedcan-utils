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

    let led_files: Vec<std::fs::File> = LED_NUMS
        .iter()
        .map(|n| std::fs::File::create(format!("{BASE_GPIO_PATH}{n}/value")).unwrap())
        .collect();

    for mut f in led_files.iter() {
        f.write_all(b"0").unwrap();
        sleep(sleep_time);
        f.write_all(b"1").unwrap();
        sleep(sleep_time);
    }

    Ok(())
}
