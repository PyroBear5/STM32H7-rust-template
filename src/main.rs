#![no_main]
#![no_std]

mod utils; // to use utils.rs

use defmt_rtt as _; // global logger
use panic_probe as _; // panic handler
use stm32h7xx_hal as _; // HAL

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");
    utils::exit()
}
