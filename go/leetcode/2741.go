package leetcode

func specialPerm(nums []int) int {
	n := len(nums)
	u := 1<<n - 1
	memo := make([][]int, u)
	for i := range memo {
		memo[i] = make([]int, n)
		for j := range memo[i] {
			memo[i][j] = -1
		}
	}

	var dfs func(int, int) int
	dfs = func(s, i int) (res int) {
		if s == 0 {
			return 1
		}
		p := &memo[s][i]
		if *p != -1 {
			return *p
		}
		for j, x := range nums {
			if s>>j&1 > 0 && (nums[i]%x == 0 || x%nums[i] == 0) {
				res += dfs(s^(1<<j), j)
			}
		}
		*p = res
		return
	}

	ans := 0
	for i := range nums {
		ans += dfs(u^(1<<i), i)
	}

	return ans % 1_000_000_007
}
