package leetcode

func findSubtreeSizes(parent []int, s string) []int {
	n := len(parent)
	g := make([][]int, n)
	for i := 1; i < n; i++ {
		p := parent[i]
		g[p] = append(g[p], i)
	}

	anc := [26]int{}
	for i := range anc {
		anc[i] = -1
	}

	var rebuild func(int)
	rebuild = func(x int) {
		sx := s[x] - 'a'
		old := anc[sx]
		anc[sx] = x
		for i, y := range g[x] {
			if a := anc[s[y]-'a']; a != -1 {
				g[a] = append(g[a], y)
				g[x][i] = -1
			}
			rebuild(y)
		}
		anc[sx] = old
	}

	rebuild(0)

	size := make([]int, n)
	var dfs func(int)
	dfs = func(x int) {
		size[x] = 1
		for _, y := range g[x] {
			if y != -1 {
				dfs(y)
				size[x] += size[y]
			}
		}
	}
	dfs(0)
	return size
}
