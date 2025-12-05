use std::fmt::Display;

pub struct FinishTime {
    minutes: u8,       // 7 bits, offset 0x00
    seconds: u8,       // 7 bits, offset 0x00.7
    milliseconds: u16, // 10 bits, offset 0x01.6
}

impl FinishTime {
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

impl From<&mut bitreader::BitReader<'_>> for FinishTime {
    fn from(value: &mut bitreader::BitReader<'_>) -> Self {
        let minutes: u8 = value
            .read_u8(7)
            .expect("Failed to read minutes of finish time");

        let seconds: u8 = value
            .read_u8(7)
            .expect("Failed to read seconds of finish time");

        let milliseconds: u16 = value
            .read_u16(10)
            .expect("Failed to read milliseconds of finish time");

        Self {
            minutes,
            seconds,
            milliseconds,
        }
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
