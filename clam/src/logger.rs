#![allow(static_mut_refs)]

use heapless::String;

use crate::eprint;
use core::fmt::Write;

static mut LOG_BUFFER: String<64> = String::new();

pub struct Logger;

impl log::Log for Logger {
    fn log(&self, record: &log::Record) {
        unsafe { 
            write!(LOG_BUFFER, "{}", record.level()).unwrap();
            let _ = eprint(&LOG_BUFFER);
        }
    }
    
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }
    
    fn flush(&self) {}
}

#[cfg(test)]
mod test {
    use super::*;
    extern crate std;

    use std::println;

    #[test]
    fn test_() {
        println!("test");
    }
}