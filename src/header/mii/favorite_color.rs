use std::convert::Infallible;

use crate::byte_handler::{ByteHandlerError, FromByteHandler};

/// Errors that can occur while constructing a [`FavoriteColor`].
#[derive(thiserror::Error, Debug)]
pub enum FavoriteColorError {
    /// The color byte did not map to a known [`FavoriteColor`] variant.
    #[error("Color is invalid")]
    ColorInvalid,
    /// A `ByteHandler` operation failed.
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    /// Infallible conversion error; cannot occur at runtime.
    #[error("")]
    Infallible(#[from] Infallible),
}

/// The Mii's favorite color, as selected in the Mii editor.
///
/// This color is used to tint the Mii's outfit and other UI elements associated with the Mii.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FavoriteColor {
    Red,
    Orange,
    Yellow,
    LimeGreen,
    ForestGreen,
    RoyalBlue,
    SkyBlue,
    Pink,
    Purple,
    Brown,
    White,
    Black,
}

/// Converts a raw byte value from the Mii data format into a [`FavoriteColor`].
///
/// # Errors
///
/// Returns [`FavoriteColorError::ColorInvalid`] if the byte does not correspond
/// to any known favorite color (valid range is 0–11).
impl TryFrom<u8> for FavoriteColor {
    type Error = FavoriteColorError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Red),
            1 => Ok(Self::Orange),
            2 => Ok(Self::Yellow),
            3 => Ok(Self::LimeGreen),
            4 => Ok(Self::ForestGreen),
            5 => Ok(Self::RoyalBlue),
            6 => Ok(Self::SkyBlue),
            7 => Ok(Self::Pink),
            8 => Ok(Self::Purple),
            9 => Ok(Self::Brown),
            10 => Ok(Self::White),
            11 => Ok(Self::Black),
            _ => Err(FavoriteColorError::ColorInvalid),
        }
    }
}

/// Converts a [`FavoriteColor`] into its raw byte representation for the Mii data format.
impl From<FavoriteColor> for u8 {
    fn from(value: FavoriteColor) -> Self {
        match value {
            FavoriteColor::Red => 0,
            FavoriteColor::Orange => 1,
            FavoriteColor::Yellow => 2,
            FavoriteColor::LimeGreen => 3,
            FavoriteColor::ForestGreen => 4,
            FavoriteColor::RoyalBlue => 5,
            FavoriteColor::SkyBlue => 6,
            FavoriteColor::Pink => 7,
            FavoriteColor::Purple => 8,
            FavoriteColor::Brown => 9,
            FavoriteColor::White => 10,
            FavoriteColor::Black => 11,
        }
    }
}

/// Deserializes a [`FavoriteColor`] from a `ByteHandler`.
///
/// Expects byte `0x01` of the Mii data. The color index is extracted by shifting
/// the byte right by 1 and masking the lower 4 bits.
impl FromByteHandler for FavoriteColor {
    type Err = FavoriteColorError;

    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<crate::byte_handler::ByteHandler>,
        Self::Err: From<T::Error>,
    {
        let handler = handler.try_into()?;
        Self::try_from((handler.copy_byte(0) >> 1) & 0x0F)
    }
}
