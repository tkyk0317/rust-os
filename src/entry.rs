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
    let mut prev = 0;
    let mut len = 0;
    loop {
        let c = uart::recv();
        if c == 0x0d {
            uart::send("\n");

            // 前の入力がqであれば抜ける
            if len == 1 && prev == 0x71 {
                return;
            }
            uart::send("> ");

            // コマンド初期化
            len = 0;
        }
        else {
            uart::send_byte(&[c]);

            // 前の入力を保存
            prev = c;
            len += 1;
        }
    }
}

