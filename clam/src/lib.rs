#![no_std]
use core::result::Result;

use syscalls::{Errno, Sysno, syscall};

pub mod logger;

const STDIN: usize = 0;
const STDOUT: usize = 1;
const STDERR: usize = 2;

pub fn print<B: AsRef<[u8]> + ?Sized>(message: &B) -> Result<usize, Errno> {
    let message = message.as_ref();
    unsafe { syscall!(Sysno::write, STDOUT, message.as_ptr(), message.len()) }
}

#[macro_export]
macro_rules! eprintln {
    ($fst:expr $(, $args: expr)*) => {
        let res = eprint($fst);
        $(
            let _ = res.and_then(|_| eprint($args));
        )*
    };
}

pub fn eprint<B: AsRef<[u8]> + ?Sized>(message: &B) -> Result<usize, Errno> {
    let message = message.as_ref();
    unsafe { syscall!(Sysno::write, STDERR, message.as_ptr(), message.len()) }
}

pub fn getcwd(buf: &mut [u8]) -> Result<usize, Errno> {
    unsafe { syscall!(Sysno::getcwd, buf.as_mut_ptr(), buf.len()) }
}

pub fn read(buf: &mut [u8]) -> Result<usize, Errno> {
    unsafe { syscall!(Sysno::read, STDIN, buf.as_mut_ptr(), buf.len()) }
}
pub fn exit() -> Result<usize, Errno> {
    unsafe { syscall!(Sysno::exit, 0) }
}
