use std::{convert::TryInto, error::Error, fmt::Display};

use crate::oodle;

#[derive(Debug)]
pub enum BundleDataErr {
    DataCorrupted,
    DecompressionFailed,
}

impl Error for BundleDataErr {}

impl Display for BundleDataErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BundleDataErr::DataCorrupted => write!(f, "data corrupted"),
            BundleDataErr::DecompressionFailed => write!(f, "decompression failed"),
        }
    }
}

#[derive(Debug)]
pub struct BundleHeader {
    uncompressed_size: u32,
    total_payload_size: u32,
    header_payload_size: u32,
    compressor_id: u32,
    unknown_flag: u32,
    uncompressed_size_u64: u64,
    total_payload_size_u64: u64,
    block_count: u32,
    block_granularity: u32,
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
        let block_granularity = to_u32(&raw_data[40..44])?;
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
            block_granularity,
            block_sizes: block_sizes.unwrap(),
        })
    }
}

pub struct Bundle {
    pub(crate) header: BundleHeader,
    pub(crate) blocks: Vec<Vec<u8>>,
}

impl Bundle {
    pub fn try_new(raw_data: &[u8], oodle: &oodle::Oodle) -> Result<Self, Box<dyn Error>> {
        let header = BundleHeader::try_from(raw_data)?;
        let mut block_offset = 60 + header.block_count as usize * 4;
        let mut data_left_size = header.uncompressed_size as usize;
        let blocks: Result<Vec<Vec<u8>>, BundleDataErr> = header
            .block_sizes
            .iter()
            .map(|size| {
                let raw_block = raw_data[(block_offset)..(block_offset + *size as usize)]
                    .try_into()
                    .map_err(|_| BundleDataErr::DataCorrupted)?;
                block_offset += *size as usize;
                let block_uncompressed_size =
                    core::cmp::min(data_left_size, header.block_granularity as usize);
                let data = oodle
                    .decompress(raw_block, block_uncompressed_size)
                    .ok_or(BundleDataErr::DecompressionFailed);
                data_left_size -= block_uncompressed_size;
                data
            })
            .collect();

        Ok(Self {
            header,
            blocks: blocks.unwrap(),
        })
    }
}

#[derive(Debug)]
pub struct IndexBundleInfo {
    pub name: String,
    pub uncompressed_size: u32,
}

#[derive(Debug)]
pub struct IndexFileInfo {
    pub hash: u64,
    pub bundle_index: u32,
    pub offset: u32,
    pub size: u32,
}

#[derive(Debug)]
pub struct IndexPathInfo {
    pub hash: u64,
    pub payload_offset: u32,
    pub payload_size: u32,
    pub payload_rec_size: u32,
}

#[derive(Debug)]
pub struct IndexBundle {
    pub bundle_infos: Vec<IndexBundleInfo>,
    pub bundle_files: Vec<IndexFileInfo>,
    pub bundle_paths: Vec<IndexPathInfo>,
}

impl TryFrom<&[u8]> for IndexBundle {
    type Error = BundleDataErr;

    fn try_from(raw_data: &[u8]) -> Result<Self, Self::Error> {
        let index_count = to_u32(&raw_data[0..4])?;
        let mut offset = 4;
        let bundle_infos: Result<Vec<IndexBundleInfo>, BundleDataErr> = (0..index_count)
            .map(|_| {
                let name_len = to_u32(&raw_data[offset..(offset + 4)])? as usize;
                let name_raw = raw_data[(offset + 4)..(offset + name_len + 4)]
                    .try_into()
                    .map_err(|_| BundleDataErr::DataCorrupted);
                let uncompressed_size =
                    to_u32(&raw_data[(offset + name_len + 4)..(offset + name_len + 8)])?;

                offset += 8 + name_len;

                Ok(IndexBundleInfo {
                    name: String::from_utf8(name_raw.unwrap()).unwrap(),
                    uncompressed_size,
                })
            })
            .collect();

        if let Err(error) = bundle_infos {
            return Err(error);
        }

        let file_count = to_u32(&raw_data[offset..(offset + 4)])?;
        offset += 4;
        let file_infos: Result<Vec<IndexFileInfo>, BundleDataErr> = (0..file_count)
            .map(|_| {
                let hash = to_u64(&raw_data[offset..(offset + 8)])?;
                let bundle_index = to_u32(&raw_data[(offset + 8)..(offset + 12)])?;
                let file_offset = to_u32(&raw_data[(offset + 12)..(offset + 16)])?;
                let size = to_u32(&raw_data[(offset + 16)..(offset + 20)])?;

                offset += 20;

                Ok(IndexFileInfo {
                    hash,
                    bundle_index,
                    offset: file_offset,
                    size,
                })
            })
            .collect();

        if let Err(error) = file_infos {
            return Err(error);
        }

        let path_count = to_u32(&raw_data[offset..(offset + 4)])?;
        offset += 4;
        let path_infos: Result<Vec<IndexPathInfo>, BundleDataErr> = (0..path_count)
            .map(|_| {
                let hash = to_u64(&raw_data[offset..(offset + 8)])?;
                let payload_offset = to_u32(&raw_data[(offset + 8)..(offset + 12)])?;
                let payload_size = to_u32(&raw_data[(offset + 12)..(offset + 16)])?;
                let payload_rec_size = to_u32(&raw_data[(offset + 16)..(offset + 20)])?;

                offset += 20;

                Ok(IndexPathInfo {
                    hash,
                    payload_offset,
                    payload_size,
                    payload_rec_size,
                })
            })
            .collect();

        if let Err(error) = path_infos {
            return Err(error);
        }

        Ok(Self {
            bundle_infos: bundle_infos.unwrap(),
            bundle_files: file_infos.unwrap(),
            bundle_paths: path_infos.unwrap(),
        })
    }
}
