package leetcode

import (
	"container/heap"
	"sort"
)

func maxSpending(values [][]int) (ans int64) {
	m, n := len(values), len(values[0])
	idx := make([]int, m)
	for i := range idx {
		idx[i] = i
	}
	h := &spHp{idx, values}
	heap.Init(h)

	for d := 1; d <= m*n; d++ {
		a := values[idx[0]]
		ans += int64(a[len(a)-1]) * int64(d)
		if len(a) > 1 {
			values[idx[0]] = a[:len(a)-1]
			heap.Fix(h, 0)
		} else {
			heap.Pop(h)
		}
	}

	return
}

type spHp struct {
	sort.IntSlice
	values [][]int
}

func (h spHp) Less(i, j int) bool {
	a, b := h.values[h.IntSlice[i]], h.values[h.IntSlice[j]]
	return a[len(a)-1] < b[len(b)-1]
}
func (spHp) Push(any)        {}
func (h *spHp) Pop() (_ any) { a := h.IntSlice; h.IntSlice = a[:len(a)-1]; return }
