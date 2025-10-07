use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let n = rains.len();
        let mut fa = (0..=n).collect::<Vec<_>>();

        fn find(x: usize, fa: &mut Vec<usize>) -> usize {
            if fa[x] != x {
                fa[x] = find(fa[x], fa);
            }
            fa[x]
        }

        let mut ans = vec![-1; n];
        let mut full_day = HashMap::new();

        for (i, lake) in rains.into_iter().enumerate() {
            if lake == 0 {
                ans[i] = 1;
                continue;
            }

            if let Some(&j) = full_day.get(&lake) {
                let dry_day = find(j + 1, &mut fa);
                if dry_day >= i {
                    return vec![];
                }
                ans[dry_day] = lake;
                fa[dry_day] = find(dry_day + 1, &mut fa)
            }
            fa[i] = i + 1;
            full_day.insert(lake, i);
        }

        ans
    }
}
