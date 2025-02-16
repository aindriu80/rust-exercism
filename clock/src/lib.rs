use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    fn get_hours_minutes(hours: i32, minutes: i32) -> Self {
        let _t = (hours * 60 + minutes).rem_euclid(24 * 60);
        let h = _t / 60;
        let m = _t % 60;

        Self {
            hours: h,
            minutes: m,
        }
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::get_hours_minutes(hours, minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::get_hours_minutes(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
