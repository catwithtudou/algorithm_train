package leetcode

func rotateTheBox(boxGrid [][]byte) [][]byte {
	m, n := len(boxGrid), len(boxGrid[0])
	ans := make([][]byte, n)
	for i := 0; i < n; i++ {
		ans[i] = make([]byte, m)
	}

	for i, row := range boxGrid {
		cnt := 0
		for j, ch := range row {
			if ch == '#' {
				cnt++
				ch = '.'
			}

			ans[j][m-1-i] = ch

			if j == n-1 || row[j+1] == '*' {
				for k := j; k > j-cnt; k-- {
					ans[k][m-1-i] = '#'
				}
				cnt = 0
			}
		}
	}

	return ans
}
