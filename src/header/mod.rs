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

#[derive(thiserror::Error, Debug)]
pub enum HeaderError {
    #[error("File is not RKGD")]
    NotRKGD,
    #[error("Data passed is not correct size (0x88)")]
    NotCorrectSize,
    #[error("Friend ghost number out of range (1-30)")]
    FriendNumberOutOfRange,
    #[error("Lap split index not semantically valid")]
    LapSplitIndexError,
    #[error("In Game Time Error: {0}")]
    InGameTimeError(#[from] InGameTimeError),
    #[error("Slot ID Error: {0}")]
    SlotIdError(#[from] SlotIdError),
    #[error("Combo Error: {0}")]
    ComboError(#[from] ComboError),
    #[error("Date Error: {0}")]
    DateError(#[from] DateError),
    #[error("Controller Error: {0}")]
    ControllerError(#[from] ControllerError),
    #[error("Transmission Mod Error: {0}")]
    TransmissionModError(#[from] TransmissionModError),
    #[error("Ghost Type Error: {0}")]
    GhostTypeError(#[from] GhostTypeError),
    #[error("Mii Error: {0}")]
    MiiError(#[from] MiiError),
    #[error("Io Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Country Error: {0}")]
    CountryError(#[from] CountryError),
    #[error("Subregion Error: {0}")]
    SubregionError(#[from] SubregionError),
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
}

/// All the data in the Header of an RKGD
/// <https://wiki.tockdom.com/wiki/RKG_(File_Format)#File_Header>
pub struct Header {
    raw_data: [u8; 0x88],
    finish_time: InGameTime,
    slot_id: SlotId,
    combo: Combo,
    date_set: Date,
    controller: Controller,
    is_compressed: bool,
    transmission_mod: TransmissionMod,
    ghost_type: GhostType,
    is_automatic_drift: bool,
    decompressed_input_data_length: u16,
    lap_count: u8,
    lap_split_times: [InGameTime; 11],
    location: Location,
    mii: Mii,
    mii_crc16: u16,
}

impl Header {
    /// Reads header from a file at the path
    pub fn new_from_path<P: AsRef<std::path::Path>>(p: P) -> Result<Self, HeaderError> {
        let mut rkg_data = [0u8; 0x88];
        std::fs::File::open(p)?.read_exact(&mut rkg_data)?;
        Self::new(&rkg_data)
    }

    /// Reads header from slice
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
        for index in 0..lap_count {
            let start = (0x11 + index * 3) as usize;
            lap_split_times[index as usize] =
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

    /// Returns true if Mii CRC16 is correct (i.e. Mii data not illegally tampered with)
    pub fn verify_mii_crc16(&self) -> bool {
        // Verify CRC16 based on the actual bytes in the header buffer (0x3C to 0x86)
        crc16(&self.raw_data[0x3C..0x86]) == self.mii_crc16()
    }

    /// Recalculates and updates Mii CRC16
    pub fn fix_mii_crc16(&mut self) {
        // Calculate CRC16 based on the actual bytes in the header buffer (0x3C to 0x86)
        // This ensures the CRC matches what's actually written to the file
        self.mii_crc16 = crc16(&self.raw_data[0x3C..0x86]);
        self.raw_data[0x86..0x88].copy_from_slice(&self.mii_crc16.to_be_bytes());
    }

    /// Getter
    pub fn raw_data(&self) -> &[u8; 0x88] {
        &self.raw_data
    }

    pub fn raw_data_mut(&mut self) -> &mut [u8; 0x88] {
        &mut self.raw_data
    }

    pub fn finish_time(&self) -> &InGameTime {
        &self.finish_time
    }

    pub fn set_finish_time(&mut self, finish_time: InGameTime) {
        self.finish_time = finish_time;
        write_in_game_time(self.raw_data_mut(), 0x04, 0, &finish_time);
    }

    pub fn slot_id(&self) -> SlotId {
        self.slot_id
    }

    pub fn set_slot_id(&mut self, slot_id: SlotId) {
        self.slot_id = slot_id;
        write_bits(self.raw_data_mut(), 0x07, 0, 6, u8::from(slot_id) as u64);
    }

    pub fn combo(&self) -> &Combo {
        &self.combo
    }

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

    pub fn date_set(&self) -> &Date {
        &self.date_set
    }

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

    pub fn controller(&self) -> Controller {
        self.controller
    }

    pub fn set_controller(&mut self, controller: Controller) {
        self.controller = controller;
        write_bits(self.raw_data_mut(), 0x0B, 4, 4, u8::from(controller) as u64);
    }

    pub fn is_compressed(&self) -> bool {
        self.is_compressed
    }

    pub(crate) fn set_compressed(&mut self, is_compressed: bool) {
        self.is_compressed = is_compressed;
        write_bits(self.raw_data_mut(), 0x0C, 4, 1, is_compressed as u64);
    }

    pub fn transmission_mod(&self) -> TransmissionMod {
        self.transmission_mod
    }

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

    pub fn ghost_type(&self) -> GhostType {
        self.ghost_type
    }

    pub fn set_ghost_type(&mut self, ghost_type: GhostType) {
        self.ghost_type = ghost_type;
        write_bits(self.raw_data_mut(), 0x0C, 7, 7, u8::from(ghost_type) as u64);
    }

    pub fn is_automatic_drift(&self) -> bool {
        self.is_automatic_drift
    }

    pub fn set_automatic_drift(&mut self, is_automatic_drift: bool) {
        self.is_automatic_drift = is_automatic_drift;
        write_bits(self.raw_data_mut(), 0x0D, 6, 1, is_automatic_drift as u64);
    }

    pub fn decompressed_input_data_length(&self) -> u16 {
        self.decompressed_input_data_length
    }

    pub fn lap_count(&self) -> u8 {
        self.lap_count
    }

    pub fn lap_split_times(&self) -> &[InGameTime] {
        &self.lap_split_times[0..self.lap_count as usize]
    }

    pub fn lap_split_time(&self, idx: usize) -> Result<InGameTime, HeaderError> {
        if idx >= self.lap_count as usize {
            return Err(HeaderError::LapSplitIndexError);
        }
        Ok(self.lap_split_times[idx])
    }

    pub fn set_lap_split_time(&mut self, index: usize, lap_split_time: InGameTime) {
        if index >= self.lap_count as usize {
            return;
        }
        self.lap_split_times[index] = lap_split_time;

        write_bits(
            self.raw_data_mut(),
            0x11 + index * 0x03,
            0,
            7,
            lap_split_time.minutes() as u64,
        );
        write_bits(
            self.raw_data_mut(),
            0x11 + index * 0x03,
            7,
            7,
            lap_split_time.seconds() as u64,
        );
        write_bits(
            self.raw_data_mut(),
            0x12 + index * 0x03,
            6,
            10,
            lap_split_time.milliseconds() as u64,
        );
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

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

    pub fn mii(&self) -> &Mii {
        &self.mii
    }

    pub fn mii_mut(&mut self) -> &mut Mii {
        &mut self.mii
    }

    pub fn set_mii(&mut self, mii: Mii) {
        self.mii_crc16 = crc16(mii.raw_data());
        self.raw_data_mut()[0x3C..0x86].copy_from_slice(mii.raw_data());
        self.mii = mii;
    }

    pub fn mii_crc16(&self) -> u16 {
        self.mii_crc16
    }
}

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
