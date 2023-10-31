use std::fs;

fn main() {
    let dir = std::env::var("OUT_DIR").unwrap();

    fs::write(format!("{dir}/file.txt"), "hello World").unwrap();
}
