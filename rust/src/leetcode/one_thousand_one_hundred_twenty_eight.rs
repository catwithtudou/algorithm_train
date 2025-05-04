pub struct Solution;

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;

        let mut cnt = vec![vec![0; 10]; 10];

        for d in dominoes {
            let (mut a, mut b) = (d[0] as usize, d[1] as usize);
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            ans += cnt[a][b];
            cnt[a][b] += 1;
        }

        ans
    }
}
