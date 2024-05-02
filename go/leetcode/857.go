package leetcode

import (
	"container/heap"
	"sort"

	"golang.org/x/exp/slices"
)

func mincostToHireWorkers(quality []int, wage []int, k int) float64 {
	type pair struct {
		q, w int
	}
	pairs := make([]pair, len(quality))
	for i, q := range quality {
		pairs[i] = pair{q, wage[i]}
	}
	slices.SortFunc(pairs, func(a, b pair) int {
		return a.w*b.q - a.q*b.w
	})

	h := maxHeap{make([]int, k)}
	sumQ := 0
	for i, p := range pairs[:k] {
		h.IntSlice[i] = p.q
		sumQ += p.q
	}
	heap.Init(&h)

	ans := float64(sumQ*pairs[k-1].w) / float64(pairs[k-1].q)

	for _, p := range pairs[k:] {
		if p.q < h.IntSlice[0] {
			sumQ -= h.IntSlice[0] - p.q
			h.IntSlice[0] = p.q
			heap.Fix(&h, 0)
			ans = minFloat64(ans, float64(sumQ*p.w)/float64(p.q))
		}
	}

	return ans
}

type maxHeap struct{ sort.IntSlice }

func (h maxHeap) Less(i, j int) bool {
	return h.IntSlice[i] > h.IntSlice[j]
}
func (h maxHeap) Push(any)     {}
func (h maxHeap) Pop() (_ any) { return }

func minFloat64(a, b float64) float64 {
	if a < b {
		return a
	}
	return b
}
