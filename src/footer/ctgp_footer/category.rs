/// Error type for CTGP category-related failures
#[derive(thiserror::Error, Debug)]
pub enum CategoryError {
    #[error("Nonexistent Category")]
    NonexistentCategory,
}

/// Enum representing what CTGP categorizes the run as.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Category {
    NoShortcut,
    Normal,
    Shortcut,
    Glitch,
    NoShortcutTAS,
    NormalTAS,
    ShortcutTAS,
    GlitchTAS,
    NoShortcut200cc,
    Normal200cc,
    Shortcut200cc,
    Glitch200cc,
    NoShortcut200ccTAS,
    Normal200ccTAS,
    Shortcut200ccTAS,
    Glitch200ccTAS,
}

impl Category {
    /// Attempts to construct a [`Category`] from raw CTGP footer bytes.
    ///
    /// The `category` byte encodes the ruleset and speed class, while the
    /// `shortcut` byte disambiguates between [`Category::Normal`]/[`Category::Shortcut`]
    /// (and their TAS/200cc equivalents) when `category` is `0x00`, `0x03`, `0x04`, or `0x07`.
    ///
    /// # Arguments
    ///
    /// * `category` - The primary category byte from CTGP metadata.
    /// * `shortcut` - The shortcut flag byte; a non-zero value indicates shortcuts are used.
    ///
    /// # Errors
    ///
    /// Returns [`CategoryError::NonexistentCategory`] if the `category` byte does not
    /// correspond to any known category.
    ///
    /// # Examples
    ///
    /// ```
    /// use rkg_utils::footer::ctgp_footer::category::Category;
    ///
    /// assert_eq!(Category::try_from(0x00, 0).unwrap(), Category::Normal);
    /// assert_eq!(Category::try_from(0x00, 1).unwrap(), Category::Shortcut);
    /// assert_eq!(Category::try_from(0x01, 0).unwrap(), Category::Glitch);
    /// assert!(Category::try_from(0xFF, 0).is_err());
    /// ```
    pub fn try_from(category: u8, shortcut: u8) -> Result<Self, CategoryError> {
        match category {
            0x00 => {
                if shortcut == 0 {
                    Ok(Self::Normal)
                } else {
                    Ok(Self::Shortcut)
                }
            }
            0x01 => Ok(Self::Glitch),
            0x02 => Ok(Self::NoShortcut),

            0x03 => {
                if shortcut == 0 {
                    Ok(Self::NormalTAS)
                } else {
                    Ok(Self::ShortcutTAS)
                }
            }
            0x13 => Ok(Self::GlitchTAS),
            0x23 => Ok(Self::NoShortcutTAS),

            0x04 => {
                if shortcut == 0 {
                    Ok(Self::Normal200cc)
                } else {
                    Ok(Self::Shortcut200cc)
                }
            }
            0x05 => Ok(Self::Glitch200cc),
            0x06 => Ok(Self::NoShortcut200cc),

            0x07 => {
                if shortcut == 0 {
                    Ok(Self::Normal200ccTAS)
                } else {
                    Ok(Self::Shortcut200ccTAS)
                }
            }
            0x17 => Ok(Self::Glitch200ccTAS),
            0x27 => Ok(Self::NoShortcut200ccTAS),

            _ => Err(CategoryError::NonexistentCategory),
        }
    }
}
