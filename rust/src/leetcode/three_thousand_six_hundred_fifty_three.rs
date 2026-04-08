pub struct Solution;

impl Solution {
    const MOD: i64 = 1_000_000_007;

    pub fn xor_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut nums = nums.iter().map(|&x| x as i64).collect::<Vec<_>>();

        for q in queries {
            let l = q[0] as usize;
            let r = q[1] as usize;
            let k = q[2] as usize;
            let v = q[3] as i64;

            let mut i = l;
            while i <= r {
                nums[i] = (nums[i] * v) % Self::MOD;
                i += k;
            }
        }

        let mut res = 0;
        for x in nums {
            res ^= x;
        }

        res as i32
    }
}
