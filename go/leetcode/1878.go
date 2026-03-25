package leetcode

func getBiggestThree(grid [][]int) []int {
	m, n := len(grid), len(grid[0])
	diagSum := make([][]int, m+1)
	antiSum := make([][]int, m+1)
	for i := range diagSum {
		diagSum[i] = make([]int, n+1)
		antiSum[i] = make([]int, n+1)
	}

	for i, row := range grid {
		for j, v := range row {
			diagSum[i+1][j+1] = diagSum[i][j] + v
			antiSum[i+1][j] = antiSum[i][j+1] + v
		}
	}

	// 向右下连续 k 个数的和
	queryDiagonal := func(x, y, k int) int { return diagSum[x+k][y+k] - diagSum[x][y] }

	// 向左下连续 k 个数的和
	queryAntiDiagonal := func(x, y, k int) int { return antiSum[x+k][y+1-k] - antiSum[x][y+1] }

	var x, y, z int
	update := func(v int) {
		if v > x {
			x, y, z = v, x, y
		} else if v < x && v > y {
			y, z = v, y
		} else if v < y && v > z {
			z = v
		}
	}

	for i, row := range grid {
		for j, v := range row {
			update(v) // 更新菱形

			mx := min(min(i, m-1-i), min(j, n-1-j))
			for k := 1; k <= mx; k++ {
				a := queryDiagonal(i-k, j, k)           // 右上
				b := queryDiagonal(i, j-k, k)           // 左下
				c := queryAntiDiagonal(i-k+1, j-1, k-1) // 左上
				d := queryAntiDiagonal(i, j+k, k+1)     // 右下
				update(a + b + c + d)
			}
		}
	}

	ans := []int{x, y, z}
	for ans[len(ans)-1] == 0 {
		ans = ans[:len(ans)-1]
	}

	return ans
}
