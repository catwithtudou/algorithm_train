pub struct Solution;

impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let (mut min, mut max) = (i32::MAX / 2, i32::MIN / 2);
        for a in arrays {
            let (x, y) = (a[0], a[a.len() - 1]);
            ans = ans.max(y - min).max(max - x);
            min = min.min(x);
            max = max.max(y);
        }
        ans
    }
}
