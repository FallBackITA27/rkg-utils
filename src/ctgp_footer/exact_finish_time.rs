use std::fmt::Display;

/// Number of picoseconds in one second.
const PICOS_PER_SECOND: u64 = 1_000_000_000_000;

/// Number of picoseconds in one minute.
const PICOS_PER_MINUTE: u64 = 60 * PICOS_PER_SECOND;

/// A high-precision finish time broken down into minutes, seconds, and picoseconds.
///
/// The picoseconds field holds only the sub-second remainder, not the total
/// elapsed picoseconds. Use [`ExactFinishTime::time_to_picoseconds`] to obtain
/// the total duration as a single value.
#[derive(Default, Clone, Copy)]
pub struct ExactFinishTime {
    /// The minutes component of the time.
    minutes: u8,
    /// The seconds component of the time (0–59).
    seconds: u8,
    /// The sub-second component of the time in picoseconds (0–999_999_999_999).
    picoseconds: u64,
}

impl ExactFinishTime {
    /// Creates a new [`ExactFinishTime`] from its individual components.
    ///
    /// # Arguments
    ///
    /// * `minutes` - The minutes component.
    /// * `seconds` - The seconds component (0–59).
    /// * `picoseconds` - The sub-second picoseconds component (0–999_999_999_999).
    #[inline(always)]
    pub fn new(minutes: u8, seconds: u8, picoseconds: u64) -> Self {
        Self {
            minutes,
            seconds,
            picoseconds,
        }
    }

    /// Returns the minutes component of the finish time.
    pub fn minutes(self) -> u8 {
        self.minutes
    }

    /// Returns the seconds component of the finish time (0–59).
    pub fn seconds(self) -> u8 {
        self.seconds
    }

    /// Returns the sub-second picoseconds component of the finish time (0–999_999_999_999).
    pub fn picoseconds(self) -> u64 {
        self.picoseconds
    }

    /// Converts the finish time to a total number of picoseconds.
    ///
    /// Combines all three components into a single duration value:
    /// `minutes * 60 * 10^12 + seconds * 10^12 + picoseconds`.
    ///
    /// # Examples
    ///
    /// ```
    /// let t = ExactFinishTime::new(1, 30, 500_000_000_000);
    /// assert_eq!(t.time_to_picoseconds(), 90_500_000_000_000);
    /// ```
    pub fn time_to_picoseconds(self) -> u64 {
        self.minutes as u64 * PICOS_PER_MINUTE
            + self.seconds as u64 * PICOS_PER_SECOND
            + self.picoseconds
    }
}

/// Formats the finish time as `MM:SS.pppppppppppp` (12-digit picoseconds).
///
/// # Examples
///
/// ```
/// let t = ExactFinishTime::new(1, 9, 123_456_789_000);
/// assert_eq!(t.to_string(), "01:09.123456789000");
/// ```
impl Display for ExactFinishTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}.{:012}",
            self.minutes, self.seconds, self.picoseconds
        )
    }
}

/// Adds two [`ExactFinishTime`] values by summing their total picoseconds and
/// decomposing the result back into minutes, seconds, and picoseconds.
impl std::ops::Add for ExactFinishTime {
    type Output = ExactFinishTime;

    fn add(self, rhs: ExactFinishTime) -> ExactFinishTime {
        let total_ps = self.time_to_picoseconds() + rhs.time_to_picoseconds();

        let picoseconds = total_ps % PICOS_PER_SECOND;
        let total_seconds = total_ps / PICOS_PER_SECOND;
        let seconds = (total_seconds % 60) as u8;
        let minutes = (total_seconds / 60) as u8;

        ExactFinishTime::new(minutes, seconds, picoseconds)
    }
}

/// Sums an iterator of [`ExactFinishTime`] values, starting from the default
/// (zero) time and accumulating with [`Add`].
impl std::iter::Sum for ExactFinishTime {
    fn sum<I: Iterator<Item = ExactFinishTime>>(iter: I) -> Self {
        iter.fold(ExactFinishTime::default(), |a, b| a + b)
    }
}
