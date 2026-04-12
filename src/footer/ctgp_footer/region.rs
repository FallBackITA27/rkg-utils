/// Enum representing possible game regions
#[derive(Clone, Copy, Debug)]
pub enum Region {
    NtscU,
    Pal,
    NtscJ,
    Unknown,
}

impl From<u8> for Region {
    fn from(value: u8) -> Self {
        match value {
            b'E' => Self::NtscU,
            b'P' => Self::Pal,
            b'J' => Self::NtscJ,
            _ => Self::Unknown,
        }
    }
}
