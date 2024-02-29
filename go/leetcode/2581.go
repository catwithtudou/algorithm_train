package leetcode

func rootCount(edges [][]int, guesses [][]int, k int) int {
	gra := make([][]int, len(edges)+1)
	for _, e := range edges {
		a, b := e[0], e[1]
		gra[a] = append(gra[a], b)
		gra[b] = append(gra[b], a)
	}

	type pair struct {
		x, y int
	}
	s := make(map[pair]int, len(guesses))
	for _, g := range guesses {
		s[pair{g[0], g[1]}] = 1
	}

	cnt0 := 0
	var dfs func(int, int)
	dfs = func(x int, fa int) {
		for _, g := range gra[x] {
			if fa != g {
				if s[pair{x, g}] == 1 {
					cnt0++
				}
				dfs(g, x)
			}

		}
	}
	dfs(0, -1)

	ans := 0
	var reRoot func(int, int, int)
	reRoot = func(x int, fa int, cnt int) {
		if cnt >= k {
			ans++
		}

		for _, y := range gra[x] {
			if y != fa {
				reRoot(y, x, cnt-s[pair{x, y}]+s[pair{y, x}])
			}
		}
	}

	reRoot(0, -1, cnt0)

	return ans
}
