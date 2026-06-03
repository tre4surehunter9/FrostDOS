// Copyright (c) 2026 tre4surehunter9
use alloc::string::String;
use alloc::vec::Vec;

use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

use crate::println;



pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
        ScancodeStream { _private: () }
    }
}

use core::{pin::Pin, task::{Poll, Context}};
use futures_util::stream::Stream;

use futures_util::task::AtomicWaker;

static WAKER: AtomicWaker = AtomicWaker::new();

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE
            .try_get()
            .expect("scancode queue not initialized");

        // fast path
        if let Some(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Some(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            None => Poll::Pending,
        }
    }
}


pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        println!("WARNING: scancode queue uninitialized");
    }
}

use futures_util::stream::StreamExt;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use crate::print;

pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(ScancodeSet1::new(),
                                     layouts::Us104Key, HandleControl::Ignore);

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => print!("{}", character),
                    _ => {},
                }
            }
        }
    }
}


use crate::shell;
use crate::vga_buffer::{draw_cursor, erase_cursor};

pub async fn run_shell() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(
        ScancodeSet1::new(),
                                     layouts::Us104Key,
                                     HandleControl::Ignore,
    );

    let mut input_buffer = String::new();

    print_prompt();
    draw_cursor();

    loop {
        if let Some(scancode) = scancodes.next().await {
            if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
                if let Some(key) = keyboard.process_keyevent(key_event) {
                    match key {

                        DecodedKey::Unicode('\n') => {
                            erase_cursor();
                            println!();
                            shell::process_command(&input_buffer);
                            input_buffer.clear();
                            print_prompt();
                            draw_cursor();
                        }

                        DecodedKey::Unicode('\x08') => {
                            if !input_buffer.is_empty() {
                                erase_cursor();
                                input_buffer.pop();
                                print!("\x08");
                                draw_cursor();
                            }
                        }

                        DecodedKey::Unicode(c) => {
                            erase_cursor();
                            input_buffer.push(c);
                            print!("{}", c);
                            draw_cursor();
                        }

                        DecodedKey::RawKey(_) => {}
                    }
                }
            }
        }
    }
}

fn print_prompt() {
    let cwd = crate::filesystem::CWD.lock().clone();
    crate::print!("FrostDOS:{} > ", cwd);
}
