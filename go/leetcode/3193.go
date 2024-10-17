package leetcode

func numberOfPermutations(n int, requirements [][]int) int {
	req := make([]int, n)
	for i := range req {
		req[i] = -1
	}
	m := -1
	for _, r := range requirements {
		req[r[0]] = r[1]
		m = max(m, r[1])
	}
	if req[0] > 0 {
		return 0
	}

	f := make([][]int, n)
	for i := range f {
		f[i] = make([]int, m+1)
	}
	f[0][0] = 1

	for i := 1; i < n; i++ {
		l, r := req[i], req[i]
		if req[i] < 0 {
			l, r = 0, m
		}
		for j := l; j <= r; j++ {
			for k := 0; k < min(i, j); k++ {
				f[i][j] = (f[i][j] + f[i-1][j-k]) % (1e9 + 7)
			}
		}
	}

	return f[n-1][req[n-1]]
}
