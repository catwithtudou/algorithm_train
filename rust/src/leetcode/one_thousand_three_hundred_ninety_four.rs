pub struct Solution;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut cnt = vec![0; n + 1];
        for x in arr {
            if x as usize <= n {
                cnt[x as usize] += 1;
            }
        }

        for i in (1..=n).rev() {
            if cnt[i] as usize == i {
                return i as _;
            }
        }
        -1
    }
}
