package leetcode

func minimumDeleteSum(s1 string, s2 string) int {
	n, m := len(s1), len(s2)
	total := 0
	for _, c1 := range s1 {
		total += int(c1)
	}
	for _, c2 := range s2 {
		total += int(c2)
	}

	f := make([][]int, n+1)
	for i := range f {
		f[i] = make([]int, m+1)
	}

	for i, c1 := range s1 {
		for j, c2 := range s2 {
			if c1 == c2 {
				f[i+1][j+1] = f[i][j] + int(c1)
			} else {
				f[i+1][j+1] = max(f[i][j+1], f[i+1][j])
			}
		}
	}

	return total - 2*f[n][m]
}
