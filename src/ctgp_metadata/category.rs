#[derive(thiserror::Error, Debug)]
pub enum CategoryError {
    #[error("Nonexistent Category")]
    NonexistentCategory,
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// Represents the category of the run via CTGP's metadata info
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
