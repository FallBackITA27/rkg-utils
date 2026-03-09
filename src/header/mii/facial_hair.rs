use std::convert::Infallible;

use crate::{
    byte_handler::{ByteHandlerError, FromByteHandler},
    header::mii::hair::HairColor,
};

/// Represents the facial hair customization options of a Mii,
/// including beard type, mustache type, color, and mustache size and position.
#[derive(Clone, Copy)]
pub struct FacialHair {
    /// Beard shape/style.
    beard_type: BeardType,
    /// Mustache shape/style.
    mustache_type: MustacheType,
    /// Facial hair color, shared with the hair color palette.
    color: HairColor,
    /// Mustache size (0–8).
    mustache_size: u8,
    /// Vertical position of the mustache (0–16).
    mustache_y: u8,
}

impl FacialHair {
    /// Creates a new [`FacialHair`] from its individual components.
    ///
    /// # Arguments
    ///
    /// * `beard_type` - Beard shape/style.
    /// * `mustache_type` - Mustache shape/style.
    /// * `color` - Facial hair color from the [`HairColor`] palette.
    /// * `mustache_size` - Mustache size (0–8).
    /// * `mustache_y` - Vertical position of the mustache (0–16).
    ///
    /// # Errors
    ///
    /// Returns [`FacialHairError::SizeInvalid`] if `mustache_size` exceeds 8.
    /// Returns [`FacialHairError::YInvalid`] if `mustache_y` exceeds 16.
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

    /// Returns the beard shape/style.
    pub fn beard_type(&self) -> BeardType {
        self.beard_type
    }

    /// Returns the mustache shape/style.
    pub fn mustache_type(&self) -> MustacheType {
        self.mustache_type
    }

    /// Returns the facial hair color.
    pub fn color(&self) -> HairColor {
        self.color
    }

    /// Returns the mustache size (0–8).
    pub fn mustache_size(&self) -> u8 {
        self.mustache_size
    }

    /// Returns the vertical position of the mustache (0–16).
    pub fn mustache_y(&self) -> u8 {
        self.mustache_y
    }

    /// Sets the beard shape/style.
    pub fn set_beard_type(&mut self, beard_type: BeardType) {
        self.beard_type = beard_type;
    }

    /// Sets the mustache shape/style.
    pub fn set_mustache_type(&mut self, mustache_type: MustacheType) {
        self.mustache_type = mustache_type;
    }

    /// Sets the facial hair color.
    pub fn set_color(&mut self, color: HairColor) {
        self.color = color;
    }

    /// Sets the mustache size.
    ///
    /// # Errors
    ///
    /// Returns [`FacialHairError::SizeInvalid`] if `mustache_size` exceeds 8.
    pub fn set_mustache_size(&mut self, mustache_size: u8) -> Result<(), FacialHairError> {
        if mustache_size > 8 {
            return Err(FacialHairError::SizeInvalid);
        }
        self.mustache_size = mustache_size;
        Ok(())
    }

    /// Sets the vertical position of the mustache.
    ///
    /// # Errors
    ///
    /// Returns [`FacialHairError::YInvalid`] if `mustache_y` exceeds 16.
    pub fn set_mustache_y(&mut self, mustache_y: u8) -> Result<(), FacialHairError> {
        if mustache_y > 16 {
            return Err(FacialHairError::YInvalid);
        }
        self.mustache_y = mustache_y;
        Ok(())
    }
}

/// Deserializes [`FacialHair`] from a `ByteHandler`.
///
/// Extracts and unpacks the mustache vertical position, mustache size, facial hair color,
/// mustache type, and beard type from the packed Mii binary format using bit shifts and masks.
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

/// Errors that can occur while constructing or deserializing [`FacialHair`].
#[derive(thiserror::Error, Debug)]
pub enum FacialHairError {
    /// The beard type byte did not map to a known [`BeardType`] variant.
    #[error("Beard Type is invalid")]
    BeardTypeInvalid,
    /// The mustache type byte did not map to a known [`MustacheType`] variant.
    #[error("Mustache Type is invalid")]
    MustacheTypeInvalid,
    /// The facial hair color byte did not map to a known [`HairColor`] variant.
    #[error("Color is invalid")]
    ColorInvalid,
    /// The mustache size exceeds the maximum of 8.
    #[error("Size is invalid")]
    SizeInvalid,
    /// The mustache vertical position exceeds the maximum of 16.
    #[error("Y position is invalid")]
    YInvalid,
    /// A `ByteHandler` operation failed.
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    /// Infallible conversion error; cannot occur at runtime.
    #[error("")]
    Infallible(#[from] Infallible),
}

/// Beard styles available in the Mii editor.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BeardType {
    /// No beard.
    None,
    /// Short goatee.
    Goatee,
    /// Long goatee.
    GoateeLong,
    /// Long lion's mane style beard.
    LionsManeLong,
}

/// Converts a raw byte value from the Mii data format into a [`BeardType`].
///
/// Returns `Err(())` if the byte does not correspond to any known beard type.
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

/// Converts a [`BeardType`] into its raw byte representation for the Mii data format.
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

/// Mustache styles available in the Mii editor.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MustacheType {
    /// No mustache.
    None,
    /// Full, drooping walrus mustache.
    Walrus,
    /// Thin pencil mustache.
    Pencil,
    /// Horseshoe-shaped mustache.
    Horseshoe,
}

/// Converts a raw byte value from the Mii data format into a [`MustacheType`].
///
/// Returns `Err(())` if the byte does not correspond to any known mustache type.
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

/// Converts a [`MustacheType`] into its raw byte representation for the Mii data format.
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
