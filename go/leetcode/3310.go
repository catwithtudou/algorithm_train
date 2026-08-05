package leetcode

func remainingMethods(n int, k int, invocations [][]int) (ans []int) {

	g := make([][]int, n)

	for _, e := range invocations {
		g[e[0]] = append(g[e[0]], e[1])
	}

	isSus := make([]bool, n)
	var dfs func(int)
	dfs = func(x int) {
		isSus[x] = true
		for _, y := range g[x] {
			if !isSus[y] {
				dfs(y)
			}
		}
	}

	dfs(k)

	for _, e := range invocations {
		if !isSus[e[0]] && isSus[e[1]] {
			for i := range n {
				ans = append(ans, i)
			}
			return
		}
	}

	for i, b := range isSus {
		if !b {
			ans = append(ans, i)
		}
	}

	return
}
