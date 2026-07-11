package leetcode

func countCompleteComponents(n int, edges [][]int) (ans int) {
	g := make([][]int, n)
	for _, edge := range edges {
		x, y := edge[0], edge[1]
		g[x] = append(g[x], y)
		g[y] = append(g[y], x)
	}

	vis := make([]bool, n)
	var v, e int

	var dfs func(int)

	dfs = func(x int) {
		vis[x] = true
		v++
		for _, y := range g[x] {
			if !vis[y] {
				e++
				dfs(y)
			}
		}
	}

	for i := range n {
		if !vis[i] {
			v, e = 0, 0
			dfs(i)
			if v == e {
				ans++
			}
		}
	}
	return ans
}
