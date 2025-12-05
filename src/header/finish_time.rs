use std::fmt::Display;

#[derive(thiserror::Error, Debug)]
pub enum FinishTimeError {
    #[error("BitReader Error: {0}")]
    BitReaderError(#[from] bitreader::BitReaderError),
}

pub struct FinishTime {
    minutes: u8,
    seconds: u8,
    milliseconds: u16,
}

impl FinishTime {
    #[inline(always)]
    pub fn new(minutes: u8, seconds: u8, milliseconds: u16) -> Self {
        Self {
            minutes,
            seconds,
            milliseconds,
        }
    }

    pub fn minutes(&self) -> u8 {
        self.minutes
    }

    pub fn seconds(&self) -> u8 {
        self.seconds
    }

    pub fn milliseconds(&self) -> u16 {
        self.milliseconds
    }
}

impl Display for FinishTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}.{:03}",
            self.minutes, self.seconds, self.milliseconds
        )
    }
}

impl TryFrom<&mut bitreader::BitReader<'_>> for FinishTime {
    type Error = FinishTimeError;
    fn try_from(value: &mut bitreader::BitReader<'_>) -> Result<Self, Self::Error> {
        Ok(Self::new(
            value.read_u8(7)?,
            value.read_u8(7)?,
            value.read_u16(10)?,
        ))
    }
}

impl From<[u8; 3]> for FinishTime {
    fn from(value: [u8; 3]) -> Self {
        // 3 Bytes, where M = Minutes, S = Seconds and C = Millis.
        // 1. 0bMMMMMMMS
        // 2. 0bSSSSSSCC
        // 3. 0bCCCCCCCC

        // max M = 5    // 0b0000101
        // max S = 59   // 0b0111011
        // max C = 999  // 0b1111100111
        // 1. 0b00001010
        // 2. 0b11101111
        // 3. 0b11100111

        Self {
            minutes: value[0],
            seconds: value[1] >> 2,
            milliseconds: (((value[1] & 0b00000011) as u16) << 8) | (value[2] as u16),
        }
    }
}
