package leetcode

func countTree(edges [][]int) (g [][]int, cnt [2]int) {
	g = make([][]int, len(edges)+1)
	for _, v := range edges {
		x, y := v[0], v[1]
		g[x] = append(g[x], y)
		g[y] = append(g[y], x)
	}

	var dfs func(int, int, int)
	dfs = func(x, fa, depth int) {
		cnt[depth]++
		for _, y := range g[x] {
			if y != fa {
				dfs(y, x, depth^1)
			}
		}
	}
	dfs(0, -1, 0)
	return
}

func maxTargetNodesII(edges1 [][]int, edges2 [][]int) []int {
	_, cnt2 := countTree(edges2)
	max2 := max(cnt2[0], cnt2[1])

	g, cnt1 := countTree(edges1)
	ans := make([]int, len(g))
	var dfs func(int, int, int)
	dfs = func(x, fa, depth int) {
		ans[x] = cnt1[depth] + max2
		for _, y := range g[x] {
			if y != fa {
				dfs(y, x, depth^1)
			}
		}
	}
	dfs(0, -1, 0)
	return ans
}
