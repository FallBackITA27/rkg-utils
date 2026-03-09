use std::convert::Infallible;

use crate::byte_handler::{ByteHandlerError, FromByteHandler};

/// Represents the nose customization options of a Mii,
/// including nose style, size, and vertical position.
#[derive(Clone, Copy)]
pub struct Nose {
    /// Vertical position of the nose (0–18).
    y: u8,
    /// Nose size (0–8).
    size: u8,
    /// Nose shape/style.
    nose_type: NoseType,
}

impl Nose {
    /// Creates a new [`Nose`] from its individual components.
    ///
    /// # Arguments
    ///
    /// * `y` - Vertical position of the nose (0–18).
    /// * `size` - Nose size (0–8).
    /// * `nose_type` - Nose shape/style.
    ///
    /// # Errors
    ///
    /// Returns [`NoseError::SizeInvalid`] if `size` exceeds 8.
    /// Returns [`NoseError::YInvalid`] if `y` exceeds 18.
    pub fn new(y: u8, size: u8, nose_type: NoseType) -> Result<Self, NoseError> {
        if size > 8 {
            return Err(NoseError::SizeInvalid);
        }
        if y > 18 {
            return Err(NoseError::YInvalid);
        }

        Ok(Self { y, size, nose_type })
    }

    /// Returns the vertical position of the nose (0–18).
    pub fn y(&self) -> u8 {
        self.y
    }

    /// Returns the nose size (0–8).
    pub fn size(&self) -> u8 {
        self.size
    }

    /// Returns the nose shape/style.
    pub fn nose_type(&self) -> NoseType {
        self.nose_type
    }

    /// Sets the vertical position of the nose.
    ///
    /// # Errors
    ///
    /// Returns [`NoseError::YInvalid`] if `y` exceeds 18.
    pub fn set_y(&mut self, y: u8) -> Result<(), NoseError> {
        if y > 18 {
            return Err(NoseError::YInvalid);
        }
        self.y = y;
        Ok(())
    }

    /// Sets the nose size.
    ///
    /// # Errors
    ///
    /// Returns [`NoseError::SizeInvalid`] if `size` exceeds 8.
    pub fn set_size(&mut self, size: u8) -> Result<(), NoseError> {
        if size > 8 {
            return Err(NoseError::SizeInvalid);
        }
        self.size = size;
        Ok(())
    }

    /// Sets the nose shape/style.
    pub fn set_nose_type(&mut self, nose_type: NoseType) {
        self.nose_type = nose_type;
    }
}

/// Deserializes a [`Nose`] from a `ByteHandler`.
///
/// Extracts the nose type from the upper nibble of the first byte, the size from
/// the lower nibble of the first byte, and the vertical position from the upper
/// 5 bits of the second byte.
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

/// Errors that can occur while constructing or deserializing a [`Nose`].
#[derive(thiserror::Error, Debug)]
pub enum NoseError {
    /// The nose type byte did not map to a known [`NoseType`] variant.
    #[error("Type is invalid")]
    TypeInvalid,
    /// The size value exceeds the maximum of 8.
    #[error("Size is invalid")]
    SizeInvalid,
    /// The vertical position exceeds the maximum of 18.
    #[error("Y position is invalid")]
    YInvalid,
    /// A `ByteHandler` operation failed.
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    /// Infallible conversion error; cannot occur at runtime.
    #[error("")]
    Infallible(#[from] Infallible),
}

/// Nose shape options available in the Mii editor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NoseType {
    Normal,
    Rounded,
    Dot,
    Arrow,
    /// Straight, prominent Roman-style nose.
    Roman,
    Triangle,
    Button,
    RoundedInverted,
    Potato,
    /// Long, straight Grecian-style nose.
    Grecian,
    /// Short, upturned snub nose.
    Snub,
    /// Curved, downward-pointing aquiline nose.
    Aquiline,
}

/// Converts a raw byte value from the Mii data format into a [`NoseType`].
///
/// Returns `Err(())` if the byte does not correspond to any known nose type.
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

/// Converts a [`NoseType`] into its raw byte representation for the Mii data format.
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
