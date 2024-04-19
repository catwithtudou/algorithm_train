package leetcode

func minSkips(dist []int, speed int, hoursBefore int) int {
	sumDist := 0
	for _, v := range dist {
		sumDist += v
	}

	if sumDist < hoursBefore*speed {
		return -1
	}

	n := len(dist)
	memo := make([][]int, n)
	for i := range memo {
		memo[i] = make([]int, n)
		for j := range memo[i] {
			memo[i][j] = -1
		}
	}

	var dfs func(i, j int) int
	dfs = func(i, j int) int {
		if j < 0 {
			return 0
		}

		p := &memo[i][j]
		if *p != -1 {
			return *p
		}

		res := (dfs(i, j-1) + dist[j] + speed - 1) / speed * speed
		if i > 0 {
			res = min(res, dfs(i-1, j-1)+dist[j])
		}

		*p = res
		return res
	}

	for i := 0; ; i++ {
		if dfs(i, n-2)+dist[n-1] <= speed*hoursBefore {
			return i
		}
	}
}
