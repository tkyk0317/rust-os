extern crate cc;

use std::error::Error;
use cc::Build;

// アセンブラやCファイルをコンパイル
fn main() -> Result<(), Box<Error>> {
    Build::new()
        .file("boot.s")
        .flag("-mabi=ilp32")
        .compile("asm");

    Ok(())
}
