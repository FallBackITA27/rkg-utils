#[derive(thiserror::Error, Debug)]
pub enum FaceButtonError {
    #[error("Non Existent Face Button")]
    NonExistentFaceButton,
    #[error("BitReader Error: {0}")]
    BitReaderError(#[from] bitreader::BitReaderError),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FaceButton {
    Accelerator,
    Brake,
    Item,
    BrakeWhileAcceleratorHeld,
    Unknown,
}

pub fn parse_face_buttons(value: u8) -> Result<Vec<FaceButton>, FaceButtonError> {
    let mut buttons = Vec::new();
    
    if value & 0x01 != 0 {
        buttons.push(FaceButton::Accelerator);
    }
    if value & 0x02 != 0 {
        buttons.push(FaceButton::Brake);
    }
    if value & 0x04 != 0 {
        buttons.push(FaceButton::Item);
    }
    if value & 0x08 != 0 {
        buttons.push(FaceButton::BrakeWhileAcceleratorHeld);
    }
    if value & 0xF0 != 0 {
        buttons.push(FaceButton::Unknown);
    }
    
    if value != 0x00 && buttons.is_empty() {
        return Err(FaceButtonError::NonExistentFaceButton);
    }
    
    Ok(buttons)
}

#[derive(thiserror::Error, Debug)]
pub enum FaceInputError {
    #[error("Invalid Face Input")]
    InvalidFaceInput,
    #[error("Invalid Face Button: {0}")]
    InvalidButton(#[from] FaceButtonError),
    #[error("BitReader Error: {0}")]
    BitReaderError(#[from] bitreader::BitReaderError),
}

#[derive(Debug)]
pub struct FaceInput {
    buttons: Vec<FaceButton>,
    frame_duration: u8,
}

impl FaceInput {
    pub fn buttons(&self) -> &Vec<FaceButton> {
        &self.buttons
    }
    
    pub fn frame_duration(&self) -> u8 {
        self.frame_duration
    }
}

impl TryFrom<u16> for FaceInput {
    type Error = FaceInputError;
    
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let bytes = value.to_be_bytes();
        let buttons = parse_face_buttons(bytes[0])?;
        let frame_duration = bytes[1];
        
        Ok(Self {
            buttons,
            frame_duration,
        })
    }
}

impl TryFrom<&mut bitreader::BitReader<'_>> for FaceInput {
    type Error = FaceInputError;
    
    fn try_from(value: &mut bitreader::BitReader<'_>) -> Result<Self, Self::Error> {
        FaceInput::try_from(value.read_u16(16)?)
    }
}
