pub struct Solution;

impl Solution {
    pub fn result_array(
        nums: Vec<i32>,
        k: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        #[derive(Clone, Copy, Default)]
        struct Data {
            // 整个区间的乘积 mod k
            product: usize,

            // cnt[r]:
            // 区间中前缀乘积 mod k == r 的前缀个数
            cnt: [i32; 5],
        }

        impl Data {
            fn new(value: i32, k: usize) -> Self {
                let product = value.rem_euclid(k as i32) as usize;
                let mut cnt = [0; 5];
                cnt[product] = 1;

                Self { product, cnt }
            }

            fn merge(left: Self, right: Self, k: usize) -> Self {
                let mut cnt = left.cnt;

                for r in 0..k {
                    cnt[left.product * r % k] += right.cnt[r];
                }

                Self {
                    product: left.product * right.product % k,
                    cnt,
                }
            }
        }

        struct SegmentTree {
            tree: Vec<Data>,
            k: usize,
        }

        impl SegmentTree {
            fn new(nums: &[i32], k: usize) -> Self {
                let n = nums.len();
                let mut seg = Self {
                    tree: vec![Data::default(); n * 4],
                    k,
                };

                seg.build(nums, 1, 0, n - 1);
                seg
            }

            fn build(&mut self, nums: &[i32], node: usize, left: usize, right: usize) {
                if left == right {
                    self.tree[node] = Data::new(nums[left], self.k);
                    return;
                }

                let mid = (left + right) / 2;
                self.build(nums, node * 2, left, mid);
                self.build(nums, node * 2 + 1, mid + 1, right);
                self.pull(node);
            }

            fn pull(&mut self, node: usize) {
                self.tree[node] = Data::merge(
                    self.tree[node * 2],
                    self.tree[node * 2 + 1],
                    self.k,
                );
            }

            fn update(
                &mut self,
                node: usize,
                left: usize,
                right: usize,
                index: usize,
                value: i32,
            ) {
                if left == right {
                    self.tree[node] = Data::new(value, self.k);
                    return;
                }

                let mid = (left + right) / 2;

                if index <= mid {
                    self.update(node * 2, left, mid, index, value);
                } else {
                    self.update(node * 2 + 1, mid + 1, right, index, value);
                }

                self.pull(node);
            }

            fn query(
                &self,
                node: usize,
                left: usize,
                right: usize,
                query_left: usize,
                query_right: usize,
            ) -> Data {
                if query_left <= left && right <= query_right {
                    return self.tree[node];
                }

                let mid = (left + right) / 2;

                if query_right <= mid {
                    return self.query(
                        node * 2,
                        left,
                        mid,
                        query_left,
                        query_right,
                    );
                }

                if query_left > mid {
                    return self.query(
                        node * 2 + 1,
                        mid + 1,
                        right,
                        query_left,
                        query_right,
                    );
                }

                let left_data = self.query(
                    node * 2,
                    left,
                    mid,
                    query_left,
                    query_right,
                );

                let right_data = self.query(
                    node * 2 + 1,
                    mid + 1,
                    right,
                    query_left,
                    query_right,
                );

                Data::merge(left_data, right_data, self.k)
            }
        }

        let n = nums.len();
        let k = k as usize;
        let mut seg = SegmentTree::new(&nums, k);
        let mut ans = Vec::with_capacity(queries.len());

        for query in queries {
            let index = query[0] as usize;
            let value = query[1];
            let start = query[2] as usize;
            let x = query[3] as usize;

            seg.update(1, 0, n - 1, index, value);

            let data = seg.query(1, 0, n - 1, start, n - 1);
            ans.push(data.cnt[x]);
        }

        ans
    }
}