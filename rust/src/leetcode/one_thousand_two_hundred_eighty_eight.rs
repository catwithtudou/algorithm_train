pub struct Solution;

impl Solution {
    pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by_key(|a| (a[0], -a[1]));

        let mut ans = 0;
        let mut max_right = 0;
        for i in intervals {
            if i[1] > max_right {
                max_right = i[1];
                ans += 1;
            }
        }
        ans
    }
}
