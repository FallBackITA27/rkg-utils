//! This crate is meant to be a library to completely and coherently access the data in RKGD files,
//! Mario Kart Wii's native Ghost Data.
//!
//! Features:
//! - [x] Reading and Writing Vanilla Game Data (including embedded Mii data)
//! - [x] Reading and Writing CTGP Modified Data
//! - [x] Reading and Writing Pulsar (Retro Rewind) Modified Data
//! - [x] Reading and Writing MKW-SP Modified Data
//! - [ ] Implementing `TryFrom<_>` for T where T: `Into<ByteHandler>`, relies on <https://github.com/rust-lang/rust/issues/31844> currently
//! - [ ] Represent at a Type-system level which types can convert from `T1` (Bytes) to `crate::byte_handler::ByteHandler` to `T2` (Typed Structs)
//! - [ ] Optimize Little-Endian calculations with `crate::byte_handler::ByteHandler`
//! - [ ] Figure out whether Big-Endian works with `crate::byte_handler::ByteHandler`

use std::{
    array::TryFromSliceError,
    io::{Read, Write},
};

use chrono::{DateTime, Duration, NaiveDateTime, TimeDelta};
use sha1::{Digest, Sha1};

use crate::{
    crc::crc32,
    ctgp_footer::CTGPFooter,
    header::{Header, mii::Mii},
    input_data::InputData,
    sp_footer::SPFooter,
};

pub mod byte_handler;
pub mod crc;
pub mod ctgp_footer;
pub mod header;
pub mod input_data;
pub mod sp_footer;

#[cfg(test)]
mod tests;

/// Errors that can occur while parsing or modifying a [`Ghost`].
#[derive(thiserror::Error, Debug)]
pub enum GhostError {
    /// The input data is shorter than the minimum valid ghost size (`0x8E` bytes).
    #[error("Data length too short for a ghost")]
    DataLengthTooShort,
    /// The RKG file header could not be parsed.
    #[error("Header Error: {0}")]
    HeaderError(#[from] header::HeaderError),
    /// The embedded Mii data could not be parsed.
    #[error("Mii Error: {0}")]
    MiiError(#[from] header::mii::MiiError),
    /// The ghost input data could not be parsed.
    #[error("Input Data Error: {0}")]
    InputDataError(#[from] input_data::InputDataError),
    /// The CTGP footer could not be parsed.
    #[error("CTGP Footer Error: {0}")]
    CTGPFooterError(#[from] ctgp_footer::CTGPFooterError),
    /// A `ByteHandler`(byte_handler::ByteHandler) operation failed.
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] byte_handler::ByteHandlerError),
    /// A slice-to-array conversion failed (e.g. when extracting a CRC-32 word).
    #[error("Try From Slice Error: {0}")]
    TryFromSliceError(#[from] TryFromSliceError),
    /// A file I/O operation failed.
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
}

/// A fully parsed Mario Kart Wii RKG ghost file.
///
/// Holds the file header, decompressed or compressed input data, optional
/// external footers (CTGP or SP), and CRC-32 checksums. All setter
/// operations update the parsed fields; call [`update_raw_data`](Ghost::update_raw_data)
/// (or [`save_to_file`](Ghost::save_to_file), which calls it implicitly) to
/// flush all changes back into the raw byte buffer before writing.
pub struct Ghost {
    /// The complete raw file bytes, kept in sync by [`update_raw_data`](Ghost::update_raw_data).
    raw_data: Vec<u8>,
    /// The parsed 136-byte RKG file header.
    header: Header,
    /// The ghost's controller input data (compressed or decompressed).
    input_data: InputData,
    /// The CRC-32 of the header and input data only, excluding any external footer.
    base_crc32: u32,
    /// The CTGP footer appended to the file, if present.
    ctgp_footer: Option<CTGPFooter>,
    /// The SP (MKW Service Pack) footer appended to the file, if present.
    sp_footer: Option<SPFooter>,
    /// The CRC-32 of the entire file excluding its final 4 bytes.
    file_crc32: u32,
    /// When `true`, any existing external footer is preserved when saving (including footer data from any mods that this crate doesn't support).
    should_preserve_external_footer: bool,
}

impl Ghost {
    /// Parses a [`Ghost`] from an RKG file at the given path.
    ///
    /// # Errors
    ///
    /// Returns [`GhostError::IOError`] if the file cannot be opened or read,
    /// and other [`GhostError`] variants if parsing fails.
    pub fn new_from_file<T: AsRef<std::path::Path>>(path: T) -> Result<Self, GhostError> {
        let mut buf = Vec::with_capacity(0x100);
        std::fs::File::open(path)?.read_to_end(&mut buf)?;
        Self::new(&buf)
    }

    /// Parses a [`Ghost`] from a byte slice.
    ///
    /// Detects and parses an optional CTGP or SP footer if present. The
    /// base CRC-32 is read from just before the footer when one is found,
    /// or from the last 4 bytes of the file otherwise.
    ///
    /// # Errors
    ///
    /// Returns [`GhostError::DataLengthTooShort`] if `bytes` is shorter than
    /// `0x8E` bytes. Returns other [`GhostError`] variants if any field fails
    /// to parse.
    pub fn new(bytes: &[u8]) -> Result<Self, GhostError> {
        if bytes.len() < 0x8E {
            return Err(GhostError::DataLengthTooShort);
        }

        let header = Header::new(&bytes[..0x88])?;

        let file_crc32 = u32::from_be_bytes(bytes[bytes.len() - 0x04..].try_into()?);
        let mut base_crc32 = file_crc32;

        let ctgp_footer = if let Ok(ctgp_footer) = CTGPFooter::new(bytes) {
            let input_data_end_offset = bytes.len() - ctgp_footer.len() - 0x08;
            base_crc32 = u32::from_be_bytes(
                bytes[input_data_end_offset..input_data_end_offset + 0x04].try_into()?,
            );
            Some(ctgp_footer)
        } else {
            None
        };

        let sp_footer = if let Ok(sp_footer) = SPFooter::new(bytes) {
            let input_data_end_offset = bytes.len() - sp_footer.len() - 0x08;
            base_crc32 = u32::from_be_bytes(
                bytes[input_data_end_offset..input_data_end_offset + 0x04].try_into()?,
            );
            Some(sp_footer)
        } else {
            None
        };

        let input_data_len = if bytes[0x8C..0x90] == *b"Yaz1" {
            u32::from_be_bytes(bytes[0x88..0x8C].try_into().unwrap()) as usize + 0x04
        } else {
            header.decompressed_input_data_length() as usize
        };

        let input_data = InputData::new(&bytes[0x88..0x88 + input_data_len])?;

        Ok(Self {
            raw_data: bytes.to_vec(),
            header,
            input_data,
            base_crc32,
            ctgp_footer,
            sp_footer,
            file_crc32,
            should_preserve_external_footer: true,
        })
    }

    /// Flushes all parsed field modifications back into the raw byte buffer.
    ///
    /// This method recomputes the Mii CRC-16, rebuilds the raw buffer from the
    /// current header and input data, resizes the buffer if the input data
    /// length has changed, re-inserts any preserved external footer, and
    /// finally recomputes both the base CRC-32 and the file-level CRC-32. The
    /// CTGP footer SHA-1 field is also updated if a CTGP footer is present.
    ///
    /// # Errors
    ///
    /// Returns [`GhostError::MiiError`] if the Mii data is invalid, or
    /// [`GhostError::CTGPFooterError`] if the SHA-1 field cannot be written.
    pub fn update_raw_data(&mut self) -> Result<(), GhostError> {
        let mii_bytes = self.header().mii().raw_data().to_vec();
        self.header_mut().set_mii(Mii::new(mii_bytes)?);
        self.header_mut().fix_mii_crc16();

        let mut buf = Vec::from(self.header().raw_data());

        buf.extend_from_slice(self.input_data().raw_data());

        let header_len = 0x88;
        let new_input_data_end = header_len + self.input_data().raw_data().len();

        // Find input data length of old data
        let old_input_data_end = if self.raw_data[0x8C..0x90] == *b"Yaz1" {
            u32::from_be_bytes(self.raw_data[0x88..0x8C].try_into().unwrap()) as usize
        } else {
            header_len + 0x2774
        };

        if new_input_data_end > old_input_data_end {
            let diff = new_input_data_end - old_input_data_end;
            let insert_pos = old_input_data_end;
            self.raw_data
                .splice(insert_pos..insert_pos, vec![0u8; diff]);
        } else if new_input_data_end < old_input_data_end {
            let diff = old_input_data_end - new_input_data_end;
            let remove_end = old_input_data_end;
            self.raw_data.drain(remove_end - diff..remove_end);
        }

        self.raw_data[..new_input_data_end].copy_from_slice(&buf[..new_input_data_end]);
        let base_crc32 = crc32(&buf);

        if let Some(ctgp_footer) = self.ctgp_footer()
            && self.should_preserve_external_footer()
        {
            buf.extend_from_slice(&base_crc32.to_be_bytes());
            buf.extend_from_slice(ctgp_footer.raw_data());

            let footer_len = ctgp_footer.len();
            self.raw_data.drain(new_input_data_end..);
            self.raw_data
                .extend_from_slice(&buf[buf.len() - footer_len - 0x04..]);
            self.raw_data.extend_from_slice(&[0u8; 4]);
        } else if let Some(sp_footer) = self.sp_footer()
            && self.should_preserve_external_footer()
        {
            buf.extend_from_slice(&base_crc32.to_be_bytes());
            buf.extend_from_slice(sp_footer.raw_data());

            let footer_len = sp_footer.len();
            self.raw_data.drain(new_input_data_end..);
            self.raw_data
                .extend_from_slice(&buf[buf.len() - footer_len - 0x04..]);
            self.raw_data.extend_from_slice(&[0u8; 4]);
        } else if !self.should_preserve_external_footer()
            && self.raw_data.len() >= new_input_data_end + 0x08
        {
            self.raw_data.drain(new_input_data_end + 0x04..);
        } else if self.should_preserve_external_footer()
            && self.raw_data.len() >= new_input_data_end + 0x08
        {
            self.raw_data[new_input_data_end..new_input_data_end + 0x04]
                .copy_from_slice(&base_crc32.to_be_bytes());
        }

        let len = self.raw_data.len();
        let crc32 = crc32(&self.raw_data[..len - 0x04]);
        self.raw_data[len - 0x04..].copy_from_slice(&crc32.to_be_bytes());

        let sha1 = compute_sha1_hex(&self.raw_data);
        if let Some(ctgp_footer) = self.ctgp_footer_mut() {
            ctgp_footer.set_ghost_sha1(&sha1)?;
        }

        Ok(())
    }

    /// Flushes all modifications and writes the ghost to a file at the given path.
    ///
    /// # Errors
    ///
    /// Returns any error from [`update_raw_data`](Ghost::update_raw_data) or
    /// from file creation/writing.
    pub fn save_to_file<T: AsRef<std::path::Path>>(&mut self, path: T) -> Result<(), GhostError> {
        self.update_raw_data()?;
        let mut file = std::fs::File::create(path)?;
        file.write_all(&self.raw_data)?;

        Ok(())
    }

    /// Compresses the input data using Yaz1 encoding and sets the compression flag in the header.
    ///
    /// Does nothing if the input data is already compressed.
    pub fn compress_input_data(&mut self) {
        if self.input_data().is_compressed() {
            return;
        }

        self.input_data_mut().compress();
        self.header_mut().set_compressed(true);
    }

    /// Decompresses the input data and clears the compression flag in the header.
    ///
    /// Does nothing if the input data is not compressed.
    pub fn decompress_input_data(&mut self) {
        if !self.input_data().is_compressed() {
            return;
        }

        self.input_data_mut().decompress();
        self.header_mut().set_compressed(false);
    }

    /// Returns the raw file bytes.
    ///
    /// May not reflect recent modifications until [`update_raw_data`](Ghost::update_raw_data) is called.
    pub fn raw_data(&self) -> &[u8] {
        &self.raw_data
    }

    /// Returns a mutable reference to the raw file bytes.
    pub fn raw_data_mut(&mut self) -> &mut [u8] {
        &mut self.raw_data
    }

    /// Returns the parsed RKG file header.
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// Returns a mutable reference to the parsed RKG file header.
    pub fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    /// Returns the ghost's controller input data.
    pub fn input_data(&self) -> &InputData {
        &self.input_data
    }

    /// Returns a mutable reference to the ghost's controller input data.
    pub fn input_data_mut(&mut self) -> &mut InputData {
        &mut self.input_data
    }

    /// Returns the CTGP footer, if present.
    pub fn ctgp_footer(&self) -> Option<&CTGPFooter> {
        self.ctgp_footer.as_ref()
    }

    /// Returns a mutable reference to the CTGP footer, if present.
    pub fn ctgp_footer_mut(&mut self) -> Option<&mut CTGPFooter> {
        self.ctgp_footer.as_mut()
    }

    /// Returns the SP footer, if present.
    pub fn sp_footer(&self) -> Option<&SPFooter> {
        self.sp_footer.as_ref()
    }

    /// Returns the CRC-32 of the header and input data, excluding any external footer.
    pub fn base_crc32(&self) -> u32 {
        self.base_crc32
    }

    /// Returns `true` if the stored base CRC-32 matches a freshly computed
    /// checksum of the current header and input data bytes.
    pub fn verify_base_crc32(&self) -> bool {
        let mut data = Vec::from(self.header().raw_data());
        data.extend_from_slice(self.input_data().raw_data());
        self.base_crc32 == crc32(&data)
    }

    /// Returns the CRC-32 of the entire file excluding its final 4 bytes.
    pub fn file_crc32(&self) -> u32 {
        self.file_crc32
    }

    /// Returns `true` if the stored file CRC-32 matches a freshly computed
    /// checksum of the entire file excluding its final 4 bytes.
    pub fn verify_file_crc32(&self) -> bool {
        let len = self.raw_data().len();
        self.file_crc32 == crc32(&self.raw_data()[..len - 0x04])
    }

    /// Returns whether an existing external footer will be preserved when saving.
    pub fn should_preserve_external_footer(&self) -> bool {
        self.should_preserve_external_footer
    }

    /// Sets whether an existing external footer should be preserved when saving.
    pub fn set_should_preserve_external_footer(&mut self, b: bool) {
        self.should_preserve_external_footer = b;
    }
}

/// Used internally for writing bits to a buffer.
pub(crate) fn write_bits(
    buf: &mut [u8],
    byte_offset: usize,
    bit_offset: usize,
    bit_width: usize,
    value: u64,
) {
    let bytes_needed = (bit_offset + bit_width).div_ceil(8);
    let mut chunk: u64 = 0;

    for i in 0..bytes_needed {
        chunk = (chunk << 8) | buf[byte_offset + i] as u64;
    }

    let shift = bytes_needed * 8 - bit_offset - bit_width;
    let mask = ((1u64 << bit_width) - 1) << shift;

    chunk = (chunk & !mask) | ((value << shift) & mask);

    for i in (0..bytes_needed).rev() {
        buf[byte_offset + i] = (chunk & 0xFF) as u8;
        chunk >>= 8;
    }
}

/// Computes the SHA-1 hash of `input` and returns it as a 20-byte array.
pub(crate) fn compute_sha1_hex(input: &[u8]) -> [u8; 0x14] {
    let mut hasher = Sha1::new();
    hasher.update(input);
    hasher.finalize().into()
}

/// Converts a raw Wii tick count into a [`NaiveDateTime`].
///
/// Ticks run at 60.75 MHz relative to the Wii epoch of 2000-01-01 00:00:00 UTC.
pub(crate) fn datetime_from_timestamp(tick_count: u64) -> NaiveDateTime {
    let clock_rate = 60_750_000.0; // 60.75 MHz tick speed
    let epoch_shift = 946_684_800; // Shifts epoch from 1970-01-01 to 2000-01-01 (which is what the Wii uses)
    let total_seconds = tick_count as f64 / clock_rate;
    let total_nanoseconds = (total_seconds * 1_000_000_000.0) as i64;

    let duration = Duration::nanoseconds(total_nanoseconds);
    let epoch = DateTime::from_timestamp(epoch_shift, 0).unwrap();

    epoch.naive_utc() + duration
}

/// Converts a raw Wii tick count into a [`TimeDelta`] (duration from an arbitrary reference).
///
/// Ticks run at 60.75 MHz; the result is truncated to millisecond precision.
pub(crate) fn duration_from_ticks(tick_count: u64) -> TimeDelta {
    let clock_rate = 60_750_000.0; // 60.75 MHz tick speed
    let total_seconds = tick_count as f64 / clock_rate;
    let total_milliseconds = (total_seconds * 1_000.0) as i64;

    Duration::milliseconds(total_milliseconds)
}
