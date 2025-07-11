pub struct Solution;

impl Solution {
    pub fn count_days(mut days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut start = 1;
        let mut end = 0;

        for p in meetings {
            if p[0] > end {
                days -= end - start + 1;
                start = p[0];
            }
            end = end.max(p[1]);
        }
        days - (end - start + 1)
    }
}
