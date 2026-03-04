package leetcode

import "golang.org/x/exp/slices"

func numSpecial(mat [][]int) (ans int) {
	for _, row := range mat {
		rowSum := 0
		for _, x := range row {
			rowSum += x
		}
		if rowSum != 1 {
			continue
		}

		j := slices.Index(row, 1)

		colSum := 0
		for _, r := range mat {
			colSum += r[j]
		}
		if colSum == 1 {
			ans++
		}
	}
	return
}
