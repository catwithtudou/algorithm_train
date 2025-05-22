package leetcode

import (
	"container/heap"
	"sort"

	"golang.org/x/exp/slices"
)

func maxRemoval(nums []int, queries [][]int) int {
	slices.SortFunc(queries, func(a, b []int) int { return a[0] - b[0] })
	h := RemovalHp{}
	diff := make([]int, len(nums)+1)
	sumD, j := 0, 0
	for i, x := range nums {
		sumD += diff[i]
		for ; j < len(queries) && queries[j][0] <= i; j++ {
			heap.Push(&h, queries[j][1])
		}
		for sumD < x && h.Len() > 0 && h.IntSlice[0] >= i {
			sumD++
			diff[heap.Pop(&h).(int)+1]--
		}
		if sumD < x {
			return -1
		}
	}

	return h.Len()
}

type RemovalHp struct{ sort.IntSlice }

func (h RemovalHp) Less(i, j int) bool  { return h.IntSlice[i] > h.IntSlice[j] }
func (h *RemovalHp) Push(v interface{}) { h.IntSlice = append(h.IntSlice, v.(int)) }
func (h *RemovalHp) Pop() interface{} {
	a := h.IntSlice
	v := a[len(a)-1]
	h.IntSlice = a[:len(a)-1]
	return v
}
