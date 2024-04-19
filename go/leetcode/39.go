package leetcode

func combinationSum(candidates []int, target int) (ans [][]int) {
	res := make([]int, 0)
	var dfs func(target, idx int)
	dfs = func(target, idx int) {
		if idx == len(candidates) {
			return
		}
		if target == 0 {
			ans = append(ans, append([]int(nil), res...))
			return
		}

		dfs(target, idx+1)
		if target-candidates[idx] >= 0 {
			res = append(res, candidates[idx])
			dfs(target-candidates[idx], idx)
			res = res[:len(res)-1]
		}
	}
	dfs(target, 0)
	return ans
}
