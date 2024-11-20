pub struct Solution;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nxt: Vec<i32> = (1..n - 1).collect();
        let mut ans = vec![0; queries.len()];
        let mut cnt = n - 1;
        for (i, q) in queries.iter().enumerate() {
            let (l, r) = (q[0] as usize, q[1] as usize);
            if nxt[l] > 0 && nxt[l] < r as i32 {
                let mut j = nxt[l] as usize;
                while j < r  {
                    cnt -= 1;
                    j = nxt[j] as usize;
                    nxt[j] = 0;
                }
                nxt[l] = r as i32;
            }
            ans[i] = cnt
        }
        ans
    }
}
