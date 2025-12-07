// https://wiki.tockdom.com/wiki/RKG_(File_Format)#Controller_Input_Data

use bitreader::BitReader;

use crate::input_data::face_input::FaceInput;

pub mod face_input;

#[derive(thiserror::Error, Debug)]
pub enum InputDataError {
    #[error("Face Input Error: {0}")]
    FaceInputError(#[from] face_input::FaceInputError),
    #[error("BitReader Error: {0}")]
    BitReaderError(#[from] bitreader::BitReaderError),
}

pub struct InputData {
    face_input_count: u16,
    direction_input_count: u16,
    trick_input_count: u16,

    face_inputs: Vec<FaceInput>,
    // direction_inputs: Vec<DirectionInput>,
    // trick_inputs: Vec<TrickInputs>,
}

impl InputData {
    // Currently this only has the uncompressed input data structure in mind
    pub fn new(input_data: &[u8]) -> Result<Self, InputDataError> {
        // TODO: Determine/decompress compressed input data here

        let mut input_reader = BitReader::new(input_data);

        let face_input_count = input_reader.read_u16(16)?;
        let direction_input_count = input_reader.read_u16(16)?;
        let trick_input_count = input_reader.read_u16(16)?;

        let mut face_inputs: Vec<FaceInput> = Vec::new();
        for _ in 0..face_input_count {
            face_inputs.push(FaceInput::try_from(&mut input_reader)?);
        }

        Ok(Self {
            face_input_count,
            direction_input_count,
            trick_input_count,
            face_inputs,
            // direction_inputs,
            // trick_inputs,
        })
    }

    pub fn face_input_count(&self) -> u16 {
        self.face_input_count
    }

    pub fn direction_input_count(&self) -> u16 {
        self.direction_input_count
    }

    pub fn trick_input_count(&self) -> u16 {
        self.trick_input_count
    }

    pub fn face_inputs(&self) -> &[FaceInput] {
        &self.face_inputs
    }
}
