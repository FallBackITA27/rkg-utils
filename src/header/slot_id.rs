// https://wiki.tockdom.com/wiki/Slot

use std::convert::Infallible;
use std::fmt::Display;

use crate::byte_handler::{ByteHandler, ByteHandlerError, FromByteHandler};

#[derive(thiserror::Error, Debug)]
pub enum SlotIdError {
    #[error("Non Existent Slot ID")]
    NonExistentSlotId,
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    #[error("")]
    Infallible(#[from] Infallible),
}

/// All Nintendo-Assigned Slot IDs. Some have special effects such as switching OST.
/// https://wiki.tockdom.com/wiki/List_of_Identifiers#Courses
/// https://wiki.tockdom.com/wiki/Slot
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SlotId {
    // Normal Tracks
    LuigiCircuit,
    MooMooMeadows,
    MushroomGorge,
    ToadsFactory,
    MarioCircuit,
    CoconutMall,
    DKSnowboardCross,
    WariosGoldMine,
    DaisyCircuit,
    KoopaCape,
    MapleTreeway,
    GrumbleVolcano,
    DryDryRuins,
    MoonviewHighway,
    BowsersCastle,
    RainbowRoad,
    GCNPeachBeach,
    DSYoshiFalls,
    SNESGhostValley2,
    N64MarioRaceway,
    N64SherbetLand,
    GBAShyGuyBeach,
    DSDelfinoSquare,
    GCNWaluigiStadium,
    DSDesertHills,
    GBABowserCastle3,
    N64DKJungleParkway,
    GCNMarioCircuit,
    SNESMarioCircuit3,
    DSPeachGardens,
    GCNDKMountain,
    N64BowsersCastle,

    // Battle Arenas
    BlockPlaza,
    DelfinoPier,
    FunkyStadium,
    ChainChompWheel,
    ThwompDesert,
    SNESBattleCourse4,
    GBABattleCourse3,
    N64Skscraper,
    GCNCookieLand,
    DSTwilightHouse,

    // Other Slots
    GalaxyColosseum, // Only technically possible one?
    WinningScene,
    LosingScene,
    Credits,
}

impl Display for SlotId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            // Normal Tracks
            SlotId::LuigiCircuit => "Luigi Circuit",
            SlotId::MooMooMeadows => "Moo Moo Meadows",
            SlotId::MushroomGorge => "Mushroom Gorge",
            SlotId::ToadsFactory => "Toad's Factory",
            SlotId::MarioCircuit => "Mario Circuit",
            SlotId::CoconutMall => "Coconut Mall",
            SlotId::DKSnowboardCross => "DK Summit",
            SlotId::WariosGoldMine => "Wario's Gold Mine",
            SlotId::DaisyCircuit => "Daisy Circuit",
            SlotId::KoopaCape => "Koopa Cape",
            SlotId::MapleTreeway => "Maple Treeway",
            SlotId::GrumbleVolcano => "Grumble Volcano",
            SlotId::DryDryRuins => "Dry Dry Ruins",
            SlotId::MoonviewHighway => "Moonview Highway",
            SlotId::BowsersCastle => "Bowser's Castle",
            SlotId::RainbowRoad => "Rainbow Road",

            SlotId::GCNPeachBeach => "GCN Peach Beach",
            SlotId::DSYoshiFalls => "DS Yoshi Falls",
            SlotId::SNESGhostValley2 => "SNES Ghost Valley 2",
            SlotId::N64MarioRaceway => "N64 Mario Raceway",
            SlotId::N64SherbetLand => "N64 Sherbet Land",
            SlotId::GBAShyGuyBeach => "GBA Shy Guy Beach",
            SlotId::DSDelfinoSquare => "DS Delfino Square",
            SlotId::GCNWaluigiStadium => "GCN Waluigi Stadium",
            SlotId::DSDesertHills => "DS Desert Hills",
            SlotId::GBABowserCastle3 => "GBA Bowser Castle 3",
            SlotId::N64DKJungleParkway => "N64 DK Jungle Parkway",
            SlotId::GCNMarioCircuit => "GCN Mario Circuit",
            SlotId::SNESMarioCircuit3 => "SNES Mario Circuit 3",
            SlotId::DSPeachGardens => "DS Peach Gardens",
            SlotId::GCNDKMountain => "GCN DK Mountain",
            SlotId::N64BowsersCastle => "N64 Bowser's Castle",

            // Battle Arenas
            SlotId::BlockPlaza => "Block Plaza",
            SlotId::DelfinoPier => "Delfino Pier",
            SlotId::FunkyStadium => "Funky Stadium",
            SlotId::ChainChompWheel => "Chain Chomp Wheel",
            SlotId::ThwompDesert => "Thwomp Desert",
            SlotId::SNESBattleCourse4 => "SNES Battle Course 4",
            SlotId::GBABattleCourse3 => "GBA Battle Course 3",
            SlotId::N64Skscraper => "N64 Skyscraper",
            SlotId::GCNCookieLand => "GCN Cookie Land",
            SlotId::DSTwilightHouse => "DS Twilight House",

            // Other Slots
            SlotId::GalaxyColosseum => "Galaxy Colosseum",
            SlotId::WinningScene => "Winning Scene",
            SlotId::LosingScene => "Losing Scene",
            SlotId::Credits => "Credits",
        };

        write!(f, "{}", s)
    }
}

impl From<SlotId> for u8 {
    fn from(value: SlotId) -> u8 {
        match value {
            SlotId::LuigiCircuit => 0x08,
            SlotId::MooMooMeadows => 0x01,
            SlotId::MushroomGorge => 0x02,
            SlotId::ToadsFactory => 0x04,
            SlotId::MarioCircuit => 0x00,
            SlotId::CoconutMall => 0x05,
            SlotId::DKSnowboardCross => 0x06,
            SlotId::WariosGoldMine => 0x07,
            SlotId::DaisyCircuit => 0x09,
            SlotId::KoopaCape => 0x0F,
            SlotId::MapleTreeway => 0x0B,
            SlotId::GrumbleVolcano => 0x03,
            SlotId::DryDryRuins => 0x0E,
            SlotId::MoonviewHighway => 0x0A,
            SlotId::BowsersCastle => 0x0C,
            SlotId::RainbowRoad => 0x0D,
            SlotId::GCNPeachBeach => 0x10,
            SlotId::DSYoshiFalls => 0x14,
            SlotId::SNESGhostValley2 => 0x19,
            SlotId::N64MarioRaceway => 0x1A,
            SlotId::N64SherbetLand => 0x1B,
            SlotId::GBAShyGuyBeach => 0x1F,
            SlotId::DSDelfinoSquare => 0x17,
            SlotId::GCNWaluigiStadium => 0x12,
            SlotId::DSDesertHills => 0x15,
            SlotId::GBABowserCastle3 => 0x1E,
            SlotId::N64DKJungleParkway => 0x1D,
            SlotId::GCNMarioCircuit => 0x11,
            SlotId::SNESMarioCircuit3 => 0x18,
            SlotId::DSPeachGardens => 0x16,
            SlotId::GCNDKMountain => 0x13,
            SlotId::N64BowsersCastle => 0x1C,
            SlotId::BlockPlaza => 0x21,
            SlotId::DelfinoPier => 0x20,
            SlotId::FunkyStadium => 0x23,
            SlotId::ChainChompWheel => 0x22,
            SlotId::ThwompDesert => 0x24,
            SlotId::SNESBattleCourse4 => 0x27,
            SlotId::GBABattleCourse3 => 0x28,
            SlotId::N64Skscraper => 0x29,
            SlotId::GCNCookieLand => 0x25,
            SlotId::DSTwilightHouse => 0x26,
            SlotId::GalaxyColosseum => 0xC9,
            SlotId::WinningScene => 0x37,
            SlotId::LosingScene => 0x38,
            SlotId::Credits => 0x3A,
        }
    }
}

impl TryFrom<u8> for SlotId {
    type Error = SlotIdError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x08 => Ok(SlotId::LuigiCircuit),
            0x01 => Ok(SlotId::MooMooMeadows),
            0x02 => Ok(SlotId::MushroomGorge),
            0x04 => Ok(SlotId::ToadsFactory),
            0x00 => Ok(SlotId::MarioCircuit),
            0x05 => Ok(SlotId::CoconutMall),
            0x06 => Ok(SlotId::DKSnowboardCross),
            0x07 => Ok(SlotId::WariosGoldMine),
            0x09 => Ok(SlotId::DaisyCircuit),
            0x0F => Ok(SlotId::KoopaCape),
            0x0B => Ok(SlotId::MapleTreeway),
            0x03 => Ok(SlotId::GrumbleVolcano),
            0x0E => Ok(SlotId::DryDryRuins),
            0x0A => Ok(SlotId::MoonviewHighway),
            0x0C => Ok(SlotId::BowsersCastle),
            0x0D => Ok(SlotId::RainbowRoad),
            0x10 => Ok(SlotId::GCNPeachBeach),
            0x14 => Ok(SlotId::DSYoshiFalls),
            0x19 => Ok(SlotId::SNESGhostValley2),
            0x1A => Ok(SlotId::N64MarioRaceway),
            0x1B => Ok(SlotId::N64SherbetLand),
            0x1F => Ok(SlotId::GBAShyGuyBeach),
            0x17 => Ok(SlotId::DSDelfinoSquare),
            0x12 => Ok(SlotId::GCNWaluigiStadium),
            0x15 => Ok(SlotId::DSDesertHills),
            0x1E => Ok(SlotId::GBABowserCastle3),
            0x1D => Ok(SlotId::N64DKJungleParkway),
            0x11 => Ok(SlotId::GCNMarioCircuit),
            0x18 => Ok(SlotId::SNESMarioCircuit3),
            0x16 => Ok(SlotId::DSPeachGardens),
            0x13 => Ok(SlotId::GCNDKMountain),
            0x1C => Ok(SlotId::N64BowsersCastle),
            0x21 => Ok(SlotId::BlockPlaza),
            0x20 => Ok(SlotId::DelfinoPier),
            0x23 => Ok(SlotId::FunkyStadium),
            0x22 => Ok(SlotId::ChainChompWheel),
            0x24 => Ok(SlotId::ThwompDesert),
            0x27 => Ok(SlotId::SNESBattleCourse4),
            0x28 => Ok(SlotId::GBABattleCourse3),
            0x29 => Ok(SlotId::N64Skscraper),
            0x25 => Ok(SlotId::GCNCookieLand),
            0x26 => Ok(SlotId::DSTwilightHouse),
            0xC9 => Ok(SlotId::GalaxyColosseum),
            0x37 => Ok(SlotId::WinningScene),
            0x38 => Ok(SlotId::LosingScene),
            0x3A => Ok(SlotId::Credits),
            _ => Err(SlotIdError::NonExistentSlotId),
        }
    }
}

impl FromByteHandler for SlotId {
    type Err = SlotIdError;
    /// Expects Header 0x07
    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<ByteHandler>,
        Self::Err: From<T::Error>,
    {
        (handler.try_into()?.copy_byte(0) >> 2).try_into()
    }
}
