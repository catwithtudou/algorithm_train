package leetcode

func numSubmat(mat [][]int) (ans int) {
	m, n := len(mat), len(mat[0])
	for top := range m {
		a := make([]int, n)
		for bottom := top; bottom < m; bottom++ {
			last := -1
			h := bottom - top + 1
			for i := range n {
				a[i] += mat[bottom][i]
				if a[i] != h {
					last = i
				} else {
					ans += i - last
				}
			}
		}
	}
	return ans
}
