#![no_std]
#![no_main]

use syscalls::{syscall, Sysno};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
const STDOUT: usize = 1;

fn print(message: &[u8]) -> Result<(), ()>{
    match unsafe { syscall!(Sysno::write, STDOUT, message.as_ptr(), message.len()) } {
        Ok(usize) => Ok(()),
        Err(e) => Err(()),
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let message = b"Hello world!\n";
    let _ = print(message);
    loop {}
}