// UART0 configuration register
const UART0_TXFIFO: u32 = 0x1001_3000;
const UART0_RXFIFO: u32 = 0x1001_3004;
const UART0_TXCTRL: u32 = 0x1001_3008;
const UART0_RXCTRL: u32 = 0x1001_300C;
const UART0_IE: u32 = 0x1001_3010;
const UART0_IP: u32 = 0x1001_3014;
const UART0_DIV: u32 = 0x1001_3018;

const FIFO_FULL: u32 = 0x8000_0000;
const FIFO_EMPTY: u32 = 0x8000_0000;

const NUMBER: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

// 初期化
pub fn init() {
    let div = UART0_DIV as *mut u32;
    let rxctrl = UART0_RXCTRL as *mut u32;
    unsafe {
        *rxctrl = 0x0000_0001;
        *div |= 6510;
    }
}

// 書き込み
pub fn send(str: &str) {
    str.as_bytes().iter().for_each(|c| send_u8(c));
}

// 書き込み（u8スライス）
pub fn send_byte(str: &[u8]) {
    str.iter().for_each(|c| send_u8(c));
}

// 書き込み（u8）
fn send_u8(c: &u8) {
    unsafe {
        // 書き込みチェック
        let txfifo = UART0_TXFIFO as *mut u32;
        while FIFO_FULL == *txfifo & FIFO_FULL {};

        // データ送信
        *(txfifo as *mut u8) = *c;
    }
}

// 書き込み（HEX）
fn send_num(c: &u8) {
    match *c {
        0 => send("0"),
        1 => send("1"),
        2 => send("2"),
        3 => send("3"),
        4 => send("4"),
        5 => send("5"),
        6 => send("6"),
        7 => send("7"),
        8 => send("8"),
        9 => send("9"),
        _ => send("Not Support Number")
    }
}

// 読み込み
pub fn recv() -> u8 {
    // データ受信チェック
    unsafe {
        let rxfifo = UART0_RXFIFO as *mut u32;
        let mut c = *rxfifo; // アクセスするとData部が消えるので一旦退避
        while FIFO_EMPTY == c & FIFO_FULL {
            c = *rxfifo;
        }
        c as u8
    }
}
