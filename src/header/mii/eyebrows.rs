use std::convert::Infallible;

use crate::{
    byte_handler::{ByteHandlerError, FromByteHandler},
    header::mii::hair::HairColor,
};

#[derive(Clone, Copy)]
pub struct Eyebrows {
    rotation: u8,
    size: u8,
    x: u8,
    y: u8,
    eyebrow_color: HairColor,
    eyebrow_type: EyebrowType,
}

impl Eyebrows {
    pub fn new(
        rotation: u8,
        size: u8,
        x: u8,
        y: u8,
        eyebrow_color: HairColor,
        eyebrow_type: EyebrowType,
    ) -> Result<Self, EyebrowsError> {
        if rotation > 11 {
            return Err(EyebrowsError::RotationInvalid);
        }
        if size > 8 {
            return Err(EyebrowsError::SizeInvalid);
        }
        if x > 12 {
            return Err(EyebrowsError::XInvalid);
        }
        if !(3..=18).contains(&y) {
            return Err(EyebrowsError::YInvalid);
        }

        Ok(Self {
            rotation,
            size,
            x,
            y,
            eyebrow_color,
            eyebrow_type,
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
    pub fn eyebrow_color(&self) -> HairColor {
        self.eyebrow_color
    }
    pub fn eyebrow_type(&self) -> EyebrowType {
        self.eyebrow_type
    }

    pub fn set_rotation(&mut self, rotation: u8) -> Result<(), EyebrowsError> {
        if rotation > 11 {
            return Err(EyebrowsError::RotationInvalid);
        }
        self.rotation = rotation;
        Ok(())
    }

    pub fn set_size(&mut self, size: u8) -> Result<(), EyebrowsError> {
        if size > 8 {
            return Err(EyebrowsError::SizeInvalid);
        }
        self.size = size;
        Ok(())
    }

    pub fn set_x(&mut self, x: u8) -> Result<(), EyebrowsError> {
        if x > 12 {
            return Err(EyebrowsError::XInvalid);
        }
        self.x = x;
        Ok(())
    }

    pub fn set_y(&mut self, y: u8) -> Result<(), EyebrowsError> {
        if !(3..=18).contains(&y) {
            return Err(EyebrowsError::YInvalid);
        }
        self.y = y;
        Ok(())
    }

    pub fn set_eyebrow_color(&mut self, eyebrow_color: HairColor) {
        self.eyebrow_color = eyebrow_color;
    }

    pub fn set_eyebrow_type(&mut self, eyebrow_type: EyebrowType) {
        self.eyebrow_type = eyebrow_type;
    }
}

#[derive(thiserror::Error, Debug)]
pub enum EyebrowsError {
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

impl FromByteHandler for Eyebrows {
    type Err = EyebrowsError;
    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<crate::byte_handler::ByteHandler>,
        Self::Err: From<T::Error>,
    {
        let mut handler = handler.try_into()?;

        let x = handler.copy_byte(3) & 0x0F;
        let eyebrow_type = EyebrowType::try_from(handler.copy_byte(0) >> 3)
            .map_err(|_| EyebrowsError::TypeInvalid)?;
        let eyebrow_color = HairColor::try_from(handler.copy_byte(2) >> 5)
            .map_err(|_| EyebrowsError::ColorInvalid)?;
        handler.shift_right(1);
        let y = handler.copy_byte(3) >> 3;
        let size = handler.copy_byte(2) & 0x0F;
        handler.shift_right(2);
        let rotation = handler.copy_byte(1) >> 3;
        Self::new(rotation, size, x, y, eyebrow_color, eyebrow_type)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EyebrowType {
    FlatAngledLarge,
    LowArchRoundedThin,
    SoftAngledLarge,
    MediumArchRoundedThin,
    RoundedMedium,
    LowArchMedium,
    RoundedThin,
    UpThin,
    MediumArchRoundedMedium,
    RoundedLarge,
    UpLarge,
    FlatAngledLargeInverted,
    MediumArchFlat,
    AngledThin,
    HorizontalLarge,
    HighArchFlat,
    Flat,
    MediumArchLarge,
    LowArchThin,
    RoundedThinInverted,
    HighArchLarge,
    Hairy,
    Dotted,
    None,
}

impl TryFrom<u8> for EyebrowType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x06 => Ok(Self::FlatAngledLarge),
            0x00 => Ok(Self::LowArchRoundedThin),
            0x0C => Ok(Self::SoftAngledLarge),
            0x01 => Ok(Self::MediumArchRoundedThin),
            0x09 => Ok(Self::RoundedMedium),
            0x13 => Ok(Self::LowArchMedium),
            0x07 => Ok(Self::RoundedThin),
            0x15 => Ok(Self::UpThin),
            0x08 => Ok(Self::MediumArchRoundedMedium),
            0x11 => Ok(Self::RoundedLarge),
            0x05 => Ok(Self::UpLarge),
            0x04 => Ok(Self::FlatAngledLargeInverted),
            0x0B => Ok(Self::MediumArchFlat),
            0x0A => Ok(Self::AngledThin),
            0x02 => Ok(Self::HorizontalLarge),
            0x03 => Ok(Self::HighArchFlat),
            0x0E => Ok(Self::Flat),
            0x14 => Ok(Self::MediumArchLarge),
            0x0F => Ok(Self::LowArchThin),
            0x0D => Ok(Self::RoundedThinInverted),
            0x16 => Ok(Self::HighArchLarge),
            0x12 => Ok(Self::Hairy),
            0x10 => Ok(Self::Dotted),
            0x17 => Ok(Self::None),
            _ => Err(()),
        }
    }
}

impl From<EyebrowType> for u8 {
    fn from(value: EyebrowType) -> Self {
        match value {
            EyebrowType::FlatAngledLarge => 0x06,
            EyebrowType::LowArchRoundedThin => 0x00,
            EyebrowType::SoftAngledLarge => 0x0C,
            EyebrowType::MediumArchRoundedThin => 0x01,
            EyebrowType::RoundedMedium => 0x09,
            EyebrowType::LowArchMedium => 0x13,
            EyebrowType::RoundedThin => 0x07,
            EyebrowType::UpThin => 0x15,
            EyebrowType::MediumArchRoundedMedium => 0x08,
            EyebrowType::RoundedLarge => 0x11,
            EyebrowType::UpLarge => 0x05,
            EyebrowType::FlatAngledLargeInverted => 0x04,
            EyebrowType::MediumArchFlat => 0x0B,
            EyebrowType::AngledThin => 0x0A,
            EyebrowType::HorizontalLarge => 0x02,
            EyebrowType::HighArchFlat => 0x03,
            EyebrowType::Flat => 0x0E,
            EyebrowType::MediumArchLarge => 0x14,
            EyebrowType::LowArchThin => 0x0F,
            EyebrowType::RoundedThinInverted => 0x0D,
            EyebrowType::HighArchLarge => 0x16,
            EyebrowType::Hairy => 0x12,
            EyebrowType::Dotted => 0x10,
            EyebrowType::None => 0x17,
        }
    }
}
