package leetcode

func findMinHeightTrees(n int, edges [][]int) []int {
	if n == 0 {
		return []int{0}
	}

	g := make([][]int, n)
	for _, v := range edges {
		x, y := v[0], v[1]
		g[x] = append(g[x], y)
		g[y] = append(g[y], x)
	}

	parents := make([]int, n)
	bfs := func(start int) (x int) {
		vis := make([]bool, n)
		vis[start] = true
		q := []int{start}
		for len(q) > 0 {
			x, q = q[0], q[1:]
			for _, y := range g[x] {
				if !vis[y] {
					vis[y] = true
					parents[y] = x
					q = append(q, y)
				}
			}
		}
		return
	}

	x := bfs(0)
	y := bfs(x)

	path := make([]int, 0, n)
	parents[x] = -1
	for y != -1 {
		path = append(path, y)
		y = parents[y]
	}

	m := len(path)
	if m%2 == 0 {
		return []int{path[m/2-1], path[m/2]}
	}

	return []int{path[m/2]}
}
