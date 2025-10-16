pub struct Solution;

impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let mut cnt = vec![0; value as usize];
        for x in nums {
            cnt[((x % value + value) % value) as usize] += 1;
        }

        let mut i = 0;
        for j in 1..value as usize {
            if cnt[j] < cnt[i] {
                i = j;
            }
        }

        value * cnt[i] + i as i32
    }
}
