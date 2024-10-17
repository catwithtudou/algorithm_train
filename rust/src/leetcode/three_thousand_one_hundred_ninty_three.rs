pub struct Solution;

impl Solution {
    pub fn number_of_permutations(n: i32, requirements: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut req = vec![-1; n];
        let mut m: usize = 0;
        for r in requirements {
            req[r[0] as usize] = r[1];
            m = m.max(r[1] as usize);
        }
        if req[0] > 0 {
            return 0;
        }

        let mut f = vec![vec![0; m as usize + 1]; n];
        f[0][0] = 1;
        for i in 1..n {
            let (mut l, mut r) = (0 as usize, m);
            if req[i] >= 0 {
                l = req[i] as usize;
                r = req[i] as usize;
            }
            for j in l..=r {
                for k in 0..=i.min(j) {
                    f[i][j] = (f[i][j] + f[i - 1][j - k]) % 1000000007;
                }
            }
        }

        f[n - 1][req[n - 1] as usize]
    }
}
