pub struct Solution;

use std::collections::HashMap;
use std::sync::OnceLock;

const JUMP_MAX: usize = 1_000_001;

static PRIME_FACTORS: OnceLock<Vec<Vec<usize>>> = OnceLock::new();

fn init_min_jumps() -> &'static Vec<Vec<usize>> {
    PRIME_FACTORS.get_or_init(|| {
        let mut prime_factors = vec![Vec::<usize>::new(); JUMP_MAX];

        for i in 2..JUMP_MAX {
            if prime_factors[i].is_empty() {
                let mut j = i;
                while j < JUMP_MAX {
                    prime_factors[j].push(i);
                    j += i;
                }
            }
        }

        prime_factors
    })
}

impl Solution {
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }

        let prime_factors = init_min_jumps();

        let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();

        for (i, &x) in nums.iter().enumerate() {
            let x = x as usize;
            for &p in &prime_factors[x] {
                groups.entry(p).or_default().push(i);
            }
        }

        let mut vis = vec![false; n];
        vis[0] = true;

        let mut q = vec![0usize];
        let mut ans = 0;

        loop {
            let tmp = std::mem::take(&mut q);

            for i in tmp {
                if i == n - 1 {
                    return ans;
                }

                let key = nums[i] as usize;

                let mut idx = groups.remove(&key).unwrap_or_default();

                if i + 1 < n {
                    idx.push(i + 1);
                }

                if i > 0 {
                    idx.push(i - 1);
                }

                for j in idx {
                    if !vis[j] {
                        vis[j] = true;
                        q.push(j);
                    }
                }
            }

            ans += 1;
        }
    }
}