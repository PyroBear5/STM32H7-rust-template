#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;
use stm32h7_template as _; // memory layout + panic handler
use stm32h7xx_hal as _;
#[defmt_test::tests]
mod tests {
    use defmt::assert;

    #[test]
    fn it_works() {
        assert!(true)
    }
}
