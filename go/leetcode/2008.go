package leetcode

func maxTaxiEarnings(n int, rides [][]int) int64 {
	roadList := make([][][2]int, n+1)
	for _, v := range rides {
		roadList[v[1]] = append(roadList[v[1]], [2]int{v[0], v[1] - v[0] + v[2]})
	}

	// 记忆数组
	memo := make([]int64, n+1)
	for i := range memo {
		memo[i] = -1
	}

	var dfs func(int) int64
	dfs = func(i int) int64 {
		if i == 1 {
			return 0
		}

		if memo[i] != -1 {
			return memo[i]
		}

		res := dfs(i - 1)
		for _, g := range roadList[i] {
			res = maxInt64(res, dfs(g[0])+int64(g[1]))
		}
		memo[i] = res
		return res
	}

	return dfs(n)
}

func maxTaxiEarningsOther(n int, rides [][]int) int64 {
	roadList := make([][][2]int, n+1)
	for _, v := range rides {
		roadList[v[1]] = append(roadList[v[1]], [2]int{v[0], v[1] - v[0] + v[2]})
	}

	f := make([]int64, n+1)
	for i := 2; i <= n; i++ {
		f[i] = f[i-1]
		for _, v := range roadList[i] {
			f[i] = maxInt64(f[i], f[v[0]]+int64(v[1]))
		}
	}

	return f[n]
}

func maxInt64(i, j int64) int64 {
	if i > j {
		return i
	}
	return j
}
