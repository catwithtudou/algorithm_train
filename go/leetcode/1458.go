package leetcode

import "math"

func maxDotProduct(nums1 []int, nums2 []int) int {
	n := len(nums1)
	m := len(nums2)
	memo := make([][]int, n)
	for i := range memo {
		memo[i] = make([]int, m)
		for j := range memo[i] {
			memo[i][j] = math.MaxInt
		}
	}

	var dfs func(int, int) int

	dfs = func(i, j int) int {
		if i < 0 || j < 0 {
			return math.MinInt
		}

		p := &memo[i][j]
		if *p != math.MaxInt {
			return *p
		}

		res1 := max(dfs(i-1, j-1), 0) + nums1[i]*nums2[j]
		res2 := dfs(i-1, j)
		res3 := dfs(i, j-1)

		*p = max(res1, max(res2, res3))
		return *p
	}

	return dfs(n-1, m-1)
}
