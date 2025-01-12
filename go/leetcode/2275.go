package leetcode

import (
	"math/bits"

	"golang.org/x/exp/slices"
)

func largestCombination(candidates []int) (ans int) {
	m := bits.Len(uint(slices.Max(candidates)))
	for i := range m {
		cnt := 0
		for _, x := range candidates {
			cnt += x >> i & 1
		}
		ans = max(ans, cnt)
	}
	return ans
}
