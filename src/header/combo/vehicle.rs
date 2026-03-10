use std::fmt::Display;

use crate::header::combo::{
    transmission::Transmission,
    weight_class::{GetWeightClass, WeightClass},
};

/// Represents all drivable vehicles in Mario Kart Wii.
///
/// Vehicle identifiers are documented at
/// <https://wiki.tockdom.com/wiki/List_of_Identifiers#Vehicles>.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Vehicle {
    StandardKartS,
    StandardKartM,
    StandardKartL,
    BoosterSeat,
    ClassicDragster,
    Offroader,
    MiniBeast,
    WildWing,
    FlameFlyer,
    CheepCharger,
    SuperBlooper,
    PiranhaProwler,
    TinyTitan,
    Daytripper,
    Jetsetter,
    BlueFalcon,
    Sprinter,
    Honeycoupe,
    StandardBikeS,
    StandardBikeM,
    StandardBikeL,
    BulletBike,
    MachBike,
    FlameRunner,
    BitBike,
    Sugarscoot,
    WarioBike,
    Quacker,
    ZipZip,
    ShootingStar,
    Magikruiser,
    Sneakster,
    Spear,
    JetBubble,
    DolphinDasher,
    Phantom,
}

/// Formats the vehicle as its display name (e.g. `"Mach Bike"`, `"Flame Runner"`).
impl Display for Vehicle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Vehicle::StandardKartS => "Standard Kart S",
            Vehicle::StandardKartM => "Standard Kart M",
            Vehicle::StandardKartL => "Standard Kart L",
            Vehicle::BoosterSeat => "Booster Seat",
            Vehicle::ClassicDragster => "Classic Dragster",
            Vehicle::Offroader => "Offroader",
            Vehicle::MiniBeast => "Mini Beast",
            Vehicle::WildWing => "Wild Wing",
            Vehicle::FlameFlyer => "Flame Flyer",
            Vehicle::CheepCharger => "Cheep Charger",
            Vehicle::SuperBlooper => "Super Blooper",
            Vehicle::PiranhaProwler => "Piranha Prowler",
            Vehicle::TinyTitan => "Tiny Titan",
            Vehicle::Daytripper => "Daytripper",
            Vehicle::Jetsetter => "Jetsetter",
            Vehicle::BlueFalcon => "Blue Falcon",
            Vehicle::Sprinter => "Sprinter",
            Vehicle::Honeycoupe => "Honeycoupe",
            Vehicle::StandardBikeS => "Standard Bike S",
            Vehicle::StandardBikeM => "Standard Bike M",
            Vehicle::StandardBikeL => "Standard Bike L",
            Vehicle::BulletBike => "Bullet Bike",
            Vehicle::MachBike => "Mach Bike",
            Vehicle::FlameRunner => "Flame Runner",
            Vehicle::BitBike => "Bit Bike",
            Vehicle::Sugarscoot => "Sugarscoot",
            Vehicle::WarioBike => "Wario Bike",
            Vehicle::Quacker => "Quacker",
            Vehicle::ZipZip => "Zip Zip",
            Vehicle::ShootingStar => "Shooting Star",
            Vehicle::Magikruiser => "Magikruiser",
            Vehicle::Sneakster => "Sneakster",
            Vehicle::Spear => "Spear",
            Vehicle::JetBubble => "Jet Bubble",
            Vehicle::DolphinDasher => "Dolphin Dasher",
            Vehicle::Phantom => "Phantom",
        };

        write!(f, "{}", s)
    }
}

/// Converts a raw byte value from the RKG header into a [`Vehicle`].
///
/// Returns `Err(())` if the byte does not correspond to any known vehicle.
impl TryFrom<u8> for Vehicle {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::StandardKartS),
            0x01 => Ok(Self::StandardKartM),
            0x02 => Ok(Self::StandardKartL),
            0x03 => Ok(Self::BoosterSeat),
            0x04 => Ok(Self::ClassicDragster),
            0x05 => Ok(Self::Offroader),
            0x06 => Ok(Self::MiniBeast),
            0x07 => Ok(Self::WildWing),
            0x08 => Ok(Self::FlameFlyer),
            0x09 => Ok(Self::CheepCharger),
            0x0A => Ok(Self::SuperBlooper),
            0x0B => Ok(Self::PiranhaProwler),
            0x0C => Ok(Self::TinyTitan),
            0x0D => Ok(Self::Daytripper),
            0x0E => Ok(Self::Jetsetter),
            0x0F => Ok(Self::BlueFalcon),
            0x10 => Ok(Self::Sprinter),
            0x11 => Ok(Self::Honeycoupe),
            0x12 => Ok(Self::StandardBikeS),
            0x13 => Ok(Self::StandardBikeM),
            0x14 => Ok(Self::StandardBikeL),
            0x15 => Ok(Self::BulletBike),
            0x16 => Ok(Self::MachBike),
            0x17 => Ok(Self::FlameRunner),
            0x18 => Ok(Self::BitBike),
            0x19 => Ok(Self::Sugarscoot),
            0x1A => Ok(Self::WarioBike),
            0x1B => Ok(Self::Quacker),
            0x1C => Ok(Self::ZipZip),
            0x1D => Ok(Self::ShootingStar),
            0x1E => Ok(Self::Magikruiser),
            0x1F => Ok(Self::Sneakster),
            0x20 => Ok(Self::Spear),
            0x21 => Ok(Self::JetBubble),
            0x22 => Ok(Self::DolphinDasher),
            0x23 => Ok(Self::Phantom),
            _ => Err(()),
        }
    }
}

/// Converts a [`Vehicle`] into its raw byte representation for the RKG header.
impl From<Vehicle> for u8 {
    fn from(value: Vehicle) -> Self {
        match value {
            Vehicle::StandardKartS => 0x00,
            Vehicle::StandardKartM => 0x01,
            Vehicle::StandardKartL => 0x02,
            Vehicle::BoosterSeat => 0x03,
            Vehicle::ClassicDragster => 0x04,
            Vehicle::Offroader => 0x05,
            Vehicle::MiniBeast => 0x06,
            Vehicle::WildWing => 0x07,
            Vehicle::FlameFlyer => 0x08,
            Vehicle::CheepCharger => 0x09,
            Vehicle::SuperBlooper => 0x0A,
            Vehicle::PiranhaProwler => 0x0B,
            Vehicle::TinyTitan => 0x0C,
            Vehicle::Daytripper => 0x0D,
            Vehicle::Jetsetter => 0x0E,
            Vehicle::BlueFalcon => 0x0F,
            Vehicle::Sprinter => 0x10,
            Vehicle::Honeycoupe => 0x11,
            Vehicle::StandardBikeS => 0x12,
            Vehicle::StandardBikeM => 0x13,
            Vehicle::StandardBikeL => 0x14,
            Vehicle::BulletBike => 0x15,
            Vehicle::MachBike => 0x16,
            Vehicle::FlameRunner => 0x17,
            Vehicle::BitBike => 0x18,
            Vehicle::Sugarscoot => 0x19,
            Vehicle::WarioBike => 0x1A,
            Vehicle::Quacker => 0x1B,
            Vehicle::ZipZip => 0x1C,
            Vehicle::ShootingStar => 0x1D,
            Vehicle::Magikruiser => 0x1E,
            Vehicle::Sneakster => 0x1F,
            Vehicle::Spear => 0x20,
            Vehicle::JetBubble => 0x21,
            Vehicle::DolphinDasher => 0x22,
            Vehicle::Phantom => 0x23,
        }
    }
}

/// Returns the [`WeightClass`] of this vehicle.
impl GetWeightClass for Vehicle {
    fn get_weight_class(&self) -> WeightClass {
        match self {
            Self::StandardKartS
            | Self::BoosterSeat
            | Self::MiniBeast
            | Self::CheepCharger
            | Self::TinyTitan
            | Self::BlueFalcon
            | Self::StandardBikeS
            | Self::BulletBike
            | Self::BitBike
            | Self::Quacker
            | Self::Magikruiser
            | Self::JetBubble => WeightClass::Small,
            Self::StandardKartM
            | Self::ClassicDragster
            | Self::WildWing
            | Self::SuperBlooper
            | Self::Daytripper
            | Self::Sprinter
            | Self::StandardBikeM
            | Self::MachBike
            | Self::Sugarscoot
            | Self::ZipZip
            | Self::Sneakster
            | Self::DolphinDasher => WeightClass::Medium,
            Self::Offroader
            | Self::StandardKartL
            | Self::FlameFlyer
            | Self::PiranhaProwler
            | Self::Jetsetter
            | Self::Honeycoupe
            | Self::StandardBikeL
            | Self::FlameRunner
            | Self::WarioBike
            | Self::ShootingStar
            | Self::Spear
            | Self::Phantom => WeightClass::Large,
        }
    }
}

impl Vehicle {
    pub const fn is_bike(&self) -> bool {
        match self {
            Self::BulletBike
            | Self::Quacker
            | Self::Magikruiser
            | Self::MachBike
            | Self::Sneakster
            | Self::StandardBikeL
            | Self::WarioBike
            | Self::ShootingStar
            | Self::Phantom
            | Self::FlameRunner
            | Self::Spear
            | Self::BitBike
            | Self::StandardBikeS
            | Self::StandardBikeM
            | Self::Jetsetter
            | Self::Sugarscoot
            | Self::ZipZip
            | Self::DolphinDasher
            | Self::JetBubble => true,
            Self::StandardKartS
            | Self::BoosterSeat
            | Self::MiniBeast
            | Self::CheepCharger
            | Self::TinyTitan
            | Self::BlueFalcon
            | Self::StandardKartM
            | Self::ClassicDragster
            | Self::WildWing
            | Self::SuperBlooper
            | Self::Daytripper
            | Self::Sprinter
            | Self::Offroader
            | Self::StandardKartL
            | Self::FlameFlyer
            | Self::PiranhaProwler
            | Self::Honeycoupe => false,
        }
    }

    pub const fn get_transmission(&self) -> Transmission {
        match self {
            Self::BulletBike
            | Self::Quacker
            | Self::Magikruiser
            | Self::MachBike
            | Self::Sneakster
            | Self::FlameRunner
            | Self::Spear
            | Self::DolphinDasher
            | Self::JetBubble => Transmission::Inside,
            Self::StandardKartS
            | Self::BoosterSeat
            | Self::MiniBeast
            | Self::CheepCharger
            | Self::TinyTitan
            | Self::BlueFalcon
            | Self::BitBike
            | Self::StandardBikeS
            | Self::StandardKartM
            | Self::ClassicDragster
            | Self::WildWing
            | Self::SuperBlooper
            | Self::Daytripper
            | Self::Sprinter
            | Self::StandardBikeM
            | Self::Sugarscoot
            | Self::ZipZip
            | Self::Offroader
            | Self::StandardKartL
            | Self::FlameFlyer
            | Self::PiranhaProwler
            | Self::Jetsetter
            | Self::Honeycoupe
            | Self::StandardBikeL
            | Self::WarioBike
            | Self::ShootingStar
            | Self::Phantom => Transmission::Outside,
        }
    }
}
