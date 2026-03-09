use std::{convert::Infallible, fmt::Display};

use crate::byte_handler::{ByteHandler, ByteHandlerError, FromByteHandler};

/// Errors that can occur while constructing a [`TransmissionMod`].
#[derive(thiserror::Error, Debug)]
pub enum TransmissionModError {
    /// The transmission mod ID byte did not map to any known [`TransmissionMod`] variant.
    #[error("Invalid transmission mod ID")]
    InvalidTransmissionMod,
    /// A `ByteHandler` operation failed.
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    /// Infallible conversion error; cannot occur at runtime.
    #[error("")]
    Infallible(#[from] Infallible),
}

/// The drift transmission modifier applied to a Retro Rewind (Pulsar) ghost.
///
/// Retro Rewind supports server-side transmission overrides that force all
/// vehicles into a specific drift mode, overriding the per-vehicle default.
/// `Vanilla` indicates no override is active.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TransmissionMod {
    /// No transmission override; vehicles use their default drift behavior.
    Vanilla,
    /// All vehicles (both karts and bikes) are forced to inside drift.
    AllInside,
    /// All bikes (but not karts) are forced to inside drift.
    AllBikeInside,
    /// All vehicles (both karts and bikes) are forced to outside drift.
    AllOutside,
}

/// Converts a raw byte value into a [`TransmissionMod`].
///
/// # Errors
///
/// Returns [`TransmissionModError::InvalidTransmissionMod`] if the byte does
/// not correspond to a known variant (valid range is `0x00`–`0x03`).
impl TryFrom<u8> for TransmissionMod {
    type Error = TransmissionModError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Vanilla),
            0x01 => Ok(Self::AllInside),
            0x02 => Ok(Self::AllBikeInside),
            0x03 => Ok(Self::AllOutside),
            _ => Err(TransmissionModError::InvalidTransmissionMod),
        }
    }
}

/// Converts a [`TransmissionMod`] into its raw byte representation.
impl From<TransmissionMod> for u8 {
    fn from(value: TransmissionMod) -> Self {
        match value {
            TransmissionMod::Vanilla => 0x00,
            TransmissionMod::AllInside => 0x01,
            TransmissionMod::AllBikeInside => 0x02,
            TransmissionMod::AllOutside => 0x03,
        }
    }
}

/// Deserializes a [`TransmissionMod`] from header byte `0x0C`.
impl FromByteHandler for TransmissionMod {
    type Err = TransmissionModError;

    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<ByteHandler>,
        Self::Err: From<T::Error>,
    {
        ((handler.try_into()?.copy_byte(0) >> 1) & 0x03).try_into()
    }
}

/// Formats the transmission mod as a human-readable name.
impl Display for TransmissionMod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vanilla => write!(f, "Vanilla"),
            Self::AllInside => write!(f, "All Inside"),
            Self::AllBikeInside => write!(f, "All Bikes Inside"),
            Self::AllOutside => write!(f, "All Outside"),
        }
    }
}
