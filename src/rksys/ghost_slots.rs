use crate::{Ghost, GhostError, header::HeaderError};

/// Errors that can occur while constructing a [`GhostSlots`].
#[derive(thiserror::Error, Debug)]
pub enum GhostSlotsError {
    #[error("Ghost Error: {0}")]
    GhostError(#[from] GhostError),
}

const GHOST_SLOTS: usize = 66;

/// Internally
/// 0..32 are the PB Ghosts
/// 32..64 are the downloaded Ghosts
/// 64 is the ghost race Ghost
/// 65 is the competition Ghost
pub struct GhostSlots([Option<Ghost>; GHOST_SLOTS]);

impl TryFrom<[u8; 0xA5000]> for GhostSlots {
    type Error = GhostSlotsError;
    fn try_from(value: [u8; 0xA5000]) -> Result<Self, Self::Error> {
        let mut ghosts = [const { None }; GHOST_SLOTS];
        for i in 0..GHOST_SLOTS {
            let data = match Ghost::new(&value[(i * 0x2800)..((i + 1) * 0x2800)]) {
                Ok(v) => v,
                Err(GhostError::HeaderError(HeaderError::NotRKGD)) => continue,
                Err(e) => return Err(GhostSlotsError::GhostError(e)),
            };

            ghosts[i] = Some(data);
        }

        Ok(Self(ghosts))
    }
}

impl GhostSlots {
    pub const fn get_nth_pb_ghost(&self, idx: u8) -> Option<&Ghost> {
        let idx = idx as usize;
        if idx > 32 {
            return None;
        }

        self.0[idx].as_ref()
    }

    pub const fn get_nth_downloaded_ghost(&self, idx: u8) -> Option<&Ghost> {
        let idx = idx as usize;
        if idx > 32 {
            return None;
        }

        self.0[idx + 32].as_ref()
    }

    pub const fn get_ghost_race_ghost(&self) -> Option<&Ghost> {
        self.0[64].as_ref()
    }

    pub const fn get_competition_ghost(&self) -> Option<&Ghost> {
        self.0[65].as_ref()
    }

    pub const fn get_nth_ghost_slot(&self, idx: u8) -> Option<&Ghost> {
        let idx = idx as usize;
        if idx > GHOST_SLOTS {
            return None;
        }

        self.0[idx].as_ref()
    }

    pub const fn len() -> usize {
        GHOST_SLOTS
    }
}
