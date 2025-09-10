pub struct Solution;

impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        let m = languages.len();
        let mut learned = vec![vec![false; (n + 1) as usize]; m];

        for (i, list) in languages.iter().enumerate() {
            for &x in list {
                learned[i][x as usize] = true;
            }
        }

        let mut total = 0;
        let mut vis = vec![false; m];
        let mut cnt = vec![0; (n + 1) as usize];

        let mut add = |u: usize, vis: &mut Vec<bool>, total: &mut i32, cnt: &mut Vec<i32>| {
            if vis[u] {
                return;
            }
            vis[u] = true; // 避免重复统计
            *total += 1;
            for &x in &languages[u] {
                cnt[x as usize] += 1;
            }
        };

        'next: for f in &friendships {
            let u = (f[0] - 1) as usize;
            let v = (f[1] - 1) as usize;

            for &x in &languages[u] {
                if learned[v][x as usize] {
                    // 两人可以相互沟通，无需学习语言
                    continue 'next;
                }
            }

            // 两人无法沟通，需要学习语言
            add(u, &mut vis, &mut total, &mut cnt);
            add(v, &mut vis, &mut total, &mut cnt);
        }

        // 返回总人数减去学习最热门语言的人数
        total - cnt.iter().max().unwrap_or(&0)
    }
}
