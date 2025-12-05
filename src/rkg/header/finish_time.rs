use std::fmt::Display;

use bitreader::BitReader;

pub struct FinishTime {
    minutes: u8,       // 7 bits, offset 0x00
    seconds: u8,       // 7 bits, offset 0x00.7
    milliseconds: u16, // 10 bits, offset 0x01.6
}

impl FinishTime {
    pub fn minutes(&self) -> u8 {
        self.minutes
    }

    pub fn seconds(&self) -> u8 {
        self.seconds
    }

    pub fn milliseconds(&self) -> u16 {
        self.milliseconds
    }

    pub fn from_reader(rkg_reader: &mut BitReader<'_>) -> Self {
        // Get finish time fields
        let minutes: u8 = rkg_reader
            .read_u8(7)
            .expect("Failed to read minutes of finish time");

        let seconds: u8 = rkg_reader
            .read_u8(7)
            .expect("Failed to read seconds of finish time");

        let milliseconds: u16 = rkg_reader
            .read_u16(10)
            .expect("Failed to read milliseconds of finish time");

        Self {
            minutes,
            seconds,
            milliseconds,
        }
    }

    pub fn new(minutes: u8, seconds: u8, milliseconds: u16) -> Self {
        Self {
            minutes,
            seconds,
            milliseconds,
        }
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