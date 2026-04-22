use crate::input_data::dpad_input::DPadButton;
use crate::input_data::face_input::FaceButton;

/// A fully decoded controller input state for one or more consecutive frames.
///
/// Combines the face button state, analog stick position, and D-pad button
/// from a single encoded input triplet (face + stick + D-pad) into
/// one convenient struct. The `frame_duration` field records how many
/// consecutive frames this exact input state was held.
#[derive(Debug, Clone, PartialEq)]
pub struct Input {
    /// The set of face buttons active during this input state.
    face_buttons: Vec<FaceButton>,
    /// Horizontal stick axis (−7 to +7; negative = left, positive = right).
    stick_x: i8,
    /// Vertical stick axis (−7 to +7; negative = down, positive = up).
    stick_y: i8,
    /// The D-pad button held during this input state.
    dpad_button: DPadButton,
    /// The number of consecutive frames this input state was held.
    frame_duration: u32,
}

impl Default for Input {
    fn default() -> Self {
        Self { face_buttons: Default::default(), stick_x: Default::default(), stick_y: Default::default(), dpad_button: DPadButton::None, frame_duration: Default::default() }
    }
}

impl Input {
    /// Creates a new [`Input`] from its individual components.
    pub fn new(
        face_buttons: Vec<FaceButton>,
        stick_x: i8,
        stick_y: i8,
        dpad_button: DPadButton,
        frame_duration: u32,
    ) -> Self {
        Self {
            face_buttons,
            stick_x,
            stick_y,
            dpad_button,
            frame_duration,
        }
    }

    /// Returns the set of face buttons active during this input state.
    pub fn face_buttons(&self) -> &[FaceButton] {
        &self.face_buttons
    }

    /// Returns the horizontal stick axis value (−7 to +7).
    pub fn stick_x(&self) -> i8 {
        self.stick_x
    }

    /// Returns the vertical stick axis value (−7 to +7).
    pub fn stick_y(&self) -> i8 {
        self.stick_y
    }

    /// Returns the D-pad button held during this input state.
    pub fn dpad_button(&self) -> DPadButton {
        self.dpad_button
    }

    /// Returns the number of consecutive frames this input state was held.
    pub fn frame_duration(&self) -> u32 {
        self.frame_duration
    }
}
