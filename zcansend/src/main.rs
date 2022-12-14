use std::io::Write;

use anyhow::Result;
use bitvec::{prelude::*, macros::internal::funty::Fundamental};
use lazy_static::lazy_static;

const BASE_GPIO_PATH: &str = "/sys/class/gpio/gpio";
lazy_static! {
    static ref MSGID_NUMS: Vec<u32> = (940..=950).collect();
}

fn main() -> Result<()> {
    let id: u16 = 0x010;
    let id_bits = id.view_bits::<Lsb0>();

    let mut msgid_files: Vec<std::fs::File> = MSGID_NUMS
    .iter()
    .map(|n| std::fs::File::create(format!("{BASE_GPIO_PATH}{n}/value")).unwrap())
    .collect();

    for (i, bit) in id_bits.iter().enumerate() {
        if i == 11 { break };
        let bit = bit.as_u16();

        msgid_files[i].write_all(&bit.to_string().as_bytes())?;
    }

    // ok, now let's make a rising edge on the input trigger.
    let mut trigger = std::fs::File::create(format!("{BASE_GPIO_PATH}1015/value"))?;
    trigger.write_all(b"0")?;
    trigger.write_all(b"1")?;

    Ok(())
}
