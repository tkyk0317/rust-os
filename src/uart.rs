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

// HEX変換
fn send_hex(d: u8) {
    // 0x30/0x37を加算しアスキーコードへ変換
    match d < 0xA {
        true => send_u8(&(d + 0x30)),
        _ => send_u8(&(d + 0x37)),
    }
}

// 書き込み（HEX）
pub fn send_num(c: u8) {
    send("0x");
    send_hex((c & 0xF0) >> 4);
    send_hex(c & 0x0F);
    send(" ");
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
