use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct Solution;

#[derive(Debug, Copy, Clone, PartialEq)]
struct ClassInfo {
    gain: f64,
    pass: i32,
    total: i32,
}

impl Eq for ClassInfo {}

impl PartialOrd for ClassInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.gain.partial_cmp(&other.gain)
    }
}

impl Ord for ClassInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let n = classes.len();
        let mut h: BinaryHeap<ClassInfo> = classes
            .into_iter()
            .map(|c| {
                let pass = c[0];
                let total = c[1];
                let gain = (total - pass) as f64 / (total as f64 * (total + 1) as f64);
                ClassInfo { gain, pass, total }
            })
            .collect();

        for _ in 0..extra_students {
            if let Some(mut top) = h.peek_mut() {
                top.pass += 1;
                top.total += 1;
                let pass = top.pass;
                let total = top.total;
                top.gain = (total - pass) as f64 / (total as f64 * (total + 1) as f64);
            }
        }

        h.into_iter()
            .map(|info| info.pass as f64 / info.total as f64)
            .sum::<f64>()
            / n as f64
    }
}
