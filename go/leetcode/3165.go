package leetcode

import "math"

const MOD = 1000000007

func maximumSumSubsequence(nums []int, queries [][]int) int {
	n := len(nums)
	tree := NewSegTree(n)
	tree.Init(nums)

	ans := int64(0)
	for _, q := range queries {
		tree.Update(q[0], q[1])
		ans = (ans + tree.Query()) % MOD
	}
	return int(ans)
}

type SegNode struct {
	v00, v01, v11, v10 int64
}

func NewSegNode() *SegNode {
	return &SegNode{0, 0, 0, 0}
}

func (sn *SegNode) Set(v int64) {
	sn.v00, sn.v01, sn.v10 = 0, 0, 0
	sn.v11 = int64(math.Max(float64(v), 0))
}

func (sn *SegNode) Best() int64 {
	return sn.v11
}

type SegTree struct {
	n    int
	tree []*SegNode
}

func NewSegTree(n int) *SegTree {
	tree := make([]*SegNode, 4*n+1)
	for i := range tree {
		tree[i] = NewSegNode()
	}
	return &SegTree{n, tree}
}

func (s *SegTree) Init(nums []int) {
	s.internalInit(nums, 1, 1, s.n)
}

func (s *SegTree) internalInit(nums []int, x, l, r int) {
	if l == r {
		s.tree[x].Set(int64(nums[l-1]))
		return
	}
	mid := (l + r) / 2
	s.internalInit(nums, x*2, l, mid)
	s.internalInit(nums, x*2+1, mid+1, r)
	s.pushup(x)
}

func (s *SegTree) Update(x, v int) {
	s.internalUpdate(1, 1, s.n, x+1, int64(v))
}

func (s *SegTree) Query() int64 {
	return s.tree[1].Best()
}

func (s *SegTree) internalUpdate(x, l, r int, pos int, v int64) {
	if l > pos || r < pos {
		return
	}
	if l == r {
		s.tree[x].Set(v)
		return
	}
	mid := (l + r) / 2
	s.internalUpdate(x*2, l, mid, pos, v)
	s.internalUpdate(x*2+1, mid+1, r, pos, v)
	s.pushup(x)
}

func (st *SegTree) pushup(x int) {
	l, r := x*2, x*2+1
	st.tree[x].v00 = maxInt64(st.tree[l].v00+st.tree[r].v10, st.tree[l].v01+st.tree[r].v00)
	st.tree[x].v01 = maxInt64(st.tree[l].v00+st.tree[r].v11, st.tree[l].v01+st.tree[r].v01)
	st.tree[x].v10 = maxInt64(st.tree[l].v10+st.tree[r].v10, st.tree[l].v11+st.tree[r].v00)
	st.tree[x].v11 = maxInt64(st.tree[l].v10+st.tree[r].v11, st.tree[l].v11+st.tree[r].v01)
}
