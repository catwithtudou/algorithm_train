package leetcode

import (
	"container/heap"
	"sort"
)

func maxEvents(events [][]int) (ans int) {
	mx := 0
	for _, v := range events {
		mx = max(mx, v[1])
	}

	group := make([][]int, mx+1)
	for _, v := range events {
		group[v[0]] = append(group[v[0]], v[1])
	}

	h := &maxEventsHp{}

	for i, g := range group {
		for h.Len() > 0 && h.IntSlice[0] < i {
			heap.Pop(h)
		}

		for _, end := range g {
			heap.Push(h, end)
		}

		if h.Len() > 0 {
			ans++
			heap.Pop(h)
		}
	}
	return
}

type maxEventsHp struct{ sort.IntSlice }

func (h *maxEventsHp) Push(v any) { h.IntSlice = append(h.IntSlice, v.(int)) }
func (h *maxEventsHp) Pop() any {
	v := h.IntSlice[len(h.IntSlice)-1]
	h.IntSlice = h.IntSlice[:len(h.IntSlice)-1]
	return v
}
