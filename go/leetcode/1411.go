package leetcode

func numOfWays(n int) int {
	const mod = 1_000_000_007

	type tuple struct{ i, j, preRow, curRow int }

	memo := make(map[tuple]int)

	var dfs func(int, int, int, int) int

	dfs = func(i, j, preRow, curRow int) int {
		if i == 0 {
			return 1
		}

		if j == 3 {
			return dfs(i-1, 0, curRow, 0)
		}

		t := tuple{i, j, preRow, curRow}
		if v, ok := memo[t]; ok {
			return v
		}

		res := 0
		for color := range 3 {
			if preRow > 0 && color == preRow>>(j*2)&3 ||
				j > 0 && color == curRow>>((j-1)*2)&3 {
				continue
			}

			res = (res + dfs(i, j+1, preRow, curRow|(color<<(j*2)))) % mod
		}

		memo[t] = res
		return res
	}

	return dfs(n, 0, 0, 0)
}
