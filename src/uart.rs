// UART0への出力
const UART0: usize = 0x1001_3000;

// 出力
pub fn send(str: &[u8]) {
    let uart = UART0 as *mut u8;
    for c in str.iter() {
        unsafe {
            *uart = *c as u8;
        }
    }
 }
