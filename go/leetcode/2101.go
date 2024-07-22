package leetcode

func maximumDetonation(bombs [][]int) int {
	n := len(bombs)
	g := make([][]int, n)
	for i, p := range bombs {
		x, y, r := p[0], p[1], p[2]
		for j, q := range bombs {
			dx := x - q[0]
			dy := y - q[1]
			if j != i && dx*dx+dy*dy <= r*r {
				g[i] = append(g[i], j)
			}
		}
	}
	vis := make([]bool, n)
	var dfs func(int) int
	dfs = func(i int) int {
		vis[i] = true
		cnt := 1
		for _, y := range g[i] {
			if !vis[y] {
				cnt += dfs(y)
			}
		}
		return cnt
	}

	ans := 0
	for i := range g {
		vis = make([]bool, n)
		ans = max(ans, dfs(i))
	}

	return ans
}
