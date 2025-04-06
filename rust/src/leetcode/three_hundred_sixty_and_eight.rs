pub struct Solution;

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut from = vec![-1; n];
        let mut f = vec![0; n];
        nums.sort();

        let mut max_i = 0;
        for i in 0..n {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && f[j] > f[i] {
                    f[i] = f[j];
                    from[i] = j as i32;
                }
            }
            f[i] += 1;
            if f[i] > f[max_i] {
                max_i = i;
            }
        }

        let mut path =Vec::new();
        let mut i = max_i as i32;
        while i >= 0  {
            path.push(nums[i as usize]);
            i = from[i as usize];
        }

        path
    }
}
