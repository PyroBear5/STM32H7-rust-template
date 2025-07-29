#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;
// TODO: If you did not use cargo generate -> insert your crate_name
use {{crate_name}} as _;
use stm32h7xx_hal as _;

#[defmt_test::tests]
mod tests {
    use defmt::assert_eq;

    #[test]
    fn test_add_function() {
        // TODO: If you did not use cargo generate -> insert your crate_name
        assert_eq!({{crate_name}}::add(1, 2), 3);

    }
}
