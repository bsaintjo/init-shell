#![no_std]
use core::result::Result;

use syscalls::{Sysno, syscall};

pub use syscalls::Errno;

pub mod logger;

const STDIN: usize = 0;
const STDOUT: usize = 1;
const STDERR: usize = 2;

#[derive(Debug, thiserror::Error)]
pub enum ShellError {
    #[error("Syscall error")]
    Errno(Errno),

    #[error("Unknown command")]
    UnknownCommand,
}

impl From<Errno> for ShellError {
    fn from(errno: Errno) -> Self {
        ShellError::Errno(errno)
    }
}

pub fn print<B: AsRef<[u8]> + ?Sized>(message: &B) -> Result<usize, ShellError> {
    let message = message.as_ref();
    unsafe {
        syscall!(Sysno::write, STDOUT, message.as_ptr(), message.len()).map_err(ShellError::from)
    }
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

pub fn eprint<B: AsRef<[u8]> + ?Sized>(message: &B) -> Result<usize, ShellError> {
    let message = message.as_ref();
    unsafe {
        syscall!(Sysno::write, STDERR, message.as_ptr(), message.len()).map_err(ShellError::from)
    }
}

pub fn getcwd(buf: &mut [u8]) -> Result<usize, ShellError> {
    unsafe { syscall!(Sysno::getcwd, buf.as_mut_ptr(), buf.len()).map_err(ShellError::from) }
}

pub fn read(buf: &mut [u8]) -> Result<usize, ShellError> {
    unsafe { syscall!(Sysno::read, STDIN, buf.as_mut_ptr(), buf.len()).map_err(ShellError::from) }
}

pub fn exit() -> Result<(), Errno> {
    let _ = unsafe { syscall!(Sysno::exit, 0) };
    Ok(())
}
