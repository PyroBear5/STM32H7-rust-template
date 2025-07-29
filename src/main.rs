#![no_main]
#![no_std]

// TODO: If you did not use cargo generate -> insert your crate_name
use {{crate_name}}::*;
use defmt_rtt as _;
use panic_probe as _;
use stm32h7xx_hal as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    hello_world();
    exit();
}
