#![no_std]
#![no_main]

use init_shell::{getcwd, print};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

const BUFFER_SIZE: usize = 1024;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut buffer = [0u8; BUFFER_SIZE];
    // let message = b"Hello world!\n";
    let _ = getcwd(buffer.as_mut());
    let _ = print(&buffer);
    let _ = print(b" $\n");
    loop {}
}