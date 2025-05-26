package leetcode

func largestPathValue(colors string, edges [][]int) (ans int) {
	n := len(colors)
	graph := make([][]int, n)
	for _, e := range edges {
		x, y := e[0], e[1]
		if x == y {
			return -1
		}
		graph[x] = append(graph[x], y)
	}

	memo := make([][]int, n)
	var dfs func(int) []int
	dfs = func(x int) []int {
		if memo[x] != nil {
			return memo[x]
		}
		memo[x] = []int{}
		res := make([]int, 26)
		for _, y := range graph[x] {
			cy := dfs(y)
			if len(cy) == 0 {
				return cy
			}
			for i, c := range cy {
				res[i] = max(res[i], c)
			}
		}
		res[colors[x]-'a']++
		memo[x] = res
		return res
	}

	for i, x := range colors {
		res := dfs(i)
		if len(res) == 0 {
			return -1
		}
		ans = max(ans, res[x-'a'])
	}
	return
}
