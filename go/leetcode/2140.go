package leetcode

func mostPoints(questions [][]int) int64 {
	n := len(questions)
	memo := make([]int64, n)
	var dfs func(int) int64
	dfs = func(i int) int64 {
		if i >= n {
			return 0
		}
		p := &memo[i]
		if *p == 0 {
			q := questions[i]
			*p = maxInt64(dfs(i+1), dfs(i+q[1]+1)+int64(q[0]))
		}
		return *p
	}

	return dfs(0)
}
