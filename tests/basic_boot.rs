#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(frostdos::test_runner)]
#![reexport_test_harness_main= "test_main"]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

use frostdos::exit_qemu;
use frostdos::Testable;
use frostdos::QemuExitCode;
use frostdos::serial_println;



#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    frostdos::test_panic_handler(info)
}

use frostdos::println;

#[test_case]
fn test_println() {
    println!("test_println output");
}
