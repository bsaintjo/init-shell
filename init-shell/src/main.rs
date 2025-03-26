#![no_std]
#![no_main]

use core::str;

use clam::{eprint, eprintln, exit, getcwd, logger::Logger, print, read, ShellError};
use log::LevelFilter;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

static LOGGER: Logger = Logger;
const BUFFER_SIZE: usize = 1024;

enum Command {
    Exit,
    Unknown,
}

fn parse_input(input: &str) -> anyhow::Result<Command> {
    eprintln!("input: ", input);
    let args = input
        .split_ascii_whitespace()
        .collect::<heapless::Vec<&str, 32>>();
    match args[0] {
        "exit" if args.len() == 1 => {
            log::debug!("EXIT");
            Ok(Command::Exit)
        }
        "quit" => {
            log::debug!("EXIT");
            Ok(Command::Exit)
        }
        _ => {
            print("Unknown command\n")?;
            Ok(Command::Unknown)
        }
    }
}

fn print_prompt(buffer: &mut [u8]) -> Result<(), ShellError> {
    getcwd(buffer.as_mut())?;
    print(&buffer)?;
    print(b" $ ")?;
    Ok(())
}

fn run() -> anyhow::Result<()> {
    let mut buffer = [0u8; BUFFER_SIZE];

    loop {
        print_prompt(&mut buffer)?;
        if let Ok(input) = read(&mut buffer) {
            let cmd = parse_input(str::from_utf8(&buffer[..input])?)?;
            match cmd {
                Command::Exit => {
                    exit();
                }
                Command::Unknown => {
                    continue;
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Trace);

    if let Err(_) = run() {
        eprint("Unknown command\n");
    };

    loop {}
}
