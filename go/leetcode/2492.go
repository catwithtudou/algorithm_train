package leetcode

import "math"

func minScore(n int, roads [][]int) int {
	type edge struct{ to, dis int }

	edges := make([][]edge, n+1)

	for _, road := range roads {
		u, v, dis := road[0], road[1], road[2]
		edges[u] = append(edges[u], edge{v, dis})
		edges[v] = append(edges[v], edge{u, dis})
	}

	ans := math.MaxInt
	vis := make([]bool, n+1)

	var dfs func(int)

	dfs = func(x int) {
		vis[x] = true
		for _, e := range edges[x] {
			ans = min(ans, e.dis)
			if !vis[e.to] {
				dfs(e.to)
			}
		}
	}

	dfs(1)

	return ans
}
