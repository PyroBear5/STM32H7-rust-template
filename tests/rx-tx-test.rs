#![no_std]
#![no_main]

// NOTE: This test is used to test rx and tx -> to run this test you have to connect PD8 with PD9 on you Board -> for example via jumper cable
// NOTE: Both tests below test the same thing, but it shows how you can use peripherals in multiple tests

use defmt_rtt as _;
use panic_probe as _;
use stm32h7xx_hal as _;
use stm32h7xx_hal::{
    pac,
    prelude::*,
    serial::{Rx, SerialExt, Tx},
};

/// Holds USART3 TX/RX halves for reuse across tests
pub struct SerialContext {
    tx: Tx<pac::USART3>,
    rx: Rx<pac::USART3>,
}

#[defmt_test::tests]
mod tests {
    use super::*;
    use cortex_m::asm::delay;
    use defmt::assert_eq;
    use nb::block;

    /// Initialize clocks, GPIO and UART3 once; split into TX/RX
    #[init] // Init runs once at the start and not agian
    fn init() -> SerialContext {
        // Take ownership of device peripherals
        let dp = pac::Peripherals::take().unwrap();

        // Power and clock configuration
        let pwr = dp.PWR.constrain();
        let pwrcfg = pwr.freeze();
        let rcc = dp.RCC.constrain();
        let ccdr = rcc.sys_ck(400.MHz()).freeze(pwrcfg, &dp.SYSCFG);

        // Prepare PD8 as TX and PD9 as RX for USART3
        let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);
        let tx_pin = gpiod.pd8.into_alternate();
        let rx_pin = gpiod.pd9.into_alternate();

        // Initialize USART3 at 115_200 baud
        let serial = dp
            .USART3
            .serial(
                (tx_pin, rx_pin),
                115_200.bps(),
                ccdr.peripheral.USART3,
                &ccdr.clocks,
            )
            .unwrap();
        let (tx, rx) = serial.split();

        SerialContext { tx, rx }
    }

    #[test]
    fn uart_loopback_hello(ctx: &mut SerialContext) {
        // Send and verify "HELLO"
        let test_data = b"HELLO";
        for &b in test_data {
            block!(ctx.tx.write(b)).unwrap();

            // Wait for byte to arrive, with timeout
            let mut timeout = 0;
            let received = loop {
                match ctx.rx.read() {
                    Ok(val) => break val,
                    Err(nb::Error::WouldBlock) => {
                        timeout += 1;
                        if timeout > 50_000 {
                            defmt::panic!("UART RX timeout on byte {}", b);
                        }
                        delay(100);
                    }
                    Err(_) => defmt::panic!("UART RX error"),
                }
            };

            assert_eq!(received, b);
        }
    }

    #[test]
    fn uart_loopback_abcdefg(ctx: &mut SerialContext) {
        // Send and verify "ABCDEFG"
        let test_data = b"ABCDEFG";
        for &b in test_data {
            block!(ctx.tx.write(b)).unwrap();

            let mut timeout = 0;
            let received = loop {
                match ctx.rx.read() {
                    Ok(val) => break val,
                    Err(nb::Error::WouldBlock) => {
                        timeout += 1;
                        if timeout > 50_000 {
                            defmt::panic!("UART RX timeout on byte {}", b);
                        }
                        delay(100);
                    }
                    Err(_) => defmt::panic!("UART RX error"),
                }
            };

            assert_eq!(received, b);
        }
    }
}
