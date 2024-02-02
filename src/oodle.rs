use std::{error::Error, ffi::CString, fmt::Display};

#[link(name = "oodlerelay")]
extern "C" {
    fn oodlerelay_init(path: *const libc::c_char) -> i64;
    fn oodle_compress(
        compressor: i32,
        level: u64,
        src: *const u8,
        srclen: u64,
        dst: *mut u8,
    ) -> i64;
    #[allow(unused)]
    fn oodle_decompress(src: *const u8, srclen: u64, dst: *mut u8, dstlen: u64) -> i64;
    fn oodle_get_compressed_buffer_size_needed(compressor: u8, srclen: u64) -> i64;
}

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
#[allow(unused)]
pub enum Compressor {
    LZH,
    LZHLW,
    LZNIB,
    None,
    LZB16,
    LZBLW,
    LZA,
    LZNA,
    Kraken,
    Mermaid,
    BitKnit,
    Selkie,
    Hydra,
    Leviathan,
}

#[derive(Debug, Clone, Copy)]
#[repr(u64)]
#[allow(unused)]
pub enum Level {
    None,
    SuperFast,
    VeryFast,
    Fast,
    Normal,
    Optimal1,
    Optimal2,
    Optimal3,
    Optimal4,
    Optimal5,
    Optimal6,
    Optimal7,
    Optimal8,
    Optimal9,
}

pub struct Oodle {
    relay_init_status: i64,
}

impl Oodle {
    // TODO: make the path recognizable as an environment variable
    pub fn new(path: &str) -> Self {
        Self {
            relay_init_status: unsafe {
                let oodle_path = CString::new(path).unwrap();
                oodlerelay_init(oodle_path.as_ptr())
            },
        }
    }

    pub fn compress(&self, src: &[u8], compressor: Compressor, level: Level) -> Vec<u8> {
        let mut dst = vec![];

        if self.relay_init_status < 0 {
            panic!("Oodle not initialized");
        }

        let required_len =
            unsafe { oodle_get_compressed_buffer_size_needed(compressor as u8, src.len() as u64) }
                as usize;
        dst.resize(required_len, 0);

        let len = unsafe {
            oodle_compress(
                compressor as i32,
                level as u64,
                src.as_ptr(),
                src.len() as u64,
                dst.as_mut_ptr(),
            )
        } as usize;
        dst.resize(len, 0);

        dst
    }

    #[allow(unused)]
    pub fn decompress(&self, src: &[u8]) -> Option<Vec<u8>> {
        let mut dst = vec![];

        if self.relay_init_status < 0 {
            panic!("Oodle not initialized");
        }

        let len = unsafe {
            oodle_decompress(
                src.as_ptr(),
                src.len() as u64,
                dst.as_mut_ptr(),
                dst.len() as u64,
            )
        } as usize;

        if len != dst.len() {
            return None;
        }

        Some(dst)
    }
}

#[cfg(test)]
mod tests {
    use super::Oodle;

    #[test]
    fn test_normal_kraken_compression() {
        let input: Vec<u8> = vec![1; 10];

        let oodle = Oodle::new("resources/oo2core_8_win64.dll");

        let actual = oodle.compress(&input, super::Compressor::Kraken, super::Level::Normal);

        let expected = vec![204, 6, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_decompression() {
        let input: Vec<u8> = vec![204, 6, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

        let oodle = Oodle::new("resources/oo2core_8_win64.dll");

        let actual = oodle.decompress(&input);

        let expected = vec![1; 10];

        assert!(actual.is_some());
        assert_eq!(actual.unwrap(), expected);
    }
}
