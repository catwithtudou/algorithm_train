package leetcode

func minimumTotalPrice(n int, edges [][]int, price []int, trips [][]int) int {
	graph := make([][]int, n)
	for _, edge := range edges {
		graph[edge[0]] = append(graph[edge[0]], edge[1])
		graph[edge[1]] = append(graph[edge[1]], edge[0])
	}

	cnt := make([]int, n)
	for _, trip := range trips {
		start, end := trip[0], trip[1]
		var dfs func(int, int) bool
		dfs = func(x, fa int) bool {
			if x == end {
				cnt[x]++
				return true
			}
			for _, y := range graph[x] {
				if y != fa && dfs(y, x) {
					cnt[x]++
					return true
				}
			}
			return false
		}
		dfs(start, -1)
	}

	var dfs func(int, int) (int, int)
	dfs = func(x, fa int) (int, int) {
		notCP := cnt[x] * price[x]
		cp := notCP / 2
		for _, y := range graph[x] {
			if y == fa {
				continue
			}
			notC, c := dfs(y, x)
			notCP += min(notC, c)
			cp += notC
		}
		return notCP, cp
	}

	return min(dfs(0, -1))
}
