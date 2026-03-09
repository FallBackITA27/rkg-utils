use std::convert::Infallible;

use crate::byte_handler::{ByteHandlerError, FromByteHandler};

/// Represents the head customization options of a Mii,
/// including face shape, skin tone, and facial feature overlay.
#[derive(Clone, Copy)]
pub struct Head {
    /// Face/head shape.
    shape: HeadShape,
    /// Skin tone.
    skin_tone: SkinTone,
    /// Facial feature overlay applied to the face.
    face_features: FaceFeatures,
}

/// Errors that can occur while deserializing a [`Head`].
#[derive(thiserror::Error, Debug)]
pub enum HeadError {
    /// The head shape byte did not map to a known [`HeadShape`] variant.
    #[error("Shape is invalid")]
    ShapeInvalid,
    /// The skin tone byte did not map to a known [`SkinTone`] variant.
    #[error("SkinTone is invalid")]
    SkinToneInvalid,
    /// The face features byte did not map to a known [`FaceFeatures`] variant.
    #[error("FaceFeatures is invalid")]
    FaceFeaturesInvalid,
    /// A `ByteHandler` operation failed.
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    /// Infallible conversion error; cannot occur at runtime.
    #[error("")]
    Infallible(#[from] Infallible),
}

impl Head {
    /// Creates a new [`Head`] from its individual components.
    ///
    /// # Arguments
    ///
    /// * `shape` - Face/head shape.
    /// * `skin_tone` - Skin tone.
    /// * `face_features` - Facial feature overlay.
    pub fn new(shape: HeadShape, skin_tone: SkinTone, face_features: FaceFeatures) -> Self {
        Self {
            shape,
            skin_tone,
            face_features,
        }
    }

    /// Returns the face/head shape.
    pub fn shape(&self) -> HeadShape {
        self.shape
    }

    /// Returns the skin tone.
    pub fn skin_tone(&self) -> SkinTone {
        self.skin_tone
    }

    /// Returns the facial feature overlay.
    pub fn face_features(&self) -> FaceFeatures {
        self.face_features
    }

    /// Sets the face/head shape.
    pub fn set_shape(&mut self, shape: HeadShape) {
        self.shape = shape;
    }

    /// Sets the skin tone.
    pub fn set_skin_tone(&mut self, skin_tone: SkinTone) {
        self.skin_tone = skin_tone;
    }

    /// Sets the facial feature overlay.
    pub fn set_face_features(&mut self, face_features: FaceFeatures) {
        self.face_features = face_features;
    }
}

/// Deserializes a [`Head`] from a `ByteHandler`.
///
/// The handler is shifted right by 5 bits to align the head shape, then shifted
/// right by 1 more bit to extract the skin tone and face features from the
/// packed Mii binary format using bit masks.
impl FromByteHandler for Head {
    type Err = HeadError;
    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<crate::byte_handler::ByteHandler>,
        Self::Err: From<T::Error>,
    {
        let mut handler = handler.try_into()?;
        handler.shift_right(5);
        let shape_value = handler.copy_byte(0);
        handler.shift_right(1);
        let byte = handler.copy_byte(1);

        Ok(Head {
            shape: HeadShape::try_from(shape_value).map_err(|_| HeadError::ShapeInvalid)?,
            skin_tone: SkinTone::try_from((byte >> 4) & 0x07)
                .map_err(|_| HeadError::SkinToneInvalid)?,
            face_features: FaceFeatures::try_from(byte & 0x0F)
                .map_err(|_| HeadError::FaceFeaturesInvalid)?,
        })
    }
}

/// Face/head shape options available in the Mii editor.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum HeadShape {
    Sharp,
    Rounded,
    SharpRoundedSmall,
    Large,
    SharpSmall,
    Flat,
    Angular,
    FlatRounded,
}

/// Converts a raw byte value from the Mii data format into a [`HeadShape`].
///
/// Returns `Err(())` if the byte does not correspond to any known head shape.
impl TryFrom<u8> for HeadShape {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Sharp),
            0x01 => Ok(Self::Rounded),
            0x02 => Ok(Self::SharpRoundedSmall),
            0x03 => Ok(Self::Large),
            0x04 => Ok(Self::SharpSmall),
            0x05 => Ok(Self::Flat),
            0x06 => Ok(Self::Angular),
            0x07 => Ok(Self::FlatRounded),
            _ => Err(()),
        }
    }
}

/// Converts a [`HeadShape`] into its raw byte representation for the Mii data format.
impl From<HeadShape> for u8 {
    fn from(value: HeadShape) -> Self {
        match value {
            HeadShape::Sharp => 0x00,
            HeadShape::Rounded => 0x01,
            HeadShape::SharpRoundedSmall => 0x02,
            HeadShape::Large => 0x03,
            HeadShape::SharpSmall => 0x04,
            HeadShape::Flat => 0x05,
            HeadShape::Angular => 0x06,
            HeadShape::FlatRounded => 0x07,
        }
    }
}

/// Skin tone options available in the Mii editor.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SkinTone {
    Beige,
    Natural,
    WarmIvory,
    Ivory,
    Honey,
    Chestnut,
}

/// Converts a raw byte value from the Mii data format into a [`SkinTone`].
///
/// Returns `Err(())` if the byte does not correspond to any known skin tone.
impl TryFrom<u8> for SkinTone {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Beige),
            0x01 => Ok(Self::Natural),
            0x02 => Ok(Self::WarmIvory),
            0x03 => Ok(Self::Ivory),
            0x04 => Ok(Self::Honey),
            0x05 => Ok(Self::Chestnut),
            _ => Err(()),
        }
    }
}

/// Converts a [`SkinTone`] into its raw byte representation for the Mii data format.
impl From<SkinTone> for u8 {
    fn from(value: SkinTone) -> Self {
        match value {
            SkinTone::Beige => 0x00,
            SkinTone::Natural => 0x01,
            SkinTone::WarmIvory => 0x02,
            SkinTone::Ivory => 0x03,
            SkinTone::Honey => 0x04,
            SkinTone::Chestnut => 0x05,
        }
    }
}

/// Facial feature overlays available in the Mii editor.
///
/// These are decorative markings drawn on top of the Mii's face shape.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FaceFeatures {
    /// No facial feature overlay.
    None,
    /// Porcelain-style cheek blush.
    CheekPorcelain,
    /// Porcelain-style cheek blush with blue eye shadow.
    CheekPorcelainEyeShadowBlue,
    Freckles,
    /// Dark circles or bags under the eyes.
    UnderTheEyes,
    /// Marks suggesting facial pain or stress lines.
    FacialPain,
    /// Rounded cheek blush marks.
    Cheeks,
    /// Chin shading or cleft.
    Chin,
    /// Drooping brow lines.
    BrowDroop,
    /// Lion's mane-style beard overlay.
    LionsManeBeard,
    /// Downturned mouth frown lines.
    MouthFrown,
    /// Nasolabial folds with crow's feet and frown lines.
    FoldsCrowsFrown,
}

/// Converts a raw byte value from the Mii data format into a [`FaceFeatures`] variant.
///
/// Returns `Err(())` if the byte does not correspond to any known face features value.
impl TryFrom<u8> for FaceFeatures {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::None),
            0x01 => Ok(Self::CheekPorcelain),
            0x02 => Ok(Self::CheekPorcelainEyeShadowBlue),
            0x03 => Ok(Self::Freckles),
            0x04 => Ok(Self::UnderTheEyes),
            0x05 => Ok(Self::FacialPain),
            0x06 => Ok(Self::Cheeks),
            0x07 => Ok(Self::Chin),
            0x08 => Ok(Self::BrowDroop),
            0x09 => Ok(Self::LionsManeBeard),
            0x0A => Ok(Self::MouthFrown),
            0x0B => Ok(Self::FoldsCrowsFrown),
            _ => Err(()),
        }
    }
}

/// Converts a [`FaceFeatures`] variant into its raw byte representation for the Mii data format.
impl From<FaceFeatures> for u8 {
    fn from(value: FaceFeatures) -> Self {
        match value {
            FaceFeatures::None => 0x00,
            FaceFeatures::CheekPorcelain => 0x01,
            FaceFeatures::CheekPorcelainEyeShadowBlue => 0x02,
            FaceFeatures::Freckles => 0x03,
            FaceFeatures::UnderTheEyes => 0x04,
            FaceFeatures::FacialPain => 0x05,
            FaceFeatures::Cheeks => 0x06,
            FaceFeatures::Chin => 0x07,
            FaceFeatures::BrowDroop => 0x08,
            FaceFeatures::LionsManeBeard => 0x09,
            FaceFeatures::MouthFrown => 0x0A,
            FaceFeatures::FoldsCrowsFrown => 0x0B,
        }
    }
}
