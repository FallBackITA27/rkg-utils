use std::convert::Infallible;

use crate::{
    byte_handler::{ByteHandlerError, FromByteHandler},
    header::mii::hair::HairColor,
};

#[derive(Clone, Copy)]
pub struct FacialHair {
    beard_type: BeardType,
    mustache_type: MustacheType,
    color: HairColor,
    mustache_size: u8,
    mustache_y: u8,
}
impl FacialHair {
    pub fn new(
        beard_type: BeardType,
        mustache_type: MustacheType,
        color: HairColor,
        mustache_size: u8,
        mustache_y: u8,
    ) -> Result<Self, FacialHairError> {
        if mustache_size > 8 {
            return Err(FacialHairError::SizeInvalid);
        }
        if mustache_y > 16 {
            return Err(FacialHairError::YInvalid);
        }

        Ok(Self {
            beard_type,
            mustache_type,
            color,
            mustache_size,
            mustache_y,
        })
    }

    pub fn beard_type(&self) -> BeardType {
        self.beard_type
    }
    pub fn mustache_type(&self) -> MustacheType {
        self.mustache_type
    }
    pub fn color(&self) -> HairColor {
        self.color
    }
    pub fn mustache_size(&self) -> u8 {
        self.mustache_size
    }
    pub fn mustache_y(&self) -> u8 {
        self.mustache_y
    }

    pub fn set_beard_type(&mut self, beard_type: BeardType) {
        self.beard_type = beard_type;
    }

    pub fn set_mustache_type(&mut self, mustache_type: MustacheType) {
        self.mustache_type = mustache_type;
    }

    pub fn set_color(&mut self, color: HairColor) {
        self.color = color;
    }

    pub fn set_mustache_size(&mut self, mustache_size: u8) -> Result<(), FacialHairError> {
        if mustache_size > 8 {
            return Err(FacialHairError::SizeInvalid);
        }
        self.mustache_size = mustache_size;
        Ok(())
    }

    pub fn set_mustache_y(&mut self, mustache_y: u8) -> Result<(), FacialHairError> {
        if mustache_y > 16 {
            return Err(FacialHairError::YInvalid);
        }
        self.mustache_y = mustache_y;
        Ok(())
    }
}

impl FromByteHandler for FacialHair {
    type Err = FacialHairError;
    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<crate::byte_handler::ByteHandler>,
        Self::Err: From<T::Error>,
    {
        let mut handler = handler.try_into()?;
        let mustache_y = handler.copy_byte(1) & 0x1F;
        handler.shift_right(1);

        let mustache_size = handler.copy_byte(1) >> 4;
        let color = HairColor::try_from(handler.copy_byte(0) & 0x07)
            .map_err(|_| FacialHairError::ColorInvalid)?;
        let mustache_type = MustacheType::try_from(handler.copy_byte(0) >> 5)
            .map_err(|_| FacialHairError::MustacheTypeInvalid)?;
        let beard_type = BeardType::try_from((handler.copy_byte(0) >> 3) & 0x03)
            .map_err(|_| FacialHairError::BeardTypeInvalid)?;

        Self::new(beard_type, mustache_type, color, mustache_size, mustache_y)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FacialHairError {
    #[error("Beard Type is invalid")]
    BeardTypeInvalid,
    #[error("Mustache Type is invalid")]
    MustacheTypeInvalid,
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
pub enum BeardType {
    None,
    Goatee,
    GoateeLong,
    LionsManeLong,
}
impl TryFrom<u8> for BeardType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Goatee),
            2 => Ok(Self::GoateeLong),
            3 => Ok(Self::LionsManeLong),
            _ => Err(()),
        }
    }
}

impl From<BeardType> for u8 {
    fn from(value: BeardType) -> Self {
        match value {
            BeardType::None => 0,
            BeardType::Goatee => 1,
            BeardType::GoateeLong => 2,
            BeardType::LionsManeLong => 3,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MustacheType {
    None,
    Walrus,
    Pencil,
    Horseshoe,
}

impl TryFrom<u8> for MustacheType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Walrus),
            2 => Ok(Self::Pencil),
            3 => Ok(Self::Horseshoe),
            _ => Err(()),
        }
    }
}

impl From<MustacheType> for u8 {
    fn from(value: MustacheType) -> Self {
        match value {
            MustacheType::None => 0,
            MustacheType::Walrus => 1,
            MustacheType::Pencil => 2,
            MustacheType::Horseshoe => 3,
        }
    }
}
