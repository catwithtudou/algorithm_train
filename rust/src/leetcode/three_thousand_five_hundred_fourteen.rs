pub struct Solution;

impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let nums: Vec<usize> = nums.into_iter().map(|x| x as usize).collect();

        let max_value = nums.iter().copied().max().unwrap();

        // 等价于 Go 的 1 << bits.Len(maxValue)
        let size = (max_value + 1).next_power_of_two();

        // has[v] 表示是否存在两个数，其异或结果为 v
        let mut has = vec![false; size];

        for (i, &x) in nums.iter().enumerate() {
            for &y in &nums[i..] {
                has[x ^ y] = true;
            }
        }

        // has3[v] 表示是否存在三个数，其异或结果为 v
        let mut has3 = vec![false; size];

        for (xy, &exists) in has.iter().enumerate() {
            if !exists {
                continue;
            }

            for &z in &nums {
                has3[xy ^ z] = true;
            }
        }

        has3.into_iter().filter(|&exists| exists).count() as i32
    }
}