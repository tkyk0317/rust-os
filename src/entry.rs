use crate::uart;

pub fn start() {
    uart::send(b"Booting...\n");
    uart::send(b"Boot End!!!\n");
}

