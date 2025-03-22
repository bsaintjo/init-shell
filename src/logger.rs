use crate::eprint;

pub struct Logger;

impl log::Log for Logger {
    fn log(&self, record: &log::Record) {
        eprint(b"logging;\n").unwrap();
    }
    
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        true
    }
    
    fn flush(&self) {}
}