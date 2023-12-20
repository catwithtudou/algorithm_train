package leetcode

func findPeakGrid(mat [][]int) []int {
	l, r := 0, len(mat)-1
	for l < r {
		m := l + (r-l)/2
		c := rowMax(mat[m])
		if mat[m][c] > mat[m+1][c] {
			r = m
		} else {
			l = m + 1
		}
	}
	return []int{l, rowMax(mat[l])}
}

func rowMax(row []int) int {
	maxNum := 0
	for i := 1; i < len(row); i++ {
		if row[i] > row[maxNum] {
			maxNum = i
		}
	}
	return maxNum
}
