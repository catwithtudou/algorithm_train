pub struct Solution;

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i64 {
        let mut cnt_3 = vec![0; nums.len()];
        let mut cnt_4 = 0 as i64;
        for l in 2..nums.len() {
            let mut cnt_2 = 0;
            for j in 0..l {
                if nums[j] < nums[l] {
                    cnt_4 += cnt_3[j] as i64;
                    cnt_2 += 1;
                } else {
                    cnt_3[j] += cnt_2;
                }
            }
        }

        cnt_4
    }
}
