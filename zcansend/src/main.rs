use std::fs::File;
use std::io::Write;

use anyhow::Result;
use bitvec::{macros::internal::funty::Fundamental, prelude::*};
use clap::Parser;
use clap_num::maybe_hex;

const BASE_GPIO_PATH: &str = "/sys/class/gpio/gpio";

const TRIGGER_BASE: u32 = 1015;
const ID_BASE: u32 = 940;
const DLC_BASE: u32 = 936;
const DATA_BASE: u32 = 951;

struct GpioHandles {
    trigger: File,
    msg_id: Vec<File>,
    dlc: Vec<File>,
    data: Vec<File>,
}

impl GpioHandles {
    fn do_trigger(&mut self) -> Result<()> {
        self.trigger.write_all(b"0")?;
        self.trigger.write_all(b"1")?;

        Ok(())
    }

    fn write_id(&mut self, id_bits: &BitSlice<impl BitStore>) -> Result<()> {
        for (i, bit) in id_bits.iter().take(11).enumerate() {
            let bit = bit.as_u8();

            self.msg_id[i].write_all(bit.to_string().as_bytes())?;
        }

        Ok(())
    }

    fn write_dlc(&mut self, dlc_bits: &BitSlice<impl BitStore>) -> Result<()> {
        for (i, bit) in dlc_bits.iter().take(4).enumerate() {
            let bit = bit.as_u8();

            self.dlc[i].write_all(bit.to_string().as_bytes())?;
        }

        Ok(())
    }

    fn write_data_lower(&mut self, bits: &BitSlice<impl BitStore>) -> Result<()> {
        for (i, bit) in bits.iter().take(32).enumerate() {
            let bit = bit.as_u8();

            println!("lower: {i}");

            self.data[i].write_all(bit.to_string().as_bytes())?;
        }

        Ok(())
    }

    fn write_data_upper(&mut self, bits: &BitSlice<impl BitStore>) -> Result<()> {
        for (i, bit) in bits.iter().take(32).enumerate() {
            let bit = bit.as_u8();

            println!("upper: {i}");

            self.data[i + 32].write_all(bit.to_string().as_bytes())?;
        }

        Ok(())
    }
}

#[derive(Parser)]
struct Args {
    id: u16,
    dlc: u8,
    #[clap(value_parser=maybe_hex::<u64>)]
    data: u64,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut gh = GpioHandles {
        trigger: File::create(format!("{BASE_GPIO_PATH}{TRIGGER_BASE}/value")).unwrap(),
        msg_id: (ID_BASE..ID_BASE + 11)
            .into_iter()
            .map(|n| File::create(format!("{BASE_GPIO_PATH}{n}/value")).unwrap())
            .collect(),
        dlc: (DLC_BASE..DLC_BASE + 4)
            .into_iter()
            .map(|n| File::create(format!("{BASE_GPIO_PATH}{n}/value")).unwrap())
            .collect(),
        data: (DATA_BASE..DATA_BASE + 64)
            .into_iter()
            .map(|n| File::create(format!("{BASE_GPIO_PATH}{n}/value")).unwrap())
            .collect(),
    };

    println!("size of data iter: {}", gh.data.len());

    let id_bits = args.id.view_bits::<Lsb0>();
    let dlc_bits = args.dlc.view_bits::<Lsb0>();
    let data_bits_upper = (args.data & 0xFFFF_FFFF) as u32;
    let data_bits_lower = ((args.data >> 32) & 0xFFFF_FFFF) as u32;

    println!("lower: {data_bits_lower} upper: {data_bits_upper}");

    // we can't use view_bits for a 64 bit integral on a 32 bit platform, apparently

    gh.write_id(id_bits)?;
    gh.write_dlc(dlc_bits)?;
    gh.write_data_lower(data_bits_lower.view_bits::<Lsb0>())?;
    gh.write_data_upper(data_bits_upper.view_bits::<Lsb0>())?;

    // ok, now let's make a rising edge on the input trigger.
    gh.do_trigger()
}
