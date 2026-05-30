package leetcode

import (
	"math/bits"

	"github.com/emirpasic/gods/v2/trees/redblacktree"
)

type segRe []int

func (t segRe) update(o, l, r, i, val int) {
	if l == r {
		t[o] = val
		return
	}
	mid := (l + r) >> 1
	if i <= mid {
		t.update(o<<1, l, mid, i, val)
	} else {
		t.update(o<<1|1, mid+1, r, i, val)
	}
	t[o] = max(t[o<<1], t[o<<1|1])
}

func (t segRe) query(o, l, r, R int) int {
	if r <= R {
		return t[o]
	}

	mid := (l + r) >> 1

	if R <= mid {
		return t.query(o<<1, l, mid, R)
	}

	return max(t[o<<1], t.query(o<<1|1, mid+1, r, R))
}

func getResults(queries [][]int) (ans []bool) {

	m := 0

	for _, q := range queries {
		m = max(m, q[1])
	}

	m++

	set := redblacktree.New[int, struct{}]()

	set.Put(0, struct{}{})
	set.Put(m, struct{}{})
	t := make(segRe, 2<<bits.Len(uint(m)))

	for _, q := range queries {
		x := q[1]
		pre, _ := set.Floor(x - 1)
		if q[0] == 1 {
			nxt, _ := set.Ceiling(x)
			set.Put(x, struct{}{})
			t.update(1, 0, m, x, x-pre.Key)
			t.update(1, 0, m, nxt.Key, nxt.Key-x)
		} else {
			maxGap := max(t.query(1, 0, m, pre.Key), x-pre.Key)
			ans = append(ans, maxGap >= q[2])
		}
	}

	return
}
