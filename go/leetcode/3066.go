package leetcode

import (
	"container/heap"
	"sort"
)

func minOperations3066(nums []int, k int) (ans int) {
	h := &hp3066{nums}
	heap.Init(h)
	for h.IntSlice[0] < k {
		x := heap.Pop(h).(int)
		h.IntSlice[0] += x * 2
		heap.Fix(h, 0)
		ans++
	}
	return
}

type hp3066 struct{ sort.IntSlice }

func (hp3066) Push(any)    {}
func (h *hp3066) Pop() any { a := h.IntSlice; v := a[len(a)-1]; h.IntSlice = a[:len(a)-1]; return v }
