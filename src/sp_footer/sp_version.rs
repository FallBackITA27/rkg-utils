use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub struct SPVersion {
    major: u8,
    minor: u8,
    revision: u8,
}

impl SPVersion {
    /// Returns an Option<Vec<>> of possible release versions
    pub fn from(footer_version: u32) -> Option<Vec<Self>> {
        let mut possible_versions = Vec::new();
        match footer_version {
            0 => {
                possible_versions.push(Self {
                    major: 0,
                    minor: 1,
                    revision: 0,
                });
                Some(possible_versions)
            }

            1 => {
                for revision in 1..=3 {
                    possible_versions.push(Self {
                        major: 0,
                        minor: 1,
                        revision,
                    });
                }
                Some(possible_versions)
            }

            2 => {
                for revision in 4..=7 {
                    possible_versions.push(Self {
                        major: 0,
                        minor: 1,
                        revision,
                    });
                }
                Some(possible_versions)
            }

            3 => {
                for revision in 8..=9 {
                    possible_versions.push(Self {
                        major: 0,
                        minor: 1,
                        revision,
                    });
                }
                Some(possible_versions)
            }

            4 | 5 => {
                possible_versions.push(Self {
                    major: 0,
                    minor: 1,
                    revision: 10,
                });
                Some(possible_versions)
            }

            _ => None,
        }
    }
}

impl Display for SPVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.revision)
    }
}
