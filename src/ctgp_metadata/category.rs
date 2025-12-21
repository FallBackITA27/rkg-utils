#[derive(thiserror::Error, Debug)]
pub enum CategoryError {
    #[error("Nonexistent Category")]
    NonexistentCategory,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Category {
    NoShortcut,
    Shortcut,
    Glitch,
    NoShortcutTAS,
    ShortcutTAS,
    GlitchTAS,
}

impl Category {
    pub fn try_from(category: u8, shortcut: u8) -> Result<Self, CategoryError> {
        match category {
            0x01 => Ok(Self::Glitch),
            0x02 => Ok(Self::NoShortcut),
            0x13 => Ok(Self::GlitchTAS),
            0x23 => Ok(Self::NoShortcutTAS),
            0x00 => {
                if shortcut == 0 {
                    Ok(Self::NoShortcut)
                } else {
                    Ok(Self::Shortcut)
                }
            }
            0x03 => {
                if shortcut == 0 {
                    Ok(Self::NoShortcutTAS)
                } else {
                    Ok(Self::ShortcutTAS)
                }
            }
            _ => Err(CategoryError::NonexistentCategory),
        }
    }
}
