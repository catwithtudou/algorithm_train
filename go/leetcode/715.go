package leetcode

import "github.com/emirpasic/gods/trees/redblacktree"

// 解法1：有序集合

type RangeModule struct {
	*redblacktree.Tree
}

func Constructor() RangeModule {
	return RangeModule{redblacktree.NewWithIntComparator()}
}

func (t *RangeModule) AddRange(left int, right int) {
	if node, ok := t.Floor(left); ok { // 找到 l_i <= left
		r := node.Value.(int)
		if r >= right {
			return
		}
		if r >= left { // 抛弃原区间，后续增加[left,right)
			left = node.Key.(int)
			t.Remove(left)
		}
	}

	// 遍历[l_i,r_i)以外的区间
	for node, ok := t.Ceiling(left); ok && node.Key.(int) <= right; node, ok = t.Ceiling(left) {
		right = max(right, node.Value.(int))
		t.Remove(node.Key)
	}
	t.Put(left, right)
}

func (t *RangeModule) QueryRange(left int, right int) bool {
	node, ok := t.Floor(left)
	return ok && node.Value.(int) >= right
}

func (t *RangeModule) RemoveRange(left int, right int) {
	if node, ok := t.Floor(left); ok {
		l, r := node.Key.(int), node.Value.(int)
		if r >= right {
			if l == left {
				t.Remove(l)
			} else {
				node.Value = left
			}
			if right != r {
				t.Put(right, r)
			}
			return
		}
		if r > left {
			if l == left {
				t.Remove(l)
			} else {
				node.Value = left
			}
		}
	}
	for node, ok := t.Ceiling(left); ok && node.Key.(int) < right; node, ok = t.Ceiling(left) {
		r := node.Value.(int)
		t.Remove(node.Key)
		if r > right {
			t.Put(right, r)
			break
		}
	}
}

// 解法2：线段树（动态开点）

const N int = 1e9

type node struct {
	lch, rch *node
	added    bool
	lazy     int
}

type segmentTree struct {
	root *node
}

func (t *segmentTree) update(n *node, l, r, i, j, x int) {
	if l >= i && r <= j {
		n.added = x == 1
		n.lazy = x
		return
	}
	t.pushdown(n)
	m := int(uint(l+r) >> 1)
	if i <= m {
		t.update(n.lch, l, m, i, j, x)
	}
	if j > m {
		t.update(n.rch, m+1, r, i, j, x)
	}
	t.pushup(n)
}

func (t *segmentTree) query(n *node, l, r, i, j int) bool {
	if l >= i && r <= j {
		return n.added
	}
	t.pushdown(n)
	v := true
	m := int(uint(l+r) >> 1)
	if i <= m {
		v = v && t.query(n.lch, l, m, i, j)
	}
	if j > m {
		v = v && t.query(n.rch, m+1, r, i, j)
	}
	return v
}

func (t *segmentTree) pushup(n *node) {
	n.added = n.lch.added && n.rch.added
}

func (t *segmentTree) pushdown(n *node) {
	if n.lch == nil {
		n.lch = new(node)
	}
	if n.rch == nil {
		n.rch = new(node)
	}
	if n.lazy != 0 {
		n.lch.added = n.lazy == 1
		n.rch.added = n.lazy == 1
		n.lch.lazy = n.lazy
		n.rch.lazy = n.lazy
		n.lazy = 0
	}
}

/**
 * Your RangeModule object will be instantiated and called as such:
 * obj := Constructor();
 * obj.AddRange(left,right);
 * param_2 := obj.QueryRange(left,right);
 * obj.RemoveRange(left,right);
 */
