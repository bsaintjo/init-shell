#![no_std]
use core::result::Result;

use syscalls::{syscall, Errno, Sysno};

const STDOUT: usize = 1;

pub fn print(message: &[u8]) -> Result<usize, Errno> {
    unsafe { syscall!(Sysno::write, STDOUT, message.as_ptr(), message.len()) }
}

pub fn getcwd(buf: &mut [u8]) -> Result<usize, Errno> {
    unsafe { syscall!(Sysno::getcwd, buf.as_mut_ptr(), buf.len()) }
}