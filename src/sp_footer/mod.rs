use crate::{
    byte_handler::{ByteHandler, FromByteHandler},
    ctgp_footer::exact_finish_time::ExactFinishTime,
    header::in_game_time::{InGameTime, InGameTimeError},
    sp_footer::sp_version::SPVersion,
};

pub mod sp_version;

#[derive(thiserror::Error, Debug)]
pub enum SPFooterError {
    #[error("Ghost is not SPGD")]
    NotSPGD,
    #[error("Invalid MKW-SP footer version")]
    InvalidFooterVersion,
    #[error("In Game Time Error: {0}")]
    InGameTimeError(#[from] InGameTimeError),
    #[error("Lap split index not semantically valid")]
    LapSplitIndexError,
    #[error("Try From Slice Error: {0}")]
    TryFromSliceError(#[from] std::array::TryFromSliceError),
}

pub struct SPFooter {
    raw_data: Vec<u8>,
    footer_version: u32,
    possible_sp_versions: Option<Vec<SPVersion>>,
    track_sha1: [u8; 0x14],
    exact_finish_time: ExactFinishTime,
    exact_lap_times: [ExactFinishTime; 11],
    has_speed_mod: bool,
    has_ultra_shortcut: bool,
    has_horizontal_wall_glitch: bool,
    has_wallride: bool,
    shroomstrat: Option<[u8; 11]>,
    is_vanilla_mode_enabled: Option<bool>,
    has_simplified_controls: Option<bool>,
    set_in_mirror: Option<bool>,
    len: u32,
    lap_count: u8,
}

impl SPFooter {
    /// Expects full RKG data
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

        // 0xXXXvsmXX
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

    pub fn raw_data(&self) -> &[u8] {
        &self.raw_data
    }

    pub fn footer_version(&self) -> u32 {
        self.footer_version
    }

    pub fn possible_sp_versions(&self) -> Option<&Vec<SPVersion>> {
        self.possible_sp_versions.as_ref()
    }

    pub fn track_sha1(&self) -> &[u8; 0x14] {
        &self.track_sha1
    }

    pub fn exact_finish_time(&self) -> ExactFinishTime {
        self.exact_finish_time
    }

    pub fn exact_lap_times(&self) -> &[ExactFinishTime] {
        &self.exact_lap_times[..self.lap_count as usize]
    }

    pub fn exact_lap_time(&self, idx: usize) -> Result<ExactFinishTime, SPFooterError> {
        if idx >= self.lap_count as usize {
            return Err(SPFooterError::LapSplitIndexError);
        }
        Ok(self.exact_lap_times[idx])
    }

    pub fn has_speed_mod(&self) -> bool {
        self.has_speed_mod
    }

    pub fn has_ultra_shortcut(&self) -> bool {
        self.has_ultra_shortcut
    }

    pub fn has_horizontal_wall_glitch(&self) -> bool {
        self.has_horizontal_wall_glitch
    }

    pub fn has_wallride(&self) -> bool {
        self.has_wallride
    }

    pub fn shroomstrat(&self) -> Option<&[u8]> {
        self.shroomstrat
            .as_ref()
            .map(|s| &s[..self.lap_count as usize])
    }

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

    pub fn is_vanilla_mode_enabled(&self) -> Option<bool> {
        self.is_vanilla_mode_enabled
    }

    pub fn has_simplified_controls(&self) -> Option<bool> {
        self.has_simplified_controls
    }

    pub fn set_in_mirror(&self) -> Option<bool> {
        self.set_in_mirror
    }

    /// Returns length of footer excluding file CRC32
    pub fn len(&self) -> usize {
        self.len as usize
    }

    /// Returns `true` if the footer is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
