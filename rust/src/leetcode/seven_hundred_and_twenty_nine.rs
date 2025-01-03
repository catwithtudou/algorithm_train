pub struct Solution;

use std::collections::BTreeMap;

struct MyCalendar {
    tm: BTreeMap<i32, i32>,
}

impl MyCalendar {
    fn new() -> Self {
        MyCalendar {
            tm: BTreeMap::new(),
        }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> bool {
        if let Some((&key, &val)) = self.tm.range(start_time + 1..).next() {
            if val < end_time {
                return false;
            }
        }
        self.tm.insert(start_time, end_time);
        return true;
    }
}
