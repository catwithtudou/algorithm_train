package leetcode

import (
	"container/heap"
	"math"
)

func smallestRange(nums [][]int) []int {
	h := make(hpRange, len(nums))
	r := math.MinInt
	for i, arr := range nums {
		h[i] = tuple{arr[0], i, 0}
		r = max(r, arr[0])
	}
	heap.Init(&h)

	ansL, ansR := h[0].x, r
	for h[0].j+1 < len(nums[h[0].i]) {
		x := nums[h[0].i][h[0].j+1]
		r = max(r, x)
		h[0].x = x
		h[0].j++
		heap.Fix(&h, 0)
		l := h[0].x
		if r-l < ansR-ansL {
			ansL, ansR = l, r
		}
	}
	return []int{ansL, ansR}
}

type tuple struct{ x, i, j int }
type hpRange []tuple

func (h hpRange) Len() int           { return len(h) }
func (h hpRange) Less(i, j int) bool { return h[i].x < h[j].x }
func (h hpRange) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (hpRange) Push(any)             {}
func (hpRange) Pop() (_ any)         { return }
