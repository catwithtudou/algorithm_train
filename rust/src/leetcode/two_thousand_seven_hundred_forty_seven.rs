pub struct Solution;

impl Solution {
    pub fn count_servers(
        n: i32,
        mut logs: Vec<Vec<i32>>,
        x: i32,
        mut queries: Vec<i32>,
    ) -> Vec<i32> {
        logs.sort_unstable_by_key(|p| p[1]);
        let mut idx = (0..queries.len()).collect::<Vec<_>>();
        idx.sort_unstable_by_key(|&i| queries[i]);
        let mut cnt = vec![0; n as usize + 1];
        let mut not_r = n ;
        let mut l = 0;
        let mut r = 0;
        for qid in idx {
            let q = queries[qid];
            while r < logs.len() && logs[r][1] <= q {
                let i = logs[r][0] as usize;
                if cnt[i] == 0 {
                    not_r -= 1;
                }
                cnt[i] += 1;
                r += 1;
            }
            while l < logs.len() && logs[l][1] < q - x {
                let i = logs[l][0] as usize;
                cnt[i] -= 1;
                if cnt[i] == 0 {
                    not_r += 1;
                }
                l += 1;
            }
            queries[qid] = not_r;
        }
        queries
    }
}
