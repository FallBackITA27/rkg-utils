use crate::header::location::constants::{Country, LocationFinder, Subregion, Version};

pub mod constants;

pub struct Location {
    country: Country,
    subregion: Subregion,
    version: Version,
}

impl Location {
    pub fn country(&self) -> Country {
        self.country
    }

    pub fn subregion(&self) -> Subregion {
        self.subregion
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn find(country_id: u8, subregion_id: u8, version: Option<Version>) -> Option<Location> {
        match LocationFinder::find(country_id, subregion_id, version) {
            LocationFinder::None => None,
            LocationFinder::Exact(v) | LocationFinder::Adjusted(v) => Some(v),
        }
    }

    pub fn find_exact(country_id: u8, subregion_id: u8, version: Version) -> Option<Location> {
        match LocationFinder::find(country_id, subregion_id, Some(version)) {
            LocationFinder::None | LocationFinder::Adjusted(_) => None,
            LocationFinder::Exact(v) => Some(v),
        }
    }

    pub fn change_version(&self, version: Version) -> Option<Self> {
        Self::find_exact(self.country.into(), self.subregion.into(), version)
    }
}

impl Default for Location {
    fn default() -> Self {
        Location {
            country: Country::NotSet,
            subregion: Subregion::NotSet,
            version: Version::Vanilla,
        }
    }
}
