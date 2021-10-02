use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock { total_minutes: i32 }

const MINUTES_IN_A_DAY: i32 = 60 * 24;

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = (self.total_minutes / 60) % 24;
        let minutes = self.total_minutes % 60;

        write!(f, "{:02}:{:02}", hours, minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;

        Clock {
            total_minutes: (total_minutes % MINUTES_IN_A_DAY + MINUTES_IN_A_DAY) % MINUTES_IN_A_DAY
        }
    }
    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        let total_minutes = self.total_minutes + minutes;

        Clock {
            total_minutes: (total_minutes % MINUTES_IN_A_DAY + MINUTES_IN_A_DAY) % MINUTES_IN_A_DAY
        }
    }
}
