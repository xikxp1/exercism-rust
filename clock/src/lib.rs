use std::fmt::Display;

const MINUTES_IN_HOUR: i32 = 60;
const MINUTES_IN_DAY: i32 = 24 * 60;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut ovl_minutes = (hours * MINUTES_IN_HOUR + minutes) % MINUTES_IN_DAY;
        if ovl_minutes < 0 {
            ovl_minutes += MINUTES_IN_DAY;
        }
        let mut new_minutes = ovl_minutes % MINUTES_IN_HOUR;
        if new_minutes < 0 {
            new_minutes += MINUTES_IN_HOUR;
        }
        Clock {
            hours: ovl_minutes / MINUTES_IN_HOUR,
            minutes: new_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
