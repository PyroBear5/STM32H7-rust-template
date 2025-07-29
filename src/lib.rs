#![no_std]
use cortex_m_rt as rt;
use cortex_m_semihosting::debug;

/// Exit for the program
pub fn exit() -> ! {
    loop {
        debug::exit(debug::EXIT_SUCCESS);
    }
}

/// Hardfault-Handler: Exit with error
#[rt::exception]
unsafe fn HardFault(_frame: &rt::ExceptionFrame) -> ! {
    loop {
        debug::exit(debug::EXIT_FAILURE);
    }
}

/// Same as hardfault-handler, but does not display twice
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

pub fn hello_world() {
    defmt::println!("Hello, world!");
}

pub fn add(n1: i32, n2: i32) -> i32 {
    // Example function, which can easily get tested
    n1 + n2
}
