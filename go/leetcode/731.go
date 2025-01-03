package leetcode

type segNode struct {
	left, right *segNode
	l, mid, r   int
	v, add      int
}

func newSegNode(l, r int) *segNode {
	return &segNode{
		l:   l,
		r:   r,
		mid: int(uint(l+r) >> 1),
	}
}

type segmentTree731 struct {
	root *segNode
}

func newSegmentTree731() *segmentTree731 {
	return &segmentTree731{
		root: newSegNode(1, 1e9+1),
	}
}

func (t *segmentTree731) pushup(n *segNode) {
	n.v = max(n.left.v, n.right.v)
}

func (t *segmentTree731) pushdown(n *segNode) {
	if n.left == nil {
		n.left = newSegNode(n.l, n.mid)
	}
	if n.right == nil {
		n.right = newSegNode(n.mid+1, n.r)
	}
	if n.add != 0 {
		n.left.add += n.add
		n.right.add += n.add
		n.left.v += n.add
		n.right.v += n.add
		n.add = 0
	}
}

func (t *segmentTree731) modify(l, r, v int, n *segNode) {
	if l > r {
		return
	}
	if n.l >= l && n.r <= r {
		n.v += v
		n.add += v
		return
	}
	t.pushdown(n)

	if l <= n.mid {
		t.modify(l, r, v, n.left)
	}
	if r > n.mid {
		t.modify(l, r, v, n.right)
	}
	t.pushup(n)
}

func (t *segmentTree731) query(l, r int, n *segNode) int {
	if l > r {
		return 0
	}

	if n.l >= l && n.r <= r {
		return n.v
	}
	t.pushdown(n)

	v := 0
	if l <= n.mid {
		v = max(v, t.query(l, r, n.left))
	}
	if r > n.mid {
		v = max(v, t.query(l, r, n.right))
	}

	return v
}

type MyCalendarTwo struct {
	tr *segmentTree731
}

func ConstructorMyCalendarTwo() MyCalendarTwo {
	return MyCalendarTwo{newSegmentTree731()}
}

func (this *MyCalendarTwo) Book(startTime int, endTime int) bool {
	if this.tr.query(startTime+1, endTime, this.tr.root) >= 2 {
		return false
	}
	this.tr.modify(startTime+1, endTime, 1, this.tr.root)
	return true
}

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * obj := Constructor();
 * param_1 := obj.Book(startTime,endTime);
 */
