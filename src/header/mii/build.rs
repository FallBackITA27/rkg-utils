use std::convert::Infallible;

use crate::byte_handler::{ByteHandlerError, FromByteHandler};

/// Errors that can occur while constructing a [`Build`].
#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    /// The weight value exceeds the maximum allowed value of 127.
    #[error("Weight is invalid")]
    WeightInvalid,
    /// The height value exceeds the maximum allowed value of 127.
    #[error("Height is invalid")]
    HeightInvalid,
    /// A `ByteHandler` operation failed.
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    /// Infallible conversion error; cannot occur at runtime.
    #[error("")]
    Infallible(#[from] Infallible),
}

/// Represents the body proportions of a Mii, storing height and weight values.
///
/// Both values are clamped to the range 0–127 as defined by the Mii data format.
#[derive(Clone, Copy)]
pub struct Build {
    /// The Mii's height (0–127).
    height: u8,
    /// The Mii's weight (0–127).
    weight: u8,
}

impl Build {
    #[inline(always)]
    /// Creates a new [`Build`] from raw height and weight values.
    ///
    /// # Arguments
    ///
    /// * `height` - The Mii's height (0–127).
    /// * `weight` - The Mii's weight (0–127).
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::HeightInvalid`] if `height` exceeds 127.
    /// Returns [`BuildError::WeightInvalid`] if `weight` exceeds 127.
    pub fn new(height: u8, weight: u8) -> Result<Self, BuildError> {
        if height > 127 {
            return Err(BuildError::HeightInvalid);
        }
        if weight > 127 {
            return Err(BuildError::WeightInvalid);
        }
        Ok(Self { height, weight })
    }

    /// Returns the Mii's height (0–127).
    pub fn height(&self) -> u8 {
        self.height
    }

    /// Sets the Mii's height to the given value.
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::HeightInvalid`] if `height` exceeds 127.
    pub fn set_height(&mut self, height: u8) -> Result<(), BuildError> {
        if height > 127 {
            return Err(BuildError::HeightInvalid);
        }
        self.height = height;
        Ok(())
    }

    /// Returns the Mii's weight (0–127).
    pub fn weight(&self) -> u8 {
        self.weight
    }

    /// Sets the Mii's weight to the given value.
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::WeightInvalid`] if `weight` exceeds 127.
    pub fn set_weight(&mut self, weight: u8) -> Result<(), BuildError> {
        if weight > 127 {
            return Err(BuildError::WeightInvalid);
        }
        self.height = weight;
        Ok(())
    }
}

/// Deserializes a [`Build`] from a `ByteHandler`.
///
/// The height is read from the first byte and the weight from the second byte.
impl FromByteHandler for Build {
    type Err = BuildError;

    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<crate::byte_handler::ByteHandler>,
        Self::Err: From<T::Error>,
    {
        let handler = handler.try_into()?;

        Self::new(handler.copy_byte(0), handler.copy_byte(1))
    }
}
