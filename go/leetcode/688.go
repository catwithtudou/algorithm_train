package leetcode

func knightProbability(n int, k int, row int, column int) float64 {
	dirs := []struct{ x, y int }{{-2, -1}, {-1, -2}, {1, -2}, {2, -1}, {2, 1}, {1, 2}, {-1, 2}, {-2, 1}}
	memo := make([][][]float64, k+1)
	for i := range memo {
		memo[i] = make([][]float64, n)
		for j := range memo[i] {
			memo[i][j] = make([]float64, n)
		}
	}
	var dfs func(int, int, int) float64
	dfs = func(k, i, j int) float64 {
		if i < 0 || j < 0 || i >= n || j >= n {
			return 0
		}
		if k == 0 {
			return 1
		}
		p := &memo[k][i][j]
		if *p != 0 {
			return *p
		}
		res := 0.0
		for _, d := range dirs {
			res += dfs(k-1, i+d.x, j+d.y)
		}
		res /= 8
		*p = res
		return res
	}
	return dfs(k, row, column)
}
