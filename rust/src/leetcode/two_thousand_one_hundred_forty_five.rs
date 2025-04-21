pub struct Solution;

impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut s = 0;
        let (mut min_s,mut max_s) = (0,0);
        for &d in differences.iter() {
            s += d as i64 ;
            min_s = min_s.min(s);
            max_s = max_s.max(s);
        }
        0.max((upper-lower+1) as i64 -max_s+min_s) as _
    }
}