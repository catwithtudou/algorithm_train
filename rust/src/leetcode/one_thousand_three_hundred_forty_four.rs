pub struct Solution;

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let d = ((hour * 30) as f64 - minutes as f64 * 5.5).abs();
        d.min(360.0 - d)
    }
}
