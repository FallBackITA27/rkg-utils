use crate::header::controller::Controller;
use crate::input_data::dpad_input::{DPadButton, DPadInput};
use crate::input_data::face_input::{FaceButton, FaceInput};
use crate::input_data::input::Input;
use crate::input_data::stick_input::StickInput;

pub mod dpad_input;
pub mod face_input;
pub mod input;
pub mod stick_input;

/// Errors that can occur while parsing [`InputData`].
#[derive(thiserror::Error, Debug)]
pub enum InputDataError {
    /// Input data is impossibly short.
    #[error("Input data length is too short")]
    InputDataLengthTooShort,
    /// Input data is malformed.
    #[error("Input data is malformed")]
    InputDataMalformed,
    /// A face input entry could not be parsed.
    #[error("Face Input Error: {0}")]
    FaceInputError(#[from] face_input::FaceInputError),
    /// A D-pad input entry could not be parsed.
    #[error("DPad Input Error: {0}")]
    DPadInputError(#[from] dpad_input::DPadInputError),
    /// A stick input entry could not be parsed.
    #[error("Stick Input Error: {0}")]
    StickInputError(#[from] stick_input::StickInputError),
}

/// The controller input stream from a Mario Kart Wii RKG ghost file.
///
/// Stores the raw bytes (compressed or decompressed) alongside three decoded
/// run-length encoded input streams: face buttons, analog stick, and D-pad.
/// Adjacent identical entries across byte boundaries are merged during parsing
/// so that each entry in the decoded vectors represents a single contiguous
/// hold period.
///
/// The binary layout is documented at
/// <https://wiki.tockdom.com/wiki/RKG_(File_Format)#Controller_Input_Data>.
pub struct InputData {
    /// The raw input data bytes as they appear in the file (may be Yaz1 compressed).
    raw_data: Vec<u8>,
    /// The number of face input entries as recorded in the stream header.
    face_input_count: u16,
    /// The number of stick input entries as recorded in the stream header.
    stick_input_count: u16,
    /// The number of D-pad input entries as recorded in the stream header.
    dpad_input_count: u16,
    /// The decoded and merged face button input stream.
    face_inputs: Vec<FaceInput>,
    /// The decoded and merged analog stick input stream.
    stick_inputs: Vec<StickInput>,
    /// The decoded D-pad input stream.
    dpad_inputs: Vec<DPadInput>,
}

impl InputData {
    /// Parses controller input data from raw RKG bytes starting at offset `0x88`.
    ///
    /// If the bytes beginning at offset 4 carry a `Yaz1` magic header, the
    /// data is decompressed before parsing. Otherwise the slice is zero-padded
    /// to `0x2774` bytes. After parsing, adjacent identical face and stick
    /// entries are merged to represent each continuous hold as a single entry.
    ///
    /// # Errors
    ///
    /// Returns an [`InputDataError`] variant if any individual input entry
    /// fails to parse.
    pub fn new(input_data: &[u8]) -> Result<Self, InputDataError> {
        if input_data.len() < 0x08 {
            return Err(InputDataError::InputDataLengthTooShort);
        }

        let mut raw_data = Vec::from(input_data);

        let input_data = if input_data[4..8] == [0x59, 0x61, 0x7A, 0x31] {
            // YAZ1 header, decompress
            yaz1_decompress(&input_data[4..]).unwrap()
        } else {
            raw_data.resize(0x2774, 0x00);
            Vec::from(input_data)
        };

        let face_input_count = u16::from_be_bytes([input_data[0], input_data[1]]);
        let stick_input_count = u16::from_be_bytes([input_data[2], input_data[3]]);
        let dpad_input_count = u16::from_be_bytes([input_data[4], input_data[5]]);
        // bytes 6-7: padding

        if (input_data.len() as u64)
            < ((face_input_count as u64 + stick_input_count as u64 + dpad_input_count as u64) * 2)
                + 8
        {
            return Err(InputDataError::InputDataMalformed);
        }

        let mut current_byte = 8;
        let mut face_inputs: Vec<FaceInput> = Vec::with_capacity(face_input_count as usize);
        while current_byte < 8 + face_input_count * 2 {
            let idx = current_byte as usize;
            let input = &input_data[idx..idx + 2];
            face_inputs.push(FaceInput::try_from(input)?);
            current_byte += 2;
        }

        current_byte = 8 + face_input_count * 2;
        let mut stick_inputs: Vec<StickInput> = Vec::with_capacity(stick_input_count as usize);
        while current_byte < 8 + (face_input_count + stick_input_count) * 2 {
            let idx = current_byte as usize;
            let input = &input_data[idx..idx + 2];
            stick_inputs.push(StickInput::try_from(input)?);
            current_byte += 2;
        }

        current_byte = 8 + (face_input_count + stick_input_count) * 2;
        let mut dpad_inputs: Vec<DPadInput> = Vec::with_capacity(dpad_input_count as usize);
        while current_byte < 8 + (face_input_count + stick_input_count + dpad_input_count) * 2 {
            let idx = current_byte as usize;
            let input = &input_data[idx..idx + 2];
            dpad_inputs.push(DPadInput::try_from(input)?);
            current_byte += 2;
        }

        // Combine adjacent inputs when the same button is held across multiple bytes
        // (each input byte has a 255-frame limit, so buttons held longer need additional bytes)
        for index in (0..face_inputs.len() - 1).rev() {
            if face_inputs[index] == face_inputs[index + 1] {
                let f1 = face_inputs[index].frame_duration();
                let f2 = face_inputs[index + 1].frame_duration();
                face_inputs[index].set_frame_duration(f1 + f2);
                face_inputs.remove(index + 1);
            }
        }

        for index in (0..stick_inputs.len() - 1).rev() {
            if stick_inputs[index] == stick_inputs[index + 1] {
                let f1 = stick_inputs[index].frame_duration();
                let f2 = stick_inputs[index + 1].frame_duration();
                stick_inputs[index].set_frame_duration(f1 + f2);
                stick_inputs.remove(index + 1);
            }
        }

        Ok(Self {
            raw_data,
            face_input_count,
            stick_input_count,
            dpad_input_count,
            face_inputs,
            stick_inputs,
            dpad_inputs,
        })
    }

    /// Returns an input state at a specified frame in the race.
    ///
    /// Returns `None` if the frame is 0 or otherwise out-of-range.
    pub fn get_input_at_frame(&self, frame: u32) -> Option<Input> {
        if frame == 0 {
            return None;
        }

        let mut face_idx = 0;
        let mut stick_idx = 0;
        let mut dpad_idx = 0;
        let mut face_offset = 0u32;
        let mut stick_offset = 0u32;
        let mut dpad_offset = 0u32;
        let mut frames_so_far = 0u32;

        while face_idx < self.face_inputs.len()
            || stick_idx < self.stick_inputs.len()
            || dpad_idx < self.dpad_inputs.len()
        {
            let face = self.face_inputs.get(face_idx);
            let stick = self.stick_inputs.get(stick_idx);
            let dpad = self.dpad_inputs.get(dpad_idx);

            let face_remaining = face
                .map(|f| f.frame_duration() - face_offset)
                .unwrap_or(u32::MAX);
            let stick_remaining = stick
                .map(|s| s.frame_duration() - stick_offset)
                .unwrap_or(u32::MAX);
            let dpad_remaining = dpad
                .map(|d| d.frame_duration() - dpad_offset)
                .unwrap_or(u32::MAX);

            let duration = face_remaining.min(stick_remaining).min(dpad_remaining);

            if duration == u32::MAX {
                break;
            }

            frames_so_far += duration;

            if frame <= frames_so_far {
                return Some(Input::new(
                    face.map(|f| f.buttons().clone()).unwrap_or_default(),
                    stick.map(|s| s.x()).unwrap_or(0),
                    stick.map(|s| s.y()).unwrap_or(0),
                    dpad.map(|d| d.button()).unwrap_or(DPadButton::None),
                    duration,
                ));
            }

            face_offset += duration;
            stick_offset += duration;
            dpad_offset += duration;

            if let Some(face) = face
                && face_offset >= face.frame_duration()
            {
                face_idx += 1;
                face_offset = 0;
            }
            if let Some(stick) = stick
                && stick_offset >= stick.frame_duration()
            {
                stick_idx += 1;
                stick_offset = 0;
            }
            if let Some(dpad) = dpad
                && dpad_offset >= dpad.frame_duration()
            {
                dpad_idx += 1;
                dpad_offset = 0;
            }
        }

        None
    }

    /// Returns the three input streams merged into a single frame-accurate sequence of [`Input`] values.
    ///
    /// The face, stick, and D-pad streams are interleaved by advancing through
    /// all three simultaneously and emitting a new [`Input`] whenever any
    /// stream transitions to its next entry. Each emitted [`Input`] covers
    /// exactly the frames until the next transition across any stream.
    pub fn inputs(&self) -> Vec<Input> {
        let mut result = Vec::new();

        // Track current position in each input stream
        let mut face_idx = 0;
        let mut stick_idx = 0;
        let mut dpad_idx = 0;

        // Track how many frames consumed from current input in each stream
        let mut face_offset = 0u32;
        let mut stick_offset = 0u32;
        let mut dpad_offset = 0u32;

        // Continue until all streams are exhausted
        while face_idx < self.face_inputs.len()
            || stick_idx < self.stick_inputs.len()
            || dpad_idx < self.dpad_inputs.len()
        {
            // Get current input from each stream (or defaults if exhausted)
            let face = self.face_inputs.get(face_idx);
            let stick = self.stick_inputs.get(stick_idx);
            let dpad = self.dpad_inputs.get(dpad_idx);

            // Calculate remaining frames for current input in each stream
            let face_remaining = face
                .map(|f| f.frame_duration() - face_offset)
                .unwrap_or(u32::MAX);
            let stick_remaining = stick
                .map(|s| s.frame_duration() - stick_offset)
                .unwrap_or(u32::MAX);
            let dpad_remaining = dpad
                .map(|d| d.frame_duration() - dpad_offset)
                .unwrap_or(u32::MAX);

            // Find the minimum remaining frames (when next change occurs)
            let duration = face_remaining.min(stick_remaining).min(dpad_remaining);

            if duration == u32::MAX {
                // if all streams exhausted
                break;
            }

            // Create combined input for this duration
            let combined = Input::new(
                face.map(|f| f.buttons().clone()).unwrap_or_default(),
                stick.map(|s| s.x()).unwrap_or(0),
                stick.map(|s| s.y()).unwrap_or(0),
                dpad.map(|d| d.button()).unwrap_or(DPadButton::None),
                duration,
            );
            result.push(combined);

            // Update offsets and advance indices where needed
            face_offset += duration;
            stick_offset += duration;
            dpad_offset += duration;

            if let Some(face) = face
                && face_offset >= face.frame_duration()
            {
                face_idx += 1;
                face_offset = 0;
            }
            if let Some(stick) = stick
                && stick_offset >= stick.frame_duration()
            {
                stick_idx += 1;
                stick_offset = 0;
            }
            if let Some(dpad) = dpad
                && dpad_offset >= dpad.frame_duration()
            {
                dpad_idx += 1;
                dpad_offset = 0;
            }
        }

        result
    }

    /// Returns `true` if the face input stream contains an illegal drift or brake input.
    ///
    /// An input is illegal if drift is active without brake, or if brake and
    /// accelerator are pressed simultaneously without drift when the previous
    /// frame had accelerator but not brake (indicating a missing drift flag).
    pub fn contains_illegal_brake_or_drift_inputs(&self) -> bool {
        for (idx, input) in self.face_inputs().iter().enumerate() {
            let current_buttons = input.buttons();
            if current_buttons.contains(&FaceButton::Drift)
                && !current_buttons.contains(&FaceButton::Brake)
            {
                // Illegal drift input
                return true;
            } else if idx > 0 {
                let previous_buttons = self.face_inputs()[idx - 1].buttons();
                if current_buttons.contains(&FaceButton::Brake)
                    && current_buttons.contains(&FaceButton::Accelerator)
                    && !current_buttons.contains(&FaceButton::Drift)
                    && previous_buttons.contains(&FaceButton::Accelerator)
                    && !previous_buttons.contains(&FaceButton::Brake)
                {
                    // Illegal brake input (drift flag isn't 1 when it should be)
                    return true;
                }
            }
        }
        false
    }

    /// Returns `true` if the raw input data begins with a Yaz1 magic header at offset 4.
    pub fn is_compressed(&self) -> bool {
        self.raw_data[4..8] == [0x59, 0x61, 0x7A, 0x31]
    }

    /// Compresses the raw input data using Yaz1 encoding.
    ///
    /// Does nothing if the data is already compressed.
    pub(crate) fn compress(&mut self) {
        if !self.is_compressed() {
            self.raw_data = yaz1_compress(&self.raw_data);
        }
    }

    /// Decompresses the raw input data from Yaz1 encoding.
    ///
    /// Does nothing if the data is not compressed.
    pub(crate) fn decompress(&mut self) {
        if self.is_compressed() {
            self.raw_data = yaz1_decompress(&self.raw_data[4..]).unwrap();
        }
    }

    /// Returns the raw input data bytes as they appear in the file.
    pub fn raw_data(&self) -> &[u8] {
        &self.raw_data
    }

    /// Returns `true` if the stick input stream contains any illegal stick position
    /// for the given controller type. More info on illegal input ranges here:
    /// <https://github.com/malleoz/mkw-replay?tab=readme-ov-file#regarding-input-ranges>
    /// <https://youtu.be/KUjS7qWWu9c?t=489>
    ///
    /// The Wii Wheel has a fully unrestricted input range and is never considered to
    /// have illegal inputs.
    pub fn contains_illegal_stick_inputs(&self, controller: Controller) -> bool {
        // Definition of illegal stick inputs [x, y]
        const ILLEGAL_STICK_INPUTS: [[i8; 2]; 44] = [
            // These inputs are illegal for GCN, CCP, and Nunchuk (24 total)
            [-7, 7],
            [-7, 6],
            [-7, 5],
            [-7, -7],
            [-7, -6],
            [-7, -5],
            [-6, 7],
            [-6, 6],
            [-6, -7],
            [-6, -6],
            [-5, 7],
            [-5, -7],
            [7, 7],
            [7, 6],
            [7, 5],
            [7, -7],
            [7, -6],
            [7, -5],
            [6, 7],
            [6, 6],
            [6, -7],
            [6, -6],
            [5, 7],
            [5, -7],
            // Illegal stick inputs for specifically GCN/CCP (additional 20)
            [-7, 4],
            [-6, 5],
            [-5, 6],
            [-4, 7],
            [-3, 7],
            [3, 7],
            [4, 7],
            [4, 6],
            [4, -7],
            [5, 6],
            [5, 5],
            [5, -6],
            [6, 5],
            [6, 4],
            [6, -5],
            [7, 4],
            [7, 3],
            [7, 2],
            [7, -3],
            [7, -4],
        ];

        let illegal_stick_inputs = match controller {
            Controller::Nunchuk => &ILLEGAL_STICK_INPUTS[..24],
            Controller::Classic | Controller::Gamecube => &ILLEGAL_STICK_INPUTS,
            Controller::WiiWheel => {
                return false;
            }
        };

        for current_stick_input in self.stick_inputs().iter() {
            for illegal_stick_input in illegal_stick_inputs.iter() {
                if current_stick_input == illegal_stick_input {
                    return true;
                }
            }
        }

        false
    }

    /// Returns the decoded face button input stream.
    pub fn face_inputs(&self) -> &[FaceInput] {
        &self.face_inputs
    }

    /// Returns the decoded analog stick input stream.
    pub fn stick_inputs(&self) -> &[StickInput] {
        &self.stick_inputs
    }

    /// Returns the decoded D-pad input stream.
    pub fn dpad_inputs(&self) -> &[DPadInput] {
        &self.dpad_inputs
    }

    /// Returns the number of face input entries as recorded in the stream header.
    pub fn face_input_count(&self) -> u16 {
        self.face_input_count
    }

    /// Returns the number of stick input entries as recorded in the stream header.
    pub fn stick_input_count(&self) -> u16 {
        self.stick_input_count
    }

    /// Returns the number of D-pad input entries as recorded in the stream header.
    pub fn dpad_input_count(&self) -> u16 {
        self.dpad_input_count
    }
}

/// Decompresses a Yaz1-encoded byte slice into raw input data.
///
/// The slice must begin with the `Yaz1` magic followed by the uncompressed
/// size as a big-endian `u32` and 8 bytes of padding. The result is
/// zero-padded to `0x2774` bytes.
///
/// Returns `None` if the magic is missing, the data is truncated, or the
/// decompressed output does not match the expected size.
///
/// Adapted from <https://github.com/AtishaRibeiro/InputDisplay/blob/master/InputDisplay/Core/Yaz1dec.cs>.
pub(crate) fn yaz1_decompress(data: &[u8]) -> Option<Vec<u8>> {
    // YAZ1 files start with "Yaz1" magic header
    if data.len() < 16 || &data[0..4] != b"Yaz1" {
        return None;
    }

    let uncompressed_size = u32::from_be_bytes([data[4], data[5], data[6], data[7]]) as usize;

    let mut result = Vec::with_capacity(uncompressed_size);

    let decompressed = decompress_block(
        data,
        16, // Start after 16-byte header
        uncompressed_size,
    );

    if let Some(mut dec) = decompressed {
        result.append(&mut dec);
    }

    if result.len() == uncompressed_size {
        result.resize(0x2774, 0);
        Some(result)
    } else {
        None
    }
}

/// Decompresses a single Yaz1 block starting at `offset` within `src`.
///
/// Returns `None` if the source data is truncated mid-block. The output is
/// exactly `uncompressed_size` bytes when successful.
fn decompress_block(src: &[u8], offset: usize, uncompressed_size: usize) -> Option<Vec<u8>> {
    let mut dst = Vec::with_capacity(uncompressed_size);
    let mut src_pos = offset;

    let mut valid_bit_count = 0; // number of valid bits left in "code" byte
    let mut curr_code_byte = 0u8;

    while dst.len() < uncompressed_size {
        // Read new "code" byte if the current one is used up
        if valid_bit_count == 0 {
            if src_pos >= src.len() {
                return None;
            }
            curr_code_byte = src[src_pos];
            src_pos += 1;
            valid_bit_count = 8;
        }

        if (curr_code_byte & 0x80) != 0 {
            // Straight copy
            if src_pos >= src.len() {
                return None;
            }
            dst.push(src[src_pos]);
            src_pos += 1;
        } else {
            // RLE part
            if src_pos + 1 >= src.len() {
                return None;
            }

            let byte1 = src[src_pos];
            src_pos += 1;
            let byte2 = src[src_pos];
            src_pos += 1;

            let dist = (((byte1 & 0xF) as usize) << 8) | (byte2 as usize);
            let copy_source = dst.len().wrapping_sub(dist + 1);

            let mut num_bytes = (byte1 >> 4) as usize;
            if num_bytes == 0 {
                if src_pos >= src.len() {
                    return None;
                }
                num_bytes = src[src_pos] as usize + 0x12;
                src_pos += 1;
            } else {
                num_bytes += 2;
            }

            // Copy run - must handle overlapping copies
            for i in 0..num_bytes {
                if copy_source + i >= dst.len() {
                    return None;
                }
                let byte = dst[copy_source + i];
                dst.push(byte);
            }
        }

        // Use next bit from "code" byte
        curr_code_byte <<= 1;
        valid_bit_count -= 1;
    }

    Some(dst)
}

/// Compresses raw input data using Yaz1 encoding.
///
/// Trailing zero bytes (used to pad decompressed data to `0x2774` bytes) are
/// stripped before compression. The output includes a full Yaz1 file header
/// containing the compressed size, the `Yaz1` magic, the uncompressed size,
/// and 8 bytes of padding.
///
/// Adapted from <https://github.com/AtishaRibeiro/TT-Rec-Tools/blob/dev/ghostmanager/Scripts/YAZ1_comp.js>.
pub(crate) fn yaz1_compress(src: &[u8]) -> Vec<u8> {
    // first remove padded 0s (decompressed input data is padded with 0s to 0x2774 bytes)
    let mut trailing_bytes_to_remove = 0usize;
    for idx in (0..src.len()).rev() {
        if src[idx] == 0 {
            trailing_bytes_to_remove += 1;
        } else {
            break;
        }
    }

    let src = &src[0..src.len() - trailing_bytes_to_remove];

    let mut dst = Vec::new();
    let src_size = src.len();
    let mut src_pos = 0;
    let mut prev_flag = false;
    let mut prev_num_bytes = 0;
    let mut prev_match_pos = 0;

    let mut code_byte = 0u8;
    let mut valid_bit_count = 0;
    let mut chunk = Vec::with_capacity(24); // 8 codes * 3 bytes maximum

    while src_pos < src_size {
        let (num_bytes, match_pos) = nintendo_encode(
            src,
            src_size,
            src_pos,
            &mut prev_flag,
            &mut prev_num_bytes,
            &mut prev_match_pos,
        );

        if num_bytes < 3 {
            // Straight copy
            chunk.push(src[src_pos]);
            src_pos += 1;
            // Set flag for straight copy
            code_byte |= 0x80 >> valid_bit_count;
        } else {
            // RLE part
            let dist = src_pos - match_pos - 1;

            if num_bytes >= 0x12 {
                // 3 byte encoding
                let byte1 = (dist >> 8) as u8;
                let byte2 = (dist & 0xff) as u8;
                chunk.push(byte1);
                chunk.push(byte2);

                // Maximum runlength for 3 byte encoding
                let num_bytes = num_bytes.min(0xff + 0x12);
                let byte3 = (num_bytes - 0x12) as u8;
                chunk.push(byte3);
            } else {
                // 2 byte encoding
                let byte1 = (((num_bytes - 2) << 4) | (dist >> 8)) as u8;
                let byte2 = (dist & 0xff) as u8;
                chunk.push(byte1);
                chunk.push(byte2);
            }
            src_pos += num_bytes;
        }

        valid_bit_count += 1;

        // Write eight codes
        if valid_bit_count == 8 {
            dst.push(code_byte);
            dst.extend_from_slice(&chunk);

            code_byte = 0;
            valid_bit_count = 0;
            chunk.clear();
        }
    }

    // Write remaining codes
    if valid_bit_count > 0 {
        dst.push(code_byte);
        dst.extend_from_slice(&chunk);
    }

    let mut compressed_data = Vec::new();

    // Write Yaz1 header
    compressed_data.extend_from_slice(&((dst.len() + 8) as u32).to_be_bytes()); // size of compressed data
    compressed_data.extend_from_slice(b"Yaz1");
    compressed_data.extend_from_slice(&(src_size as u32).to_be_bytes());
    compressed_data.extend_from_slice(&[0u8; 8]); // padding
    compressed_data.extend_from_slice(&dst);
    compressed_data
}

/// Determines the best encoding for the byte at `pos` using the Nintendo Yaz1 heuristic.
///
/// If the previous call set a lookahead flag, the cached values from that call
/// are returned immediately. Otherwise [`simple_encode`] is run at `pos` and
/// at `pos + 1`; if the next position's match is 2 or more bytes longer, the
/// lookahead flag is set and a literal copy is emitted for the current position.
fn nintendo_encode(
    src: &[u8],
    size: usize,
    pos: usize,
    prev_flag: &mut bool,
    prev_num_bytes: &mut usize,
    prev_match_pos: &mut usize,
) -> (usize, usize) {
    // If prevFlag is set, use the previously calculated values
    if *prev_flag {
        *prev_flag = false;
        return (*prev_num_bytes, *prev_match_pos);
    }

    *prev_flag = false;
    let (num_bytes, match_pos) = simple_encode(src, size, pos);

    // If this position is RLE encoded, compare to copying 1 byte and next position encoding
    if num_bytes >= 3 {
        let (num_bytes1, match_pos1) = simple_encode(src, size, pos + 1);
        *prev_num_bytes = num_bytes1;
        *prev_match_pos = match_pos1;

        // If the next position encoding is +2 longer, choose it
        if num_bytes1 >= num_bytes + 2 {
            *prev_flag = true;
            return (1, match_pos);
        }
    }

    (num_bytes, match_pos)
}

/// Finds the longest match for `src[pos..]` within the preceding `0x1000`-byte
/// window using a simple linear scan.
///
/// Returns `(num_bytes, match_pos)` where `num_bytes` is the length of the
/// longest match found (1 if no match of length ≥ 3 was found) and
/// `match_pos` is the starting offset of that match in `src`.
fn simple_encode(src: &[u8], size: usize, pos: usize) -> (usize, usize) {
    let mut start_pos = pos as i32 - 0x1000;
    let mut num_bytes = 1;
    let mut match_pos = 0;

    if start_pos < 0 {
        start_pos = 0;
    }
    let start_pos = start_pos as usize;

    for i in start_pos..pos {
        let mut j = 0;
        // Match the JavaScript loop condition exactly: j < size-pos
        while j < size - pos {
            if src[i + j] != src[j + pos] {
                break;
            }
            j += 1;
        }

        if j > num_bytes {
            num_bytes = j;
            match_pos = i;
        }
    }

    if num_bytes == 2 {
        num_bytes = 1;
    }

    (num_bytes, match_pos)
}
