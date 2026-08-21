package leetcode

import (
	"math/bits"
	"slices"
	"sort"
)

func findKthSmallest(coins []int, k int) int64 {
	subsetLcm := make([]int, 1<<len(coins))
	subsetLcm[0] = 1
	for i, x := range coins {
		bit := 1 << i
		for mask, l := range subsetLcm[:bit] {
			subsetLcm[bit|mask] = lcm(l, x)
		}
	}

	ans := sort.Search(slices.Min(coins)*k, func(m int) bool {
		cnt := 0
		for i := uint(1); i < 1<<len(coins); i++ {
			c := m / subsetLcm[i]
			if bits.OnesCount(i)%2 == 0 {
				c = -c
			}
			cnt += c
		}
		return cnt >= k
	})

	return int64(ans)
}
