package leetcode

import (
	"container/heap"
	"sort"

	"golang.org/x/exp/slices"
)

func mostBooked(n int, meetings [][]int) (ans int) {
	slices.SortFunc(meetings, func(a, b []int) int { return a[0] - b[0] })

	idle := meetHp{make([]int, n)}

	for i := range n {
		idle.IntSlice[i] = i
	}

	using := meetHp2{}
	cnt := make([]int, n)

	for _, m := range meetings {
		start, end := m[0], m[1]

		for len(using) > 0 && using[0].end <= start {
			heap.Push(&idle, heap.Pop(&using).(meetPair).i)
		}

		var i int

		if idle.Len() > 0 {
			i = heap.Pop(&idle).(int)
		} else {
			p := heap.Pop(&using).(meetPair)
			i = p.i
			end += p.end - start
		}

		heap.Push(&using, meetPair{end, i})
		cnt[i]++
	}

	for i, c := range cnt {
		if c > cnt[ans] {
			ans = i
		}
	}

	return
}

type meetHp struct{ sort.IntSlice }

func (h *meetHp) Push(v any) { h.IntSlice = append(h.IntSlice, v.(int)) }
func (h *meetHp) Pop() any   { a := h.IntSlice; v := a[len(a)-1]; h.IntSlice = a[:len(a)-1]; return v }

type meetPair struct{ end, i int }

type meetHp2 []meetPair

func (h meetHp2) Len() int { return len(h) }
func (h meetHp2) Less(i, j int) bool {
	return h[i].end < h[j].end || h[i].end == h[j].end && h[i].i < h[j].i
}
func (h meetHp2) Swap(i, j int) { h[i], h[j] = h[j], h[i] }

func (h *meetHp2) Push(v any) { *h = append(*h, v.(meetPair)) }
func (h *meetHp2) Pop() any   { a := *h; v := a[len(a)-1]; *h = a[:len(a)-1]; return v }
