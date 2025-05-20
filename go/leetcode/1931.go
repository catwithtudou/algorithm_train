package leetcode

func colorTheGrid(m int, n int) int {
	pow3 := make([]int, m)
	pow3[0] = 1
	for i := 1; i < m; i++ {
		pow3[i] = pow3[i-1] * 3
	}
	valid := []int{}
next:
	for color := range pow3[m-1] * 3 {
		for i := range m - 1 {
			if color/pow3[i+1]%3 == color/pow3[i]%3 {
				continue next
			}
		}
		valid = append(valid, color)
	}

	nv := len(valid)
	nxt := make([][]int, nv)
	for i, color1 := range valid {
	next2:
		for j, color2 := range valid {
			for _, p3 := range pow3 {
				if color1/p3%3 == color2/p3%3 {
					continue next2
				}
			}
			nxt[i] = append(nxt[i], j)
		}
	}

	memo := make([][]int, n)
	for i := range memo {
		memo[i] = make([]int, nv)
		for j := range memo[i] {
			memo[i][j] = -1
		}
	}

	var dfs func(int, int) int
	dfs = func(i, j int) (res int) {
		if i == 0 {
			return 1
		}
		p := &memo[i][j]
		if *p != -1 {
			return *p
		}
		defer func() {
			*p = res
		}()

		for _, k := range nxt[j] {
			res += dfs(i-1, k)
		}
		return res % mod
	}

	ans := 0
	for j := range nv {
		ans += dfs(n-1, j)
	}
	return ans % mod
}
