use crate::{
    byte_handler::{ByteHandler, FromByteHandler},
    footer::{ctgp_footer::exact_finish_time::ExactFinishTime, sp_footer::sp_version::SPVersion},
    header::in_game_time::{InGameTime, InGameTimeError},
};

pub mod sp_version;

/// Errors that can occur while parsing an [`SPFooter`].
#[derive(thiserror::Error, Debug)]
pub enum SPFooterError {
    /// The ghost file does not contain the expected `SPGD` magic bytes.
    #[error("Ghost is not SPGD")]
    NotSPGD,
    /// The footer version number exceeds the maximum supported value (5).
    #[error("Invalid MKW-SP footer version")]
    InvalidFooterVersion,
    /// An in-game time field could not be parsed.
    #[error("In Game Time Error: {0}")]
    InGameTimeError(#[from] InGameTimeError),
    /// A lap split index was out of range for the recorded lap count.
    #[error("Lap split index not semantically valid")]
    LapSplitIndexError,
    /// A slice-to-array conversion failed while reading footer data.
    #[error("Try From Slice Error: {0}")]
    TryFromSliceError(#[from] std::array::TryFromSliceError),
}

/// Parsed representation of the MKW-SP footer appended to Mario Kart Wii ghost files.
///
/// The footer stores metadata written by MKW Service Pack at the end of each recorded ghost,
/// including high-precision timing, version information, track hash, mushroom strategy,
/// and various gameplay flags. Some fields are only present in later footer versions.
pub struct SPFooter {
    /// The raw bytes of the footer (excluding the trailing CRC32).
    raw_data: Vec<u8>,
    /// The footer format version, which determines available fields and layout.
    footer_version: u32,
    /// One or more MKW-SP release versions consistent with the footer version number.
    /// `None` if the footer version is unrecognised.
    possible_sp_versions: Option<Vec<SPVersion>>,
    /// SHA-1 hash of the track file associated with this ghost.
    track_sha1: [u8; 0x14],
    /// Sub-millisecond-accurate finish time, computed as the sum of all exact lap times.
    exact_finish_time: ExactFinishTime,
    /// Sub-millisecond-accurate lap times, one per recorded lap (up to 11).
    exact_lap_times: [ExactFinishTime; 11],
    /// Whether a speed modifier was active during the run.
    has_speed_mod: bool,
    /// Whether an ultra shortcut was performed during the run.
    has_ultra_shortcut: bool,
    /// Whether a horizontal wall glitch was performed during the run.
    has_horizontal_wall_glitch: bool,
    /// Whether a wallride was performed during the run.
    has_wallride: bool,
    /// Per-lap mushroom usage counts. `None` for footer version 0, which lacks this data.
    shroomstrat: Option<[u8; 11]>,
    /// Whether vanilla mode was enabled during the run. `None` for footer versions below 3.
    is_vanilla_mode_enabled: Option<bool>,
    /// Whether simplified controls were enabled during the run. `None` for footer versions below 4.
    has_simplified_controls: Option<bool>,
    /// Whether the run was set in mirror mode. `None` for footer versions below 5.
    set_in_mirror: Option<bool>,
    /// Length of the footer in bytes, excluding the trailing CRC32.
    len: u32,
    /// Number of laps recorded in the ghost.
    lap_count: u8,
}

impl SPFooter {
    /// Parses an [`SPFooter`] from a complete RKG ghost file byte slice.
    ///
    /// Validates the `SPGD` magic and footer version, then reads all footer fields
    /// including high-precision lap times, track SHA-1, shroomstrat, and gameplay flags.
    /// Fields introduced in later footer versions are stored as `Option`, and will be
    /// `None` when the footer version does not include them.
    ///
    /// # Arguments
    ///
    /// * `data` - The full raw bytes of the RKG ghost file, including the MKW-SP footer.
    ///
    /// # Errors
    ///
    /// Returns an [`SPFooterError`] if:
    /// - The `SPGD` magic bytes are absent ([`SPFooterError::NotSPGD`]).
    /// - The footer version exceeds 5 ([`SPFooterError::InvalidFooterVersion`]).
    /// - Any byte slice conversion, integer parse, or time parse fails.
    pub fn new(data: &[u8]) -> Result<Self, SPFooterError> {
        if data[data.len() - 0x08..data.len() - 0x04] != *b"SPGD" {
            return Err(SPFooterError::NotSPGD);
        }

        let footer_len = (u32::from_be_bytes(
            data[data.len() - 0x0C..data.len() - 0x08]
                .try_into()
                .unwrap(),
        ) + 0x08) as usize;

        let lap_count = data[0x10];
        let laps_data = &data[0x11..0x32];

        let footer_data = &data[data.len() - footer_len - 0x04..data.len() - 0x04];

        let footer_version = u32::from_be_bytes(footer_data[..0x04].try_into().unwrap());

        if footer_version > 5 {
            return Err(SPFooterError::InvalidFooterVersion);
        }

        let possible_sp_versions = SPVersion::from(footer_version);

        let mut current_offset = 0x04;

        let track_sha1 = footer_data[current_offset..current_offset + 0x14]
            .to_owned()
            .try_into()
            .unwrap();
        current_offset += 0x14;

        // Exact lap split calculation
        let mut previous_subtractions = 0i64;
        let mut exact_lap_times = [ExactFinishTime::default(); 11];
        let mut in_game_time_offset = 0x00usize;
        let mut subtraction_ps = 0i64;

        for exact_lap_time in exact_lap_times.iter_mut().take(lap_count as usize) {
            let mut true_time_subtraction = ((f32::from_be_bytes(
                footer_data[current_offset..current_offset + 0x04].try_into()?,
            ) as f64)
                * 1e+9)
                .floor() as i64;

            let lap_time = InGameTime::from_byte_handler(
                &laps_data[in_game_time_offset..=in_game_time_offset + 0x02],
            )?;

            // subtract the sum of the previous laps' difference because the lap differences add up to
            // have its decimal portion be equal to the total time
            true_time_subtraction -= previous_subtractions;

            if true_time_subtraction > 1e+9 as i64 {
                true_time_subtraction -= subtraction_ps;
                subtraction_ps = if subtraction_ps == 0 { 1e+9 as i64 } else { 0 };
            }
            previous_subtractions += true_time_subtraction;
            *exact_lap_time = ExactFinishTime::new(
                lap_time.minutes(),
                lap_time.seconds(),
                (lap_time.milliseconds() as i64 * 1e+9 as i64 + true_time_subtraction) as u64,
            );
            in_game_time_offset += 0x03;
            current_offset += 0x04;
        }

        let exact_finish_time = exact_lap_times[..lap_count as usize].iter().copied().sum();

        current_offset += (11 - lap_count as usize) * 0x04;

        let bools = ByteHandler::from(footer_data[current_offset]);
        let has_speed_mod = bools.read_bool(7);
        let has_ultra_shortcut = bools.read_bool(6);
        let has_horizontal_wall_glitch = bools.read_bool(5);
        let has_wallride = bools.read_bool(4);

        let shroomstrat;

        if footer_version >= 1 {
            let shroom_data: [u8; 3] = footer_data[current_offset..current_offset + 0x03]
                .try_into()
                .unwrap();

            let mut shroom_arr = [0u8; 11];
            let mut shrooms = [0u8; 3];

            let raw = u32::from_be_bytes([0, shroom_data[0], shroom_data[1], shroom_data[2]]);
            shrooms[0] = ((raw >> 15) & 0x1F) as u8;
            shrooms[1] = ((raw >> 10) & 0x1F) as u8;
            shrooms[2] = ((raw >> 5) & 0x1F) as u8;

            for shroom in shrooms.iter() {
                if *shroom != 0 {
                    shroom_arr[*shroom as usize - 1] += 1;
                }
            }
            shroomstrat = Some(shroom_arr);
        } else {
            shroomstrat = None;
        }

        current_offset += 0x02;

        let bools = ByteHandler::from(footer_data[current_offset]);

        let is_vanilla_mode_enabled = if footer_version >= 3 {
            Some(bools.read_bool(4))
        } else {
            None
        };

        let has_simplified_controls = if footer_version >= 4 {
            Some(bools.read_bool(3))
        } else {
            None
        };

        let set_in_mirror = if footer_version >= 5 {
            Some(bools.read_bool(2))
        } else {
            None
        };

        Ok(Self {
            raw_data: footer_data.to_owned(),
            footer_version,
            possible_sp_versions,
            track_sha1,
            exact_finish_time,
            exact_lap_times,
            has_speed_mod,
            has_ultra_shortcut,
            has_horizontal_wall_glitch,
            has_wallride,
            shroomstrat,
            is_vanilla_mode_enabled,
            has_simplified_controls,
            set_in_mirror,
            len: footer_len as u32,
            lap_count,
        })
    }

    /// Returns the raw bytes of the footer, excluding the trailing CRC32.
    pub fn raw_data(&self) -> &[u8] {
        &self.raw_data
    }

    /// Returns the footer format version number.
    pub fn footer_version(&self) -> u32 {
        self.footer_version
    }

    /// Returns the possible MKW-SP release versions consistent with this footer's version number.
    ///
    /// Returns `None` if the footer version did not match any known release.
    pub fn possible_sp_versions(&self) -> Option<&Vec<SPVersion>> {
        self.possible_sp_versions.as_ref()
    }

    /// Returns the SHA-1 hash of the track file associated with this ghost.
    pub fn track_sha1(&self) -> &[u8; 0x14] {
        &self.track_sha1
    }

    /// Returns the sub-millisecond-accurate finish time, computed as the sum of all exact lap times.
    pub fn exact_finish_time(&self) -> ExactFinishTime {
        self.exact_finish_time
    }

    /// Returns the sub-millisecond-accurate lap times for all recorded laps.
    pub fn exact_lap_times(&self) -> &[ExactFinishTime] {
        &self.exact_lap_times[..self.lap_count as usize]
    }

    /// Returns the sub-millisecond-accurate time for a single lap by index.
    ///
    /// # Errors
    ///
    /// Returns [`SPFooterError::LapSplitIndexError`] if `idx` is greater than or equal to
    /// the number of recorded laps.
    pub fn exact_lap_time(&self, idx: usize) -> Result<ExactFinishTime, SPFooterError> {
        if idx >= self.lap_count as usize {
            return Err(SPFooterError::LapSplitIndexError);
        }
        Ok(self.exact_lap_times[idx])
    }

    /// Returns whether a speed modifier was active during the run.
    pub fn has_speed_mod(&self) -> bool {
        self.has_speed_mod
    }

    /// Returns whether an ultra shortcut was performed during the run.
    pub fn has_ultra_shortcut(&self) -> bool {
        self.has_ultra_shortcut
    }

    /// Returns whether a horizontal wall glitch was performed during the run.
    pub fn has_horizontal_wall_glitch(&self) -> bool {
        self.has_horizontal_wall_glitch
    }

    /// Returns whether a wallride was performed during the run.
    pub fn has_wallride(&self) -> bool {
        self.has_wallride
    }

    /// Returns the per-lap mushroom usage counts (shroomstrat) for the recorded laps.
    ///
    /// Returns `None` for footer version 0, which does not include shroomstrat data.
    pub fn shroomstrat(&self) -> Option<&[u8]> {
        self.shroomstrat
            .as_ref()
            .map(|s| &s[..self.lap_count as usize])
    }

    /// Returns a dash-separated string representation of the per-lap mushroom usage counts.
    ///
    /// Returns `None` for footer version 0, which does not include shroomstrat data.
    ///
    /// For example, a three-lap run with one mushroom on lap 2 would return `"0-1-0"`.
    pub fn shroomstrat_string(&self) -> Option<String> {
        if let Some(shroomstrat) = self.shroomstrat() {
            let mut s = String::new();

            for (idx, lap) in shroomstrat.iter().enumerate() {
                s += lap.to_string().as_str();

                if idx + 1 < self.lap_count as usize {
                    s += "-";
                }
            }
            Some(s)
        } else {
            None
        }
    }

    /// Returns whether vanilla mode was enabled during the run.
    ///
    /// Returns `None` for footer versions below 3, which do not include this field.
    pub fn is_vanilla_mode_enabled(&self) -> Option<bool> {
        self.is_vanilla_mode_enabled
    }

    /// Returns whether simplified controls were enabled during the run.
    ///
    /// Returns `None` for footer versions below 4, which do not include this field.
    pub fn has_simplified_controls(&self) -> Option<bool> {
        self.has_simplified_controls
    }

    /// Returns whether the run was set in mirror mode.
    ///
    /// Returns `None` for footer versions below 5, which do not include this field.
    pub fn set_in_mirror(&self) -> Option<bool> {
        self.set_in_mirror
    }

    /// Returns the length of the footer in bytes, excluding the trailing CRC32.
    pub fn len(&self) -> usize {
        self.len as usize
    }

    /// Returns `true` if the footer has zero length.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
