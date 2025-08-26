package leetcode

func findDiagonalOrder(mat [][]int) []int {
	m, n := len(mat), len(mat[0])
	res := make([]int, 0, m*n)

	for k := 0; k < m+n-1; k++ {
		minJ := max(k-m+1, 0)
		maxJ := min(k, n-1)

		if k%2 == 0 {
			for j := minJ; j <= maxJ; j++ {
				res = append(res, mat[k-j][j])
			}
		} else {
			for j := maxJ; j >= minJ; j-- {
				res = append(res, mat[k-j][j])
			}
		}

	}

	return res
}
