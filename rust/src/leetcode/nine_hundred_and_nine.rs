pub struct Solution;

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let mut vis = vec![false; n * n + 1];
        let mut q = vec![1];

        vis[1] = true;
        for step in 0.. {
            if q.is_empty() {
                break;
            }
            let tmp = q;
            q = vec![];
            for x in tmp {
                if x == n * n {
                    return step;
                }
                for y in x + 1..=(x + 6).min(n * n) {
                    let r = (y - 1) / n;
                    let mut c = (y - 1) % n;
                    if r % 2 > 0 {
                        c = n - 1 - c;
                    }
                    let mut nxt = board[n - 1 - r][c];
                    if nxt < 0 {
                        nxt = y as i32;
                    }
                    let nxt = nxt as usize;
                    if !vis[nxt] {
                        vis[nxt] = true;
                        q.push(nxt);
                    }
                }
            }
        }
        -1
    }
}
