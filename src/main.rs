#![no_std]
#![no_main]
#![feature(lang_items, asm)]

mod entry;
mod uart;

#[no_mangle]
pub extern "C" fn __start_rust() -> ! {
    init();
    entry::start();
    loop {
        unsafe {
            asm!("wfi" :::: "volatile");
        }
    }
}

fn init() {
    // 割り込みを無効にし、各種設定を実施
    unsafe {
        asm!("csrw mstatus, zero");
        asm!("li a0, 0x100");
        asm!("csrw mscratch, a0");
        asm!("li a0, 0x808");
        asm!("csrw mie, a0");
        asm!("li a0, 0x80000100");
        asm!("csrw mtvec, a0");
    }

    // UARTの初期化
    uart::init();

    // 割り込み有効
    unsafe {
        asm!("csrsi mstatus, 8");
    }
}

#[link_section = ".vector_table"]
#[no_mangle]
pub extern "C" fn interrupt() {
    // シリアル割り込みチェック
    uart::send("interruptted\n");
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn abort() -> ! {
    loop {}
}

#[lang="eh_personality"]
extern fn eh_personality () {}

