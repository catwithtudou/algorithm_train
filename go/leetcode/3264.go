package leetcode

import (
	"container/heap"
	"sort"
)

func getFinalState(nums []int, k int, multiplier int) []int {
	if multiplier == 1 {
		return nums
	}

	n := len(nums)
	h := make(sHp, n)
	mx := 0
	for i, x := range nums {
		h[i] = sPair{x, i}
		mx = max(mx, x)
	}
	heap.Init(&h)

	for ; k > 0 && h[0].x < mx; k-- {
		h[0].x *= multiplier
		heap.Fix(&h, 0)
	}

	sort.Slice(h, func(i, j int) bool {
		return less(h[i], h[j])
	})
	for i, p := range h {
		e := k / n
		if i < k%n {
			e++
		}
		nums[p.i] = p.x % mod * calQuickPow(multiplier, e) % mod
	}

	return nums
}

type sPair struct{ x, i int }

func less(a, b sPair) bool { return a.x < b.x || a.x == b.x && a.i < b.i }

type sHp []sPair

func (s sHp) Len() int           { return len(s) }
func (s sHp) Less(i, j int) bool { return less(s[i], s[j]) }
func (s sHp) Swap(i, j int)      { s[i], s[j] = s[j], s[i] }
func (s *sHp) Push(v any)        {}
func (s *sHp) Pop() (_ any)      { return }

func calQuickPow(x, n int) int {
	res := 1
	for ; n > 0; n /= 2 {
		if n%2 > 0 {
			res = res * x % mod
		}
		x = x * x % mod
	}
	return res
}
