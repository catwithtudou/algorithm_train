package leetcode

import (
	"container/heap"
	"math/bits"
)

type pairMinMax struct{ min, max int }

func op(a, b pairMinMax) pairMinMax {
	return pairMinMax{min(a.min, b.min), max(a.max, b.max)}
}

type ST [][16]pairMinMax

func newST(a []int) ST {
	n := len(a)
	w := bits.Len(uint(n))
	st := make(ST, n)
	for i, x := range a {
		st[i][0] = pairMinMax{x, x}
	}

	for j := 1; j < w; j++ {
		for i := range n - 1<<j + 1 {
			st[i][j] = op(st[i][j-1], st[i+1<<(j-1)][j-1])
		}
	}

	return st
}

func (st ST) query(l, r int) int {
	k := bits.Len(uint(r-l)) - 1
	p := op(st[l][k], st[r-1<<(k+1)][k])
	return p.max - p.min
}

func maxTotalValue3691(nums []int, k int) (ans int64) {

	n := len(nums)
	st := newST(nums)
	h := make(hpInfo, n)
	for i := range h {
		h[i] = tupleInfo{st.query(i, n), i, n}
	}

	for ; k > 0 && h[0].d > 0; k-- {
		ans += int64(h[0].d)
		h[0].r--
		h[0].d = st.query(h[0].l, h[0].r)
		heap.Fix(&h, 0)
	}

	return
}

type tupleInfo struct{ d, l, r int }
type hpInfo []tupleInfo

func (h hpInfo) Len() int           { return len(h) }
func (h hpInfo) Less(i, j int) bool { return h[i].d > h[j].d } // 最大堆
func (h hpInfo) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (hpInfo) Push(any)             {}
func (hpInfo) Pop() (_ any)         { return }
