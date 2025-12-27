use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub struct CTGPVersion {
    major: u8,
    minor: u8,
    revision: u16,
}

impl CTGPVersion {
    pub fn new(bytes: [u8; 4]) -> Self {
        // TODO: Figure out what the correct CTGP version is (probably through extensive testing + a lookup table)
        let major = bytes[0];
        let minor = bytes[1];
        let revision = 1182;

        Self {
            major,
            minor,
            revision,
        }
    }
}

impl Display for CTGPVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}.{:02}.{:04}",
            self.major, self.minor, self.revision
        )
    }
}
