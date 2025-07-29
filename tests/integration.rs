#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;
use {{crate_name}} as _; // memory layout + panic handler
use stm32h7xx_hal as _;

#[defmt_test::tests]
mod tests {
    use defmt::assert_eq;

    #[test]
    fn test_function_returns_4() {
        assert_eq!({{crate_name}}::add(1, 2), 3);
    }
}
