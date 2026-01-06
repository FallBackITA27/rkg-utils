// TODO: consider only using v1.3 regions for this
// AnyGlobe Changer, TT-Rec, and Auto TT Recorder (extended region stuff) all use Regions v1.3, so to keep consistency potentially only use 1.3 here

use crate::header::location::{country::Country, subregion::Subregion};

pub mod country;
pub mod subregion;

/// Represents the country and subregion of the player. https://docs.google.com/spreadsheets/d/1mSAomO_msfNllNsPeXbgU6UbJaGV5t6NvbZi6ebPFx4/edit?usp=sharing
pub struct Location {
    country: Country,
    subregion: Subregion,
}

impl Location {
    pub fn new(country_id: u8, subregion_id: u8) {
        let country = Country::try_from(country_id);
    }
}