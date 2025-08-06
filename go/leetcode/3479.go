package leetcode

import "math/bits"

type segTree []int

func (t segTree) maintain(o int) {
	t[o] = max(t[o<<1], t[o<<1|1])
}

func (t segTree) build(a []int, o, l, r int) {
	if l == r {
		t[o] = a[l]
		return
	}
	m := (l + r) >> 1
	t.build(a, o<<1, l, m)
	t.build(a, o<<1|1, m+1, r)
	t.maintain(o)
}

func (t segTree) findFirstAndUpdate(o, l, r, x int) int {
	if t[o] < x {
		return -1
	}
	if l == r {
		t[o] = -1
		return l
	}
	m := (l + r) >> 1
	res := t.findFirstAndUpdate(o<<1, l, m, x)
	if res < 0 {
		res = t.findFirstAndUpdate(o<<1|1, m+1, r, x)
	}

	t.maintain(o)
	return res
}

func newSegmentTree(a []int) segTree {
	n := len(a)
	t := make(segTree, 2<<bits.Len(uint(n-1)))
	t.build(a, 1, 0, n-1)
	return t
}

func numOfUnplacedFruitsII(fruits []int, baskets []int) (ans int) {
	t := newSegmentTree(baskets)
	for _, x := range fruits {
		if t.findFirstAndUpdate(1, 0, len(baskets)-1, x) < 0 {
			ans++
		}
	}
	return
}
