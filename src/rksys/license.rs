/// Errors that can occur while constructing a [`License`].
#[derive(thiserror::Error, Debug)]
pub enum LicenseError {
    #[error("File Invalid Type")]
    FileInvalid,
}

pub struct License; /* {
mii_name: String,
mii_id: u32,
mii_client: u32,
personal_best_ghost_flags: [bool; 32],
downloaded_ghost_flags: [bool; 32],
normal_staff_ghost_flags: [bool; 32],
expert_staff_ghost_flags: [bool; 32],
} */

impl TryFrom<[u8; 0x88C0]> for License {
    type Error = LicenseError;
    fn try_from(value: [u8; 0x88C0]) -> Result<Self, Self::Error> {
        if &value[0..4] != b"RKPD" {
            return Err(LicenseError::FileInvalid);
        }

        Ok(License)
    }
}
