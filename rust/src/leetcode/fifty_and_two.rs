pub struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        fn dfs(
            r: usize,
            queens: &mut [usize],
            col: &mut [bool],
            diag1: &mut [bool],
            diag2: &mut [bool],
            ans: &mut i32,
        ) {
            let n = col.len();
            if r == n {
                *ans += 1;
                return;
            }
            for c in 0..n {
                let rc = r - c + n - 1;
                if !col[c] && !diag1[r + c] && !diag2[rc] {
                    queens[r] = c;
                    col[c] = true;
                    diag1[r + c] = true;
                    diag2[rc] = true;
                    dfs(r + 1, queens, col, diag1, diag2, ans);
                    col[c] = false;
                    diag1[r + c] = false;
                    diag2[rc] = false;
                }
            }
        }
        let n = n as usize;
        let mut ans = 0;
        let mut queens = vec![0; n];
        let mut col = vec![false; n];
        let (mut diag1, mut diag2) = (vec![false; n * 2 - 1], vec![false; n * 2 - 1]);
        dfs(0, &mut queens, &mut col, &mut diag1, &mut diag2, &mut ans);
        ans
    }
}
