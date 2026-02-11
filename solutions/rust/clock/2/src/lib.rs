use std::fmt::Display;

const HOURS: i32 = 24;
const MINUTES: i32 = 60;
const DECIMAL: u32 = 10;

#[derive(Debug)]
pub struct Clock {
    pub hours: u32,
    pub minutes: u32,
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.minutes == other.minutes && self.hours == other.hours
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{:02}:{:02}", self.hours, self.minutes)
        // NOTE: use the `write!` as a sequential string builder.
        if self.hours < DECIMAL {
            write!(f, "0{}", self.hours)?;
        } else {
            write!(f, "{}", self.hours)?;
        };
        if self.minutes < DECIMAL {
            write!(f, ":0{}", self.minutes)
        } else {
            write!(f, ":{}", self.minutes)
        }
    }
}

impl Clock {
    pub fn new(mut hours: i32, mut minutes: i32) -> Self {
        hours %= 24;
        minutes %= HOURS * MINUTES;
        Self::from_minutes(hours * MINUTES + minutes)
    }

    pub fn from_minutes(mut i_minutes: i32) -> Self {
        if i_minutes < 0 {
            i_minutes += HOURS * MINUTES; // Already negative i_minutes
        }
        let minutes = (i_minutes % MINUTES) as u32;
        let hours = ((i_minutes / MINUTES) % HOURS) as u32;

        Self { hours, minutes }
    }

    pub fn to_minutes(&self) -> u32 {
        self.hours * 60 + self.minutes
    }

    pub fn add_minutes(&self, mut i_minutes: i32) -> Self {
        i_minutes %= HOURS * MINUTES;
        let self_minutes = self.to_minutes() as i32 + i_minutes;
        Self::from_minutes(self_minutes)
    }
}
