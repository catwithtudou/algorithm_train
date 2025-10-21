package leetcode

import (
	"sort"
)

func maxFrequency(nums []int, k int, numOperations int) (ans int) {
	cnt := map[int]int{}
	diff := map[int]int{}
	for _, x := range nums {
		cnt[x]++
		diff[x] += 0
		diff[x-k]++
		diff[x+k+1]--
	}

	// 获取 diff 的所有 key
	keys := make([]int, 0, len(diff))
	for k := range diff {
		keys = append(keys, k)
	}

	// 排序 key
	sort.Ints(keys)

	sumD := 0

	// 遍历排序后的 keys
	for _, x := range keys {
		sumD += diff[x]
		ans = max(ans, min(sumD, cnt[x]+numOperations))
	}

	return
}
