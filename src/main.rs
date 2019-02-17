#![no_std]
#![no_main]

mod entry;
mod uart;

#[no_mangle]
// bootloaderからの開始ポイント
pub extern "C" fn __start_rust() -> ! {
    entry::start();
    loop{}
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn abort() -> ! {
    loop{}
}

