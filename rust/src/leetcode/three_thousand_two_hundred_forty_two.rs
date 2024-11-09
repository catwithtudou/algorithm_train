pub struct Solution;

struct NeighborSum (Vec<[i32; 2]>);

const DIRS_NEIGHBOR_SUM: [[i32; 2]; 8] = [
    [-1, 0],
    [1, 0],
    [0, -1],
    [0, 1],
    [1, 1],
    [-1, 1],
    [-1, -1],
    [1, -1],
];



/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NeighborSum {

    fn new(grid: Vec<Vec<i32>>) -> Self {
        let n = grid.len();
        let mut s = vec![[0; 2]; n * n];
        for i in 0..n {
            for j in 0..grid[i].len() {
                let v = grid[i][j] as usize;
                for k in 0..DIRS_NEIGHBOR_SUM.len() {
                    let x = i as i32 + DIRS_NEIGHBOR_SUM[k][0];
                    let y = j as i32 + DIRS_NEIGHBOR_SUM[k][1];
                    if 0 <= x && x < n as i32 && 0 <= y && y < grid[i].len() as i32 {
                        s[v][k / 4] += grid[x as usize][y as usize];
                    }
                }
            }
        }
        NeighborSum(s)
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        self.0[value as usize][0]

    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        self.0[value as usize][1]
    }
}

