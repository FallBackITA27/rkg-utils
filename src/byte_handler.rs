
#[derive(thiserror::Error, Debug)]
pub enum ByteHandlerError {
    #[error("Couldn't convert type to ByteHandler: Too Long")]
    ConversionErrorTooLong,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) union ByteHandler {
    dword: u32,
    words: [u16; 2],
    bytes: [u8; 4],
}

impl ByteHandler {
    pub fn copy_dword(self) -> u32 {
        unsafe { self.dword }
    }

    pub fn copy_words(self) -> [u16; 2] {
        unsafe {
            self.words
        }
    }

    pub fn copy_word(self, idx: usize) -> u16 {
        if idx > 1 {
            return 0;
        }
        unsafe {
            self.words[idx]
        }
    }

    pub fn copy_bytes(self) -> [u8; 4] {
        unsafe { self.bytes }
    }

    pub fn copy_byte(self, idx: usize) -> u8 {
        if idx > 3 {
            return 0;
        }
        unsafe { self.bytes[idx] }
    }

    pub fn shift_right(&mut self, d: u8) {
        unsafe {
            self.dword >>= d;
        }
    }

    pub fn shift_left(&mut self, d: u8) {
        unsafe {
            self.dword <<= d;
        }
    }

    /// Reads the nth bit from the right counting from 0
    pub fn read_bool(&self, d: u8) -> bool {
        if d >= 32 {
            return false;
        }
        (self.copy_dword() & (1u32 << d)) != 0
    }
}

impl From<[u8; 4]> for ByteHandler {
    fn from(value: [u8; 4]) -> Self {
        ByteHandler { bytes: value }
    }
}

impl From<[u8; 3]> for ByteHandler {
    fn from(value: [u8; 3]) -> Self {
        ByteHandler {
            bytes: [value[0], value[1], value[2], 0],
        }
    }
}

impl From<[u8; 2]> for ByteHandler {
    fn from(value: [u8; 2]) -> Self {
        ByteHandler {
            bytes: [value[0], value[1], 0, 0],
        }
    }
}

impl From<u8> for ByteHandler {
    fn from(value: u8) -> Self {
        ByteHandler {
            bytes: [value, 0, 0, 0],
        }
    }
}

impl From<u32> for ByteHandler {
    fn from(value: u32) -> Self {
        ByteHandler { dword: value }
    }
}

impl From<[u16; 2]> for ByteHandler {
    fn from(value: [u16; 2]) -> Self {
        ByteHandler { words: value }
    }
}

impl From<u16> for ByteHandler {
    fn from(value: u16) -> Self {
        ByteHandler::from([value, 0])
    }
}

impl TryFrom<&[u8]> for ByteHandler {
    type Error = ByteHandlerError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value.len() {
            0 => Ok(From::from(0u32)),
            1 => Ok(From::from(value[0])),
            2 => Ok(From::from(unsafe {
                TryInto::<[u8; 2]>::try_into(value).unwrap_unchecked()
            })),
            3 => Ok(From::from(unsafe {
                TryInto::<[u8; 3]>::try_into(value).unwrap_unchecked()
            })),
            4 => Ok(From::from(unsafe {
                TryInto::<[u8; 4]>::try_into(value).unwrap_unchecked()
            })),
            _ => Err(ByteHandlerError::ConversionErrorTooLong),
        }
    }
}

impl TryFrom<&[u16]> for ByteHandler {
    type Error = ByteHandlerError;

    fn try_from(value: &[u16]) -> Result<Self, Self::Error> {
        match value.len() {
            0 => Ok(From::from(0u32)),
            1 => Ok(From::from(value[0])),
            2 => Ok(From::from(unsafe {
                TryInto::<[u16; 2]>::try_into(value).unwrap_unchecked()
            })),
            _ => Err(ByteHandlerError::ConversionErrorTooLong),
        }
    }
}

pub(crate) trait FromByteHandler: Sized {
    type Err;
    fn from_byte_handler<T: TryInto<ByteHandler>>(handler: T) -> Result<Self, Self::Err>;
}
