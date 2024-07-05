package leetcode

func modifiedMatrix(matrix [][]int) [][]int {
	for j := range matrix[0] {

		mx := 0

		for _, v := range matrix {
			mx = max(mx, v[j])
		}
		for _, row := range matrix {
			if row[j] == -1 {
				row[j] = mx
			}
		}

	}
	return matrix
}
