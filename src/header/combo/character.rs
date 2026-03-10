use std::fmt::Display;

use crate::header::combo::weight_class::{GetWeightClass, WeightClass};

/// All playable characters in Mario Kart Wii, including Mii outfit variants and
/// menu-only characters.
///
/// Character identifiers are documented at
/// <https://wiki.tockdom.com/wiki/List_of_Identifiers#Characters>.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Character {
    Mario,
    BabyPeach,
    Waluigi,
    Bowser,
    BabyDaisy,
    DryBones,
    BabyMario,
    Luigi,
    Toad,
    DonkeyKong,
    Yoshi,
    Wario,
    BabyLuigi,
    Toadette,
    KoopaTroopa,
    Daisy,
    Peach,
    Birdo,
    DiddyKong,
    KingBoo,
    BowserJr,
    DryBowser,
    FunkyKong,
    Rosalina,
    SmallMiiOutfitAMale,
    SmallMiiOutfitAFemale,
    SmallMiiOutfitBMale,
    SmallMiiOutfitBFemale,
    SmallMiiOutfitCMale,
    SmallMiiOutfitCFemale,
    MediumMiiOutfitAMale,
    MediumMiiOutfitAFemale,
    MediumMiiOutfitBMale,
    MediumMiiOutfitBFemale,
    MediumMiiOutfitCMale,
    MediumMiiOutfitCFemale,
    LargeMiiOutfitAMale,
    LargeMiiOutfitAFemale,
    LargeMiiOutfitBMale,
    LargeMiiOutfitBFemale,
    LargeMiiOutfitCMale,
    LargeMiiOutfitCFemale,
    /// Generic medium-class Mii without a specific outfit variant.
    MediumMii,
    /// Generic small-class Mii without a specific outfit variant.
    SmallMii,
    /// Generic large-class Mii without a specific outfit variant.
    LargeMii,
    /// Peach as she appears in menu screens; cannot appear in ghost files.
    MenuPeach,
    /// Daisy as she appears in menu screens; cannot appear in ghost files.
    MenuDaisy,
    /// Rosalina as she appears in menu screens; cannot appear in ghost files.
    MenuRosalina,
}

/// Formats the character as her/his display name (e.g. `"Donkey Kong"`, `"Baby Peach"`).
impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Character::Mario => "Mario",
            Character::BabyPeach => "Baby Peach",
            Character::Waluigi => "Waluigi",
            Character::Bowser => "Bowser",
            Character::BabyDaisy => "Baby Daisy",
            Character::DryBones => "Dry Bones",
            Character::BabyMario => "Baby Mario",
            Character::Luigi => "Luigi",
            Character::Toad => "Toad",
            Character::DonkeyKong => "Donkey Kong",
            Character::Yoshi => "Yoshi",
            Character::Wario => "Wario",
            Character::BabyLuigi => "Baby Luigi",
            Character::Toadette => "Toadette",
            Character::KoopaTroopa => "Koopa Troopa",
            Character::Daisy => "Daisy",
            Character::Peach => "Peach",
            Character::Birdo => "Birdo",
            Character::DiddyKong => "Diddy Kong",
            Character::KingBoo => "King Boo",
            Character::BowserJr => "Bowser Jr.",
            Character::DryBowser => "Dry Bowser",
            Character::FunkyKong => "Funky Kong",
            Character::Rosalina => "Rosalina",
            Character::SmallMiiOutfitAMale => "Small Mii Outfit A (Male)",
            Character::SmallMiiOutfitAFemale => "Small Mii Outfit A (Female)",
            Character::SmallMiiOutfitBMale => "Small Mii Outfit B (Male)",
            Character::SmallMiiOutfitBFemale => "Small Mii Outfit B (Female)",
            Character::SmallMiiOutfitCMale => "Small Mii Outfit C (Male)",
            Character::SmallMiiOutfitCFemale => "Small Mii Outfit C (Female)",
            Character::MediumMiiOutfitAMale => "Medium Mii Outfit A (Male)",
            Character::MediumMiiOutfitAFemale => "Medium Mii Outfit A (Female)",
            Character::MediumMiiOutfitBMale => "Medium Mii Outfit B (Male)",
            Character::MediumMiiOutfitBFemale => "Medium Mii Outfit B (Female)",
            Character::MediumMiiOutfitCMale => "Medium Mii Outfit C (Male)",
            Character::MediumMiiOutfitCFemale => "Medium Mii Outfit C (Female)",
            Character::LargeMiiOutfitAMale => "Large Mii Outfit A (Male)",
            Character::LargeMiiOutfitAFemale => "Large Mii Outfit A (Female)",
            Character::LargeMiiOutfitBMale => "Large Mii Outfit B (Male)",
            Character::LargeMiiOutfitBFemale => "Large Mii Outfit B (Female)",
            Character::LargeMiiOutfitCMale => "Large Mii Outfit C (Male)",
            Character::LargeMiiOutfitCFemale => "Large Mii Outfit C (Female)",
            Character::MediumMii => "Medium Mii",
            Character::SmallMii => "Small Mii",
            Character::LargeMii => "Large Mii",
            Character::MenuPeach => "Peach (Menu)",
            Character::MenuDaisy => "Daisy (Menu)",
            Character::MenuRosalina => "Rosalina (Menu)",
        };
        write!(f, "{}", s)
    }
}

impl Character {
    /// Returns `true` if this character cannot legitimately appear in a ghost file.
    ///
    /// The impossible characters are the generic Mii variants (`SmallMii`, `MediumMii`,
    /// `LargeMii`) and the menu-only versions of Peach, Daisy, and Rosalina.
    pub fn is_impossible(self) -> bool {
        match self {
            Self::Mario
            | Self::BabyPeach
            | Self::Waluigi
            | Self::Bowser
            | Self::BabyDaisy
            | Self::DryBones
            | Self::BabyMario
            | Self::Luigi
            | Self::Toad
            | Self::DonkeyKong
            | Self::Yoshi
            | Self::Wario
            | Self::BabyLuigi
            | Self::Toadette
            | Self::KoopaTroopa
            | Self::Daisy
            | Self::Peach
            | Self::Birdo
            | Self::DiddyKong
            | Self::KingBoo
            | Self::BowserJr
            | Self::DryBowser
            | Self::FunkyKong
            | Self::Rosalina
            | Self::SmallMiiOutfitAMale
            | Self::SmallMiiOutfitAFemale
            | Self::SmallMiiOutfitBMale
            | Self::SmallMiiOutfitBFemale
            | Self::SmallMiiOutfitCMale
            | Self::SmallMiiOutfitCFemale
            | Self::MediumMiiOutfitAMale
            | Self::MediumMiiOutfitAFemale
            | Self::MediumMiiOutfitBMale
            | Self::MediumMiiOutfitBFemale
            | Self::MediumMiiOutfitCMale
            | Self::MediumMiiOutfitCFemale
            | Self::LargeMiiOutfitAMale
            | Self::LargeMiiOutfitAFemale
            | Self::LargeMiiOutfitBMale
            | Self::LargeMiiOutfitBFemale
            | Self::LargeMiiOutfitCMale
            | Self::LargeMiiOutfitCFemale => false,
            Self::MenuPeach
            | Self::MenuDaisy
            | Self::MenuRosalina
            | Self::MediumMii
            | Self::SmallMii
            | Self::LargeMii => true,
        }
    }
}

/// Converts a raw byte value from the RKG header into a [`Character`].
///
/// Returns `Err(())` if the byte does not correspond to any known character.
impl TryFrom<u8> for Character {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Mario),
            0x01 => Ok(Self::BabyPeach),
            0x02 => Ok(Self::Waluigi),
            0x03 => Ok(Self::Bowser),
            0x04 => Ok(Self::BabyDaisy),
            0x05 => Ok(Self::DryBones),
            0x06 => Ok(Self::BabyMario),
            0x07 => Ok(Self::Luigi),
            0x08 => Ok(Self::Toad),
            0x09 => Ok(Self::DonkeyKong),
            0x0A => Ok(Self::Yoshi),
            0x0B => Ok(Self::Wario),
            0x0C => Ok(Self::BabyLuigi),
            0x0D => Ok(Self::Toadette),
            0x0E => Ok(Self::KoopaTroopa),
            0x0F => Ok(Self::Daisy),
            0x10 => Ok(Self::Peach),
            0x11 => Ok(Self::Birdo),
            0x12 => Ok(Self::DiddyKong),
            0x13 => Ok(Self::KingBoo),
            0x14 => Ok(Self::BowserJr),
            0x15 => Ok(Self::DryBowser),
            0x16 => Ok(Self::FunkyKong),
            0x17 => Ok(Self::Rosalina),
            0x18 => Ok(Self::SmallMiiOutfitAMale),
            0x19 => Ok(Self::SmallMiiOutfitAFemale),
            0x1A => Ok(Self::SmallMiiOutfitBMale),
            0x1B => Ok(Self::SmallMiiOutfitBFemale),
            0x1C => Ok(Self::SmallMiiOutfitCMale),
            0x1D => Ok(Self::SmallMiiOutfitCFemale),
            0x1E => Ok(Self::MediumMiiOutfitAMale),
            0x1F => Ok(Self::MediumMiiOutfitAFemale),
            0x20 => Ok(Self::MediumMiiOutfitBMale),
            0x21 => Ok(Self::MediumMiiOutfitBFemale),
            0x22 => Ok(Self::MediumMiiOutfitCMale),
            0x23 => Ok(Self::MediumMiiOutfitCFemale),
            0x24 => Ok(Self::LargeMiiOutfitAMale),
            0x25 => Ok(Self::LargeMiiOutfitAFemale),
            0x26 => Ok(Self::LargeMiiOutfitBMale),
            0x27 => Ok(Self::LargeMiiOutfitBFemale),
            0x28 => Ok(Self::LargeMiiOutfitCMale),
            0x29 => Ok(Self::LargeMiiOutfitCFemale),
            0x2A => Ok(Self::MediumMii),
            0x2B => Ok(Self::SmallMii),
            0x2C => Ok(Self::LargeMii),
            0x2D => Ok(Self::MenuPeach),
            0x2E => Ok(Self::MenuDaisy),
            0x2F => Ok(Self::MenuRosalina),
            _ => Err(()),
        }
    }
}

/// Converts a [`Character`] into its raw byte representation for the RKG header.
impl From<Character> for u8 {
    fn from(value: Character) -> Self {
        match value {
            Character::Mario => 0x00,
            Character::BabyPeach => 0x01,
            Character::Waluigi => 0x02,
            Character::Bowser => 0x03,
            Character::BabyDaisy => 0x04,
            Character::DryBones => 0x05,
            Character::BabyMario => 0x06,
            Character::Luigi => 0x07,
            Character::Toad => 0x08,
            Character::DonkeyKong => 0x09,
            Character::Yoshi => 0x0A,
            Character::Wario => 0x0B,
            Character::BabyLuigi => 0x0C,
            Character::Toadette => 0x0D,
            Character::KoopaTroopa => 0x0E,
            Character::Daisy => 0x0F,
            Character::Peach => 0x10,
            Character::Birdo => 0x11,
            Character::DiddyKong => 0x12,
            Character::KingBoo => 0x13,
            Character::BowserJr => 0x14,
            Character::DryBowser => 0x15,
            Character::FunkyKong => 0x16,
            Character::Rosalina => 0x17,
            Character::SmallMiiOutfitAMale => 0x18,
            Character::SmallMiiOutfitAFemale => 0x19,
            Character::SmallMiiOutfitBMale => 0x1A,
            Character::SmallMiiOutfitBFemale => 0x1B,
            Character::SmallMiiOutfitCMale => 0x1C,
            Character::SmallMiiOutfitCFemale => 0x1D,
            Character::MediumMiiOutfitAMale => 0x1E,
            Character::MediumMiiOutfitAFemale => 0x1F,
            Character::MediumMiiOutfitBMale => 0x20,
            Character::MediumMiiOutfitBFemale => 0x21,
            Character::MediumMiiOutfitCMale => 0x22,
            Character::MediumMiiOutfitCFemale => 0x23,
            Character::LargeMiiOutfitAMale => 0x24,
            Character::LargeMiiOutfitAFemale => 0x25,
            Character::LargeMiiOutfitBMale => 0x26,
            Character::LargeMiiOutfitBFemale => 0x27,
            Character::LargeMiiOutfitCMale => 0x28,
            Character::LargeMiiOutfitCFemale => 0x29,
            Character::MediumMii => 0x2A,
            Character::SmallMii => 0x2B,
            Character::LargeMii => 0x2C,
            Character::MenuPeach => 0x2D,
            Character::MenuDaisy => 0x2E,
            Character::MenuRosalina => 0x2F,
        }
    }
}

/// Returns the [`WeightClass`] of this character.
impl GetWeightClass for Character {
    fn get_weight_class(&self) -> WeightClass {
        match self {
            Self::BabyDaisy
            | Self::BabyLuigi
            | Self::BabyMario
            | Self::BabyPeach
            | Self::DryBones
            | Self::KoopaTroopa
            | Self::SmallMii
            | Self::SmallMiiOutfitAMale
            | Self::SmallMiiOutfitAFemale
            | Self::SmallMiiOutfitBMale
            | Self::SmallMiiOutfitBFemale
            | Self::SmallMiiOutfitCMale
            | Self::SmallMiiOutfitCFemale
            | Self::Toad
            | Self::Toadette => WeightClass::Small,
            Self::Birdo
            | Self::BowserJr
            | Self::Daisy
            | Self::MenuDaisy
            | Self::DiddyKong
            | Self::Luigi
            | Self::Mario
            | Self::MediumMii
            | Self::MediumMiiOutfitAMale
            | Self::MediumMiiOutfitAFemale
            | Self::MediumMiiOutfitBMale
            | Self::MediumMiiOutfitBFemale
            | Self::MediumMiiOutfitCMale
            | Self::MediumMiiOutfitCFemale
            | Self::Peach
            | Self::MenuPeach
            | Self::Yoshi => WeightClass::Medium,
            Self::Bowser
            | Self::DonkeyKong
            | Self::DryBowser
            | Self::FunkyKong
            | Self::KingBoo
            | Self::LargeMii
            | Self::LargeMiiOutfitAMale
            | Self::LargeMiiOutfitAFemale
            | Self::LargeMiiOutfitBMale
            | Self::LargeMiiOutfitBFemale
            | Self::LargeMiiOutfitCMale
            | Self::LargeMiiOutfitCFemale
            | Self::MenuRosalina
            | Self::Rosalina
            | Self::Waluigi
            | Self::Wario => WeightClass::Large,
        }
    }
}
