pub struct Solution;

impl Solution {
    pub fn max_distinct_elements(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut pre = i32::MIN;
        nums.sort_unstable();
        for &x in nums.iter() {
            let min_num = (x + k).min((x - k).max(pre + 1));
            if min_num > pre {
                ans += 1;
                pre = min_num;
            }
        }
        ans
    }
}
