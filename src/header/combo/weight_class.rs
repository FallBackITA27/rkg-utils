use std::fmt::Display;

/// The weight class of a character or vehicle in Mario Kart Wii.
///
/// A valid [`Combo`] requires the character and vehicle to share the same weight class.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WeightClass {
    Small,
    Medium,
    Large,
}

/// Trait for types that have an associated [`WeightClass`].
pub trait GetWeightClass {
    /// Returns the weight class of this character, vehicle, or combo.
    fn get_weight_class(&self) -> WeightClass;
}

impl Display for WeightClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WeightClass::Small => "Lightweight",
                WeightClass::Medium => "Mediumweight",
                WeightClass::Large => "Heavyweight",
            }
        )
    }
}
