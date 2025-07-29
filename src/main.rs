#![no_main]
#![no_std]

use {{crate_name}}::*;
use defmt_rtt as _;
use panic_probe as _;
use stm32h7xx_hal as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    hello_world();
    exit();
}
