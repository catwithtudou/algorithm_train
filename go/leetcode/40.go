package leetcode

import "slices"

func combinationSum2(candidates []int, target int) (ans [][]int) {
	slices.Sort(candidates)
	path := []int{}
	var dfs func(int, int)
	dfs = func(start, left int) {
		if left == 0 {
			ans = append(ans, slices.Clone(path))
			return
		}

		for i := start; i < len(candidates); i++ {
			if left < candidates[i] {
				break
			}
			if i > start && candidates[i] == candidates[i-1] {
				continue
			}
			path = append(path, candidates[i])
			dfs(i+1, left-candidates[i])
			path = path[:len(path)-1]
		}

	}

	dfs(0, target)
	return
}
