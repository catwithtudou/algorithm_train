package leetcode

func shortestDistanceAfterQueries(n int, queries [][]int) []int {
	g := make([][]int, n-1)
	for i := range g {
		g[i] = append(g[i], i+1)
	}

	vis := make([]int, n-1)
	bfs := func(i int) int {
		q := []int{0}
		for step := 1; ; step++ {
			tmp := q
			q = nil
			for _, x := range tmp {
				for _, y := range g[x] {
					if y == n-1 {
						return step
					}
					if vis[y] != i {
						vis[y] = i
						q = append(q, y)
					}
				}
			}
		}
	}

	ans := make([]int, len(queries))
	for i, q := range queries {
		g[q[0]] = append(g[q[0]], q[1])
		ans[i] = bfs(i + 1)
	}
	return ans
}
