package leetcode

import (
	"maps"
	"slices"
)

func findAllPeople(_ int, meetings [][]int, firstPerson int) []int {
	slices.SortFunc(meetings, func(a, b []int) int { return a[2] - b[2] })

	haveSecret := map[int]bool{0: true, firstPerson: true}

	// 分组循环
	m := len(meetings)
	for i := 0; i < m; {
		g := map[int][]int{}
		time := meetings[i][2]
		for ; i < m && meetings[i][2] == time; i++ {
			x, y := meetings[i][0], meetings[i][1]
			g[x] = append(g[x], y)
			g[y] = append(g[y], x)
		}

		vis := map[int]bool{}
		var dfs func(int)
		dfs = func(x int) {
			vis[x] = true
			haveSecret[x] = true
			for _, y := range g[x] {
				if !vis[y] {
					dfs(y)
				}
			}
		}
		for x := range g { // 遍历在 time 时间点参加会议的专家
			if haveSecret[x] && !vis[x] {
				dfs(x)
			}
		}
	}

	// 可以按任何顺序返回答案
	return slices.Collect(maps.Keys(haveSecret))
}
