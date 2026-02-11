use std::fmt::{Display, write};

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
        if self.hours < 10 {
            write!(f, "0{}", self.hours)?;
        } else {
            write!(f, "{}", self.hours)?;
        };
        if self.minutes < 10 {
            write!(f, ":0{}", self.minutes)
        } else {
            write!(f, ":{}", self.minutes)
        }
    }
}

impl Clock {
    pub fn new(hours: i32, i_minutes: i32) -> Self {
        let minutes = i_minutes as u32 % 60;
        // let overflow = if i_minutes < 0 { -1 } else { 0 };

        // let hours = (i_minutes / 60) + hours + overflow;
        let hours = (i_minutes / 60) + hours;
        let hours = hours as u32 % 24;

        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, i_minutes: i32) -> Self {
        dbg!(i_minutes);
        let mut hours_offset: i32 = 0;
        let mut minutes = self.minutes as i32 + (i_minutes % 60);
        dbg!(minutes);
        if minutes < 0 {
            minutes += 60;
            hours_offset += -1;
        }

        dbg!(minutes);
        dbg!(self);
        let hours = self.hours as i32 + (i_minutes / 60);
        dbg!(hours);
        let hours = hours % 24;
        dbg!(hours);

        let hours = (hours + hours_offset) as u32 % 24;
        let minutes = minutes as u32 % 60;
        dbg!(minutes);

        Clock { hours, minutes }
    }
}
