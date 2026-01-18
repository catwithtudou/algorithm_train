package leetcode

func largestMagicSquare(grid [][]int) int {
	m, n := len(grid), len(grid[0])
	rowSum := make([][]int, m)
	colSum := make([][]int, m+1)
	diagSum := make([][]int, m+1)
	antiSum := make([][]int, m+1)

	for i := range m + 1 {
		colSum[i] = make([]int, n)
		diagSum[i] = make([]int, n+1)
		antiSum[i] = make([]int, n+1)
	}

	for i, row := range grid {
		rowSum[i] = make([]int, n+1)
		for j, x := range row {
			rowSum[i][j+1] = rowSum[i][j] + x
			colSum[i+1][j] = colSum[i][j] + x
			diagSum[i+1][j+1] = diagSum[i][j] + x
			antiSum[i+1][j] = antiSum[i][j+1] + x
		}
	}

	for k := min(m, n); ; k-- {
		for i := k; i <= m; i++ {
		next:
			for j := k; j <= n; j++ {
				sum := diagSum[i][j] - diagSum[i-k][j-k]

				if antiSum[i][j-k]-antiSum[i-k][j] != sum {
					continue
				}

				for _, rowS := range rowSum[i-k : i] {
					if rowS[j]-rowS[j-k] != sum {
						continue next
					}
				}

				for c := j - k; c < j; c++ {
					if colSum[i][c]-colSum[i-k][c] != sum {
						continue next
					}
				}

				return k
			}
		}
	}
}
