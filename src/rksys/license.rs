use std::string::FromUtf16Error;

/// Errors that can occur while constructing a [`License`].
#[derive(thiserror::Error, Debug)]
pub enum LicenseError {
    #[error("File Invalid Type")]
    FileInvalid,
    #[error("Invalid UTF16 String: {0}")]
    FromUtf16Error(#[from] FromUtf16Error),
}

pub struct License {
    mii_name: String,
    mii_id: u32,
    mii_client: u32,

    /* 
    personal_best_ghost_flags: [bool; 32],
    downloaded_ghost_flags: [bool; 32],
    normal_staff_ghost_flags: [bool; 32],
    expert_staff_ghost_flags: [bool; 32],
    */
}

impl TryFrom<[u8; 0x88C0]> for License {
    type Error = LicenseError;
    fn try_from(value: [u8; 0x88C0]) -> Result<Self, Self::Error> {
        if &value[0..4] != b"RKPD" {
            return Err(LicenseError::FileInvalid);
        }

        Ok(License {
            mii_name: String::from_utf16(unsafe { std::mem::transmute::<&[u8], &[u16]>(&value[0x14..0x28]) })?,
            mii_id: u32::from_be_bytes(unsafe { value[0x28..0x2C].try_into().unwrap_unchecked() }),
            mii_client: u32::from_be_bytes(unsafe { value[0x2C..0x30].try_into().unwrap_unchecked() })
        })
    }
}

impl License {
    pub const fn mii_name(&self) -> &str {
        self.mii_name.as_str()
    }

    pub const fn mii_id(&self) -> u32 {
        self.mii_id
    }

    pub const fn mii_client(&self) -> u32 {
        self.mii_client
    }
}
