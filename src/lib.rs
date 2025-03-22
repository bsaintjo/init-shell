#![no_std]
use core::result::Result;

use syscalls::{syscall, Errno, Sysno};

pub mod logger;

const STDIN: usize = 0;
const STDOUT: usize = 1;
const STDERR: usize = 2;

pub fn print(message: &[u8]) -> Result<usize, Errno> {
    unsafe { syscall!(Sysno::write, STDOUT, message.as_ptr(), message.len()) }
}

pub fn eprint(message: &[u8]) -> Result<usize, Errno> {
    unsafe { syscall!(Sysno::write, STDERR, message.as_ptr(), message.len()) }
}

pub fn getcwd(buf: &mut [u8]) -> Result<usize, Errno> {
    unsafe { syscall!(Sysno::getcwd, buf.as_mut_ptr(), buf.len()) }
}

pub fn read(buf: &mut [u8]) -> Result<usize, Errno> {
    unsafe { syscall!(Sysno::read, STDIN, buf.as_mut_ptr(), buf.len()) }
}