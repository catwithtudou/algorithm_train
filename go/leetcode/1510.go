package leetcode

func winnerSquareGame(n int) bool {
	memo := make([]int8, n+1)
	for i := range memo {
		memo[i] = -1
	}

	var dfs func(int) bool

	dfs = func(i int) bool {
		if i == 0 {
			return false
		}

		p := &memo[i]
		if *p != -1 {
			return *p == 1
		}

		for x := 1; x*x <= i; x++ {
			if !dfs(i - x*x) {
				*p = 1
				return true
			}
		}
		*p = 0
		return false
	}

	return dfs(n)
}
