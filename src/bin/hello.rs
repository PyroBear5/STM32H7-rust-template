#![no_main]
#![no_std]

use stm32h7_template as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    stm32h7_template::exit()
}
