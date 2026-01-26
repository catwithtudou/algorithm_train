pub struct Solution;

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut min_diff = i32::MAX;
        arr.sort_unstable();
        for i in 1..arr.len() {
            let diff = arr[i] - arr[i - 1];
            if diff < min_diff {
                min_diff = diff;
                ans = vec![vec![arr[i - 1], arr[i]]];
            } else if diff == min_diff {
                ans.push(vec![arr[i - 1], arr[i]]);
            }
        }
        ans
    }
}
