use crate::oodle::Oodle;

mod oodle;

fn main() {
    let oodle = Oodle::new("resources/oo2core_8_win64.dll");
    let data = std::fs::read("./lenna.png").unwrap();

    let dst = oodle.compress(&data, oodle::Compressor::Kraken, oodle::Level::Normal);

    println!("src_len {} dst_len {}", data.len(), dst.len());
}
