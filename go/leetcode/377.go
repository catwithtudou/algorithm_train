package leetcode

func combinationSum4(nums []int, target int) int {
	memo := make([]int, target+1)
	for i := range memo {
		memo[i] = -1
	}

	var dfs func(int) int

	dfs = func(i int) (res int) {
		if i == 0 {
			return 1
		}

		p := &memo[i]
		if *p != -1 {
			return *p
		}

		for _, x := range nums {
			if x <= i {
				res += dfs(i - x)
			}
		}
		*p = res
		return
	}

	return dfs(target)
}
