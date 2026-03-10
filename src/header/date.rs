use crate::byte_handler::{ByteHandlerError, FromByteHandler};

/// Errors that can occur while constructing a [`Date`].
#[derive(thiserror::Error, Debug)]
pub enum DateError {
    /// The year is outside the valid range (2000–2035).
    #[error("Year is invalid")]
    YearInvalid,
    /// The month is outside the valid range (1–12).
    #[error("Month is invalid")]
    MonthInvalid,
    /// The day is zero, or exceeds the maximum number of days for the given month and year.
    #[error("Day is invalid")]
    DayInvalid,
    /// A `ByteHandler` operation failed.
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
}

/// All valid dates on the Wii. Year is a range between 0 and 35, symbolizing 2000 and 2035
/// respectively. Leap years are accounted for when validating February day counts.
#[derive(Debug)]
pub struct Date {
    /// Year offset from 2000 (0–35), representing 2000–2035.
    year: u8,
    /// Month (1–12).
    month: u8,
    /// Day of the month.
    day: u8,
}

impl Date {
    /// Creates a new [`Date`] from a full year, month, and day.
    ///
    /// # Arguments
    ///
    /// * `year` - The full calendar year (2000–2035).
    /// * `month` - The month (1–12).
    /// * `day` - The day of the month, validated against the given month and year.
    ///
    /// # Errors
    ///
    /// Returns [`DateError::YearInvalid`] if `year` is outside the range 2000–2035.
    /// Returns [`DateError::MonthInvalid`] if `month` is outside the range 1–12.
    /// Returns [`DateError::DayInvalid`] if `day` exceeds the maximum for the given
    /// month, accounting for leap years in February.
    pub fn new(year: u16, month: u8, day: u8) -> Result<Self, DateError> {
        let year = (year - 2000) as u8;

        if year > 35 {
            return Err(DateError::YearInvalid);
        }

        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 if day > 31 => Err(DateError::DayInvalid),
            4 | 6 | 9 | 11 if day > 30 => Err(DateError::DayInvalid),
            2 if year.is_multiple_of(4) && day > 29 => Err(DateError::DayInvalid),
            2 if !year.is_multiple_of(4) && day > 28 => Err(DateError::DayInvalid),
            1..=12 => Ok(Self { year, month, day }),
            _ => Err(DateError::MonthInvalid),
        }
    }

    /// Returns the full calendar year (2000–2035).
    ///
    /// The year is stored internally as an offset from 2000; this method adds
    /// 2000 to produce the full four-digit year.
    pub fn year(&self) -> u16 {
        (self.year as u16) + 2000
    }

    /// Returns the month (1–12).
    pub fn month(&self) -> u8 {
        self.month
    }

    /// Returns the day of the month.
    pub fn day(&self) -> u8 {
        self.day
    }
}

/// Deserializes a [`Date`] from 3 bytes at header offset `0x09..=0x0B`.
///
/// The bytes are packed as follows:
/// ```text
/// Byte 1: XXXXYYYY
/// Byte 2: YYYMMMMD
/// Byte 3: DDDDXXXX
/// ```
/// where `Y` = year bits, `M` = month bits, and `D` = day bits.
impl FromByteHandler for Date {
    type Err = DateError;
    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<crate::byte_handler::ByteHandler>,
        Self::Err: From<T::Error>,
    {
        let mut handler = handler.try_into()?;

        handler.shift_right(4);
        let year = ((handler.copy_byte(1) >> 1) as u16) + 2000;
        let day = handler.copy_byte(2) & 0x1F;
        handler.shift_right(5);
        let month = handler.copy_byte(2) & 0x0F;

        Self::new(year, month, day)
    }
}

/// Formats [`Date`] as YYYY-MM-DD
impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "20{:02}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

/// Two [`Date`] values are equal if their year, month, and day are all identical.
impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day && self.month == other.month && self.year == other.year
    }
}
