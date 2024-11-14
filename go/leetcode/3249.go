package leetcode

func countGoodNodes(edges [][]int) int {
	n := len(edges) + 1
	g := make([][]int, n)
	for _, e := range edges {
		x, y := e[0], e[1]
		g[x] = append(g[x], y)
		g[y] = append(g[y], x)
	}

	var dfs func(int, int) int
	ans := 0
	dfs = func(x, fa int) int {
		size, sz0, ok := 1, 0, true
		for _, y := range g[x] {
			if y == fa {
				continue
			}
			sz := dfs(y, x)
			if sz0 == 0 {
				sz0 = sz
			} else if sz != sz0 {
				ok = false
			}
			size += sz
		}

		if ok {
			ans++
		}
		return size
	}
	dfs(0, -1)
	return ans
}
