use std::{collections::btree_map::Range, io::{BufRead, Read}, os::raw, vec};
use std::convert::TryInto;

use crate::oodle;

struct BundleHeader {
  uncompressed_size: u32,
  total_payload_size: u32,
  block_count: u32,
  uncompressed_block_granularity: u32,
  block_sizes: Vec<u32>,
}

struct Bundle {
  header: BundleHeader,
  blocks: Vec<Vec<u8>>,
}

impl Bundle {
  fn from(raw_data: &Vec<u8>, oodle: &oodle::Oodle) -> Self {
    let uncompressed_size = u32::from_le_bytes(raw_data[0..4].try_into().expect("data corrupted"));
    let total_payload_size = u32::from_le_bytes(raw_data[4..8].try_into().expect("data corrupted"));
    let block_count = u32::from_le_bytes(raw_data[0x24..0x28].try_into().expect("data corrupted"));
    let uncompressed_block_granularity = 0; // unknown
    let block_sizes = (0..block_count).map(|b| {
      u32::from_le_bytes(raw_data[(0x28 + b as usize * 4)..(0x2C + b as usize * 4)].try_into().expect("data corrputed"))
    }).collect();

    let header = BundleHeader {
      uncompressed_size,
      total_payload_size,
      block_count,
      uncompressed_block_granularity,
      block_sizes
    };

    let mut block_offset = 0x3C + block_count as usize * 4;
    let blocks = header.block_sizes.iter().map(|size| {
      let raw_block = raw_data[(block_offset)..(block_offset + *size as usize)].try_into().expect("data corrupted");
      block_offset += *size as usize;
      oodle.decompress(raw_block, 256 * 1024).unwrap()
    }).collect();

    Self {
      header,
      blocks
    }
  }
}

struct BundleIndexInfo {
  name: String,  
  uncompressed_size: u32,
}

pub struct BundleIndex {
  bundle_info: Vec<BundleIndexInfo>,
}

struct FileInfo {
  hash: u64,
  bundle_index: u32,
  offset: u32,
  size: u32,
}

struct FileIndex {

}

impl BundleIndex {
  fn new(index_path: &std::path::Path) -> Self {
    let raw_data = std::fs::read(index_path).unwrap();
    Self {
      bundle_info: vec![],
    }
  }
}