use crate::byte_handler::FromByteHandler;
use crate::footer::ctgp_footer::{
    category::Category, ctgp_version::CTGPVersion, exact_finish_time::ExactFinishTime,
};
use crate::header::in_game_time::InGameTime;
use crate::{byte_handler::ByteHandler, input_data::yaz1_decompress};
use crate::{compute_sha1_hex, datetime_from_timestamp, duration_from_ticks};
use chrono::{TimeDelta, prelude::*};
use std::fmt::Write;

pub mod category;
pub mod ctgp_version;
pub mod exact_finish_time;

/// Errors that can occur while parsing a [`CTGPFooter`].
#[derive(thiserror::Error, Debug)]
pub enum CTGPFooterError {
    /// The ghost file does not contain the expected `CKGD` magic bytes.
    #[error("Ghost is not CKGD")]
    NotCKGD,
    /// Data passed is impossibly too short.
    #[error("Data length is too short")]
    DataLengthTooShort,
    /// Full RKG data was not passed.
    #[error("File is not an RKG file")]
    NotRKGD,
    /// The footer version byte is not one of the supported values (1, 2, 3, 5, 6, 7).
    #[error("Invalid CTGP footer version")]
    InvalidFooterVersion,
    /// A slice-to-array conversion failed while reading footer data.
    #[error("Try From Slice Error: {0}")]
    TryFromSliceError(#[from] std::array::TryFromSliceError),
    /// A lap split index was out of range for the recorded lap count.
    #[error("Lap split index not semantically valid")]
    LapSplitIndexError,
    /// The category bytes could not be mapped to a known [`Category`].
    #[error("Category Error: {0}")]
    CategoryError(#[from] category::CategoryError),
    /// An in-game time field could not be parsed.
    #[error("In Game Time Error: {0}")]
    InGameTimeError(#[from] crate::header::in_game_time::InGameTimeError),
    /// A numeric string could not be parsed as an integer.
    #[error("Parse Int Error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

/// Parsed representation of the CTGP-specific footer appended to Mario Kart Wii ghost files.
///
/// The footer stores metadata written by CTGP-R at the end of each recorded ghost, including
/// high-precision timing, version information, RTC timestamps, pause data, and various
/// integrity/cheat-detection flags.
pub struct CTGPFooter {
    /// The raw bytes of the footer (excluding the trailing CRC32).
    raw_data: Vec<u8>,
    /// The security/signature portion of the footer used for verification.
    security_data: Vec<u8>,
    /// SHA-1 hash of the track file associated with this ghost.
    track_sha1: [u8; 0x14],
    /// SHA-1 hash of the full ghost file.
    ghost_sha1: [u8; 0x14],
    /// The player's unique CTGP player ID.
    player_id: u64,
    /// Sub-millisecond-accurate finish time derived from the in-game time and CTGP's correction factor.
    exact_finish_time: ExactFinishTime,
    /// The CTGP CORE version the ghost was driven on.
    core_version: CTGPVersion,
    /// One or more CTGP release versions consistent with the footer's version bytes.
    /// `None` if the version bytes are unrecognised.
    possible_ctgp_versions: Option<Vec<CTGPVersion>>,
    /// Per-lap flags indicating whether CTGP detected a suspicious split-line intersection.
    /// `None` for footer versions below 2.
    lap_split_suspicious_intersections: Option<[bool; 10]>,
    /// Sub-millisecond-accurate lap times, one per recorded lap.
    exact_lap_times: [ExactFinishTime; 10],
    /// Real-time clock timestamp recorded when the race ended.
    rtc_race_end: NaiveDateTime,
    /// Real-time clock timestamp recorded when the race began.
    rtc_race_begins: NaiveDateTime,
    /// Total wall-clock time the game was paused during the run.
    rtc_time_paused: TimeDelta,
    /// In-game timestamps (relative to race start) at which each pause occurred.
    pause_times: Vec<InGameTime>,
    /// Whether the player had My Stuff enabled during the run.
    my_stuff_enabled: bool,
    /// Whether any My Stuff content was actually used during the run.
    my_stuff_used: bool,
    /// Whether a USB GameCube adapter was enabled during the run.
    usb_gamecube_enabled: bool,
    /// Whether CTGP detected a suspicious split-line intersection on the final lap.
    final_lap_suspicious_intersection: bool,
    /// Per-lap mushroom usage counts (shroomstrat), indexed by lap number.
    shroomstrat: [u8; 10],
    /// Whether the player was launched by a cannon during the run.
    cannoned: bool,
    /// Whether the player went out of bounds during the run.
    went_oob: bool,
    /// Whether CTGP flagged a potential slowdown event during the run.
    potential_slowdown: bool,
    /// Whether CTGP flagged potential rapid-fire input during the run.
    potential_rapidfire: bool,
    /// Whether CTGP's heuristics flagged this ghost as potentially cheated.
    potentially_cheated_ghost: bool,
    /// Whether the Mii data in the ghost file has been replaced.
    has_mii_data_replaced: bool,
    /// Whether the Mii name in the ghost file has been replaced.
    has_name_replaced: bool, // Hi Korben
    /// Whether the player respawned at any point during the run.
    respawns: bool,
    /// The run category as determined by CTGP's metadata.
    category: Category,
    /// The footer format version, which determines the layout and size of the footer.
    footer_version: u8,
    /// Length of the footer in bytes, excluding the trailing CRC32.
    len: usize,
    /// Number of laps recorded in the ghost.
    lap_count: u8,
}

impl CTGPFooter {
    /// Parses a [`CTGPFooter`] from a complete RKG ghost file byte slice.
    ///
    /// Validates the `CKGD` magic and footer version, then reads all footer fields
    /// including timing data, version info, RTC timestamps, pause events, and
    /// cheat-detection flags.
    ///
    /// # Arguments
    ///
    /// * `data` - The full raw bytes of the RKG ghost file, including the CTGP footer.
    ///
    /// # Errors
    ///
    /// Returns a [`CTGPFooterError`] if:
    /// - The `CKGD` magic bytes are absent ([`CTGPFooterError::NotCKGD`]).
    /// - The footer version is not supported ([`CTGPFooterError::InvalidFooterVersion`]).
    /// - Any byte slice conversion, integer parse, category parse, or time parse fails.
    pub fn new(data: &[u8]) -> Result<Self, CTGPFooterError> {
        if data.len() < 0x04 {
            return Err(CTGPFooterError::DataLengthTooShort);
        }

        if data[..0x04] != *b"RKGD" {
            return Err(CTGPFooterError::NotRKGD);
        }

        if data.len() < 0x08 {
            return Err(CTGPFooterError::DataLengthTooShort);
        }

        if data[data.len() - 0x08..data.len() - 0x04] != *b"CKGD" {
            return Err(CTGPFooterError::NotCKGD);
        }

        let footer_version = data[data.len() - 0x0D];

        match footer_version {
            1 | 2 | 3 | 5 | 6 | 7 => {}
            _ => {
                return Err(CTGPFooterError::InvalidFooterVersion);
            }
        }

        let len = if footer_version < 7 { 0xD4 } else { 0xE4 };

        if data.len() < len {
            return Err(CTGPFooterError::DataLengthTooShort);
        }

        let security_data_size = if footer_version < 7 { 0x48 } else { 0x58 };

        let raw_data = Vec::from(&data[data.len() - len..data.len() - 0x04]);

        let header_data = &data[..0x88];
        let input_data = &data[0x88..data.len() - len];
        let metadata = &data[data.len() - len..];
        let mut current_offset = 0usize;

        let security_data = Vec::from(&metadata[..security_data_size]);
        current_offset += security_data_size;

        let track_sha1 = metadata[current_offset..current_offset + 0x14]
            .to_owned()
            .try_into()
            .unwrap();
        current_offset += 0x14;

        let ghost_sha1 = compute_sha1_hex(data);

        let player_id =
            u64::from_be_bytes(metadata[current_offset..current_offset + 0x08].try_into()?);
        current_offset += 0x08;

        let finish_time = InGameTime::from_byte_handler(&header_data[0x04..0x07])?;
        let true_time_ps_subtraction =
            (f32::from_be_bytes(metadata[current_offset..current_offset + 0x04].try_into()?) as f64
                * 1e+9)
                .floor() as i64;
        let true_ps = finish_time.igt_to_millis() as i64 * 1e+9 as i64 + true_time_ps_subtraction;
        let true_sec = (true_ps / 1e+12 as i64) as u8;
        let true_fractional_sec = true_ps % 1e+12 as i64;

        let exact_finish_time = if true_ps > 0 {
            ExactFinishTime::new(true_sec / 60, true_sec % 60, true_fractional_sec as u64)
        } else {
            ExactFinishTime::default()
        };
        current_offset += 0x04;

        let possible_ctgp_versions;
        let core_version;
        let mut lap_split_suspicious_intersections = Some([false; 10]);

        if footer_version >= 2 {
            let version_bytes = &metadata[current_offset..current_offset + 0x04];
            core_version = CTGPVersion::core_from(version_bytes)?;
            possible_ctgp_versions = CTGPVersion::from(version_bytes);
            current_offset += 0x04;

            let laps_handler = ByteHandler::try_from(&metadata[current_offset..current_offset + 2])
                .expect("ByteHandler try_from() failed");

            if let Some(mut array) = lap_split_suspicious_intersections {
                for (index, intersection) in array.iter_mut().enumerate() {
                    *intersection = laps_handler.read_bool(index as u8 + 6);
                }
            }
            current_offset -= 0x04;
        } else {
            // Infer that the core version is 1.03.0134, since the next batch of updates after TT release was CORE 1.03.0136
            core_version = CTGPVersion::new(1, 3, 134, None);
            // Metadata version 2 was introduced in between the 1.03.1044 and 1046 update, so it must be 1.03.1044
            possible_ctgp_versions = Some(Vec::from([CTGPVersion::new(1, 3, 1044, None)]));
            lap_split_suspicious_intersections = None;
        }

        current_offset += 0x3C;

        // Exact lap split calculation
        let mut previous_subtractions = 0i64;
        let mut exact_lap_times = [ExactFinishTime::default(); 10];
        let lap_count = header_data[0x10];
        let mut in_game_time_offset = 0x11usize;
        let mut subtraction_ps = 0i64;

        for exact_lap_time in exact_lap_times.iter_mut().take(lap_count as usize) {
            let mut true_time_ps_subtraction =
                ((f32::from_be_bytes(metadata[current_offset..current_offset + 0x04].try_into()?)
                    as f64)
                    * 1e+9)
                    .floor() as i64;

            let lap_time = InGameTime::from_byte_handler(
                &header_data[in_game_time_offset..in_game_time_offset + 0x03],
            )?;

            // subtract the sum of the previous laps' difference because the lap differences add up to
            // have its decimal portion be equal to the total time
            true_time_ps_subtraction -= previous_subtractions;

            if true_time_ps_subtraction > 1e+9 as i64 {
                true_time_ps_subtraction -= subtraction_ps;
                subtraction_ps = if subtraction_ps == 0 { 1e+9 as i64 } else { 0 };
            }
            previous_subtractions += true_time_ps_subtraction;

            let true_ps = lap_time.igt_to_millis() as i64 * 1e+9 as i64 + true_time_ps_subtraction;
            let true_sec = (true_ps / 1e+12 as i64) as u8;
            let true_fractional_sec = true_ps % 1e+12 as i64;

            *exact_lap_time = if true_ps > 0 {
                ExactFinishTime::new(true_sec / 60, true_sec % 60, true_fractional_sec as u64)
            } else {
                ExactFinishTime::default()
            };
            in_game_time_offset += 0x03;
            current_offset -= 0x04;
        }

        current_offset += 0x04 * (lap_count as usize + 1);

        let rtc_race_end = datetime_from_timestamp(u64::from_be_bytes(
            metadata[current_offset..current_offset + 0x08].try_into()?,
        ));
        current_offset += 0x08;

        let rtc_race_begins = datetime_from_timestamp(u64::from_be_bytes(
            metadata[current_offset..current_offset + 0x08].try_into()?,
        ));
        current_offset += 0x08;

        let rtc_time_paused = duration_from_ticks(u64::from_be_bytes(
            metadata[current_offset..current_offset + 0x08].try_into()?,
        ));
        current_offset += 0x08;

        // Pause frame times
        let mut pause_times = Vec::new();
        let input_data = if input_data[4..8] == [0x59, 0x61, 0x7A, 0x31] {
            // YAZ1 header, decompress
            yaz1_decompress(&input_data[4..]).unwrap()
        } else {
            Vec::from(input_data)
        };

        let face_input_count = u16::from_be_bytes([input_data[0], input_data[1]]);

        let mut current_input_byte = 8;
        let mut elapsed_frames = 1u32;
        while current_input_byte < 8 + face_input_count * 2 {
            let idx = current_input_byte as usize;
            let input = &input_data[idx..idx + 2];

            if contains_ctgp_pause(input[0]) {
                // Convert frame count to InGameTime
                // Subtract 240 frames for countdown, another 2 frames because CTGP logs the pause 2 frames after it actually happens
                let mut pause_timestamp_seconds = (elapsed_frames - 242) as f64 / 59.94;
                let mut minutes = 0;
                let mut seconds = 0;

                while pause_timestamp_seconds >= 60.0 {
                    minutes += 1;
                    pause_timestamp_seconds -= 60.0;
                }

                while pause_timestamp_seconds >= 1.0 {
                    seconds += 1;
                    pause_timestamp_seconds -= 1.0;
                }

                let milliseconds = (pause_timestamp_seconds * 1000.0) as u16;

                pause_times.push(InGameTime::new(minutes, seconds, milliseconds));
            }

            elapsed_frames += input[1] as u32;
            current_input_byte += 2;
        }

        let bool_handler = ByteHandler::from(metadata[current_offset]);
        let my_stuff_enabled = bool_handler.read_bool(3);
        let my_stuff_used = bool_handler.read_bool(2);
        let usb_gamecube_enabled = bool_handler.read_bool(1);
        let final_lap_suspicious_intersection = bool_handler.read_bool(0);
        current_offset += 0x01;

        let mut shroomstrat: [u8; 10] = [0; 10];
        for _ in 0..3 {
            let lap = metadata[current_offset];
            if lap != 0 {
                shroomstrat[(lap - 1) as usize] += 1;
            }
            current_offset += 0x01;
        }

        let category = Category::try_from(metadata[current_offset + 2], metadata[current_offset])?;
        current_offset += 0x01;
        let bool_handler = ByteHandler::from(metadata[current_offset]);
        let cannoned = bool_handler.read_bool(7);
        let went_oob = bool_handler.read_bool(6);
        let potential_slowdown = bool_handler.read_bool(5);
        let potential_rapidfire = bool_handler.read_bool(4);
        let potentially_cheated_ghost = bool_handler.read_bool(3);
        let has_mii_data_replaced = bool_handler.read_bool(2);
        let has_name_replaced = bool_handler.read_bool(1);
        let respawns = bool_handler.read_bool(0);

        Ok(Self {
            raw_data,
            security_data,
            track_sha1,
            ghost_sha1,
            player_id,
            exact_finish_time,
            core_version,
            possible_ctgp_versions,
            lap_split_suspicious_intersections,
            exact_lap_times,
            rtc_race_end,
            rtc_race_begins,
            rtc_time_paused,
            pause_times,
            my_stuff_enabled,
            my_stuff_used,
            usb_gamecube_enabled,
            final_lap_suspicious_intersection,
            shroomstrat,
            cannoned,
            went_oob,
            potential_slowdown,
            potential_rapidfire,
            potentially_cheated_ghost,
            has_mii_data_replaced,
            has_name_replaced,
            respawns,
            category,
            footer_version,
            len: len - 0x04,
            lap_count,
        })
    }

    /// Returns the raw bytes of the footer, excluding the trailing CRC32.
    pub fn raw_data(&self) -> &[u8] {
        &self.raw_data
    }

    /// Returns the security/signature portion of the footer.
    pub fn security_data(&self) -> &[u8] {
        &self.security_data
    }

    /// Returns the SHA-1 hash of the track file associated with this ghost.
    pub fn track_sha1(&self) -> &[u8] {
        &self.track_sha1
    }

    /// Returns the SHA-1 hash of the full ghost file.
    pub fn ghost_sha1(&self) -> &[u8] {
        &self.ghost_sha1
    }

    /// Overwrites the stored ghost SHA-1 hash.
    ///
    /// # Errors
    ///
    /// Returns [`CTGPFooterError::TryFromSliceError`] if `ghost_sha1` is not exactly 20 bytes.
    pub(crate) fn set_ghost_sha1(&mut self, ghost_sha1: &[u8]) -> Result<(), CTGPFooterError> {
        self.ghost_sha1 = ghost_sha1.try_into()?;
        Ok(())
    }

    /// Returns the player's unique CTGP player ID.
    pub fn player_id(&self) -> u64 {
        self.player_id
    }

    /// Returns the sub-millisecond-accurate finish time for the run.
    pub fn exact_finish_time(&self) -> ExactFinishTime {
        self.exact_finish_time
    }

    /// Returns the CORE (base game) version the ghost was driven on.
    pub fn core_version(&self) -> CTGPVersion {
        self.core_version
    }

    /// Returns the possible CTGP release versions consistent with this ghost's footer bytes.
    ///
    /// Returns `None` if the version bytes did not match any known release.
    pub fn possible_ctgp_versions(&self) -> Option<&Vec<CTGPVersion>> {
        self.possible_ctgp_versions.as_ref()
    }

    /// Returns per-lap suspicious split-line intersection flags for the recorded laps.
    ///
    /// Returns `None` for ghosts recorded on footer version 1, which does not include this data.
    pub fn lap_split_suspicious_intersections(&self) -> Option<&[bool]> {
        if let Some(intersections) = &self.lap_split_suspicious_intersections {
            return Some(&intersections[0..self.lap_count as usize]);
        }
        None
    }

    /// Returns the sub-millisecond-accurate lap times for all recorded laps.
    pub fn exact_lap_times(&self) -> &[ExactFinishTime] {
        &self.exact_lap_times[0..self.lap_count as usize]
    }

    /// Returns the sub-millisecond-accurate time for a single lap by index.
    ///
    /// # Errors
    ///
    /// Returns [`CTGPFooterError::LapSplitIndexError`] if `idx` is greater than or equal to
    /// the number of recorded laps.
    pub fn exact_lap_time(&self, idx: usize) -> Result<ExactFinishTime, CTGPFooterError> {
        if idx >= self.lap_count as usize {
            return Err(CTGPFooterError::LapSplitIndexError);
        }
        Ok(self.exact_lap_times[idx])
    }

    /// Returns the real-time clock timestamp recorded when the race ended.
    pub fn rtc_race_end(&self) -> NaiveDateTime {
        self.rtc_race_end
    }

    /// Returns the real-time clock timestamp recorded when the race began.
    pub fn rtc_race_begins(&self) -> NaiveDateTime {
        self.rtc_race_begins
    }

    /// Returns the total wall-clock duration the game was paused during the run.
    pub fn rtc_time_paused(&self) -> TimeDelta {
        self.rtc_time_paused
    }

    /// Returns the in-game timestamps at which each pause occurred during the run.
    pub fn pause_times(&self) -> &Vec<InGameTime> {
        &self.pause_times
    }

    /// Returns whether the player had My Stuff enabled during the run.
    pub fn my_stuff_enabled(&self) -> bool {
        self.my_stuff_enabled
    }

    /// Returns whether any My Stuff content was actually used during the run.
    pub fn my_stuff_used(&self) -> bool {
        self.my_stuff_used
    }

    /// Returns whether a USB GameCube adapter was enabled during the run.
    pub fn usb_gamecube_enabled(&self) -> bool {
        self.usb_gamecube_enabled
    }

    /// Returns whether CTGP detected a suspicious split-line intersection on the final lap.
    pub fn final_lap_suspicious_intersection(&self) -> bool {
        self.final_lap_suspicious_intersection
    }

    /// Returns the per-lap mushroom usage counts (shroomstrat) for the recorded laps.
    pub fn shroomstrat(&self) -> &[u8] {
        &self.shroomstrat[0..self.lap_count as usize]
    }

    /// Returns a dash-separated string representation of the per-lap mushroom usage counts.
    ///
    /// For example, a three-lap run with one mushroom on lap 1 and two on lap 3
    /// would return `"1-0-2"`.
    pub fn shroomstrat_string(&self) -> String {
        let mut shroomstrat = self.shroomstrat().iter();

        let mut s = shroomstrat.next().unwrap().to_string();
        for lap_shrooms in shroomstrat {
            write!(s, "-{lap_shrooms}").unwrap()
        }

        s
    }

    /// Returns whether the player was launched by a cannon during the run.
    pub fn cannoned(&self) -> bool {
        self.cannoned
    }

    /// Returns whether the player went out of bounds during the run.
    pub fn went_oob(&self) -> bool {
        self.went_oob
    }

    /// Returns whether CTGP flagged a potential slowdown event during the run.
    pub fn potential_slowdown(&self) -> bool {
        self.potential_slowdown
    }

    /// Returns whether CTGP flagged potential rapid-fire input during the run.
    pub fn potential_rapidfire(&self) -> bool {
        self.potential_rapidfire
    }

    /// Returns whether CTGP's heuristics flagged this ghost as potentially cheated.
    pub fn potentially_cheated_ghost(&self) -> bool {
        self.potentially_cheated_ghost
    }

    /// Returns whether the Mii data in the ghost file has been replaced.
    pub fn has_mii_data_replaced(&self) -> bool {
        self.has_mii_data_replaced
    }

    /// Returns whether the Mii name in the ghost file has been replaced.
    pub fn has_name_replaced(&self) -> bool {
        self.has_name_replaced
    }

    /// Returns whether the player respawned at any point during the run.
    pub fn respawns(&self) -> bool {
        self.respawns
    }

    /// Returns the run category as determined by CTGP's metadata.
    pub fn category(&self) -> Category {
        self.category
    }

    /// Returns the footer format version number.
    pub fn footer_version(&self) -> u8 {
        self.footer_version
    }

    /// Returns the length of the CTGP footer in bytes, excluding the trailing CRC32.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the footer has zero length.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// Returns `true` if the given face-button byte indicates a CTGP pause event.
///
/// CTGP encodes pauses in bit 6 (`0x40`) of the face-button input byte.
fn contains_ctgp_pause(buttons: u8) -> bool {
    buttons & 0x40 != 0
}
