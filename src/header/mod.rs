use bitreader::BitReader;

use crate::header::{
    combo::{Combo, ComboError},
    controller::{Controller, ControllerError},
    date::{Date, DateError},
    finish_time::{FinishTime, FinishTimeError},
    ghost_type::{GhostType, GhostTypeError},
    mii::{Mii, MiiError},
    slot_id::{SlotId, SlotIdError},
};

pub mod combo;
pub mod controller;
pub mod date;
pub mod finish_time;
pub mod ghost_type;
pub mod mii;
pub mod slot_id;

#[derive(thiserror::Error, Debug)]
pub enum HeaderError {
    #[error("File is not RKGD")]
    NotRKGD,
    #[error("Data passed is not correct size (0x88)")]
    NotCorrectSize,
    #[error("BitReader Error: {0}")]
    BitReaderError(#[from] bitreader::BitReaderError),
    #[error("Finish Time Error: {0}")]
    FinishTimeError(#[from] FinishTimeError),
    #[error("Slot ID Error: {0}")]
    SlotIdError(#[from] SlotIdError),
    #[error("Combo Error: {0}")]
    ComboError(#[from] ComboError),
    #[error("Date Error: {0}")]
    DateError(#[from] DateError),
    #[error("Controller Error: {0}")]
    ControllerError(#[from] ControllerError),
    #[error("Ghost Type Error: {0}")]
    GhostTypeError(#[from] GhostTypeError),
    #[error("Mii Error: {0}")]
    MiiError(#[from] MiiError),
}

pub struct Header {
    finish_time: FinishTime,
    slot_id: SlotId,
    combo: Combo,
    date_set: Date,
    controller: Controller,
    is_compressed: bool,
    ghost_type: GhostType,
    is_automatic_drift: bool,
    decompressed_input_data_length: u16,
    lap_count: u8,
    lap_split_times: Vec<FinishTime>,
    country_code: u8,
    state_code: u8,
    location_code: u16,
    mii_data: Mii,
    mii_crc16: u16,
}

impl Header {
    pub fn new(header_data: &[u8]) -> Result<Self, HeaderError> {
        let mut header_reader = BitReader::new(header_data);

        if header_reader.read_u32(32)? != 0x524B4744 {
            return Err(HeaderError::NotRKGD);
        } else if header_data.len() != 0x88 {
            return Err(HeaderError::NotCorrectSize);
        }

        let finish_time = FinishTime::try_from(&mut header_reader)?;
        let slot_id = SlotId::try_from(&mut header_reader)?;

        header_reader.skip(2)?; // Padding

        let combo = Combo::try_from(&mut header_reader)?;
        let date_set = Date::try_from(&mut header_reader)?;
        let controller = Controller::try_from(&mut header_reader)?;

        header_reader.skip(4)?;

        let is_compressed = header_reader
            .read_bool()
            .expect("Failed to read is_compressed");

        header_reader.skip(2)?;
        let ghost_type = GhostType::try_from(&mut header_reader)?;

        let is_automatic_drift = header_reader.read_bool()?;

        header_reader.skip(1)?;

        let decompressed_input_data_length = header_reader.read_u16(16)?;

        let lap_count = header_reader.read_u8(8)?;

        let mut lap_split_times: Vec<FinishTime> = Vec::with_capacity(8);
        for _ in 1..=9 {
            lap_split_times.push(FinishTime::try_from(&mut header_reader)?);
        }

        // Skip garbage RAM data
        header_reader.skip(64)?;

        let country_code = header_reader.read_u8(8)?;
        let state_code = header_reader.read_u8(8)?;

        let location_code = header_reader.read_u16(16)?;

        header_reader.skip(32)?;

        let mii_data = Mii::try_from(&mut header_reader)?;

        // TODO: Use CRC for its intended purpose and error out if wrong OR report mismatch
        let mii_crc16 = header_reader.read_u16(16)?;

        Ok(Self {
            finish_time,
            slot_id,
            combo,
            date_set,
            controller,
            is_compressed,
            ghost_type,
            is_automatic_drift,
            decompressed_input_data_length,
            lap_count,
            lap_split_times,
            country_code,
            state_code,
            location_code,
            mii_data,
            mii_crc16,
        })
    }

    pub fn finish_time(&self) -> &FinishTime {
        &self.finish_time
    }

    pub fn slot_id(&self) -> SlotId {
        self.slot_id
    }

    pub fn combo(&self) -> &Combo {
        &self.combo
    }

    pub fn date_set(&self) -> &Date {
        &self.date_set
    }

    pub fn controller(&self) -> Controller {
        self.controller
    }

    pub fn is_compressed(&self) -> bool {
        self.is_compressed
    }

    pub fn ghost_type(&self) -> GhostType {
        self.ghost_type
    }

    pub fn is_automatic_drift(&self) -> bool {
        self.is_automatic_drift
    }

    pub fn decompressed_input_data_length(&self) -> u16 {
        self.decompressed_input_data_length
    }

    pub fn lap_count(&self) -> u8 {
        self.lap_count
    }

    pub fn lap_split_times(&self) -> &[FinishTime] {
        &self.lap_split_times
    }

    pub fn country_code(&self) -> u8 {
        self.country_code
    }

    pub fn state_code(&self) -> u8 {
        self.state_code
    }

    pub fn location_code(&self) -> u16 {
        self.location_code
    }

    pub fn mii_data(&self) -> &Mii {
        &self.mii_data
    }

    pub fn mii_crc16(&self) -> u16 {
        self.mii_crc16
    }
}
