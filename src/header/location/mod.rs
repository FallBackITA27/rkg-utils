use crate::header::location::constants::{Country, LocationFinder, Subregion, Version};

pub mod constants;

/// Represents a player's geographic location as recorded in the ghost file header,
/// consisting of a country, subregion, and regions version.
pub struct Location {
    /// The country component of the location.
    country: Country,
    /// The subregion component of the location.
    subregion: Subregion,
    /// The region version associated with this location entry.
    version: Version,
}

impl Location {
    /// Returns the country component of this location.
    pub fn country(&self) -> Country {
        self.country
    }

    /// Returns the subregion component of this location.
    pub fn subregion(&self) -> Subregion {
        self.subregion
    }

    /// Returns the region version associated with this location.
    pub fn version(&self) -> Version {
        self.version
    }


    /// Looks up a [`Location`] by country ID, subregion ID, and optional region version.
    ///
    /// If `version` is `None`, the lookup may return an adjusted match (i.e. the
    /// closest valid location). Both exact and adjusted matches are returned as `Some`.
    ///
    /// Returns `None` if no matching location can be found.
    ///
    /// # Arguments
    ///
    /// * `country_id` - The numeric country identifier.
    /// * `subregion_id` - The numeric subregion identifier.
    /// * `version` - An optional region version to constrain the lookup.
    pub fn find(country_id: u8, subregion_id: u8, version: Option<Version>) -> Option<Location> {
        match LocationFinder::find(country_id, subregion_id, version) {
            LocationFinder::None => None,
            LocationFinder::Exact(v) | LocationFinder::Adjusted(v) => Some(v),
        }
    }

    /// Looks up a [`Location`] by country ID, subregion ID, and exact version.
    ///
    /// Unlike [`Location::find`], this only returns `Some` when the lookup produces
    /// an exact match. Adjusted matches and missing entries both return `None`.
    ///
    /// # Arguments
    ///
    /// * `country_id` - The numeric country identifier.
    /// * `subregion_id` - The numeric subregion identifier.
    /// * `version` - The region version that must match exactly.
    pub fn find_exact(country_id: u8, subregion_id: u8, version: Version) -> Option<Location> {
        match LocationFinder::find(country_id, subregion_id, Some(version)) {
            LocationFinder::None | LocationFinder::Adjusted(_) => None,
            LocationFinder::Exact(v) => Some(v),
        }
    }

    /// Returns a new [`Location`] with the same country and subregion but a different version.
    ///
    /// Returns `None` if no exact location entry exists for this country/subregion
    /// combination under the requested version.
    ///
    /// # Arguments
    ///
    /// * `version` - The new region version to look up for this location's country ID and subregion ID.
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
