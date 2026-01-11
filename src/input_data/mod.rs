use crate::input_data::dpad_input::{DPadButton, DPadInput};
use crate::input_data::face_input::{FaceButton, FaceInput};
use crate::input_data::input::Input;
use crate::input_data::stick_input::StickInput;

pub mod dpad_input;
pub mod face_input;
pub mod input;
pub mod stick_input;

#[derive(thiserror::Error, Debug)]
pub enum InputDataError {
    #[error("Face Input Error: {0}")]
    FaceInputError(#[from] face_input::FaceInputError),
    #[error("DPad Input Error: {0}")]
    DPadInputError(#[from] dpad_input::DPadInputError),
    #[error("Stick Input Error: {0}")]
    StickInputError(#[from] stick_input::StickInputError),
}

/// Handles all input data being read
/// Tockdom wiki: https://wiki.tockdom.com/wiki/RKG_(File_Format)#Controller_Input_Data
pub struct InputData {
    face_input_count: u16,
    stick_input_count: u16,
    dpad_input_count: u16,
    face_inputs: Vec<FaceInput>,
    stick_inputs: Vec<StickInput>,
    dpad_inputs: Vec<DPadInput>,
}

impl InputData {
    /// Expects RKG data 0x88..(end of input data).
    /// End of input data is either 0x04 from the end of the file of vanilla ghosts,
    /// or if it's a CTGP ghost the input data end is whatever the CTGP metadata size is.
    pub fn new(input_data: &[u8]) -> Result<Self, InputDataError> {
        let input_data = if input_data[4..8] == [0x59, 0x61, 0x7A, 0x31] {
            // YAZ1 header, decompress
            yaz1_decompress(&input_data[4..]).unwrap()
        } else {
            Vec::from(input_data)
        };

        let face_input_count = u16::from_be_bytes([input_data[0], input_data[1]]);
        let stick_input_count = u16::from_be_bytes([input_data[2], input_data[3]]);
        let dpad_input_count = u16::from_be_bytes([input_data[4], input_data[5]]);
        // bytes 6-7: padding

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
            face_input_count,
            stick_input_count,
            dpad_input_count,
            face_inputs,
            stick_inputs,
            dpad_inputs,
        })
    }

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

    /// Returns true if the inputs contain an illegal drift or brake input.
    pub fn contains_illegal_inputs(&self) -> bool {
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

    pub fn face_inputs(&self) -> &[FaceInput] {
        &self.face_inputs
    }

    pub fn stick_inputs(&self) -> &[StickInput] {
        &self.stick_inputs
    }

    pub fn dpad_inputs(&self) -> &[DPadInput] {
        &self.dpad_inputs
    }

    pub fn face_input_count(&self) -> u16 {
        self.face_input_count
    }

    pub fn stick_input_count(&self) -> u16 {
        self.stick_input_count
    }

    pub fn dpad_input_count(&self) -> u16 {
        self.dpad_input_count
    }
}

/// Decompress YAZ1-compressed input data
/// Adapted from https://github.com/AtishaRibeiro/InputDisplay/blob/master/InputDisplay/Core/Yaz1dec.cs
pub fn yaz1_decompress(data: &[u8]) -> Option<Vec<u8>> {
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
        Some(result)
    } else {
        None
    }
}

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

/// Compress input data with Yaz1 compression
/// Adapted from https://github.com/AtishaRibeiro/TT-Rec-Tools/blob/dev/ghostmanager/Scripts/YAZ1_comp.js
pub fn yaz1_compress(src: &[u8]) -> Vec<u8> {
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
