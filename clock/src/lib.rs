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
        let ovl_minutes = (hours * MINUTES_IN_HOUR + minutes).rem_euclid(MINUTES_IN_DAY);
        Clock {
            hours: ovl_minutes.div_euclid(MINUTES_IN_HOUR),
            minutes: ovl_minutes.rem_euclid(MINUTES_IN_HOUR),
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
