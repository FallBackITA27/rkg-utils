use std::convert::Infallible;

use crate::byte_handler::{ByteHandlerError, FromByteHandler};

#[derive(Clone, Copy)]
pub struct Lips {
    y: u8,
    size: u8,
    lips_type: LipsType,
    lips_color: LipsColor,
}

impl Lips {
    pub fn new(
        y: u8,
        size: u8,
        lips_type: LipsType,
        lips_color: LipsColor,
    ) -> Result<Self, LipsError> {
        if y > 18 {
            return Err(LipsError::YInvalid);
        }
        if size > 8 {
            return Err(LipsError::SizeInvalid);
        }
        Ok(Self {
            y,
            size,
            lips_type,
            lips_color,
        })
    }

    pub fn y(&self) -> u8 {
        self.y
    }
    pub fn size(&self) -> u8 {
        self.size
    }
    pub fn lips_type(&self) -> LipsType {
        self.lips_type
    }
    pub fn lips_color(&self) -> LipsColor {
        self.lips_color
    }

    pub fn set_y(&mut self, y: u8) -> Result<(), LipsError> {
        if y > 18 {
            return Err(LipsError::YInvalid);
        }
        self.y = y;
        Ok(())
    }

    pub fn set_size(&mut self, size: u8) -> Result<(), LipsError> {
        if size > 8 {
            return Err(LipsError::SizeInvalid);
        }
        self.size = size;
        Ok(())
    }

    pub fn set_lips_type(&mut self, lips_type: LipsType) {
        self.lips_type = lips_type;
    }

    pub fn set_lips_color(&mut self, lips_color: LipsColor) {
        self.lips_color = lips_color;
    }
}

impl FromByteHandler for Lips {
    type Err = LipsError;
    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<crate::byte_handler::ByteHandler>,
        Self::Err: From<T::Error>,
    {
        let mut handler = handler.try_into()?;

        let lips_type =
            LipsType::try_from(handler.copy_byte(0) >> 3).map_err(|_| LipsError::TypeInvalid)?;
        let y = handler.copy_byte(1) & 0x1F;
        handler.shift_right(1);
        let lips_color = LipsColor::try_from(handler.copy_byte(0) & 0x03)
            .map_err(|_| LipsError::ColorInvalid)?;
        let size = handler.copy_byte(1) >> 4;

        Self::new(y, size, lips_type, lips_color)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LipsError {
    #[error("Type is invalid")]
    TypeInvalid,
    #[error("Color is invalid")]
    ColorInvalid,
    #[error("Size is invalid")]
    SizeInvalid,
    #[error("Y position is invalid")]
    YInvalid,
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    #[error("")]
    Infallible(#[from] Infallible),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LipsColor {
    Orange,
    Red,
    Pink,
}

impl TryFrom<u8> for LipsColor {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Orange),
            1 => Ok(Self::Red),
            2 => Ok(Self::Pink),
            _ => Err(()),
        }
    }
}

impl From<LipsColor> for u8 {
    fn from(value: LipsColor) -> Self {
        match value {
            LipsColor::Orange => 0,
            LipsColor::Red => 1,
            LipsColor::Pink => 2,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LipsType {
    Neutral,
    NeutralLips,
    Smile,
    SmileStroke,
    SmileTeeth,
    LipsSmall,
    LipsLarge,
    Wave,
    WaveAngrySmall,
    NeutralStrokeLarge,
    TeethSurprised,
    LipsExtraLarge,
    LipsUp,
    NeutralDown,
    Surprised,
    TeethMiddle,
    NeutralStroke,
    LipsExtraSmall,
    Malicious,
    LipsDual,
    NeutralComma,
    NeutralUp,
    TeethLarge,
    WaveAngry,
}

impl TryFrom<u8> for LipsType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x17 => Ok(Self::Neutral),
            0x01 => Ok(Self::NeutralLips),
            0x13 => Ok(Self::Smile),
            0x15 => Ok(Self::SmileStroke),
            0x16 => Ok(Self::SmileTeeth),
            0x05 => Ok(Self::LipsSmall),
            0x00 => Ok(Self::LipsLarge),
            0x08 => Ok(Self::Wave),
            0x0A => Ok(Self::WaveAngrySmall),
            0x10 => Ok(Self::NeutralStrokeLarge),
            0x06 => Ok(Self::TeethSurprised),
            0x0D => Ok(Self::LipsExtraLarge),
            0x07 => Ok(Self::LipsUp),
            0x09 => Ok(Self::NeutralDown),
            0x02 => Ok(Self::Surprised),
            0x11 => Ok(Self::TeethMiddle),
            0x03 => Ok(Self::NeutralStroke),
            0x04 => Ok(Self::LipsExtraSmall),
            0x0F => Ok(Self::Malicious),
            0x0B => Ok(Self::LipsDual),
            0x14 => Ok(Self::NeutralComma),
            0x12 => Ok(Self::NeutralUp),
            0x0E => Ok(Self::TeethLarge),
            0x0C => Ok(Self::WaveAngry),
            _ => Err(()),
        }
    }
}

impl From<LipsType> for u8 {
    fn from(value: LipsType) -> Self {
        match value {
            LipsType::Neutral => 0x17,
            LipsType::NeutralLips => 0x01,
            LipsType::Smile => 0x13,
            LipsType::SmileStroke => 0x15,
            LipsType::SmileTeeth => 0x16,
            LipsType::LipsSmall => 0x05,
            LipsType::LipsLarge => 0x00,
            LipsType::Wave => 0x08,
            LipsType::WaveAngrySmall => 0x0A,
            LipsType::NeutralStrokeLarge => 0x10,
            LipsType::TeethSurprised => 0x06,
            LipsType::LipsExtraLarge => 0x0D,
            LipsType::LipsUp => 0x07,
            LipsType::NeutralDown => 0x09,
            LipsType::Surprised => 0x02,
            LipsType::TeethMiddle => 0x11,
            LipsType::NeutralStroke => 0x03,
            LipsType::LipsExtraSmall => 0x04,
            LipsType::Malicious => 0x0F,
            LipsType::LipsDual => 0x0B,
            LipsType::NeutralComma => 0x14,
            LipsType::NeutralUp => 0x12,
            LipsType::TeethLarge => 0x0E,
            LipsType::WaveAngry => 0x0C,
        }
    }
}
