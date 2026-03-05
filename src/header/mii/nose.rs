use std::convert::Infallible;

use crate::byte_handler::{ByteHandlerError, FromByteHandler};

#[derive(Clone, Copy)]
pub struct Nose {
    y: u8,
    size: u8,
    nose_type: NoseType,
}

impl Nose {
    pub fn new(y: u8, size: u8, nose_type: NoseType) -> Result<Self, NoseError> {
        if size > 8 {
            return Err(NoseError::SizeInvalid);
        }
        if y > 18 {
            return Err(NoseError::YInvalid);
        }

        Ok(Self { y, size, nose_type })
    }

    pub fn y(&self) -> u8 {
        self.y
    }
    pub fn size(&self) -> u8 {
        self.size
    }
    pub fn nose_type(&self) -> NoseType {
        self.nose_type
    }

    pub fn set_y(&mut self, y: u8) -> Result<(), NoseError> {
        if y > 18 {
            return Err(NoseError::YInvalid);
        }
        self.y = y;
        Ok(())
    }

    pub fn set_size(&mut self, size: u8) -> Result<(), NoseError> {
        if size > 8 {
            return Err(NoseError::SizeInvalid);
        }
        self.size = size;
        Ok(())
    }

    pub fn set_nose_type(&mut self, nose_type: NoseType) {
        self.nose_type = nose_type;
    }
}

impl FromByteHandler for Nose {
    type Err = NoseError;
    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<crate::byte_handler::ByteHandler>,
        Self::Err: From<T::Error>,
    {
        let handler = handler.try_into()?;

        let nose_type =
            NoseType::try_from(handler.copy_byte(0) >> 4).map_err(|_| NoseError::TypeInvalid)?;
        let size = handler.copy_byte(0) & 0x0F;
        let y = handler.copy_byte(1) >> 3;

        Self::new(y, size, nose_type)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum NoseError {
    #[error("Type is invalid")]
    TypeInvalid,
    #[error("Size is invalid")]
    SizeInvalid,
    #[error("Y position is invalid")]
    YInvalid,
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    #[error("")]
    Infallible(#[from] Infallible),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NoseType {
    Normal,
    Rounded,
    Dot,
    Arrow,
    Roman,
    Triangle,
    Button,
    RoundedInverted,
    Potato,
    Grecian,
    Snub,
    Aquiline,
}

impl TryFrom<u8> for NoseType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Normal),
            0x0A => Ok(Self::Rounded),
            0x02 => Ok(Self::Dot),
            0x03 => Ok(Self::Arrow),
            0x06 => Ok(Self::Roman),
            0x00 => Ok(Self::Triangle),
            0x05 => Ok(Self::Button),
            0x04 => Ok(Self::RoundedInverted),
            0x08 => Ok(Self::Potato),
            0x09 => Ok(Self::Grecian),
            0x07 => Ok(Self::Snub),
            0x0B => Ok(Self::Aquiline),
            _ => Err(()),
        }
    }
}

impl From<NoseType> for u8 {
    fn from(value: NoseType) -> Self {
        match value {
            NoseType::Normal => 0x01,
            NoseType::Rounded => 0x0A,
            NoseType::Dot => 0x02,
            NoseType::Arrow => 0x03,
            NoseType::Roman => 0x06,
            NoseType::Triangle => 0x00,
            NoseType::Button => 0x05,
            NoseType::RoundedInverted => 0x04,
            NoseType::Potato => 0x08,
            NoseType::Grecian => 0x09,
            NoseType::Snub => 0x07,
            NoseType::Aquiline => 0x0B,
        }
    }
}
