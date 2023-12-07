package leetcode

func minReorder(n int, connections [][]int) int {
	graph := make([][][2]int, n)
	for _, edge := range connections {
		a, b := edge[0], edge[1]
		graph[a] = append(graph[a], [2]int{b, 1})
		graph[b] = append(graph[b], [2]int{a, 0})
	}
	var dfs func(int, int) int
	dfs = func(x, fa int) int {
		ans := 0
		for _, e := range graph[x] {
			if b, c := e[0], e[1]; b != fa {
				ans += c + dfs(b, x)
			}
		}
		return ans
	}

	return dfs(0, -1)
}
