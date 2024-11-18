package leetcode

func minFlipsII(grid [][]int) int {
	ans := 0
	m, n := len(grid), len(grid[0])
	for i, row := range grid[:m/2] {
		row2 := grid[m-1-i]
		for j, x := range row[:n/2] {
			cnt1 := x + row[n-1-j] + row2[j] + row2[n-1-j]
			ans += min(cnt1, 4-cnt1)
		}
	}

	// 均为奇数
	if m%2 > 0 && n%2 > 0 {
		ans += grid[m/2][n/2]
	}

	diff, cnt1 := 0, 0

	// m为奇数
	if m%2 > 0 {
		row := grid[m/2]
		for j, x := range row[:n/2] {
			if x != row[n-1-j] {
				diff++
			} else {
				cnt1 += x * 2
			}
		}
	}

	// n为奇数
	if n%2 > 0 {
		for i, row := range grid[:m/2] {
			if row[n/2] != grid[m-1-i][n/2] {
				diff++
			} else {
				cnt1 += row[n/2] * 2
			}
		}
	}

	if diff > 0 {
		ans += diff
	} else {
		ans += cnt1 % 4
	}

	return ans
}
