use crate::byte_handler::{ByteHandlerError, FromByteHandler};
use std::fmt::Display;

/// Errors that can occur while constructing a [`GhostType`].
#[derive(thiserror::Error, Debug)]
pub enum GhostTypeError {
    /// The ghost type byte did not map to any known [`GhostType`] variant.
    #[error("Nonexistent Ghost Type")]
    NonexistentGhostType,
    /// A `ByteHandler` operation failed.
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
}

/// All possible Ghost Types representable in a ghost file.
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GhostType {
    PlayerBest,
    WorldRecord,
    ContinentalRecord,
    /// A rival ghost matched against the player's time.
    Rival,
    /// A special ghost (e.g. a Nintendo event ghost).
    /// More info on special ghosts here: <https://tcrf.net/User:B_squo#About_Mario_Kart_Wii.27s_Special_Ghosts>
    Special,
    /// A ghost downloaded from the Mario Kart Channel's "Ghost Race" gamemode.
    GhostRace,
    Friend1,
    Friend2,
    Friend3,
    Friend4,
    Friend5,
    Friend6,
    Friend7,
    Friend8,
    Friend9,
    Friend10,
    Friend11,
    Friend12,
    Friend13,
    Friend14,
    Friend15,
    Friend16,
    Friend17,
    Friend18,
    Friend19,
    Friend20,
    Friend21,
    Friend22,
    Friend23,
    Friend24,
    Friend25,
    Friend26,
    Friend27,
    Friend28,
    Friend29,
    Friend30,
    NormalStaff,
    ExpertStaff,
}

impl Display for GhostType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GhostType::PlayerBest => write!(f, "Player's best time"),
            GhostType::WorldRecord => write!(f, "World record ghost"),
            GhostType::ContinentalRecord => write!(f, "Continental record ghost"),
            GhostType::Rival => write!(f, "Rival ghost"),
            GhostType::Special => write!(f, "Special ghost"),
            GhostType::GhostRace => write!(f, "Ghost Race ghost"),
            GhostType::Friend1 => write!(f, "Friend ghost #1"),
            GhostType::Friend2 => write!(f, "Friend ghost #2"),
            GhostType::Friend3 => write!(f, "Friend ghost #3"),
            GhostType::Friend4 => write!(f, "Friend ghost #4"),
            GhostType::Friend5 => write!(f, "Friend ghost #5"),
            GhostType::Friend6 => write!(f, "Friend ghost #6"),
            GhostType::Friend7 => write!(f, "Friend ghost #7"),
            GhostType::Friend8 => write!(f, "Friend ghost #8"),
            GhostType::Friend9 => write!(f, "Friend ghost #9"),
            GhostType::Friend10 => write!(f, "Friend ghost #10"),
            GhostType::Friend11 => write!(f, "Friend ghost #11"),
            GhostType::Friend12 => write!(f, "Friend ghost #12"),
            GhostType::Friend13 => write!(f, "Friend ghost #13"),
            GhostType::Friend14 => write!(f, "Friend ghost #14"),
            GhostType::Friend15 => write!(f, "Friend ghost #15"),
            GhostType::Friend16 => write!(f, "Friend ghost #16"),
            GhostType::Friend17 => write!(f, "Friend ghost #17"),
            GhostType::Friend18 => write!(f, "Friend ghost #18"),
            GhostType::Friend19 => write!(f, "Friend ghost #19"),
            GhostType::Friend20 => write!(f, "Friend ghost #20"),
            GhostType::Friend21 => write!(f, "Friend ghost #21"),
            GhostType::Friend22 => write!(f, "Friend ghost #22"),
            GhostType::Friend23 => write!(f, "Friend ghost #23"),
            GhostType::Friend24 => write!(f, "Friend ghost #24"),
            GhostType::Friend25 => write!(f, "Friend ghost #25"),
            GhostType::Friend26 => write!(f, "Friend ghost #26"),
            GhostType::Friend27 => write!(f, "Friend ghost #27"),
            GhostType::Friend28 => write!(f, "Friend ghost #28"),
            GhostType::Friend29 => write!(f, "Friend ghost #29"),
            GhostType::Friend30 => write!(f, "Friend ghost #30"),
            GhostType::NormalStaff => write!(f, "Normal staff ghost"),
            GhostType::ExpertStaff => write!(f, "Expert staff ghost"),
        }
    }
}

/// Converts a raw byte value from the RKG header into a [`GhostType`].
///
/// # Errors
///
/// Returns [`GhostTypeError::NonexistentGhostType`] if the byte does not
/// correspond to any known ghost type (valid range is `0x01`–`0x26`).
impl TryFrom<u8> for GhostType {
    type Error = GhostTypeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::PlayerBest),
            0x02 => Ok(Self::WorldRecord),
            0x03 => Ok(Self::ContinentalRecord),
            0x04 => Ok(Self::Rival),
            0x05 => Ok(Self::Special),
            0x06 => Ok(Self::GhostRace),
            0x07 => Ok(Self::Friend1),
            0x08 => Ok(Self::Friend2),
            0x09 => Ok(Self::Friend3),
            0x0A => Ok(Self::Friend4),
            0x0B => Ok(Self::Friend5),
            0x0C => Ok(Self::Friend6),
            0x0D => Ok(Self::Friend7),
            0x0E => Ok(Self::Friend8),
            0x0F => Ok(Self::Friend9),
            0x10 => Ok(Self::Friend10),
            0x11 => Ok(Self::Friend11),
            0x12 => Ok(Self::Friend12),
            0x13 => Ok(Self::Friend13),
            0x14 => Ok(Self::Friend14),
            0x15 => Ok(Self::Friend15),
            0x16 => Ok(Self::Friend16),
            0x17 => Ok(Self::Friend17),
            0x18 => Ok(Self::Friend18),
            0x19 => Ok(Self::Friend19),
            0x1A => Ok(Self::Friend20),
            0x1B => Ok(Self::Friend21),
            0x1C => Ok(Self::Friend22),
            0x1D => Ok(Self::Friend23),
            0x1E => Ok(Self::Friend24),
            0x1F => Ok(Self::Friend25),
            0x20 => Ok(Self::Friend26),
            0x21 => Ok(Self::Friend27),
            0x22 => Ok(Self::Friend28),
            0x23 => Ok(Self::Friend29),
            0x24 => Ok(Self::Friend30),
            0x25 => Ok(Self::NormalStaff),
            0x26 => Ok(Self::ExpertStaff),
            _ => Err(GhostTypeError::NonexistentGhostType),
        }
    }
}

/// Converts a [`GhostType`] into its raw byte representation for the RKG header.
impl From<GhostType> for u8 {
    fn from(value: GhostType) -> Self {
        match value {
            GhostType::PlayerBest => 0x01,
            GhostType::WorldRecord => 0x02,
            GhostType::ContinentalRecord => 0x03,
            GhostType::Rival => 0x04,
            GhostType::Special => 0x05,
            GhostType::GhostRace => 0x06,
            GhostType::Friend1 => 0x07,
            GhostType::Friend2 => 0x08,
            GhostType::Friend3 => 0x09,
            GhostType::Friend4 => 0x0A,
            GhostType::Friend5 => 0x0B,
            GhostType::Friend6 => 0x0C,
            GhostType::Friend7 => 0x0D,
            GhostType::Friend8 => 0x0E,
            GhostType::Friend9 => 0x0F,
            GhostType::Friend10 => 0x10,
            GhostType::Friend11 => 0x11,
            GhostType::Friend12 => 0x12,
            GhostType::Friend13 => 0x13,
            GhostType::Friend14 => 0x14,
            GhostType::Friend15 => 0x15,
            GhostType::Friend16 => 0x16,
            GhostType::Friend17 => 0x17,
            GhostType::Friend18 => 0x18,
            GhostType::Friend19 => 0x19,
            GhostType::Friend20 => 0x1A,
            GhostType::Friend21 => 0x1B,
            GhostType::Friend22 => 0x1C,
            GhostType::Friend23 => 0x1D,
            GhostType::Friend24 => 0x1E,
            GhostType::Friend25 => 0x1F,
            GhostType::Friend26 => 0x20,
            GhostType::Friend27 => 0x21,
            GhostType::Friend28 => 0x22,
            GhostType::Friend29 => 0x23,
            GhostType::Friend30 => 0x24,
            GhostType::NormalStaff => 0x25,
            GhostType::ExpertStaff => 0x26,
        }
    }
}

/// Deserializes a [`GhostType`] from 2 bytes at header offset `0x0C..=0x0D`.
impl FromByteHandler for GhostType {
    type Err = GhostTypeError;
    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<crate::byte_handler::ByteHandler>,
        Self::Err: From<T::Error>,
    {
        let mut handler = handler.try_into()?;
        handler.shift_right(2);
        handler.copy_byte(1).try_into()
    }
}
