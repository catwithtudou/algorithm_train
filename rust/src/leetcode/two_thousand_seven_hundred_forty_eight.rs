pub struct Solution;

impl Solution {
    pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
        let mut cnt = vec![0; 10];
        let mut ans: i32 = 0;
        let n = nums.len() as i32;
        for i in 0..n {
            for y in 1..10 {
                if cnt[y] > 0 && Self::gcd(nums[i as usize] % 10 , y as i32) == 1 {
                    ans += cnt[y];
                }
            }
            let mut temp = nums[i as usize];
            while temp >= 10 {
                temp /= 10;
            }
            cnt[temp as usize] += 1;
        }
        ans
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}
