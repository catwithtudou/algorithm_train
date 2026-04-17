package leetcode

import (
	"slices"
	"sort"
)

func solveQueries(nums []int, queries []int) []int {
	indices := make(map[int][]int)
	for i, x := range nums {
		indices[x] = append(indices[x], i)
	}

	n := len(nums)
	for x, p := range indices {
		i0 := p[0]
		p = slices.Insert(p, 0, p[len(p)-1]-n)
		indices[x] = append(p, i0+n)
	}

	for qi, i := range queries {
		p := indices[nums[i]]
		if len(p) == 3 {
			queries[qi] = -1
		} else {
			j := sort.SearchInts(p, i)
			queries[qi] = min(i-p[j-1], p[j+1]-i)
		}
	}

	return queries
}
