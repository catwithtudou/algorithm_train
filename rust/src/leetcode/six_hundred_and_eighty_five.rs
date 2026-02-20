pub struct Solution;

impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();

        // 实现并查集结构体
        struct ConUnionFind {
            anc: Vec<i32>,
        }

        impl ConUnionFind {
            fn new(n: usize) -> ConUnionFind {
                let anc: Vec<i32> = (0..n).map(|x| x as i32).collect();
                ConUnionFind { anc }
            }

            fn find(&mut self, x: i32) -> i32 {
                if self.anc[x as usize] != x {
                    self.anc[x as usize] = self.find(self.anc[x as usize]);
                }
                self.anc[x as usize]
            }

            fn union(&mut self, from: i32, to: i32) {
                let idx = self.find(from) as usize;
                self.anc[idx] = self.find(to);
            }
        }

        let mut uf = ConUnionFind::new(n + 1);
        let mut parent: Vec<_> = (0..=1000).collect();

        let mut confict_edge: Option<Vec<i32>> = None;
        let mut cycle_edge: Option<Vec<i32>> = None;

        for edge in edges {
            let from = edge[0];
            let to = edge[1];

            if parent[to as usize] != to {
                confict_edge = Some(edge.clone());
            } else {
                parent[to as usize] = from;
                if uf.find(to) == uf.find(from) {
                    cycle_edge = Some(edge.clone());
                } else {
                    uf.union(from, to);
                }
            }
        }

        if confict_edge.is_none() {
            cycle_edge.unwrap_or_else(|| vec![0])
        } else if cycle_edge.is_some() {
            let confict_edge_vec = confict_edge.as_ref().unwrap();
            vec![parent[confict_edge_vec[1] as usize], confict_edge_vec[1]]
        } else {
            confict_edge.unwrap()
        }
    }
}
