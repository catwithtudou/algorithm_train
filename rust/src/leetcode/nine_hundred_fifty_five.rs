pub struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let (n, m) = (strs.len(), strs[0].len());
        let mut ans = 0;
        let mut a = vec![String::new(); n];

        'next: for j in 0..m {
            for i in 0..n - 1 {
                let c1 = strs[i].as_bytes()[j] as char;
                let c2 = strs[i + 1].as_bytes()[j] as char;

                if (a[i].as_str(), c1) > (a[i + 1].as_str(), c2) {
                    ans += 1;
                    continue 'next;
                }
            }

            for i in 0..n {
                a[i].push(strs[i].as_bytes()[j] as char);
            }
        }

        ans
    }
}
