use crate::rksys::{
    ghost_slots::{GhostSlots, GhostSlotsError},
    license::{License, LicenseError},
};

pub mod ghost_slots;
pub mod license;

/// Errors that can occur while constructing a [`SaveData`].
#[derive(thiserror::Error, Debug)]
pub enum SaveDataError {
    #[error("File Invalid Type")]
    FileInvalid,
    #[error("License {0} Error: {1}")]
    LicenseError(u8, LicenseError),
    #[error("GhostSlots Error: {0}")]
    GhostSlotsError(#[from] GhostSlotsError),
}

#[derive(Clone, Copy)]
pub enum LicenseSlot {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl From<LicenseSlot> for u8 {
    fn from(value: LicenseSlot) -> Self {
        match value {
            LicenseSlot::TopLeft => 0,
            LicenseSlot::TopRight => 1,
            LicenseSlot::BottomLeft => 2,
            LicenseSlot::BottomRight => 3,
        }
    }
}

pub struct SaveData {
    save_version: [u8; 4],
    licenses: [License; 4],
    licenses_ghost_slots: [GhostSlots; 4],
}

impl SaveData {
    pub fn save_version(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.save_version) }
    }

    pub fn license_ghost_slots(&self, slot: LicenseSlot) -> &GhostSlots {
        let idx = u8::from(slot);
        let idx = usize::from(idx);
        &self.licenses_ghost_slots[idx]
    }

    pub fn license(&self, slot: LicenseSlot) -> &License {
        let idx = u8::from(slot);
        let idx = usize::from(idx);
        &self.licenses[idx]
    }
}

impl TryFrom<[u8; 0x2BC000]> for SaveData {
    type Error = SaveDataError;
    fn try_from(value: [u8; 0x2BC000]) -> Result<Self, Self::Error> {
        if &value[0..4] != b"RKSD" {
            return Err(SaveDataError::FileInvalid);
        }

        const fn idx(idx: usize) -> std::ops::Range<usize> {
            (8 + 0x88C0 * idx)..(8 + 0x88C0 * (idx + 1))
        }

        Ok(Self {
            save_version: unsafe { value[4..8].try_into().unwrap_unchecked() },
            licenses: [
                License::try_from(unsafe {
                    TryInto::<[u8; 0x88C0]>::try_into(&value[idx(0)]).unwrap_unchecked()
                })
                .map_err(|e| SaveDataError::LicenseError(0, e))?,
                License::try_from(unsafe {
                    TryInto::<[u8; 0x88C0]>::try_into(&value[idx(1)]).unwrap_unchecked()
                })
                .map_err(|e| SaveDataError::LicenseError(1, e))?,
                License::try_from(unsafe {
                    TryInto::<[u8; 0x88C0]>::try_into(&value[idx(2)]).unwrap_unchecked()
                })
                .map_err(|e| SaveDataError::LicenseError(2, e))?,
                License::try_from(unsafe {
                    TryInto::<[u8; 0x88C0]>::try_into(&value[idx(3)]).unwrap_unchecked()
                })
                .map_err(|e| SaveDataError::LicenseError(3, e))?,
            ],
            licenses_ghost_slots: [
                GhostSlots::try_from(unsafe {
                    TryInto::<[u8; 0xA5000]>::try_into(&value[0x28000..0xCD000]).unwrap_unchecked()
                })?,
                GhostSlots::try_from(unsafe {
                    TryInto::<[u8; 0xA5000]>::try_into(&value[0xCD000..0x172000]).unwrap_unchecked()
                })?,
                GhostSlots::try_from(unsafe {
                    TryInto::<[u8; 0xA5000]>::try_into(&value[0x172000..0x217000])
                        .unwrap_unchecked()
                })?,
                GhostSlots::try_from(unsafe {
                    TryInto::<[u8; 0xA5000]>::try_into(&value[0x217000..0x2BC000])
                        .unwrap_unchecked()
                })?,
            ],
        })
    }
}
