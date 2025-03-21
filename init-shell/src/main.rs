#![no_std]
#![no_main]

use clam::{eprint, eprintln, exit, getcwd, logger::Logger, print, read};
use log::LevelFilter;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

static LOGGER: Logger = Logger;
const BUFFER_SIZE: usize = 1024;

fn parse_input(input: &[u8]) {
    eprintln!("input: ", input);
    match input {
        b"exit\n" => {
            log::debug!("EXIT");
            exit();
        }
        b"quit\n" => {
            log::debug!("EXIT");
            exit();
        }
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Trace);

    let mut buffer = [0u8; BUFFER_SIZE];
    loop {
        let _ = getcwd(buffer.as_mut());
        let _ = print(&buffer);
        let _ = print(b" $ ");
        if let Ok(input) = read(&mut buffer) {
            parse_input(&buffer[..input]);
        }
    }
}
