package leetcode

import "slices"

func subsetsWithDup(nums []int) (ans [][]int) {
	slices.Sort(nums)
	n := len(nums)
	path := []int{}
	var dfs func(int)
	dfs = func(i int) {
		if i == n {
			ans = append(ans, slices.Clone(path))
			return
		}

		x := nums[i]
		path = append(path, x)
		dfs(i + 1)
		path = path[:len(path)-1]

		i++
		for i < n && nums[i] == x {
			i++
		}
		dfs(i)

	}
	dfs(0)
	return ans
}
