package leetcode

import (
	"slices"
	"sort"
)

func countFairPairs(nums []int, lower int, upper int) (ans int64) {
	slices.Sort(nums)

	for j, x := range nums {
		r := sort.SearchInts(nums[:j], upper-x+1)
		l := sort.SearchInts(nums[:j], lower-x)
		ans += int64(r - l)
	}

	return
}
