use crate::{
    byte_handler::{ByteHandler, ByteHandlerError, FromByteHandler},
    crc::crc16,
    header::{
        combo::{Combo, ComboError},
        controller::{Controller, ControllerError},
        date::{Date, DateError},
        ghost_type::{GhostType, GhostTypeError},
        in_game_time::{InGameTime, InGameTimeError},
        location::{
            Location,
            constants::{Country, CountryError, SubregionError, Version},
        },
        mii::{Mii, MiiError},
        slot_id::{SlotId, SlotIdError},
        transmission_mod::{TransmissionMod, TransmissionModError},
    },
    write_bits,
};

use std::io::Read;

pub mod combo;
pub mod controller;
pub mod date;
pub mod ghost_type;
pub mod in_game_time;
pub mod location;
pub mod mii;
pub mod slot_id;
pub mod transmission_mod;

/// Errors that can occur while parsing or modifying a [`Header`].
#[derive(thiserror::Error, Debug)]
pub enum HeaderError {
    /// The file does not begin with the `RKGD` magic bytes.
    #[error("File is not RKGD")]
    NotRKGD,
    /// The input slice is not exactly `0x88` bytes long.
    #[error("Data passed is not correct size (0x88)")]
    NotCorrectSize,
    /// A friend ghost number was specified outside the valid range of 1–30.
    #[error("Friend ghost number out of range (1-30)")]
    FriendNumberOutOfRange,
    /// A lap split idx was out of bounds for the recorded lap count.
    #[error("Lap split idx not semantically valid")]
    LapSplitIndexError,
    /// A finish or lap time field could not be parsed.
    #[error("In Game Time Error: {0}")]
    InGameTimeError(#[from] InGameTimeError),
    /// The slot ID field could not be parsed.
    #[error("Slot ID Error: {0}")]
    SlotIdError(#[from] SlotIdError),
    /// The character/vehicle combo field could not be parsed.
    #[error("Combo Error: {0}")]
    ComboError(#[from] ComboError),
    /// The date field could not be parsed.
    #[error("Date Error: {0}")]
    DateError(#[from] DateError),
    /// The controller field could not be parsed.
    #[error("Controller Error: {0}")]
    ControllerError(#[from] ControllerError),
    /// The transmission mod field could not be parsed.
    #[error("Transmission Mod Error: {0}")]
    TransmissionModError(#[from] TransmissionModError),
    /// The ghost type field could not be parsed.
    #[error("Ghost Type Error: {0}")]
    GhostTypeError(#[from] GhostTypeError),
    /// The embedded Mii data could not be parsed.
    #[error("Mii Error: {0}")]
    MiiError(#[from] MiiError),
    /// A file I/O operation failed.
    #[error("Io Error: {0}")]
    IoError(#[from] std::io::Error),
    /// The country code could not be parsed.
    #[error("Country Error: {0}")]
    CountryError(#[from] CountryError),
    /// The subregion code could not be parsed.
    #[error("Subregion Error: {0}")]
    SubregionError(#[from] SubregionError),
    /// A `ByteHandler` operation failed.
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
}

/// The parsed 136-byte (`0x88`) header of a Mario Kart Wii RKG ghost file.
///
/// Holds all metadata decoded from the RKGD file header, along with a copy of
/// the raw bytes kept in sync with every setter. The layout is documented at
/// <https://wiki.tockdom.com/wiki/RKG_(File_Format)#File_Header>.
pub struct Header {
    /// The raw 136-byte header block, kept in sync with all parsed fields.
    raw_data: [u8; 0x88],
    /// The ghost's recorded finish time.
    finish_time: InGameTime,
    /// The course slot the ghost was recorded on.
    slot_id: SlotId,
    /// The character and vehicle used.
    combo: Combo,
    /// The calendar date the ghost was set.
    date_set: Date,
    /// The input controller used to record the ghost.
    controller: Controller,
    /// Whether the ghost's input data is Yaz compressed.
    is_compressed: bool,
    /// The Retro Rewind transmission override active when the ghost was recorded.
    transmission_mod: TransmissionMod,
    /// The storage slot or origin category of the ghost.
    ghost_type: GhostType,
    /// Whether automatic drift (as opposed to manual) was used.
    is_automatic_drift: bool,
    /// The byte length of the input data after decompression.
    decompressed_input_data_length: u16,
    /// The number of laps recorded in the ghost.
    lap_count: u8,
    /// Per-lap split times; only the first [`lap_count`](Header::lap_count) entries are valid.
    lap_split_times: [InGameTime; 11],
    /// The player's geographic location at the time the ghost was set.
    location: Location,
    /// The Mii character embedded in the ghost header.
    mii: Mii,
    /// The CRC-16 checksum of the embedded Mii data (`0x3C`–`0x85` inclusive).
    mii_crc16: u16,
}

impl Header {
    /// Parses a [`Header`] from an RKG file at the given path.
    ///
    /// Only the first `0x88` bytes of the file are read.
    ///
    /// # Errors
    ///
    /// Returns [`HeaderError::IoError`] if the file cannot be opened or read,
    /// and other [`HeaderError`] variants if any field fails to parse.
    pub fn new_from_path<P: AsRef<std::path::Path>>(p: P) -> Result<Self, HeaderError> {
        let mut rkg_data = [0u8; 0x88];
        std::fs::File::open(p)?.read_exact(&mut rkg_data)?;
        Self::new(&rkg_data)
    }

    /// Parses a [`Header`] from a 136-byte (`0x88`) slice.
    ///
    /// # Errors
    ///
    /// Returns [`HeaderError::NotCorrectSize`] if `header_data` is not exactly
    /// `0x88` bytes long. Returns [`HeaderError::NotRKGD`] if the first four
    /// bytes are not the `RKGD` magic. Returns other [`HeaderError`] variants
    /// if any individual field fails to parse.
    pub fn new(header_data: &[u8]) -> Result<Self, HeaderError> {
        if header_data.len() != 0x88 {
            return Err(HeaderError::NotCorrectSize);
        }
        if header_data[0..4] != [0x52, 0x4B, 0x47, 0x44] {
            return Err(HeaderError::NotRKGD);
        }

        let finish_time = InGameTime::from_byte_handler(&header_data[4..7])?;
        let slot_id = SlotId::from_byte_handler(header_data[7])?;
        let combo = Combo::from_byte_handler(&header_data[0x08..0x0A])?;
        let date_set = Date::from_byte_handler(&header_data[0x09..=0x0B])?;
        let controller = Controller::from_byte_handler(header_data[0x0B])?;
        let is_compressed = ByteHandler::from(header_data[0x0C]).read_bool(3);
        let transmission_mod = TransmissionMod::from_byte_handler(header_data[0x0C])?;
        let ghost_type = GhostType::from_byte_handler(&header_data[0x0C..=0x0D])?;
        let is_automatic_drift = ByteHandler::from(header_data[0x0D]).read_bool(1);
        let decompressed_input_data_length =
            ByteHandler::try_from(&header_data[0x0E..=0x0F])?.copy_word(0);

        let lap_count = header_data[0x10];
        let mut lap_split_times = [InGameTime::default(); 11];
        for idx in 0..lap_count {
            let start = (0x11 + idx * 3) as usize;
            lap_split_times[idx as usize] =
                InGameTime::from_byte_handler(&header_data[start..start + 3])?;
        }

        let codes = ByteHandler::try_from(&header_data[0x34..=0x37]).unwrap();

        let location = Location::find(codes.copy_byte(0), codes.copy_byte(1), Some(Version::ER12))
            .unwrap_or_default();

        let mii = Mii::new(&header_data[0x3C..0x3C + 0x4A])?;

        let mii_crc16 = ByteHandler::try_from(&header_data[0x86..=0x87])?.copy_word(0);

        Ok(Self {
            raw_data: header_data.try_into().unwrap(),
            finish_time,
            slot_id,
            combo,
            date_set,
            controller,
            is_compressed,
            transmission_mod,
            ghost_type,
            is_automatic_drift,
            decompressed_input_data_length,
            lap_count,
            lap_split_times,
            location,
            mii,
            mii_crc16,
        })
    }

    /// Returns `true` if the stored Mii CRC-16 matches a computed
    /// checksum of the Mii bytes at offsets `0x3C`–`0x85`.
    pub fn verify_mii_crc16(&self) -> bool {
        crc16(&self.raw_data[0x3C..0x86]) == self.mii_crc16()
    }

    /// Recomputes the Mii CRC-16 from the current raw header bytes and writes
    /// the updated value to both the parsed field and the raw buffer.
    pub fn fix_mii_crc16(&mut self) {
        self.mii_crc16 = crc16(&self.raw_data[0x3C..0x86]);
        self.raw_data[0x86..0x88].copy_from_slice(&self.mii_crc16.to_be_bytes());
    }

    /// Returns the raw 136-byte header block.
    pub fn raw_data(&self) -> &[u8; 0x88] {
        &self.raw_data
    }

    /// Returns a mutable reference to the raw 136-byte header block.
    pub fn raw_data_mut(&mut self) -> &mut [u8; 0x88] {
        &mut self.raw_data
    }

    /// Returns the ghost's recorded finish time.
    pub fn finish_time(&self) -> &InGameTime {
        &self.finish_time
    }

    /// Sets the finish time and updates the raw data accordingly.
    pub fn set_finish_time(&mut self, finish_time: InGameTime) {
        self.finish_time = finish_time;
        write_in_game_time(self.raw_data_mut(), 0x04, 0, &finish_time);
    }

    /// Returns the course slot the ghost was recorded on.
    pub fn slot_id(&self) -> SlotId {
        self.slot_id
    }

    /// Sets the course slot and updates the raw data accordingly.
    pub fn set_slot_id(&mut self, slot_id: SlotId) {
        self.slot_id = slot_id;
        write_bits(self.raw_data_mut(), 0x07, 0, 6, u8::from(slot_id) as u64);
    }

    /// Returns the character and vehicle combo used in the ghost.
    pub fn combo(&self) -> &Combo {
        &self.combo
    }

    /// Sets the character/vehicle combo and updates the raw data accordingly.
    pub fn set_combo(&mut self, combo: Combo) {
        write_bits(
            self.raw_data_mut(),
            0x08,
            0,
            6,
            u8::from(combo.vehicle()) as u64,
        );
        write_bits(
            self.raw_data_mut(),
            0x08,
            6,
            6,
            u8::from(combo.character()) as u64,
        );

        self.combo = combo;
    }

    /// Returns the date the ghost was set.
    pub fn date_set(&self) -> &Date {
        &self.date_set
    }

    /// Sets the ghost's date and updates the raw data accordingly.
    pub fn set_date_set(&mut self, date_set: Date) {
        write_bits(
            self.raw_data_mut(),
            0x09,
            4,
            7,
            (date_set.year() - 2000) as u64,
        );
        write_bits(self.raw_data_mut(), 0x0A, 3, 4, date_set.month() as u64);
        write_bits(self.raw_data_mut(), 0x0A, 7, 5, date_set.day() as u64);

        self.date_set = date_set;
    }

    /// Returns the input controller used to record the ghost.
    pub fn controller(&self) -> Controller {
        self.controller
    }

    /// Sets the controller and updates the raw data accordingly.
    pub fn set_controller(&mut self, controller: Controller) {
        self.controller = controller;
        write_bits(self.raw_data_mut(), 0x0B, 4, 4, u8::from(controller) as u64);
    }

    /// Returns whether the ghost's input data is Yaz1 compressed.
    pub fn is_compressed(&self) -> bool {
        self.is_compressed
    }

    /// Sets the compression flag and updates the raw data accordingly.
    ///
    /// This is `pub(crate)` because compression state should be managed by the
    /// RKG file layer, not set directly by callers.
    pub(crate) fn set_compressed(&mut self, is_compressed: bool) {
        self.is_compressed = is_compressed;
        write_bits(self.raw_data_mut(), 0x0C, 4, 1, is_compressed as u64);
    }

    /// Returns the Retro Rewind (Pulsar) transmission override active for this ghost.
    pub fn transmission_mod(&self) -> TransmissionMod {
        self.transmission_mod
    }

    /// Sets the transmission mod and updates the raw data accordingly.
    pub fn set_transmission_mod(&mut self, transmission_mod: TransmissionMod) {
        self.transmission_mod = transmission_mod;
        write_bits(
            self.raw_data_mut(),
            0x0C,
            5,
            2,
            u8::from(transmission_mod) as u64,
        );
    }

    /// Returns the ghost type of this ghost.
    pub fn ghost_type(&self) -> GhostType {
        self.ghost_type
    }

    /// Sets the ghost type and updates the raw data accordingly.
    pub fn set_ghost_type(&mut self, ghost_type: GhostType) {
        self.ghost_type = ghost_type;
        write_bits(self.raw_data_mut(), 0x0C, 7, 7, u8::from(ghost_type) as u64);
    }

    /// Returns whether automatic drift was used during the recorded run.
    pub fn is_automatic_drift(&self) -> bool {
        self.is_automatic_drift
    }

    /// Sets the automatic drift flag and updates the raw data accordingly.
    pub fn set_automatic_drift(&mut self, is_automatic_drift: bool) {
        self.is_automatic_drift = is_automatic_drift;
        write_bits(self.raw_data_mut(), 0x0D, 6, 1, is_automatic_drift as u64);
    }

    /// Returns the byte length of the input data block after decompression.
    pub fn decompressed_input_data_length(&self) -> u16 {
        self.decompressed_input_data_length
    }

    /// Returns the number of laps recorded in this ghost.
    pub fn lap_count(&self) -> u8 {
        self.lap_count
    }

    /// Returns a slice of the valid lap split times (length equal to [`lap_count`](Header::lap_count)).
    pub fn lap_split_times(&self) -> &[InGameTime] {
        &self.lap_split_times[0..self.lap_count as usize]
    }

    /// Returns the lap split time at the given zero-based idx.
    ///
    /// # Errors
    ///
    /// Returns [`HeaderError::LapSplitIndexError`] if `idx` is greater than or
    /// equal to [`lap_count`](Header::lap_count).
    pub fn lap_split_time(&self, idx: usize) -> Result<InGameTime, HeaderError> {
        if idx >= self.lap_count as usize {
            return Err(HeaderError::LapSplitIndexError);
        }
        Ok(self.lap_split_times[idx])
    }

    /// Sets the lap split time at the given zero-based idx and updates the raw data accordingly.
    ///
    /// Does nothing if `idx` is greater than or equal to [`lap_count`](Header::lap_count).
    pub fn set_lap_split_time(&mut self, idx: usize, lap_split_time: InGameTime) {
        if idx >= self.lap_count as usize {
            return;
        }
        self.lap_split_times[idx] = lap_split_time;

        write_bits(
            self.raw_data_mut(),
            0x11 + idx * 0x03,
            0,
            7,
            lap_split_time.minutes() as u64,
        );
        write_bits(
            self.raw_data_mut(),
            0x11 + idx * 0x03,
            7,
            7,
            lap_split_time.seconds() as u64,
        );
        write_bits(
            self.raw_data_mut(),
            0x12 + idx * 0x03,
            6,
            10,
            lap_split_time.milliseconds() as u64,
        );
    }

    /// Returns the player's geographic location when the ghost was set.
    pub fn location(&self) -> &Location {
        &self.location
    }

    /// Sets the player's location and updates the raw data accordingly.
    ///
    /// When the country is [`Country::NotSet`], the subregion byte is written
    /// as `0xFF` (Not Set).
    pub fn set_location(&mut self, location: Location) {
        write_bits(
            self.raw_data_mut(),
            0x34,
            0,
            8,
            u8::from(location.country()) as u64,
        );

        let subregion_id = if location.country() != Country::NotSet {
            u8::from(location.subregion()) as u64
        } else {
            0xFF
        };

        write_bits(self.raw_data_mut(), 0x35, 0, 8, subregion_id);

        self.location = location;
    }

    /// Returns a reference to the Mii embedded in the ghost header.
    pub fn mii(&self) -> &Mii {
        &self.mii
    }

    /// Returns a mutable reference to the Mii embedded in the ghost header.
    pub fn mii_mut(&mut self) -> &mut Mii {
        &mut self.mii
    }

    /// Replaces the embedded Mii, updates the raw header bytes at `0x3C`–`0x85`,
    /// and recomputes the Mii CRC-16.
    pub fn set_mii(&mut self, mii: Mii) {
        self.mii_crc16 = crc16(mii.raw_data());
        self.raw_data_mut()[0x3C..0x86].copy_from_slice(mii.raw_data());
        self.mii = mii;
    }

    /// Returns the CRC-16 checksum of the embedded Mii data as stored in the header.
    pub fn mii_crc16(&self) -> u16 {
        self.mii_crc16
    }
}

/// Writes a packed [`InGameTime`] value (7 minutes + 7 seconds + 10 milliseconds bits)
/// into `buf` starting at the given byte and bit offset.
fn write_in_game_time(
    buf: &mut [u8],
    byte_offset: usize,
    bit_offset: usize,
    in_game_time: &InGameTime,
) {
    let mut bit_offset = bit_offset + byte_offset * 8;

    write_bits(
        buf,
        bit_offset / 8,
        bit_offset % 8,
        7,
        in_game_time.minutes() as u64,
    );

    bit_offset += 7;

    write_bits(
        buf,
        bit_offset / 8,
        bit_offset % 8,
        7,
        in_game_time.seconds() as u64,
    );

    bit_offset += 7;

    write_bits(
        buf,
        bit_offset / 8,
        bit_offset % 8,
        10,
        in_game_time.milliseconds() as u64,
    );
}
