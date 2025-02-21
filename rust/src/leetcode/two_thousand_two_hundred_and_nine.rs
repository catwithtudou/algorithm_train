pub struct Solution;

impl Solution {
    pub fn minimum_white_tiles(floor: String, num_carpets: i32, carpet_len: i32) -> i32 {
        let n = floor.len();
        let num_carpets = num_carpets as usize;
        let carpet_len = carpet_len as usize;
        let floor = floor.as_bytes();

        let mut f = vec![vec![0; n]; num_carpets + 1];

        f[0][0] = (floor[0] - b'0') as i32;
        for j in 1..n {
            f[0][j] = f[0][j - 1] + (floor[j] - b'0') as i32;
        }

        for i in 1..=num_carpets  {
            for j in (i * carpet_len)..n {
                f[i][j] =
                    f[i - 1][j - carpet_len].min(f[i][j - 1] + (floor[j] - b'0') as i32);
            }
        }

        f[num_carpets][n - 1]
    }
}
