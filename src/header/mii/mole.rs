use std::convert::Infallible;

use crate::byte_handler::{ByteHandlerError, FromByteHandler};

#[derive(Clone, Copy)]
pub struct Mole {
    has_mole: bool,
    x: u8,
    y: u8,
    size: u8,
}
impl Mole {
    pub fn new(has_mole: bool, x: u8, y: u8, size: u8) -> Result<Self, MoleError> {
        if x > 16 {
            return Err(MoleError::XInvalid);
        }
        if y > 30 {
            return Err(MoleError::YInvalid);
        }
        if size > 8 {
            return Err(MoleError::SizeInvalid);
        }
        Ok(Self {
            has_mole,
            x,
            y,
            size,
        })
    }

    pub fn has_mole(&self) -> bool {
        self.has_mole
    }
    pub fn x(&self) -> u8 {
        self.x
    }
    pub fn y(&self) -> u8 {
        self.y
    }
    pub fn size(&self) -> u8 {
        self.size
    }

    pub fn set_has_mole(&mut self, has_mole: bool) {
        self.has_mole = has_mole;
    }

    pub fn set_x(&mut self, x: u8) -> Result<(), MoleError> {
        if x > 16 {
            return Err(MoleError::XInvalid);
        }
        self.x = x;
        Ok(())
    }

    pub fn set_y(&mut self, y: u8) -> Result<(), MoleError> {
        if y > 30 {
            return Err(MoleError::YInvalid);
        }
        self.y = y;
        Ok(())
    }

    pub fn set_size(&mut self, size: u8) -> Result<(), MoleError> {
        if size > 8 {
            return Err(MoleError::SizeInvalid);
        }
        self.size = size;
        Ok(())
    }
}
impl FromByteHandler for Mole {
    type Err = MoleError;
    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<crate::byte_handler::ByteHandler>,
        Self::Err: From<T::Error>,
    {
        let mut handler = handler.try_into()?;
        let has_mole = handler.read_bool(15);
        handler.shift_right(1);
        let x = handler.copy_byte(1) & 0x1F;
        handler.shift_right(2);
        let y = handler.copy_byte(1) >> 3;
        let size = handler.copy_byte(0) & 0x0F;
        Self::new(has_mole, x, y, size)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MoleError {
    #[error("Size is invalid")]
    SizeInvalid,
    #[error("Y position is invalid")]
    YInvalid,
    #[error("X position is invalid")]
    XInvalid,
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    #[error("")]
    Infallible(#[from] Infallible),
}
