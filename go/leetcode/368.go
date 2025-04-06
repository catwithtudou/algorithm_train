package leetcode

import "sort"

func largestDivisibleSubset(nums []int) []int {
	n := len(nums)
	from := make([]int, n)
	for i := range from {
		from[i] = -1
	}

	sort.Ints(nums)
	memo := make([]int, n)
	var dfs func(int) int
	dfs = func(i int) (res int) {
		p := &memo[i]
		if *p > 0 {
			return *p
		}
		defer func() { *p = res }()

		x := nums[i]
		for j, v := range nums[:i] {
			if x%v != 0 {
				continue
			}
			f := dfs(j)
			if f > res {
				res = f
				from[i] = j
			}
		}
		return res + 1
	}

	maxF, maxI := 0, 0
	for i := range n {
		f := dfs(i)
		if f > maxF {
			maxF = f
			maxI = i
		}
	}

	path := make([]int, 0, maxF)
	for i := maxI; i >= 0; i = from[i] {
		path = append(path, nums[i])
	}

	return path
}
