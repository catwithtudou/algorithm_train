package leetcode

import (
	"sort"

	"golang.org/x/exp/slices"
)

func kSum(nums []int, k int) int64 {
	sum, total := 0, 0
	for i, num := range nums {
		if num >= 0 {
			sum += num
			total += num
		} else {
			total -= num
			nums[i] = -num
		}
	}

	slices.Sort(nums)
	kthS := sort.Search(total, func(sumLimit int) bool {
		cnt := 1
		var dfs func(int, int)
		dfs = func(i int, s int) {
			if cnt == k || i == len(nums) || s+nums[i] > sumLimit {
				return
			}
			cnt++
			dfs(i+1, s+nums[i])
			dfs(i+1, s)
		}
		dfs(0, 0)
		return cnt == k
	})

	return int64(sum - kthS)
}
