pub struct Solution;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let k = k as usize;
        let n = 1usize << k;
        let mut has = vec![false; n];
        let mut cnt = 0usize;
        let mask = n -1;
        let mut x = 0usize;

        for (i,&ch) in s.as_bytes().iter().enumerate() {
            x = ((x<<1)&mask) | ((ch&1)as usize);

            if i < k-1 || has[x] {
                continue;
            }

            has[x]=true;
            cnt+=1;

            if cnt ==n  {
                return true;
            }
        }

        false
    }
}
