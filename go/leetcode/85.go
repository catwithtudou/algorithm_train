package leetcode

func maximalRectangle(matrix [][]byte) (ans int) {
	heights := make([]int, len(matrix[0])+1)

	for _, row := range matrix {
		for j, c := range row {
			if c == '0' {
				heights[j] = 0
			} else {
				heights[j]++
			}
		}
		ans = max(ans, largestRectangleArea(heights))
	}

	return
}

func largestRectangleArea(heights []int) (ans int) {
	st := []int{-1}

	for right, h := range heights {
		for len(st) > 1 && heights[st[len(st)-1]] >= h {
			i := st[len(st)-1]
			st = st[:len(st)-1]
			area := heights[i] * (right - st[len(st)-1] - 1)
			ans = max(ans, area)
		}
		st = append(st, right)
	}

	return
}
