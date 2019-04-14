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
    // 割り込み無効化
    unsafe {
        asm!("csrw mstatus, zero");
        asm!("csrw mie, zero");
    }

    // UARTの初期化
    uart::init();

    // timer割り込み初期化
    init_timer();

    // 割り込み有効化
    unsafe {
        // mscratch設定
        asm!("li a0, 0x100");
        asm!("csrw mscratch, a0");

        // 割り込み有効
        asm!("csrsi mstatus, 8");
        asm!("li a0, 0x080");
        asm!("csrw mie, a0");
     }
}

fn init_timer() {
    // タイマー値初期化
    let mtimecmp = 0x0200_4000 as *mut u64;
    let mtime = 0x0200_BFF8 as *const u64;
    unsafe {
        *mtimecmp = (*mtime + 1000);
    }
}

#[no_mangle]
pub extern "C" fn __interrupt(mcause: u32) {
    // timer割り込み判定
    if mcause == 0x80000007 {
        timer_interrupt();
    }
    else {
        uart::send("no timer\n");
    }
}

// timer割り込み処理
fn timer_interrupt() {
    // mtimecmp更新
    let mtimecmp = 0x0200_4000 as *mut u64;
    let mtime = 0x0200_BFF8 as *const u64;
    unsafe {
        *mtimecmp = (*mtime + 1000);
    }
    uart::send("timer interrupt\n");
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

