pub struct Solution;

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        fn max_len(candidates: &Vec<i32>, k: i32) -> i32 {
            let mut res = 0;
            for &num in candidates.iter() {
                if num & (1 << k) != 0 {
                    res += 1;
                }
            }
            res
        }

        let mut res = 0;
        for i in 0..24 {
            // 遍历二进制位
            res = res.max(max_len(&candidates, i));
        }
        res
    }
}
