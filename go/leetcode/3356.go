package leetcode

import "sort"

func minZeroArray(nums []int, queries [][]int) int {
	q := len(queries)
	diff := make([]int, len(nums)+1)
	ans := sort.Search(q+1, func(i int) bool {
		clear(diff)
		for _, v := range queries[:i] {
			diff[v[0]] += v[2]
			diff[v[1]+1] -= v[2]
		}
		sumD := 0
		for j, x := range nums {
			sumD += diff[j]
			if x > sumD {
				return false
			}
		}
		return true
	})
	if ans > q {
		return -1
	}
	return ans

}
