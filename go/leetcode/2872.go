package leetcode

func maxKDivisibleComponents(n int, edges [][]int, values []int, k int) (ans int) {
	g := make([][]int, n)
	for _, e := range edges {
		g[e[0]] = append(g[e[0]], e[1])
		g[e[1]] = append(g[e[1]], e[0])
	}

	var dfs func(int, int) int

	dfs = func(x, fa int) int {
		s := values[x]
		for _, y := range g[x] {
			if y != fa {
				s += dfs(y, x)
			}
		}
		if s%k == 0 {
			ans++
		}
		return s
	}

	dfs(0, -1)

	return
}
