package leetcode

import "sort"

func findOriginalArray(changed []int) []int {
	count := make(map[int]int, len(changed))
	sort.Ints(changed)
	for _, v := range changed {
		count[v]++
	}

	res := make([]int, 0, len(changed)/2+1)
	for _, v := range changed {
		if count[v] == 0 {
			continue
		}
		count[v]--

		if count[v*2] == 0 {
			return []int{}
		}
		count[v*2]--

		res = append(res, v)
	}

	return res
}
