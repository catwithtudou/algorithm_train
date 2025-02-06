package leetcode

import "slices"

func permuteUnique(nums []int) (ans [][]int) {
	slices.Sort(nums)
	n := len(nums)
	path := make([]int, n)
	visited := make([]bool, n)
	var dfs func(int)
	dfs = func(i int) {
		if i == n {
			ans = append(ans, slices.Clone(path))
			return
		}
		for j, vis := range visited {
			if vis {
				continue
			}
			if j > 0 && nums[j] == nums[j-1] && !visited[j-1] {
				continue
			}
			path[i] = nums[j]
			visited[j] = true
			dfs(i + 1)
			visited[j] = false
		}

	}
	dfs(0)
	return ans
}
