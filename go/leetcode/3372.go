package leetcode

func getTree(edges [][]int, k int) func(int, int, int) int {
	g := make([][]int, len(edges)+1)
	for _, e := range edges {
		x, y := e[0], e[1]
		g[x] = append(g[x], y)
		g[y] = append(g[y], x)
	}

	var dfs func(int, int, int) int
	dfs = func(x, fa, depth int) int {
		if depth > k {
			return 0
		}
		cnt := 1
		for _, y := range g[x] {
			if y != fa {
				cnt += dfs(y, x, depth+1)
			}
		}
		return cnt
	}

	return dfs
}

func maxTargetNodes(edges1 [][]int, edges2 [][]int, k int) []int {
	max2 := 0
	if k > 0 {
		dfs := getTree(edges2, k-1)
		for i := range len(edges2) + 1 {
			max2 = max(max2, dfs(i, -1, 0))
		}
	}

	dfs := getTree(edges1, k)
	ans := make([]int, len(edges1)+1)
	for i := range ans {
		ans[i] = dfs(i, -1, 0) + max2
	}
	return ans
}
