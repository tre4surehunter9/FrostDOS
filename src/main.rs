// Copyright (c) 2026 tre4surehunter9

#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main ="test_main"]

use core::panic::PanicInfo;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32)
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn()
{
    fn run (&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]")
    }
}

mod vga_buffer;
mod serial;
mod memory;


use palladiumos::task::{Task, simple_executor::SimpleExecutor};
use bootloader::{BootInfo, entry_point};
use palladiumos::hlt_loop;
entry_point!(kernel_main);

extern crate alloc;

use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
// start function
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use palladiumos::memory::translate_addr;
    use x86_64::{structures::paging::Translate, VirtAddr};
    use palladiumos::memory;
    use x86_64::structures::paging::Page;
    use palladiumos::memory::BootInfoFrameAllocator;
    use palladiumos::allocator;
    use palladiumos::task::keyboard;
    use palladiumos::task::executor::Executor;
    use palladiumos::task::keyboard::run_shell;
    // welcome message
    palladiumos::shell::print_welcome();
    palladiumos::init();

    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
    .expect("heap initialization failed");

    palladiumos::filesystem::init_default_files();

    let mut executor = Executor::new();
    executor.spawn(Task::new(run_shell()));
    executor.run();


    palladiumos::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    palladiumos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    palladiumos::test_panic_handler(info)
}

