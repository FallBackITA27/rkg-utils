use std::{array::TryFromSliceError, io::Read};

use crate::{
    ctgp_metadata::CTGPMetadata, header::Header, input_data::InputData,
};

pub mod byte_handler;
pub mod ctgp_metadata;
pub mod header;
pub mod input_data;

/*
 * TODO:
 * Unfinished/unimplemented functionality
 * ----------------------------------------------
 * Write/save to new file, recalculate crc32
 * Be able to modify variables in ghost files
 * Implement TryFrom<_> for T where T: Into<ByteHandler>, relies on https://github.com/rust-lang/rust/issues/31844 currently
 * Represent at a Type-system level which types can convert from T to TypeHandler to whichever Struct
 * Optimize Little-Endian calculations
 * Figure out whether Big-Endian works
 */

#[cfg(test)]
mod tests;

#[derive(thiserror::Error, Debug)]
pub enum GhostError {
    #[error("Header Error: {0}")]
    HeaderError(#[from] header::HeaderError),
    #[error("Input Data Error: {0}")]
    InputDataError(#[from] input_data::InputDataError),
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] byte_handler::ByteHandlerError),
    #[error("Try From Slice Error (CRC32): {0}")]
    TryFromSliceError(#[from] TryFromSliceError),
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
}

pub struct Ghost {
    header: header::Header,
    input_data: input_data::InputData,
    ctgp_metadata: Option<ctgp_metadata::CTGPMetadata>,
    crc32: u32,
}

impl Ghost {
    pub fn new_from_file<T: AsRef<std::path::Path>>(path: T) -> Result<Self, GhostError> {
        let mut buf = Vec::with_capacity(0x100);
        std::fs::File::open(path)?.read_to_end(&mut buf)?;
        Self::new(&buf)
    }

    pub fn new(bytes: &[u8]) -> Result<Self, GhostError> {
        let input_data_end_offset;

        let header = Header::new(&bytes[..0x88])?;

        let ctgp_metadata = if let Ok(ctgp_metadata) = CTGPMetadata::new(bytes) {
            input_data_end_offset = bytes.len() - ctgp_metadata.len();
            Some(ctgp_metadata)
        } else {
            input_data_end_offset = bytes.len() - 0x04;
            None
        };

        let input_data = InputData::new(&bytes[0x88..input_data_end_offset])?;

        let crc32 = u32::from_be_bytes(bytes[bytes.len() - 0x04..].try_into()?);

        Ok(Self {
            header,
            input_data,
            ctgp_metadata,
            crc32,
        })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    pub fn input_data(&self) -> &InputData {
        &self.input_data
    }

    pub fn ctgp_metadata(&self) -> &Option<CTGPMetadata> {
        &self.ctgp_metadata
    }

    pub fn crc32(&self) -> u32 {self.crc32}
}
