use std::{convert::TryInto, error::Error, fmt::Display};

use crate::oodle;

#[derive(Debug)]
enum BundleDataErr {
    DataCorrupted,
}

impl Error for BundleDataErr {}

impl Display for BundleDataErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BundleDataErr::DataCorrupted => write!(f, "data corrupted"),
        }
    }
}

struct BundleHeader {
    uncompressed_size: u32,
    total_payload_size: u32,
    header_payload_size: u32,
    compressor_id: u32,
    unknown_flag: u32,
    uncompressed_size_u64: u64,
    total_payload_size_u64: u64,
    block_count: u32,
    uncompressed_block_granularity: u32,
    block_sizes: Vec<u32>,
}

fn to_u32(x: &[u8]) -> Result<u32, BundleDataErr> {
    Ok(u32::from_le_bytes(
        x.try_into().map_err(|_| BundleDataErr::DataCorrupted)?,
    ))
}

fn to_u64(x: &[u8]) -> Result<u64, BundleDataErr> {
    Ok(u64::from_le_bytes(
        x.try_into().map_err(|_| BundleDataErr::DataCorrupted)?,
    ))
}

impl TryFrom<&[u8]> for BundleHeader {
    type Error = BundleDataErr;

    fn try_from(raw_data: &[u8]) -> Result<Self, Self::Error> {
        let uncompressed_size = to_u32(&raw_data[0..4])?;
        let total_payload_size = to_u32(&raw_data[4..8])?;
        let header_payload_size = to_u32(&raw_data[8..12])?;
        let compressor_id = to_u32(&raw_data[12..16])?;
        let unknown_flag = to_u32(&raw_data[16..20])?;
        let uncompressed_size_u64 = to_u64(&raw_data[20..28])?;
        let total_payload_size_u64 = to_u64(&raw_data[28..36])?;
        let block_count = to_u32(&raw_data[36..40])?;
        let uncompressed_block_granularity = to_u32(&raw_data[40..44])?;
        let block_sizes: Result<Vec<u32>, BundleDataErr> = (0..block_count)
            .map(|b| to_u32(&raw_data[(60 + b as usize * 4)..(64 + b as usize * 4)]))
            .collect();

        if let Err(error) = block_sizes {
            return Err(error);
        }

        Ok(Self {
            uncompressed_size,
            total_payload_size,
            header_payload_size,
            compressor_id,
            unknown_flag,
            uncompressed_size_u64,
            total_payload_size_u64,
            block_count,
            uncompressed_block_granularity,
            block_sizes: block_sizes.unwrap(),
        })
    }
}

struct Bundle {
    header: BundleHeader,
    blocks: Vec<Vec<u8>>,
}

impl Bundle {
    fn try_new(raw_data: &[u8], oodle: &oodle::Oodle) -> Result<Self, Box<dyn Error>> {
        let header = BundleHeader::try_from(raw_data)?;
        let mut block_offset = 60 + header.block_count as usize * 4;
        let blocks: Result<Vec<Vec<u8>>, BundleDataErr> = header
            .block_sizes
            .iter()
            .map(|size| {
                let raw_block = raw_data[(block_offset)..(block_offset + *size as usize)]
                    .try_into()
                    .map_err(|_| BundleDataErr::DataCorrupted)?;
                block_offset += *size as usize;
                oodle
                    .decompress(raw_block, header.uncompressed_block_granularity as usize)
                    .ok_or(BundleDataErr::DataCorrupted)
            })
            .collect();

        Ok(Self {
            header,
            blocks: blocks.unwrap(),
        })
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

struct FileIndex {}

impl BundleIndex {
    fn new(index_path: &std::path::Path) -> Self {
        let raw_data = std::fs::read(index_path).unwrap();
        Self {
            bundle_info: vec![],
        }
    }
}
