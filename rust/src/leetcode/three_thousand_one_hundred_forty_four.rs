pub struct Solution;

impl Solution {
    pub fn minimum_substrings_in_partition(s: String) -> i32 {
        let n = s.len();
        let mut f = vec![0; n + 1];
        for i in 0..n {
            f[i + 1] = i32::MAX;
            let mut cnt = [0; 26];
            let (mut k, mut max_cnt) = (0, 0);
            for j in (0..=i).rev() {
                let b = (s.as_bytes()[j] - b'a') as usize;
                if cnt[b] == 0 {
                    k += 1;
                }
                cnt[b] += 1;
                max_cnt=max_cnt.max(cnt[b]);
                if (i-j+1) as i32 == k *max_cnt {
                    f[i+1]=f[i+1].min(f[j]+1);
                }
            }
        }
        f[n]
    }
}