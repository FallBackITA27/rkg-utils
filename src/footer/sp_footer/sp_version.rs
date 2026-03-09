use std::fmt::Display;

/// Represents a MKW Service Pack (SP) version number with major, minor, and revision components.
///
/// Versions are formatted as `major.minor.revision` (e.g. `0.1.4`).
#[derive(Clone, Copy, Debug)]
pub struct SPVersion {
    /// Major version number.
    major: u8,
    /// Minor version number.
    minor: u8,
    /// Revision number.
    revision: u8,
}

impl SPVersion {
    /// Attempts to map an SP footer version integer to one or more possible release versions.
    ///
    /// Because multiple SP releases can share the same footer version number, this returns a
    /// [`Vec`] of all [`SPVersion`]s consistent with the given footer version. Returns
    /// [`None`] if the footer version is not recognised.
    ///
    /// # Arguments
    ///
    /// * `footer_version` - The raw SP footer version integer.
    ///
    /// # Returns
    ///
    /// * `Some(Vec<SPVersion>)` - One or more possible versions matching the footer version.
    /// * `None` - The footer version does not correspond to any known SP release.
    ///
    /// # Examples
    ///
    /// ```
    /// use rkg_utils::footer::sp_footer::sp_version::SPVersion;
    ///
    /// // Unambiguous mapping
    /// let versions = SPVersion::from(0).unwrap();
    /// assert_eq!(versions.len(), 1);
    ///
    /// // Ambiguous mapping — multiple releases share this footer version
    /// let versions = SPVersion::from(1).unwrap();
    /// assert_eq!(versions.len(), 3);
    ///
    /// // Unknown footer version
    /// assert!(SPVersion::from(99).is_none());
    /// ```
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

/// Formats the version as `major.minor.revision` (e.g. `0.1.4`).
impl Display for SPVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.revision)
    }
}
