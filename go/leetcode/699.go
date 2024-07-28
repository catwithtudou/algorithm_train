package leetcode

type squaresNode struct {
	left      *squaresNode
	right     *squaresNode
	l, mid, r int
	v, add    int
}

func newSquaresNode(l, r int) *squaresNode {
	return &squaresNode{
		l:   l,
		r:   r,
		mid: (l + r) >> 1,
	}
}

type segmentSquaresTree struct {
	root *squaresNode
}

func newSegmentSquaresTree() *segmentSquaresTree {
	return &segmentSquaresTree{
		root: newSquaresNode(1, 1e9),
	}
}

func (s *segmentSquaresTree) modify(l, r, v int, n *squaresNode) {
	if l > r {
		return
	}
	if n.l >= l && n.r <= r {
		n.v = v
		n.add = v
		return
	}

	s.pushdown(n)
	if l <= n.mid {
		s.modify(l, r, v, n.left)
	}
	if r > n.mid {
		s.modify(l, r, v, n.right)
	}
	s.pushup(n)
}

func (s *segmentSquaresTree) query(l, r int, n *squaresNode) int {
	if l > r {
		return 0
	}
	if n.l >= l && n.r <= r {
		return n.v
	}
	s.pushdown(n)
	v := 0
	if l <= n.mid {
		v = max(v, s.query(l, r, n.left))
	}
	if r > n.mid {
		v = max(v, s.query(l, r, n.right))
	}
	return v
}

func (s *segmentSquaresTree) pushup(n *squaresNode) {
	n.v = max(n.left.v, n.right.v)
}

func (s *segmentSquaresTree) pushdown(n *squaresNode) {
	if n.left == nil {
		n.left = newSquaresNode(n.l, n.mid)
	}
	if n.right == nil {
		n.right = newSquaresNode(n.mid+1, n.r)
	}

	if n.add != 0 {
		n.left.add = n.add
		n.right.add = n.add
		n.left.v = n.add
		n.right.v = n.add
		n.add = 0
	}
}

func fallingSquares(positions [][]int) []int {
	ans := make([]int, len(positions))
	s := newSegmentSquaresTree()
	mx := 0
	for i, p := range positions {
		l, w, r := p[0], p[1], p[0]+p[1]-1
		h := s.query(l, r, s.root) + w
		mx = max(mx, h)
		ans[i] = mx
		s.modify(l, r, h, s.root)
	}
	return ans
}
