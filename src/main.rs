use std::vec;

mod oodle;

fn main() {
  let oodle = oodle::Oodle::new("resources/oo2core_8_win64.dll");
  let data = std::fs::read("./lenna.bmp").unwrap();

  let mut dst = vec![];
  oodle.compress(&data, &mut dst, oodle::Compressor::Kraken, oodle::Level::Normal);

  println!("src_len {} dst_len {}", data.len(), dst.len());
}
