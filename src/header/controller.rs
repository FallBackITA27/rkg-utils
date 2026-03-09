use std::convert::Infallible;
use std::fmt::Display;

use crate::byte_handler::{ByteHandlerError, FromByteHandler};

/// Errors that can occur while constructing a [`Controller`].
#[derive(thiserror::Error, Debug)]
pub enum ControllerError {
    /// The controller ID byte did not map to any known [`Controller`] variant.
    #[error("Nonexistent Controller ID")]
    NonexistentControllerID,
    /// A `ByteHandler` operation failed.
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    /// Infallible conversion error; cannot occur at runtime.
    #[error("")]
    Infallible(#[from] Infallible),
}

/// The input controller used to record a Mario Kart Wii ghost.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Controller {
    WiiWheel,
    Nunchuk,
    Classic,
    Gamecube,
}

/// Formats the controller as its display name (e.g. `"Wii Wheel"`, `"Gamecube"`).
impl Display for Controller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WiiWheel => write!(f, "Wii Wheel"),
            Self::Nunchuk => write!(f, "Nunchuk"),
            Self::Classic => write!(f, "Classic"),
            Self::Gamecube => write!(f, "Gamecube"),
        }
    }
}

/// Deserializes a [`Controller`] from header byte `0x0B`.
///
/// The controller ID is stored in the lower nibble of the byte.
impl FromByteHandler for Controller {
    type Err = ControllerError;

    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<crate::byte_handler::ByteHandler>,
        Self::Err: From<T::Error>,
    {
        (handler.try_into()?.copy_byte(0) & 0x0F).try_into()
    }
}

/// Converts a raw byte value from the RKG header into a [`Controller`].
///
/// # Errors
///
/// Returns [`ControllerError::NonexistentControllerID`] if the byte does not
/// correspond to any known controller (valid range is 0–3).
impl TryFrom<u8> for Controller {
    type Error = ControllerError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::WiiWheel),
            1 => Ok(Self::Nunchuk),
            2 => Ok(Self::Classic),
            3 => Ok(Self::Gamecube),
            _ => Err(ControllerError::NonexistentControllerID),
        }
    }
}

/// Converts a [`Controller`] into its raw byte representation for the RKG header.
impl From<Controller> for u8 {
    fn from(value: Controller) -> Self {
        match value {
            Controller::WiiWheel => 0,
            Controller::Nunchuk => 1,
            Controller::Classic => 2,
            Controller::Gamecube => 3,
        }
    }
}
