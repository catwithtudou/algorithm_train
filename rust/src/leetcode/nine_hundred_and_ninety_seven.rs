pub struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut in_d = vec![0; n as usize + 1];
        let mut out_d = vec![0; n as usize + 1];
        for (_, edge) in trust.iter().enumerate() {
            in_d[edge[1] as usize] += 1;
            out_d[edge[0] as usize] += 1;
        }

        for i in 1..=n as usize {
            if in_d[i] == n - 1 && out_d[i] == 0 {
                return i as i32;
            }
        }

        -1
    }
}
