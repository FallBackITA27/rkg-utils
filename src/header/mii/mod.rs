use chrono::{DateTime, Duration, NaiveDateTime};

use crate::{
    byte_handler::{ByteHandler, ByteHandlerError, FromByteHandler},
    header::mii::{
        birthday::{Birthday, BirthdayError},
        build::{Build, BuildError},
        eyebrows::{Eyebrows, EyebrowsError},
        eyes::{Eyes, EyesError},
        facial_hair::{FacialHair, FacialHairError},
        favorite_color::{FavoriteColor, FavoriteColorError},
        glasses::{Glasses, GlassesError},
        hair::{Hair, HairError},
        head::{Head, HeadError},
        lips::{Lips, LipsError},
        mii_type::{MiiType, MiiTypeError},
        mole::{Mole, MoleError},
        nose::{Nose, NoseError},
    },
    write_bits,
};

use std::io::{Read, Write};

pub mod birthday;
pub mod build;
pub mod eyebrows;
pub mod eyes;
pub mod facial_hair;
pub mod favorite_color;
pub mod glasses;
pub mod hair;
pub mod head;
pub mod lips;
pub mod mii_type;
pub mod mole;
pub mod nose;

/// Errors that can occur while parsing or modifying a [`Mii`].
#[derive(thiserror::Error, Debug)]
pub enum MiiError {
    /// A UTF-16 byte sequence could not be decoded as a valid string.
    #[error("FromUtf16Error: {0}")]
    FromUtf16Error(#[from] std::string::FromUtf16Error),
    /// The input data was not exactly `0x4A` bytes long.
    #[error("Invalid data length")]
    InvalidLength,
    /// A birthday field could not be parsed.
    #[error("Birthday Error: {0}")]
    BirthdayError(#[from] BirthdayError),
    /// A favorite color field could not be parsed.
    #[error("FavColor Error: {0}")]
    FavColorError(#[from] FavoriteColorError),
    /// A build (height/weight) field could not be parsed.
    #[error("Build Error: {0}")]
    BuildError(#[from] BuildError),
    /// A head (shape/skin tone/face features) field could not be parsed.
    #[error("Head Error: {0}")]
    HeadError(#[from] HeadError),
    /// A hair field could not be parsed.
    #[error("Hair Error: {0}")]
    HairError(#[from] HairError),
    /// An eyebrows field could not be parsed.
    #[error("Eyebrows Error: {0}")]
    EyebrowsError(#[from] EyebrowsError),
    /// An eyes field could not be parsed.
    #[error("Eyes Error: {0}")]
    EyesError(#[from] EyesError),
    /// A nose field could not be parsed.
    #[error("Nose Error: {0}")]
    NoseError(#[from] NoseError),
    /// A lips field could not be parsed.
    #[error("Lips Error: {0}")]
    LipsError(#[from] LipsError),
    /// A glasses field could not be parsed.
    #[error("Glasses Error: {0}")]
    GlassesError(#[from] GlassesError),
    /// A facial hair field could not be parsed.
    #[error("Facial Hair Error: {0}")]
    FacialHairError(#[from] FacialHairError),
    /// A mole field could not be parsed.
    #[error("Mole Error: {0}")]
    MoleError(#[from] MoleError),
    /// A Mii type field could not be parsed.
    #[error("Mii Type Error: {0}")]
    MiiTypeError(#[from] MiiTypeError),
    /// A `ByteHandler` operation failed.
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    /// A file I/O operation failed.
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
}

/// A fully parsed Mii character, as stored in the Wii's Mii data format.
///
/// Holds all customization fields decoded from the 74-byte (`0x4A`) Mii binary
/// block, along with a copy of the original raw bytes. Any setter method updates
/// both the parsed field and the corresponding bits in [`raw_data`](Mii::raw_data),
/// and sets [`is_modified`](Mii::is_modified) to `true`.
///
/// The binary layout is documented at <http://wiibrew.org/wiki/Mii_Data#Mii_format>.
pub struct Mii {
    /// The raw 74-byte Mii data block, kept in sync with all parsed fields.
    raw_data: [u8; 0x4A],
    /// Whether any field has been modified since the Mii was parsed.
    is_modified: bool,
    /// Whether the Mii is female (`true`) or male (`false`).
    is_girl: bool,
    /// The Mii's birthday.
    birthday: Birthday,
    /// The Mii's favorite color, used to tint outfits and UI elements.
    favorite_color: FavoriteColor,
    /// Whether this Mii has been marked as a favorite.
    is_favorite: bool,
    /// The Mii's display name (up to 10 characters).
    name: String,
    /// The Mii's body proportions (height and weight).
    build: Build,
    /// Whether this is a normal, foreign, or special Mii.
    mii_type: MiiType,
    /// The date and time the Mii was created, derived from the embedded timestamp.
    creation_date: NaiveDateTime,
    /// The full 32-bit Mii ID, including the type bits and creation timestamp.
    mii_id: u32,
    /// The ID of the Wii console that created this Mii.
    system_id: u32,
    /// The Mii's head shape, skin tone, and facial feature overlay.
    head: Head,
    /// Whether StreetPass/Mingle is disabled for this Mii.
    mingle_off: bool,
    /// Whether this Mii was downloaded from the Mii Channel.
    downloaded: bool,
    /// The Mii's hair style, color, and flip setting.
    hair: Hair,
    /// The Mii's eyebrow style, color, size, rotation, and position.
    eyebrows: Eyebrows,
    /// The Mii's eye style, color, size, rotation, and position.
    eyes: Eyes,
    /// The Mii's nose style, size, and vertical position.
    nose: Nose,
    /// The Mii's lip style, color, size, and vertical position.
    lips: Lips,
    /// The Mii's glasses style, color, size, and vertical position.
    glasses: Glasses,
    /// The Mii's beard and mustache style, color, size, and position.
    facial_hair: FacialHair,
    /// The Mii's mole visibility, position, and size.
    mole: Mole,
    /// The name of the player who created this Mii (up to 10 characters).
    creator_name: String,
}

impl Mii {
    /// Parses a [`Mii`] from a 74-byte (`0x4A`) data block.
    ///
    /// Accepts any type that can be converted into `[u8; 0x4A]`, such as a
    /// `&[u8]` slice or a byte array.
    ///
    /// # Errors
    ///
    /// Returns [`MiiError::InvalidLength`] if the input cannot be converted to
    /// exactly `0x4A` bytes. Returns other [`MiiError`] variants if any
    /// individual field fails to parse.
    pub fn new(mii_data: impl TryInto<[u8; 0x4A]>) -> Result<Self, MiiError> {
        let raw_data = mii_data.try_into().map_err(|_| MiiError::InvalidLength)?;

        let bytes = ByteHandler::try_from(&raw_data[0..=1])?;
        let is_girl = bytes.read_bool(6);
        let birthday = Birthday::from_byte_handler(bytes)?;

        let favorite_color = FavoriteColor::from_byte_handler(raw_data[1])?;
        let is_favorite = raw_data[1].is_multiple_of(2);

        let name = utf16be_to_string(&raw_data[0x02..=0x15])?;

        let build = Build::from_byte_handler(&raw_data[0x16..=0x17])?;

        let mii_type = MiiType::try_from(&raw_data[0x18] >> 5)?;

        let mut mii_id = raw_data[0x18..0x1C].to_owned();
        mii_id[0] &= 0x1F;
        let creation_date =
            creation_date_from_timestamp(u32::from_be_bytes(mii_id.try_into().unwrap()));

        let mii_id = ByteHandler::try_from(&raw_data[0x18..=0x1B])?.copy_dword();
        let system_id = ByteHandler::try_from(&raw_data[0x1C..=0x1F])?.copy_dword();

        let bytes = ByteHandler::try_from(&raw_data[0x20..=0x21])?;
        let mingle_off = bytes.read_bool(10);
        let downloaded = bytes.read_bool(8);
        let head = Head::from_byte_handler(bytes)?;
        let hair = Hair::from_byte_handler(&raw_data[0x22..=0x23])?;
        let eyebrows = Eyebrows::from_byte_handler(&raw_data[0x24..=0x27])?;
        let eyes = Eyes::from_byte_handler(&raw_data[0x28..=0x2B])?;
        let nose = Nose::from_byte_handler(&raw_data[0x2C..=0x2D])?;
        let lips = Lips::from_byte_handler(&raw_data[0x2E..=0x2F])?;
        let glasses = Glasses::from_byte_handler(&raw_data[0x30..=0x31])?;
        let facial_hair = FacialHair::from_byte_handler(&raw_data[0x32..=0x33])?;
        let mole = Mole::from_byte_handler(&raw_data[0x34..=0x35])?;

        let creator_name = utf16be_to_string(&raw_data[0x36..=0x49])?;

        let is_modified = false;

        Ok(Self {
            raw_data,
            is_modified,
            is_girl,
            birthday,
            favorite_color,
            is_favorite,
            name,
            build,
            mii_type,
            creation_date,
            mii_id,
            system_id,
            head,
            mingle_off,
            downloaded,
            hair,
            eyebrows,
            eyes,
            nose,
            lips,
            glasses,
            facial_hair,
            mole,
            creator_name,
        })
    }

    /// Parses a [`Mii`] from a raw Mii file or an RKG ghost file.
    ///
    /// If the file begins with the `RKGD` magic bytes, Mii data is read
    /// starting at offset `0x3C` within the file. Otherwise the file is
    /// read from the beginning. In both cases exactly `0x4A` bytes are used.
    ///
    /// # Errors
    ///
    /// Returns [`MiiError::InvalidLength`] if the file contains fewer than
    /// `0x4A` usable bytes. Returns [`MiiError::IOError`] for file I/O
    /// failures, and other [`MiiError`] variants if any field fails to parse.
    pub fn new_from_file<T: AsRef<std::path::Path>>(path: T) -> Result<Self, MiiError> {
        let mut data = Vec::with_capacity(0x4A);
        std::fs::File::open(&path)?.read_exact(&mut data)?;

        if data[..4] == *b"RKGD" {
            // Mii data starts at offset 0x3C in an RKG file
            data = Vec::from(&data[0x3C..]);
        }

        if data.len() < 0x4A {
            return Err(MiiError::InvalidLength);
        }

        Self::new(&data[..0x4A])
    }

    /// Writes the Mii's raw data to a file, creating or truncating it as needed.
    ///
    /// # Errors
    ///
    /// Returns [`MiiError::IOError`] if the file cannot be created or written.
    pub fn save_to_file<T: AsRef<std::path::Path>>(&self, path: T) -> Result<(), MiiError> {
        let mut file = std::fs::File::create(path)?;
        file.write_all(self.raw_data())?;

        Ok(())
    }

    /// Returns the raw 74-byte Mii data block.
    pub fn raw_data(&self) -> &[u8] {
        &self.raw_data
    }

    /// Returns a mutable reference to the raw 74-byte Mii data block.
    pub fn raw_data_mut(&mut self) -> &mut [u8] {
        &mut self.raw_data
    }

    /// Returns whether any field has been modified since the Mii was parsed.
    pub fn is_modified(&self) -> bool {
        self.is_modified
    }

    /// Returns whether the Mii is female.
    pub fn is_girl(&self) -> bool {
        self.is_girl
    }

    /// Sets whether the Mii is female and updates the raw data accordingly.
    pub fn set_is_girl(&mut self, is_girl: bool) {
        self.is_girl = is_girl;
        write_bits(self.raw_data_mut(), 0x00, 1, 1, is_girl as u64);
        self.is_modified = true;
    }

    /// Returns the Mii's birthday.
    pub fn birthday(&self) -> Birthday {
        self.birthday
    }

    /// Sets the Mii's birthday and updates the raw data accordingly.
    pub fn set_birthday(&mut self, birthday: Birthday) {
        // "public fortnite" - fawwe

        self.birthday = birthday;

        let month = birthday.month().unwrap_or(0) as u64;
        let day = birthday.day().unwrap_or(0) as u64;

        write_bits(self.raw_data_mut(), 0x00, 2, 4, month);
        write_bits(self.raw_data_mut(), 0x00, 6, 5, day);

        self.is_modified = true;
    }

    /// Returns the Mii's favorite color.
    pub fn favorite_color(&self) -> FavoriteColor {
        self.favorite_color
    }

    /// Sets the Mii's favorite color and updates the raw data accordingly.
    pub fn set_favorite_color(&mut self, favorite_color: FavoriteColor) {
        self.favorite_color = favorite_color;
        write_bits(
            self.raw_data_mut(),
            0x01,
            3,
            4,
            u8::from(favorite_color) as u64,
        );
        self.is_modified = true;
    }

    /// Returns whether this Mii has been marked as a favorite.
    pub fn is_favorite(&self) -> bool {
        self.is_favorite
    }

    /// Sets whether this Mii is marked as a favorite and updates the raw data accordingly.
    pub fn set_is_favorite(&mut self, is_favorite: bool) {
        self.is_favorite = is_favorite;
        write_bits(self.raw_data_mut(), 0x01, 7, 1, is_favorite as u64);
        self.is_modified = true;
    }

    /// Returns the Mii's display name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the Mii's display name and updates the raw data accordingly.
    ///
    /// Names longer than 10 characters are silently ignored.
    pub fn set_name(&mut self, name: &str) {
        if name.len() > 10 {
            return;
        }
        self.name = name.to_string();
        let name_bytes = string_to_utf16be(name);
        let mut padded = [0u8; 0x14];
        padded[..name_bytes.len()].copy_from_slice(&name_bytes);
        self.raw_data_mut()[0x02..0x16].copy_from_slice(&padded);
        self.is_modified = true;
    }

    /// Returns the Mii's body proportions (height and weight).
    pub fn build(&self) -> Build {
        self.build
    }

    /// Sets the Mii's body proportions and updates the raw data accordingly.
    pub fn set_build(&mut self, build: Build) {
        self.build = build;

        write_bits(self.raw_data_mut(), 0x16, 1, 7, build.height() as u64);
        write_bits(self.raw_data_mut(), 0x17, 1, 7, build.weight() as u64);

        self.is_modified = true;
    }

    /// Returns whether this is a normal, foreign, or special Mii.
    pub fn mii_type(&self) -> MiiType {
        self.mii_type
    }

    /// Sets the Mii type and updates the type bits in the raw Mii ID field.
    ///
    /// Does nothing if the type is already set to the given value.
    pub fn set_mii_type(&mut self, mii_type: MiiType) {
        if self.mii_type() == mii_type {
            return;
        }
        let raw_data = self.raw_data_mut();
        write_bits(raw_data, 0x18, 0, 3, u8::from(mii_type) as u64);
        self.mii_id = u32::from_be_bytes(raw_data[0x18..0x1C].try_into().unwrap());
        self.mii_type = mii_type;
    }

    /// Returns the date and time the Mii was created.
    pub fn creation_date(&self) -> NaiveDateTime {
        self.creation_date
    }

    /// Sets the Mii's creation date and updates the timestamp bits in the raw Mii ID field.
    ///
    /// Note: the valid range of the timestamp has not been fully determined.
    pub fn set_creation_date(&mut self, creation_date: NaiveDateTime) {
        // TODO: determine limits for timestamp
        let raw_data = self.raw_data_mut();
        write_bits(
            raw_data,
            0x18,
            3,
            29,
            timestamp_from_creation_date(creation_date) as u64,
        );
        self.mii_id = u32::from_be_bytes(raw_data[0x18..0x1C].try_into().unwrap());
        self.creation_date = creation_date;
    }

    /// Returns the full 32-bit Mii ID, which encodes both the type and creation timestamp.
    pub fn mii_id(&self) -> u32 {
        self.mii_id
    }

    /// Returns the ID of the Wii console that created this Mii.
    pub fn system_id(&self) -> u32 {
        self.system_id
    }

    /// Sets the system ID and updates the raw data accordingly.
    pub fn set_system_id(&mut self, system_id: u32) {
        self.system_id = system_id;
        self.raw_data_mut()[0x1C..0x20].copy_from_slice(&system_id.to_be_bytes());
        self.is_modified = true;
    }

    /// Returns the Mii's head shape, skin tone, and facial feature overlay.
    pub fn head(&self) -> Head {
        self.head
    }

    /// Sets the Mii's head options and updates the raw data accordingly.
    pub fn set_head(&mut self, head: Head) {
        self.head = head;

        write_bits(
            self.raw_data_mut(),
            0x20,
            0,
            3,
            u8::from(head.shape()) as u64,
        );
        write_bits(
            self.raw_data_mut(),
            0x20,
            3,
            3,
            u8::from(head.skin_tone()) as u64,
        );
        write_bits(
            self.raw_data_mut(),
            0x20,
            6,
            4,
            u8::from(head.face_features()) as u64,
        );

        self.is_modified = true;
    }

    /// Returns whether StreetPass/Mingle is enabled for this Mii.
    pub fn is_mingle_enabled(&self) -> bool {
        !self.mingle_off
    }

    /// Sets whether StreetPass/Mingle is enabled for this Mii and updates the raw data accordingly.
    pub fn set_mingle_enabled(&mut self, enabled: bool) {
        self.mingle_off = !enabled;
        write_bits(self.raw_data_mut(), 0x21, 5, 1, !enabled as u64);
        self.is_modified = true;
    }

    /// Returns whether this Mii was downloaded from the Mii Channel.
    pub fn downloaded(&self) -> bool {
        self.downloaded
    }

    /// Sets whether this Mii was downloaded from the Mii Channel and updates the raw data accordingly.
    pub fn set_downloaded(&mut self, downloaded: bool) {
        self.downloaded = downloaded;
        write_bits(self.raw_data_mut(), 0x21, 7, 1, downloaded as u64);
        self.is_modified = true;
    }

    /// Returns the Mii's hair style, color, and flip setting.
    pub fn hair(&self) -> Hair {
        self.hair
    }

    /// Sets the Mii's hair options and updates the raw data accordingly.
    pub fn set_hair(&mut self, hair: Hair) {
        self.hair = hair;

        write_bits(
            self.raw_data_mut(),
            0x22,
            0,
            7,
            u8::from(hair.hair_type()) as u64,
        );
        write_bits(
            self.raw_data_mut(),
            0x22,
            7,
            3,
            u8::from(hair.hair_color()) as u64,
        );
        write_bits(self.raw_data_mut(), 0x23, 2, 1, hair.is_flipped() as u64);

        self.is_modified = true;
    }

    /// Returns the Mii's eyebrow options.
    pub fn eyebrows(&self) -> Eyebrows {
        self.eyebrows
    }

    /// Sets the Mii's eyebrow options and updates the raw data accordingly.
    pub fn set_eyebrows(&mut self, eyebrows: Eyebrows) {
        self.eyebrows = eyebrows;

        write_bits(
            self.raw_data_mut(),
            0x24,
            0,
            5,
            u8::from(eyebrows.eyebrow_type()) as u64,
        );
        write_bits(self.raw_data_mut(), 0x24, 5, 5, eyebrows.rotation() as u64);
        write_bits(
            self.raw_data_mut(),
            0x26,
            0,
            3,
            u8::from(eyebrows.eyebrow_color()) as u64,
        );
        write_bits(self.raw_data_mut(), 0x26, 3, 4, eyebrows.size() as u64);
        write_bits(self.raw_data_mut(), 0x26, 7, 5, eyebrows.y() as u64);
        write_bits(self.raw_data_mut(), 0x27, 4, 4, eyebrows.x() as u64);

        self.is_modified = true;
    }

    /// Returns the Mii's eye options.
    pub fn eyes(&self) -> Eyes {
        self.eyes
    }

    /// Sets the Mii's eye options and updates the raw data accordingly.
    pub fn set_eyes(&mut self, eyes: Eyes) {
        self.eyes = eyes;

        write_bits(
            self.raw_data_mut(),
            0x28,
            0,
            6,
            u8::from(eyes.eye_type()) as u64,
        );
        write_bits(self.raw_data_mut(), 0x28, 6, 5, eyes.rotation() as u64);
        write_bits(self.raw_data_mut(), 0x29, 3, 5, eyes.y() as u64);
        write_bits(
            self.raw_data_mut(),
            0x2A,
            0,
            3,
            u8::from(eyes.eye_color()) as u64,
        );
        write_bits(self.raw_data_mut(), 0x2A, 3, 4, eyes.size() as u64);
        write_bits(self.raw_data_mut(), 0x2A, 7, 4, eyes.x() as u64);

        self.is_modified = true;
    }

    /// Returns the Mii's nose options.
    pub fn nose(&self) -> Nose {
        self.nose
    }

    /// Sets the Mii's nose options and updates the raw data accordingly.
    pub fn set_nose(&mut self, nose: Nose) {
        self.nose = nose;

        write_bits(
            self.raw_data_mut(),
            0x2C,
            0,
            4,
            u8::from(nose.nose_type()) as u64,
        );
        write_bits(self.raw_data_mut(), 0x2C, 4, 4, nose.size() as u64);
        write_bits(self.raw_data_mut(), 0x2D, 0, 5, nose.y() as u64);

        self.is_modified = true;
    }

    /// Returns the Mii's lip options.
    pub fn lips(&self) -> Lips {
        self.lips
    }

    /// Sets the Mii's lip options and updates the raw data accordingly.
    pub fn set_lips(&mut self, lips: Lips) {
        self.lips = lips;

        write_bits(
            self.raw_data_mut(),
            0x2E,
            0,
            5,
            u8::from(lips.lips_type()) as u64,
        );
        write_bits(
            self.raw_data_mut(),
            0x2E,
            5,
            2,
            u8::from(lips.lips_color()) as u64,
        );
        write_bits(self.raw_data_mut(), 0x2E, 7, 4, lips.size() as u64);
        write_bits(self.raw_data_mut(), 0x2F, 3, 5, lips.y() as u64);

        self.is_modified = true;
    }

    /// Returns the Mii's glasses options.
    pub fn glasses(&self) -> Glasses {
        self.glasses
    }

    /// Sets the Mii's glasses options and updates the raw data accordingly.
    pub fn set_glasses(&mut self, glasses: Glasses) {
        self.glasses = glasses;

        write_bits(
            self.raw_data_mut(),
            0x30,
            0,
            4,
            u8::from(glasses.glasses_type()) as u64,
        );
        write_bits(
            self.raw_data_mut(),
            0x30,
            4,
            3,
            u8::from(glasses.glasses_color()) as u64,
        );
        write_bits(self.raw_data_mut(), 0x30, 7, 4, glasses.size() as u64);
        write_bits(self.raw_data_mut(), 0x31, 3, 5, glasses.y() as u64);

        self.is_modified = true;
    }

    /// Returns the Mii's facial hair options.
    pub fn facial_hair(&self) -> FacialHair {
        self.facial_hair
    }

    /// Sets the Mii's facial hair options and updates the raw data accordingly.
    pub fn set_facial_hair(&mut self, facial_hair: FacialHair) {
        self.facial_hair = facial_hair;

        write_bits(
            self.raw_data_mut(),
            0x32,
            0,
            2,
            u8::from(facial_hair.mustache_type()) as u64,
        );
        write_bits(
            self.raw_data_mut(),
            0x32,
            2,
            2,
            u8::from(facial_hair.beard_type()) as u64,
        );
        write_bits(
            self.raw_data_mut(),
            0x32,
            4,
            3,
            u8::from(facial_hair.color()) as u64,
        );
        write_bits(
            self.raw_data_mut(),
            0x32,
            7,
            4,
            facial_hair.mustache_size() as u64,
        );
        write_bits(
            self.raw_data_mut(),
            0x33,
            3,
            5,
            facial_hair.mustache_y() as u64,
        );

        self.is_modified = true;
    }

    /// Returns the Mii's mole options.
    pub fn mole(&self) -> Mole {
        self.mole
    }

    /// Sets the Mii's mole options and updates the raw data accordingly.
    pub fn set_mole(&mut self, mole: Mole) {
        self.mole = mole;

        write_bits(self.raw_data_mut(), 0x34, 0, 1, mole.has_mole() as u64);
        write_bits(self.raw_data_mut(), 0x34, 1, 4, mole.size() as u64);
        write_bits(self.raw_data_mut(), 0x34, 5, 5, mole.y() as u64);
        write_bits(self.raw_data_mut(), 0x35, 2, 5, mole.x() as u64);

        self.is_modified = true;
    }

    /// Returns the name of the player who created this Mii.
    pub fn creator_name(&self) -> &str {
        &self.creator_name
    }

    /// Sets the creator name and updates the raw data accordingly.
    ///
    /// Names longer than 10 characters are silently ignored.
    pub fn set_creator_name(&mut self, creator_name: &str) {
        if creator_name.len() > 10 {
            return;
        }
        self.creator_name = creator_name.to_string();

        let creator_name_bytes = string_to_utf16be(creator_name);

        let mut padded = [0u8; 0x14];
        padded[..creator_name_bytes.len()].copy_from_slice(&creator_name_bytes);
        self.raw_data_mut()[0x36..0x4A].copy_from_slice(&padded);

        self.is_modified = true;
    }
}

/// Decodes a big-endian UTF-16 byte slice into a [`String`], stopping at the
/// first null character (`U+0000`).
///
/// # Errors
///
/// Returns a [`std::string::FromUtf16Error`] if the byte sequence contains
/// invalid UTF-16 code units.
fn utf16be_to_string(bytes: &[u8]) -> Result<String, std::string::FromUtf16Error> {
    let utf16: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|c| u16::from_be_bytes([c[0], c[1]]))
        .take_while(|&u| u != 0)
        .collect();

    String::from_utf16(&utf16)
}

/// Encodes a UTF-8 string as a big-endian UTF-16 byte sequence.
///
/// The result is not null-terminated; callers are responsible for any padding.
fn string_to_utf16be(string: &str) -> Vec<u8> {
    string
        .encode_utf16()
        .collect::<Vec<u16>>()
        .iter()
        .flat_map(|&u| u.to_be_bytes())
        .collect()
}

/// Converts a raw 29-bit Mii creation timestamp into a [`NaiveDateTime`].
///
/// The timestamp counts ticks at a rate of 4 ticks per second (i.e. each tick
/// is 0.25 seconds), relative to the Mii epoch of 2006-01-01 00:00:00 UTC.
fn creation_date_from_timestamp(value: u32) -> NaiveDateTime {
    let clock_rate = 0.25; // 3 second ticks
    let epoch_shift = 1_136_073_600; // Shifts epoch from 1970-01-01 to 2006-01-01 (which is what Miis use)
    let total_seconds = (value as f64 / clock_rate).floor() as i64;

    let duration = Duration::seconds(total_seconds);
    let epoch = DateTime::from_timestamp(epoch_shift, 0).unwrap();

    epoch.naive_utc() + duration
}

/// Converts a [`NaiveDateTime`] into a raw 29-bit Mii creation timestamp.
///
/// The timestamp counts ticks at a rate of 4 ticks per second (i.e. each tick
/// is 0.25 seconds), relative to the Mii epoch of 2006-01-01 00:00:00 UTC.
fn timestamp_from_creation_date(date: NaiveDateTime) -> u32 {
    let clock_rate = 0.25; // 3 second ticks
    let epoch_shift = 1_136_073_600; // Shifts epoch from 1970-01-01 to 2006-01-01 (which is what Miis use)
    let epoch = DateTime::from_timestamp(epoch_shift, 0).unwrap();

    let duration = date.signed_duration_since(epoch.naive_utc());
    let total_seconds = duration.num_seconds();

    (total_seconds as f64 * clock_rate).floor() as u32
}
