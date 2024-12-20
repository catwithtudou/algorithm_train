pub struct Solution;

impl Solution {
    pub fn min_anagram_length(s: String) -> i32 {
        let n = s.len();
        for k in 1..=n / 2 {
            if n % k > 0 {
                continue;
            }
            let mut cnt0 = vec![0; 26];
            for i in 0..k {
                cnt0[s.as_bytes()[i] as usize - 'a' as usize] += 1;
            }
            let mut flag = false;
            let mut i = 2 * k;
            while i <= n {
                let mut cnt1 = vec![0; 26];
                for j in i - k..i {
                    cnt1[s.as_bytes()[j] as usize - 'a' as usize] += 1;
                }
                if cnt0 != cnt1 {
                    flag = true;
                    break;
                }
                i += k;
            }
            if flag {
                continue;
            }
            return k as i32;
        }

        n as i32
    }
}
