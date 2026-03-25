pub struct Solution;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums.len();
        let mut has = vec![false; 1 << n];
        for s in nums {
            let num = u32::from_str_radix(&s, 2).unwrap();
            has[num as usize] = true;
        }

        let mut ans = 0;
        while has[ans] {
            ans += 1;
        }

        format!("{:0width$b}", ans, width = n)
    }
}