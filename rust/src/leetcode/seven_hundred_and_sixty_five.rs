struct UnionFind {
    parent: Vec<i32>,
    pub count: i32,
}

impl UnionFind {
    fn new(n: i32) -> Self {
        UnionFind {
            count: n,
            parent: (0..n).map(|idx| idx).collect(),
        }
    }

    fn find(&mut self, x: i32) -> i32 {
        if x == self.parent[x as usize] {
            return x;
        }
        self.parent[x as usize] = self.find(self.parent[x as usize]);
        self.parent[x as usize]
    }

    fn union(&mut self, x: i32, y: i32) {
        let  fx = self.find(x) as usize;
        let  fy = self.find(y) as usize;
        if fx != fy {
            self.parent[fx] = fy as i32;
            self.count -= 1;
        }
    }
}


pub struct Solution;

impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        let  len: i32 = row.len() as i32;
        let pair: i32 = len / 2;
        let mut uf = UnionFind::new(pair);
        for i in (0..len).filter(|x| x % 2 == 0) {
            uf.union(row[i as usize] / 2, row[(i + 1) as usize] / 2)
        }
        pair - uf.count
    }
}

#[cfg(test)]
mod seven_hundred_and_sixty_five_test {
    use super::*;

    #[test]
    fn min_swaps_couples() {
        assert_eq!(Solution::min_swaps_couples(vec![0,2,1,3]), 1);
    }
}