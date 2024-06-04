package leetcode

func countPairsOfConnectableServers(edges [][]int, signalSpeed int) []int {
	n := len(edges) + 1
	type edge struct{ to, wt int }
	g := make([][]edge, n)
	for _, e := range edges {
		x, y, wt := e[0], e[1], e[2]
		g[x] = append(g[x], edge{y, wt})
		g[y] = append(g[y], edge{x, wt})
	}

	ans := make([]int, n)
	for i, e := range g {
		if len(e) == 1 {
			continue
		}
		cnt := 0
		var dfs func(int, int, int)
		dfs = func(cur int, fa int, sum int) {
			if sum%signalSpeed == 0 {
				cnt++
			}
			for _, ed := range g[cur] {
				if ed.to != fa {
					dfs(ed.to, cur, sum+ed.wt)
				}
			}
			return
		}
		sum := 0
		for _, ed := range e {
			cnt = 0
			dfs(ed.to, i, ed.wt)
			ans[i] += cnt * sum
			sum += cnt
		}

	}

	return ans
}
