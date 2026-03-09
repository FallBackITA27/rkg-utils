use std::convert::Infallible;

use crate::byte_handler::{ByteHandlerError, FromByteHandler};

/// Represents the mole customization options of a Mii,
/// including whether a mole is shown and its position and size.
#[derive(Clone, Copy)]
pub struct Mole {
    /// Whether the mole is visible on the Mii's face.
    has_mole: bool,
    /// Horizontal position of the mole (0–16).
    x: u8,
    /// Vertical position of the mole (0–30).
    y: u8,
    /// Mole size (0–8).
    size: u8,
}

impl Mole {
    /// Creates a new [`Mole`] from its individual components.
    ///
    /// # Arguments
    ///
    /// * `has_mole` - Whether the mole is visible.
    /// * `x` - Horizontal position (0–16).
    /// * `y` - Vertical position (0–30).
    /// * `size` - Mole size (0–8).
    ///
    /// # Errors
    ///
    /// Returns [`MoleError::XInvalid`] if `x` exceeds 16.
    /// Returns [`MoleError::YInvalid`] if `y` exceeds 30.
    /// Returns [`MoleError::SizeInvalid`] if `size` exceeds 8.
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

    /// Returns whether the mole is visible on the Mii's face.
    pub fn has_mole(&self) -> bool {
        self.has_mole
    }

    /// Returns the horizontal position of the mole (0–16).
    pub fn x(&self) -> u8 {
        self.x
    }

    /// Returns the vertical position of the mole (0–30).
    pub fn y(&self) -> u8 {
        self.y
    }

    /// Returns the mole size (0–8).
    pub fn size(&self) -> u8 {
        self.size
    }

    /// Sets whether the mole is visible on the Mii's face.
    pub fn set_has_mole(&mut self, has_mole: bool) {
        self.has_mole = has_mole;
    }

    /// Sets the horizontal position of the mole.
    ///
    /// # Errors
    ///
    /// Returns [`MoleError::XInvalid`] if `x` exceeds 16.
    pub fn set_x(&mut self, x: u8) -> Result<(), MoleError> {
        if x > 16 {
            return Err(MoleError::XInvalid);
        }
        self.x = x;
        Ok(())
    }

    /// Sets the vertical position of the mole.
    ///
    /// # Errors
    ///
    /// Returns [`MoleError::YInvalid`] if `y` exceeds 30.
    pub fn set_y(&mut self, y: u8) -> Result<(), MoleError> {
        if y > 30 {
            return Err(MoleError::YInvalid);
        }
        self.y = y;
        Ok(())
    }

    /// Sets the mole size.
    ///
    /// # Errors
    ///
    /// Returns [`MoleError::SizeInvalid`] if `size` exceeds 8.
    pub fn set_size(&mut self, size: u8) -> Result<(), MoleError> {
        if size > 8 {
            return Err(MoleError::SizeInvalid);
        }
        self.size = size;
        Ok(())
    }
}

/// Deserializes a [`Mole`] from a `ByteHandler`.
///
/// Extracts the mole visibility flag, horizontal position, vertical position,
/// and size from the packed Mii binary format using bit shifts and masks.
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

/// Errors that can occur while constructing or deserializing a [`Mole`].
#[derive(thiserror::Error, Debug)]
pub enum MoleError {
    /// The size value exceeds the maximum of 8.
    #[error("Size is invalid")]
    SizeInvalid,
    /// The vertical position exceeds the maximum of 30.
    #[error("Y position is invalid")]
    YInvalid,
    /// The horizontal position exceeds the maximum of 16.
    #[error("X position is invalid")]
    XInvalid,
    /// A `ByteHandler` operation failed.
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    /// Infallible conversion error; cannot occur at runtime.
    #[error("")]
    Infallible(#[from] Infallible),
}
