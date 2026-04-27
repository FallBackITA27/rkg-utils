pub struct DWCUserData {
    pseudo_user_id: u64,
    pseudo_player_id: u32,
    authentic_user_id: u64,
    authentic_player_id: u32,
    friend_code: u32,
    crc32: u32,
}

impl From<[u8; 0x40]> for DWCUserData {
    fn from(value: [u8; 0x40]) -> Self {
        Self {
            pseudo_user_id: u64::from_be_bytes(unsafe {
                value[0x04..0x0C].try_into().unwrap_unchecked()
            }),
            pseudo_player_id: u32::from_be_bytes(unsafe {
                value[0x0C..0x10].try_into().unwrap_unchecked()
            }),
            authentic_user_id: u64::from_be_bytes(unsafe {
                value[0x10..0x18].try_into().unwrap_unchecked()
            }),
            authentic_player_id: u32::from_be_bytes(unsafe {
                value[0x18..0x1C].try_into().unwrap_unchecked()
            }),
            friend_code: u32::from_be_bytes(unsafe {
                value[0x1C..0x20].try_into().unwrap_unchecked()
            }),
            crc32: u32::from_be_bytes(unsafe { value[0x3C..0x40].try_into().unwrap_unchecked() }),
        }
    }
}

impl DWCUserData {
    pub const fn pseudo_user_id(&self) -> u64 {
        self.pseudo_user_id
    }
    pub const fn pseudo_player_id(&self) -> u32 {
        self.pseudo_player_id
    }
    pub const fn authentic_user_id(&self) -> u64 {
        self.authentic_user_id
    }
    pub const fn authentic_player_id(&self) -> u32 {
        self.authentic_player_id
    }
    pub const fn friend_code(&self) -> u32 {
        self.friend_code
    }
    pub const fn crc32(&self) -> u32 {
        self.crc32
    }
}
