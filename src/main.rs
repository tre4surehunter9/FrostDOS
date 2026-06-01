// main.rs

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


use frostdos::task::{Task, simple_executor::SimpleExecutor};
use bootloader::{BootInfo, entry_point};
use frostdos::hlt_loop;
entry_point!(kernel_main);

extern crate alloc;

use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
// start function
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use frostdos::memory::translate_addr;
    use x86_64::{structures::paging::Translate, VirtAddr};
    use frostdos::memory;
    use x86_64::structures::paging::Page;
    use frostdos::memory::BootInfoFrameAllocator;
    use frostdos::allocator;
    use frostdos::task::keyboard;
    use frostdos::task::executor::Executor;
    // welcome message
    println!("Welcome to FrostDOS 0.1.0{}", "!");
    frostdos::init();

    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    allocator::init_heap(&mut mapper, &mut frame_allocator)
    .expect("heap initialization failed");

    let heap_value = Box::new(41);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    core::mem::drop(reference_counted);

    let x = Box::new(41);

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    frostdos::hlt_loop();
}

// in main.rs
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    frostdos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    frostdos::test_panic_handler(info)
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}
