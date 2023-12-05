package leetcode

func minimumFuelCost(roads [][]int, seats int) int64 {
	graph := make([][]int, len(roads)+1)
	for _, e := range roads {
		x, y := e[0], e[1]
		graph[x] = append(graph[x], y)
		graph[y] = append(graph[y], x)
	}

	var ans int64
	var dfs func(int, int) int
	dfs = func(x, fa int) int {
		size := 1
		for _, y := range graph[x] {
			if y != fa { // 避免递归父节点
				size += dfs(y, x)
			}
		}
		if x > 0 {
			// 计算子树的贡献度
			ans += int64((size-1)/seats + 1)
		}
		return size
	}

	dfs(0, -1)
	return ans
}
