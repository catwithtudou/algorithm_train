pub struct Solution;

impl Solution {
    pub fn minimum_total_price(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>, trips: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for edge in &edges {
            graph[edge[0] as usize].push(edge[1] as usize);
            graph[edge[1] as usize].push(edge[0] as usize);
        }

        let mut cnt = vec![0; n];
        for trip in &trips {
            Self::road_dfs(trip[0] as usize, n, &mut cnt, &graph, trip[1] as usize);
        }

        let (not_cp, cp) = Self::price_dfs(0, 0, &price, &cnt, &graph);
        not_cp.min(cp)
    }

    fn road_dfs(x: usize, fa: usize, cnt: &mut Vec<i32>, graph: &Vec<Vec<usize>>, end: usize) -> bool {
        if x == end {
            cnt[x] += 1;
            return true;
        }
        for &y in &graph[x] {
            if y != fa && Self::road_dfs(y, x, cnt, graph, end) {
                cnt[x] += 1;
                return true;
            }
        }
        return false;
    }

    fn price_dfs(x: usize, fa: usize, price: &Vec<i32>, cnt: &Vec<i32>, graph: &Vec<Vec<usize>>) -> (i32, i32) {
        let mut not_cp = price[x] * cnt[x];
        let mut cp = not_cp / 2;
        for &y in &graph[x] {
            if y == fa {
                continue;
            }
            let (not_c, c) = Self::price_dfs(y, x, price, cnt, graph);
            not_cp += not_c.min(c);
            cp += not_c;
        }
        (not_cp, cp)
    }
}