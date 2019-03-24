use crate::uart;

pub fn start() {
    uart::send("Boot Start\n");
    shell();
    uart::send("Boot End\n");
}

// シェル
fn shell() {
    uart::send("> ");
    let mut i = 0;
    let mut buf = [0x00u8; 128];
    loop {
        let c = uart::recv();

        // 改行コードをキャッチ
        if c == 0x0d {
            uart::send("\n");

            // 前の入力がquitであれば抜ける
            if i == 4 && true == is_quit(&buf[0..i]) {
                return;
            }

            // バッファ位置を初期化し、プロンプト表示
            i = 0;
            uart::send("> ");
        }
        else {
            uart::send_byte(&[c]);

            // 長さを超えた場合、巻き戻る
            if i > buf.len() - 1 {
                i = 0;
            }

            // 入力文字を保存
            buf[i] = c;
            i += 1;
        }
    }
}

// quitコマンド確認
fn is_quit(buf: &[u8]) -> bool {
    buf[0] as char == 'q' && buf[1] as char == 'u' && buf[2] as char == 'i' && buf[3] as char == 't'
}
