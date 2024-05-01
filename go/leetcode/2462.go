package leetcode

import (
	"container/heap"
	"sort"

	"golang.org/x/exp/slices"
)

func totalCost(costs []int, k int, candidates int) int64 {
	var ans int64
	n := len(costs)
	if candidates*2+k > n {
		slices.Sort(costs)
		for _, v := range costs[:k] {
			ans += int64(v)
		}
		return ans
	}

	leftH := minHp{costs[:candidates]}
	rightH := minHp{costs[n-candidates:]}
	heap.Init(&leftH)
	heap.Init(&rightH)
	for i, j := candidates, n-candidates-1; k > 0; k-- {
		if leftH.IntSlice[0] <= rightH.IntSlice[0] {
			ans += int64(leftH.replace(costs[i]))
			i++
		} else {
			ans += int64(rightH.replace(costs[j]))
			j--
		}
	}

	return ans
}

type minHp struct{ sort.IntSlice }

func (h *minHp) Push(v any) { h.IntSlice = append(h.IntSlice, v.(int)) }
func (h *minHp) Pop() any   { a := h.IntSlice; v := a[len(a)-1]; h.IntSlice = a[:len(a)-1]; return v }
func (h *minHp) replace(v int) int {
	top := h.IntSlice[0]
	h.IntSlice[0] = v
	heap.Fix(h, 0)
	return top
}
