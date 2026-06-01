use crate::println;
use alloc::vec::Vec;

pub fn process_command(input: &str) {
    let input = input.trim();
    if input.is_empty() {
        return;
    }

    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let command = parts[0];
    let args = if parts.len() > 1 { parts[1] } else { "" };

    match command {
        "help" => cmd_help(),
        "echo" => cmd_echo(args),
        "clear" => cmd_clear(),
        "about" => cmd_about(),
        _ => {
            println!("Unknown command: '{}'. Type 'help' for a list of commands.", command);
        }
    }
}


fn cmd_help() {
    println!("Available commands:");
    println!("  help        - Show this help message");
    println!("  echo <text> - Print text to the screen");
    println!("  clear       - Clear the screen");
    println!("  about       - Show information about this kernel");
}

fn cmd_echo(args: &str) {
    println!("{}", args);
}

fn cmd_clear() {
   crate::vga_buffer::clear_screen();
}

fn cmd_about() {
    println!("FrostDOS v0.2.0 - A kernel in Rust");
    println!("Based on Philipp Oppermann's 'Writing an OS in Rust'");
    println!("https://os.phil-opp.com/");
}

pub fn print_welcome() {
    println!("FrostDOS v0.2.0");
    println!("----------------------------------------");
}
