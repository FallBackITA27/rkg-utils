use std::convert::Infallible;

use crate::byte_handler::{ByteHandlerError, FromByteHandler};

/// Represents the glasses customization options of a Mii,
/// including glasses type, color, size, and vertical position.
#[derive(Clone, Copy)]
pub struct Glasses {
    /// Vertical position of the glasses (0–20).
    y: u8,
    /// Glasses size (0–7).
    size: u8,
    /// Glasses frame shape/style.
    glasses_type: GlassesType,
    /// Glasses frame color.
    glasses_color: GlassesColor,
}

impl Glasses {
    /// Creates a new [`Glasses`] from its individual components.
    ///
    /// # Arguments
    ///
    /// * `y` - Vertical position of the glasses (0–20).
    /// * `size` - Glasses size (0–7).
    /// * `glasses_type` - Glasses frame shape/style.
    /// * `glasses_color` - Glasses frame color.
    ///
    /// # Errors
    ///
    /// Returns [`GlassesError::SizeInvalid`] if `size` exceeds 7.
    /// Returns [`GlassesError::YInvalid`] if `y` exceeds 20.
    pub fn new(
        y: u8,
        size: u8,
        glasses_type: GlassesType,
        glasses_color: GlassesColor,
    ) -> Result<Self, GlassesError> {
        if size > 7 {
            return Err(GlassesError::SizeInvalid);
        }
        if y > 20 {
            return Err(GlassesError::YInvalid);
        }

        Ok(Self {
            y,
            size,
            glasses_type,
            glasses_color,
        })
    }

    /// Returns the vertical position of the glasses (0–20).
    pub fn y(&self) -> u8 {
        self.y
    }

    /// Returns the glasses size (0–7).
    pub fn size(&self) -> u8 {
        self.size
    }

    /// Returns the glasses frame shape/style.
    pub fn glasses_type(&self) -> GlassesType {
        self.glasses_type
    }

    /// Returns the glasses frame color.
    pub fn glasses_color(&self) -> GlassesColor {
        self.glasses_color
    }

    /// Sets the vertical position of the glasses.
    ///
    /// # Errors
    ///
    /// Returns [`GlassesError::YInvalid`] if `y` exceeds 20.
    pub fn set_y(&mut self, y: u8) -> Result<(), GlassesError> {
        if y > 20 {
            return Err(GlassesError::YInvalid);
        }
        self.y = y;
        Ok(())
    }

    /// Sets the glasses size.
    ///
    /// # Errors
    ///
    /// Returns [`GlassesError::SizeInvalid`] if `size` exceeds 7.
    pub fn set_size(&mut self, size: u8) -> Result<(), GlassesError> {
        if size > 7 {
            return Err(GlassesError::SizeInvalid);
        }
        self.size = size;
        Ok(())
    }

    /// Sets the glasses frame shape/style.
    pub fn set_glasses_type(&mut self, glasses_type: GlassesType) {
        self.glasses_type = glasses_type;
    }

    /// Sets the glasses frame color.
    pub fn set_glasses_color(&mut self, glasses_color: GlassesColor) {
        self.glasses_color = glasses_color;
    }
}

/// Deserializes [`Glasses`] from a `ByteHandler`.
///
/// Extracts and unpacks the glasses type, vertical position, color, and size
/// from the packed Mii binary format using bit shifts and masks.
impl FromByteHandler for Glasses {
    type Err = GlassesError;
    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<crate::byte_handler::ByteHandler>,
        Self::Err: From<T::Error>,
    {
        let mut handler = handler.try_into()?;

        let glasses_type = GlassesType::try_from(handler.copy_byte(0) >> 4)
            .map_err(|_| GlassesError::TypeInvalid)?;
        let y = handler.copy_byte(1) & 0x1F;
        handler.shift_right(1);
        let glasses_color = GlassesColor::try_from(handler.copy_byte(0) & 0x03)
            .map_err(|_| GlassesError::ColorInvalid)?;
        let size = handler.copy_byte(1) >> 4;

        Self::new(y, size, glasses_type, glasses_color)
    }
}

/// Errors that can occur while constructing or deserializing [`Glasses`].
#[derive(thiserror::Error, Debug)]
pub enum GlassesError {
    /// The glasses type byte did not map to a known [`GlassesType`] variant.
    #[error("Type is invalid")]
    TypeInvalid,
    /// The glasses color byte did not map to a known [`GlassesColor`] variant.
    #[error("Color is invalid")]
    ColorInvalid,
    /// The size value exceeds the maximum of 7.
    #[error("Size is invalid")]
    SizeInvalid,
    /// The vertical position exceeds the maximum of 20.
    #[error("Y position is invalid")]
    YInvalid,
    /// A `ByteHandler` operation failed.
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    /// Infallible conversion error; cannot occur at runtime.
    #[error("")]
    Infallible(#[from] Infallible),
}

/// Glasses frame color options available in the Mii editor.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GlassesColor {
    Black,
    Brown,
    Red,
    Blue,
    Yellow,
    Gray,
}

/// Converts a raw byte value from the Mii data format into a [`GlassesColor`].
///
/// Returns `Err(())` if the byte does not correspond to any known glasses color.
impl TryFrom<u8> for GlassesColor {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Black),
            1 => Ok(Self::Brown),
            2 => Ok(Self::Red),
            3 => Ok(Self::Blue),
            4 => Ok(Self::Yellow),
            5 => Ok(Self::Gray),
            _ => Err(()),
        }
    }
}

/// Converts a [`GlassesColor`] into its raw byte representation for the Mii data format.
impl From<GlassesColor> for u8 {
    fn from(value: GlassesColor) -> Self {
        match value {
            GlassesColor::Black => 0,
            GlassesColor::Brown => 1,
            GlassesColor::Red => 2,
            GlassesColor::Blue => 3,
            GlassesColor::Yellow => 4,
            GlassesColor::Gray => 5,
        }
    }
}

/// Glasses frame shape options available in the Mii editor.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GlassesType {
    /// No glasses.
    None,
    Square,
    Rectangle,
    Rounded,
    Oval,
    CatEye,
    SemiOpaqueAviator,
    SemiOpaqueRectangle,
    SemiOpaqueCatEye,
}

/// Converts a raw byte value from the Mii data format into a [`GlassesType`].
///
/// Returns `Err(())` if the byte does not correspond to any known glasses type.
impl TryFrom<u8> for GlassesType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Square),
            2 => Ok(Self::Rectangle),
            3 => Ok(Self::Rounded),
            4 => Ok(Self::Oval),
            5 => Ok(Self::CatEye),
            6 => Ok(Self::SemiOpaqueAviator),
            7 => Ok(Self::SemiOpaqueRectangle),
            8 => Ok(Self::SemiOpaqueCatEye),
            _ => Err(()),
        }
    }
}

/// Converts a [`GlassesType`] into its raw byte representation for the Mii data format.
impl From<GlassesType> for u8 {
    fn from(value: GlassesType) -> Self {
        match value {
            GlassesType::None => 0,
            GlassesType::Square => 1,
            GlassesType::Rectangle => 2,
            GlassesType::Rounded => 3,
            GlassesType::Oval => 4,
            GlassesType::CatEye => 5,
            GlassesType::SemiOpaqueAviator => 6,
            GlassesType::SemiOpaqueRectangle => 7,
            GlassesType::SemiOpaqueCatEye => 8,
        }
    }
}
