use std::convert::Infallible;

use crate::byte_handler::{ByteHandlerError, FromByteHandler};

#[derive(Clone, Copy)]
pub struct Eyes {
    rotation: u8,
    size: u8,
    x: u8,
    y: u8,
    eye_color: EyeColor,
    eye_type: EyeType,
}

impl Eyes {
    pub fn new(
        rotation: u8,
        size: u8,
        x: u8,
        y: u8,
        eye_color: EyeColor,
        eye_type: EyeType,
    ) -> Result<Self, EyesError> {
        if size > 7 {
            return Err(EyesError::SizeInvalid);
        }
        if rotation > 7 {
            return Err(EyesError::RotationInvalid);
        }
        if y > 18 {
            return Err(EyesError::YInvalid);
        }
        if x > 12 {
            return Err(EyesError::XInvalid);
        }

        Ok(Self {
            rotation,
            size,
            x,
            y,
            eye_color,
            eye_type,
        })
    }

    pub fn rotation(&self) -> u8 {
        self.rotation
    }
    pub fn size(&self) -> u8 {
        self.size
    }
    pub fn x(&self) -> u8 {
        self.x
    }
    pub fn y(&self) -> u8 {
        self.y
    }
    pub fn eye_color(&self) -> EyeColor {
        self.eye_color
    }
    pub fn eye_type(&self) -> EyeType {
        self.eye_type
    }

    pub fn set_rotation(&mut self, rotation: u8) -> Result<(), EyesError> {
        if rotation > 7 {
            return Err(EyesError::RotationInvalid);
        }
        self.rotation = rotation;
        Ok(())
    }

    pub fn set_size(&mut self, size: u8) -> Result<(), EyesError> {
        if size > 7 {
            return Err(EyesError::SizeInvalid);
        }
        self.size = size;
        Ok(())
    }

    pub fn set_x(&mut self, x: u8) -> Result<(), EyesError> {
        if x > 12 {
            return Err(EyesError::XInvalid);
        }
        self.x = x;
        Ok(())
    }

    pub fn set_y(&mut self, y: u8) -> Result<(), EyesError> {
        if y > 18 {
            return Err(EyesError::YInvalid);
        }
        self.y = y;
        Ok(())
    }

    pub fn set_eye_color(&mut self, eye_color: EyeColor) {
        self.eye_color = eye_color;
    }

    pub fn set_eye_type(&mut self, eye_type: EyeType) {
        self.eye_type = eye_type;
    }
}

#[derive(thiserror::Error, Debug)]
pub enum EyesError {
    #[error("Type is invalid")]
    TypeInvalid,
    #[error("Color is invalid")]
    ColorInvalid,
    #[error("Rotation is invalid")]
    RotationInvalid,
    #[error("Size is invalid")]
    SizeInvalid,
    #[error("Y position is invalid")]
    YInvalid,
    #[error("X position is invalid")]
    XInvalid,
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    #[error("")]
    Infallible(#[from] Infallible),
}

impl FromByteHandler for Eyes {
    type Err = EyesError;
    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<crate::byte_handler::ByteHandler>,
        Self::Err: From<T::Error>,
    {
        let mut handler = handler.try_into()?;

        let eye_type =
            EyeType::try_from(handler.copy_byte(0) >> 2).map_err(|_| EyesError::TypeInvalid)?;
        let eye_color =
            EyeColor::try_from(handler.copy_byte(2) >> 5).map_err(|_| EyesError::ColorInvalid)?;
        let y = handler.copy_byte(1) & 0x1F;
        handler.shift_right(5);
        let rotation = handler.copy_byte(1) & 0x1F;
        let x = handler.copy_byte(3) & 0x0F;
        let size = handler.copy_byte(3) >> 4;

        Self::new(rotation, size, x, y, eye_color, eye_type)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EyeColor {
    Black,
    Gray,
    Brown,
    Hazel,
    Blue,
    Green,
}

impl TryFrom<u8> for EyeColor {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Black),
            0x01 => Ok(Self::Gray),
            0x02 => Ok(Self::Brown),
            0x03 => Ok(Self::Hazel),
            0x04 => Ok(Self::Blue),
            0x05 => Ok(Self::Green),
            _ => Err(()),
        }
    }
}

impl From<EyeColor> for u8 {
    fn from(value: EyeColor) -> Self {
        match value {
            EyeColor::Black => 0x00,
            EyeColor::Gray => 0x01,
            EyeColor::Brown => 0x02,
            EyeColor::Hazel => 0x03,
            EyeColor::Blue => 0x04,
            EyeColor::Green => 0x05,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EyeType {
    Normal,
    NormalLash,
    WhiteLash,
    WhiteNoBottom,
    OvalAngledWhite,
    AngryWhite,
    DotLashType1,
    Line,
    DotLine,
    OvalWhite,
    RoundedWhite,
    NormalShadow,
    CircleWhite,
    Circle,
    CircleWhiteStroke,
    NormalOvalNoBottom,
    NormalOvalLarge,
    NormalRoundedNoBottom,
    SmallLash,
    Small,
    TwoSmall,
    NormalLongLash,
    WhiteTwoLashes,
    WhiteThreeLashes,
    DotAngry,
    DotAngled,
    Oval,
    SmallWhite,
    WhiteAngledNoBottom,
    WhiteAngledNoLeft,
    SmallWhiteTwoLashes,
    LeafWhiteLash,
    WhiteLargeNoBottom,
    Dot,
    DotLashType2,
    DotThreeLashes,
    WhiteOvalTop,
    WhiteOvalBottom,
    WhiteOvalBottomFlat,
    WhiteOvalTwoLashes,
    WhiteOvalThreeLashes,
    WhiteOvalNoBottomTwoLashes,
    DotWhite,
    WhiteOvalTopFlat,
    WhiteThinLeaf,
    StarThreeLashes,
    LineTwoLashes,
    CrowsFeet,
}

impl TryFrom<u8> for EyeType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x02 => Ok(Self::Normal),
            0x04 => Ok(Self::NormalLash),
            0x00 => Ok(Self::WhiteLash),
            0x08 => Ok(Self::WhiteNoBottom),
            0x27 => Ok(Self::OvalAngledWhite),
            0x11 => Ok(Self::AngryWhite),
            0x01 => Ok(Self::DotLashType1),
            0x1A => Ok(Self::Line),
            0x10 => Ok(Self::DotLine),
            0x0F => Ok(Self::OvalWhite),
            0x1B => Ok(Self::RoundedWhite),
            0x14 => Ok(Self::NormalShadow),
            0x21 => Ok(Self::CircleWhite),
            0x0B => Ok(Self::Circle),
            0x13 => Ok(Self::CircleWhiteStroke),
            0x20 => Ok(Self::NormalOvalNoBottom),
            0x09 => Ok(Self::NormalOvalLarge),
            0x0C => Ok(Self::NormalRoundedNoBottom),
            0x17 => Ok(Self::SmallLash),
            0x22 => Ok(Self::Small),
            0x15 => Ok(Self::TwoSmall),
            0x19 => Ok(Self::NormalLongLash),
            0x28 => Ok(Self::WhiteTwoLashes),
            0x23 => Ok(Self::WhiteThreeLashes),
            0x05 => Ok(Self::DotAngry),
            0x29 => Ok(Self::DotAngled),
            0x0D => Ok(Self::Oval),
            0x24 => Ok(Self::SmallWhite),
            0x25 => Ok(Self::WhiteAngledNoBottom),
            0x06 => Ok(Self::WhiteAngledNoLeft),
            0x18 => Ok(Self::SmallWhiteTwoLashes),
            0x1E => Ok(Self::LeafWhiteLash),
            0x1F => Ok(Self::WhiteLargeNoBottom),
            0x12 => Ok(Self::Dot),
            0x1C => Ok(Self::DotLashType2),
            0x2E => Ok(Self::DotThreeLashes),
            0x07 => Ok(Self::WhiteOvalTop),
            0x2C => Ok(Self::WhiteOvalBottom),
            0x26 => Ok(Self::WhiteOvalBottomFlat),
            0x2A => Ok(Self::WhiteOvalTwoLashes),
            0x2D => Ok(Self::WhiteOvalThreeLashes),
            0x1D => Ok(Self::WhiteOvalNoBottomTwoLashes),
            0x03 => Ok(Self::DotWhite),
            0x2B => Ok(Self::WhiteOvalTopFlat),
            0x16 => Ok(Self::WhiteThinLeaf),
            0x0A => Ok(Self::StarThreeLashes),
            0x0E => Ok(Self::LineTwoLashes),
            0x2F => Ok(Self::CrowsFeet),
            _ => Err(()),
        }
    }
}

impl From<EyeType> for u8 {
    fn from(value: EyeType) -> Self {
        match value {
            EyeType::Normal => 0x02,
            EyeType::NormalLash => 0x04,
            EyeType::WhiteLash => 0x00,
            EyeType::WhiteNoBottom => 0x08,
            EyeType::OvalAngledWhite => 0x27,
            EyeType::AngryWhite => 0x11,
            EyeType::DotLashType1 => 0x01,
            EyeType::Line => 0x1A,
            EyeType::DotLine => 0x10,
            EyeType::OvalWhite => 0x0F,
            EyeType::RoundedWhite => 0x1B,
            EyeType::NormalShadow => 0x14,
            EyeType::CircleWhite => 0x21,
            EyeType::Circle => 0x0B,
            EyeType::CircleWhiteStroke => 0x13,
            EyeType::NormalOvalNoBottom => 0x20,
            EyeType::NormalOvalLarge => 0x09,
            EyeType::NormalRoundedNoBottom => 0x0C,
            EyeType::SmallLash => 0x17,
            EyeType::Small => 0x22,
            EyeType::TwoSmall => 0x15,
            EyeType::NormalLongLash => 0x19,
            EyeType::WhiteTwoLashes => 0x28,
            EyeType::WhiteThreeLashes => 0x23,
            EyeType::DotAngry => 0x05,
            EyeType::DotAngled => 0x29,
            EyeType::Oval => 0x0D,
            EyeType::SmallWhite => 0x24,
            EyeType::WhiteAngledNoBottom => 0x25,
            EyeType::WhiteAngledNoLeft => 0x06,
            EyeType::SmallWhiteTwoLashes => 0x18,
            EyeType::LeafWhiteLash => 0x1E,
            EyeType::WhiteLargeNoBottom => 0x1F,
            EyeType::Dot => 0x12,
            EyeType::DotLashType2 => 0x1C,
            EyeType::DotThreeLashes => 0x2E,
            EyeType::WhiteOvalTop => 0x07,
            EyeType::WhiteOvalBottom => 0x2C,
            EyeType::WhiteOvalBottomFlat => 0x26,
            EyeType::WhiteOvalTwoLashes => 0x2A,
            EyeType::WhiteOvalThreeLashes => 0x2D,
            EyeType::WhiteOvalNoBottomTwoLashes => 0x1D,
            EyeType::DotWhite => 0x03,
            EyeType::WhiteOvalTopFlat => 0x2B,
            EyeType::WhiteThinLeaf => 0x16,
            EyeType::StarThreeLashes => 0x0A,
            EyeType::LineTwoLashes => 0x0E,
            EyeType::CrowsFeet => 0x2F,
        }
    }
}
