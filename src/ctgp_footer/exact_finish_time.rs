use std::fmt::Display;

const PICOS_PER_SECOND: u64 = 1_000_000_000_000;
const PICOS_PER_MINUTE: u64 = 60 * PICOS_PER_SECOND;

#[derive(Default, Clone, Copy)]
pub struct ExactFinishTime {
    minutes: u8,
    seconds: u8,
    picoseconds: u64,
}

impl ExactFinishTime {
    #[inline(always)]
    pub fn new(minutes: u8, seconds: u8, picoseconds: u64) -> Self {
        Self {
            minutes,
            seconds,
            picoseconds,
        }
    }

    pub fn minutes(self) -> u8 {
        self.minutes
    }

    pub fn seconds(self) -> u8 {
        self.seconds
    }

    pub fn picoseconds(self) -> u64 {
        self.picoseconds
    }

    pub fn time_to_picoseconds(self) -> u64 {
        self.minutes as u64 * PICOS_PER_MINUTE
            + self.seconds as u64 * PICOS_PER_SECOND
            + self.picoseconds
    }
}

impl Display for ExactFinishTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}.{:012}",
            self.minutes, self.seconds, self.picoseconds
        )
    }
}

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

impl std::iter::Sum for ExactFinishTime {
    fn sum<I: Iterator<Item = ExactFinishTime>>(iter: I) -> Self {
        iter.fold(ExactFinishTime::default(), |a, b| a + b)
    }
}
