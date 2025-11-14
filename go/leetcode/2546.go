package leetcode

func rangeAddQueries(n int, queries [][]int) [][]int {
	matrix := make([][]int, n)
	for i := range matrix {
		matrix[i] = make([]int, n)
	}

	for _, query := range queries {
		for i := query[0]; i <= query[2]; i++ {
			for j := query[1]; j <= query[3]; j++ {
				matrix[i][j]++
			}
		}
	}

	return matrix
}
