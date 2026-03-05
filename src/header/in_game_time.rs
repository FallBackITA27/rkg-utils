use std::fmt::Display;

use crate::byte_handler::{ByteHandler, ByteHandlerError, FromByteHandler};

#[derive(thiserror::Error, Debug)]
pub enum InGameTimeError {
    #[error("Insufficiently Long Iterator")]
    InsufficientlyLongIterator,
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
}

// Struct size is 32 bits, copy is fine
#[derive(Default, Clone, Copy)]
pub struct InGameTime {
    minutes: u8,
    seconds: u8,
    milliseconds: u16,
}

impl InGameTime {
    #[inline(always)]
    pub fn new(minutes: u8, seconds: u8, milliseconds: u16) -> Self {
        Self {
            minutes,
            seconds,
            milliseconds,
        }
    }

    pub fn minutes(self) -> u8 {
        self.minutes
    }

    pub fn seconds(self) -> u8 {
        self.seconds
    }

    pub fn milliseconds(self) -> u16 {
        self.milliseconds
    }

    pub fn is_technically_valid(self) -> bool {
        self.minutes > 5 || self.seconds > 59 || self.milliseconds > 999
    }

    pub fn igt_to_millis(self) -> u32 {
        (self.milliseconds as u32) + (self.seconds as u32) * 1000 + (self.minutes as u32) * 60000
    }
}

impl Display for InGameTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}.{:03}",
            self.minutes, self.seconds, self.milliseconds
        )
    }
}

impl FromByteHandler for InGameTime {
    type Err = InGameTimeError;

    /// 3 Bytes, where M = Minutes, S = Seconds and C = Millis.
    /// 1. 0bMMMMMMMS
    /// 2. 0bSSSSSSCC
    /// 3. 0bCCCCCCCC
    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<ByteHandler>,
        Self::Err: From<T::Error>,
    {
        let mut handler = handler.try_into()?;

        handler.shift_right(1);
        let minutes = handler.copy_byte(0);
        let seconds = handler.copy_byte(1) >> 1;
        handler.shift_left(9);

        Ok(Self {
            minutes,
            seconds,
            milliseconds: handler.copy_word(0) & 0x3FF,
        })
    }
}

impl std::ops::Add for InGameTime {
    type Output = InGameTime;

    fn add(self, rhs: InGameTime) -> InGameTime {
        let total_millis = self.igt_to_millis() + rhs.igt_to_millis();

        let milliseconds = (total_millis % 1000) as u16;
        let total_seconds = total_millis / 1000;
        let seconds = (total_seconds % 60) as u8;
        let minutes = (total_seconds / 60) as u8;

        InGameTime::new(minutes, seconds, milliseconds)
    }
}

impl std::iter::Sum for InGameTime {
    fn sum<I: Iterator<Item = InGameTime>>(iter: I) -> Self {
        iter.fold(InGameTime::default(), |a, b| a + b)
    }
}
