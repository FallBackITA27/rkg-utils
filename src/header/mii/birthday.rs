use std::convert::Infallible;

use crate::byte_handler::{ByteHandlerError, FromByteHandler};

/// Errors that can occur while constructing a [`Birthday`].
#[derive(thiserror::Error, Debug)]
pub enum BirthdayError {
    /// The month value is outside the valid range (1–12).
    #[error("Month is invalid")]
    MonthInvalid,
    /// The day value is zero when a month is set, or exceeds the maximum
    /// number of days for the given month.
    #[error("Day is invalid")]
    DayInvalid,
    /// A `ByteHandler` operation failed.
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    /// Infallible conversion error; cannot occur at runtime.
    #[error("")]
    Infallible(#[from] Infallible),
}

/// Represents the birthday component of a Mii, storing an optional month and day.
///
/// Both fields are `None` when the Mii's birthday is not set (i.e. month byte is 0).
/// A month without a day is not a valid state and will be rejected by [`Birthday::new`].
#[derive(Clone, Copy)]
pub struct Birthday {
    /// The month of the birthday (1–12), or `None` if not set.
    month: Option<u8>,
    /// The day of the birthday, or `None` if not set.
    day: Option<u8>,
}

impl Birthday {
    /// Creates a new [`Birthday`] from a raw month and day value.
    ///
    /// A month of `0` is treated as "not set" and produces a [`Birthday`] where
    /// both `month` and `day` are `None`. Any other month value must be paired
    /// with a non-zero day that is valid for that month.
    ///
    /// # Arguments
    ///
    /// * `month` - The month value (0 = not set, 1–12 = January–December).
    /// * `day` - The day value (must be non-zero and valid for the given month).
    ///
    /// # Errors
    ///
    /// Returns [`BirthdayError::DayInvalid`] if:
    /// - A non-zero month is provided but `day` is `0`.
    /// - The day exceeds the maximum for the given month (31, 30, or 29 for February).
    ///
    /// Returns [`BirthdayError::MonthInvalid`] if `month` is greater than 12.
    pub fn new(month: u8, day: u8) -> Result<Self, BirthdayError> {
        match month {
            0 => Ok(Self {
                month: None,
                day: None,
            }),
            _month if day == 0 => Err(BirthdayError::DayInvalid), // Birthday with only a month is not possible
            1 | 3 | 5 | 7 | 8 | 10 | 12 if day > 31 => Err(BirthdayError::DayInvalid),
            4 | 6 | 9 | 11 if day > 30 => Err(BirthdayError::DayInvalid),
            2 if day > 29 => Err(BirthdayError::DayInvalid),
            1..=12 => Ok(Self {
                month: Some(month),
                day: Some(day),
            }),
            _ => Err(BirthdayError::MonthInvalid),
        }
    }

    /// Returns the month component of the birthday, or `None` if not set.
    pub fn month(&self) -> Option<u8> {
        self.month
    }

    /// Returns the day component of the birthday, or `None` if not set.
    pub fn day(&self) -> Option<u8> {
        self.day
    }
}

/// Deserializes a [`Birthday`] from a `ByteHandler`.
///
/// The handler is shifted right by 2 bits before extracting the month from the
/// lower nibble of the first byte and the day from the upper 5 bits of the second byte.
impl FromByteHandler for Birthday {
    type Err = BirthdayError;
    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<crate::byte_handler::ByteHandler>,
        Self::Err: From<T::Error>,
    {
        let mut handler = handler.try_into()?;
        handler.shift_right(2);
        Self::new(handler.copy_byte(0) & 0x0F, handler.copy_byte(1) >> 3)
    }
}
