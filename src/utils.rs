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
