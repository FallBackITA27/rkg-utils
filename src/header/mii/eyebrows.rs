use std::convert::Infallible;

use crate::{
    byte_handler::{ByteHandlerError, FromByteHandler},
    header::mii::hair::HairColor,
};

/// Represents the eyebrow customization options of a Mii.
///
/// All positional and size values are validated against the ranges permitted
/// by the Mii data format on construction.
#[derive(Clone, Copy)]
pub struct Eyebrows {
    /// Eyebrow rotation (0–11).
    rotation: u8,
    /// Eyebrow size (0–8).
    size: u8,
    /// Horizontal position of the eyebrows (0–12).
    x: u8,
    /// Vertical position of the eyebrows (3–18).
    y: u8,
    /// Eyebrow color, shared with the hair color palette.
    eyebrow_color: HairColor,
    /// Eyebrow shape/style.
    eyebrow_type: EyebrowType,
}

impl Eyebrows {
    /// Creates a new [`Eyebrows`] from its individual components.
    ///
    /// # Arguments
    ///
    /// * `rotation` - Eyebrow rotation (0–11).
    /// * `size` - Eyebrow size (0–8).
    /// * `x` - Horizontal position (0–12).
    /// * `y` - Vertical position (3–18).
    /// * `eyebrow_color` - Eyebrow color from the [`HairColor`] palette.
    /// * `eyebrow_type` - Eyebrow shape/style.
    ///
    /// # Errors
    ///
    /// Returns [`EyebrowsError::RotationInvalid`] if `rotation` exceeds 11.
    /// Returns [`EyebrowsError::SizeInvalid`] if `size` exceeds 8.
    /// Returns [`EyebrowsError::XInvalid`] if `x` exceeds 12.
    /// Returns [`EyebrowsError::YInvalid`] if `y` is outside the range 3–18.
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

    /// Returns the eyebrow rotation (0–11).
    pub fn rotation(&self) -> u8 {
        self.rotation
    }

    /// Returns the eyebrow size (0–8).
    pub fn size(&self) -> u8 {
        self.size
    }

    /// Returns the horizontal position of the eyebrows (0–12).
    pub fn x(&self) -> u8 {
        self.x
    }

    /// Returns the vertical position of the eyebrows (3–18).
    pub fn y(&self) -> u8 {
        self.y
    }

    /// Returns the eyebrow color.
    pub fn eyebrow_color(&self) -> HairColor {
        self.eyebrow_color
    }

    /// Returns the eyebrow shape/style.
    pub fn eyebrow_type(&self) -> EyebrowType {
        self.eyebrow_type
    }

    /// Sets the eyebrow rotation.
    ///
    /// # Errors
    ///
    /// Returns [`EyebrowsError::RotationInvalid`] if `rotation` exceeds 11.
    pub fn set_rotation(&mut self, rotation: u8) -> Result<(), EyebrowsError> {
        if rotation > 11 {
            return Err(EyebrowsError::RotationInvalid);
        }
        self.rotation = rotation;
        Ok(())
    }

    /// Sets the eyebrow size.
    ///
    /// # Errors
    ///
    /// Returns [`EyebrowsError::SizeInvalid`] if `size` exceeds 8.
    pub fn set_size(&mut self, size: u8) -> Result<(), EyebrowsError> {
        if size > 8 {
            return Err(EyebrowsError::SizeInvalid);
        }
        self.size = size;
        Ok(())
    }

    /// Sets the horizontal position of the eyebrows.
    ///
    /// # Errors
    ///
    /// Returns [`EyebrowsError::XInvalid`] if `x` exceeds 12.
    pub fn set_x(&mut self, x: u8) -> Result<(), EyebrowsError> {
        if x > 12 {
            return Err(EyebrowsError::XInvalid);
        }
        self.x = x;
        Ok(())
    }

    /// Sets the vertical position of the eyebrows.
    ///
    /// # Errors
    ///
    /// Returns [`EyebrowsError::YInvalid`] if `y` is outside the range 3–18.
    pub fn set_y(&mut self, y: u8) -> Result<(), EyebrowsError> {
        if !(3..=18).contains(&y) {
            return Err(EyebrowsError::YInvalid);
        }
        self.y = y;
        Ok(())
    }

    /// Sets the eyebrow color.
    pub fn set_eyebrow_color(&mut self, eyebrow_color: HairColor) {
        self.eyebrow_color = eyebrow_color;
    }

    /// Sets the eyebrow shape/style.
    pub fn set_eyebrow_type(&mut self, eyebrow_type: EyebrowType) {
        self.eyebrow_type = eyebrow_type;
    }
}

/// Errors that can occur while constructing or deserializing [`Eyebrows`].
#[derive(thiserror::Error, Debug)]
pub enum EyebrowsError {
    /// The eyebrow type byte did not map to a known [`EyebrowType`] variant.
    #[error("Type is invalid")]
    TypeInvalid,
    /// The eyebrow color byte did not map to a known [`HairColor`] variant.
    #[error("Color is invalid")]
    ColorInvalid,
    /// The rotation value exceeds the maximum of 11.
    #[error("Rotation is invalid")]
    RotationInvalid,
    /// The size value exceeds the maximum of 8.
    #[error("Size is invalid")]
    SizeInvalid,
    /// The vertical position is outside the valid range (3–18).
    #[error("Y position is invalid")]
    YInvalid,
    /// The horizontal position exceeds the maximum of 12.
    #[error("X position is invalid")]
    XInvalid,
    /// A `ByteHandler` operation failed.
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    /// Infallible conversion error; cannot occur at runtime.
    #[error("")]
    Infallible(#[from] Infallible),
}

/// Deserializes [`Eyebrows`] from a `ByteHandler`.
///
/// Extracts and unpacks the eyebrow type, color, horizontal position, vertical position,
/// size, and rotation from the packed Mii binary format using a series of bit shifts and masks.
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

/// All eyebrow shapes available in the Mii editor.
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

/// Converts a raw byte value from the Mii data format into an [`EyebrowType`].
///
/// Returns `Err(())` if the byte does not correspond to any known eyebrow type.
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

/// Converts an [`EyebrowType`] into its raw byte representation for the Mii data format.
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
