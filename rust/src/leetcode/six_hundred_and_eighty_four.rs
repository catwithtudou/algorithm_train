pub struct Solution;


impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut parent: Vec<_> = (0..=1000).collect();
        let mut ans = vec![];

        fn find(parent: &mut Vec<usize>, mut idx: usize) -> usize {
            while parent[idx] != idx {
                parent[idx] = parent[parent[idx]];
                idx = parent[idx];
            }
            idx
        }

        fn union(parent: &mut Vec<usize>, from: usize, to: usize) -> bool {
            let (from, to) = (find(parent, from), find(parent, to));
            if from == to {
                return false;
            }
            parent[from] = to;
            true
        }

        for edge in edges.iter() {
            if !union(&mut parent, edge[0] as usize, edge[1] as usize){
                ans = edge.clone();
                break;
            }
        }
        ans
    }
}
