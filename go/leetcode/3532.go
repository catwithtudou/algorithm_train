package leetcode

import "sort"

func pathExistenceQueries(n int, nums []int, maxDiff int, queries [][]int) []bool {
	ids := []int{}
	for i := range n - 1 {
		if nums[i+1]-nums[i] > maxDiff {
			ids = append(ids, i)
		}
	}

	ans := make([]bool, len(queries))
	for i, q := range queries {
		ans[i] = sort.SearchInts(ids, q[0]) == sort.SearchInts(ids, q[1])
	}

	return ans
}
