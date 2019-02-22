use crate::uart;

pub fn start() {
    uart::init();
    uart::send("Boot Start\n");
    shell();
    uart::send("Boot End\n");
}

// シェル
fn shell() {
    uart::send("> ");
    loop {
        let c = uart::recv();
        if c == 0x0a || c == 0x0d {
            uart::send("\n");
            uart::send("> ");
        }
        else {
            uart::send_byte(&[c]);
        }
    }
}

