package leetcode

func firstCompleteIndex(arr []int, mat [][]int) int {
	n, m := len(mat), len(mat[0])
	mp := make(map[int][2]int)
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			mp[mat[i][j]] = [2]int{i, j}
		}
	}
	rowCnt, colCnt := make([]int, n), make([]int, m)
	for i := 0; i < len(arr); i++ {
		v := mp[arr[i]]
		rowCnt[v[0]]++
		colCnt[v[1]]++
		if rowCnt[v[0]] == m || colCnt[v[1]] == n {
			return i
		}
	}
	return -1
}
