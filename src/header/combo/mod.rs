pub mod character;
pub mod transmission;
pub mod vehicle;
pub mod weight_class;

use crate::{
    byte_handler::{ByteHandler, ByteHandlerError, FromByteHandler},
    header::combo::{
        character::Character,
        transmission::Transmission,
        vehicle::Vehicle,
        weight_class::{GetWeightClass, WeightClass},
    },
};
use std::fmt::Display;

/// Represents a valid character and vehicle combination from a Mario Kart Wii RKG ghost file.
///
/// A combo is only valid when the character and vehicle share the same [`weight_class::WeightClass`].
/// Construction via [`Combo::new`] enforces this constraint.
pub struct Combo {
    /// The character used in the run.
    character: Character,
    /// The vehicle used in the run.
    vehicle: Vehicle,
}

impl Combo {
    pub const fn get_transmission(&self) -> Transmission {
        self.vehicle.get_transmission()
    }
}

/// Formats the combo as `"{character} on {vehicle}"` (e.g. `"Mario on Mach Bike"`).
impl Display for Combo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} on {}", self.character(), self.vehicle())
    }
}

/// Errors that can occur while constructing or deserializing a [`Combo`].
#[derive(thiserror::Error, Debug)]
pub enum ComboError {
    /// The input iterator did not contain enough bytes to extract a combo.
    #[error("Insufficiently Long Iterator")]
    InsufficientlyLongIterator,
    /// The character and vehicle belong to different weight classes.
    #[error("The combo has incongruent weight classes")]
    IncongruentWeightClasses,
    /// The vehicle byte did not map to a known [`Vehicle`] variant.
    #[error("Invalid Vehicle ID")]
    InvalidVehicleId,
    /// The character byte did not map to a known [`Character`] variant.
    #[error("Invalid Character ID")]
    InvalidCharacterId,
    /// The character ID corresponds to a character that cannot appear in ghost files.
    #[error("Impossible Character ID")]
    ImpossibleCharacterId,
    /// A `ByteHandler` operation failed.
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
}

impl Combo {
    /// Creates a new [`Combo`] from a vehicle and character.
    ///
    /// # Errors
    ///
    /// Returns [`ComboError::IncongruentWeightClasses`] if the character and vehicle
    /// do not share the same [`WeightClass`].
    #[inline(always)]
    pub fn new(vehicle: Vehicle, character: Character) -> Result<Self, ComboError> {
        if character.get_weight_class() != vehicle.get_weight_class() {
            return Err(ComboError::IncongruentWeightClasses);
        }

        Ok(Self { vehicle, character })
    }

    /// Returns the character used in the run.
    pub const fn character(&self) -> Character {
        self.character
    }

    /// Returns the vehicle used in the run.
    pub const fn vehicle(&self) -> Vehicle {
        self.vehicle
    }
}

/// Deserializes a [`Combo`] from a `ByteHandler` containing 2 bytes at header offset `0x08..0x0A`.
///
/// The bytes are packed as follows:
/// ```text
/// Byte 1: VVVVVVCC
/// Byte 2: CCCCXXXX
/// ```
/// where `V` = vehicle ID bits and `C` = character ID bits.
impl FromByteHandler for Combo {
    type Err = ComboError;

    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<ByteHandler>,
        Self::Err: From<T::Error>,
    {
        let mut handler = handler.try_into()?;

        handler.shift_right(2); // 1. 00VVVVVV
        let vehicle = handler.copy_byte(0);

        handler.shift_right(2); // 2. VVCCCCCC
        let character = handler.copy_byte(1) & 0x3F;

        Self::new(
            Vehicle::try_from(vehicle).map_err(|_| ComboError::InvalidVehicleId)?,
            Character::try_from(character).map_err(|_| ComboError::InvalidCharacterId)?,
        )
    }
}

/// Returns the weight class of the combo, which is always equal to both the
/// character's and vehicle's weight class (enforced at construction time).
impl GetWeightClass for Combo {
    fn get_weight_class(&self) -> WeightClass {
        self.character.get_weight_class()
    }
}
