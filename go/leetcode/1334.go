package leetcode

import "math"

// floyd
func findTheCity(n int, edges [][]int, distanceThreshold int) int {
	ans := math.MaxInt32 / 2
	ansIndex := -1

	mp := make([][]int, n)
	for i := 0; i < n; i++ {
		mp[i] = make([]int, n)
		for j := 0; j < n; j++ {
			mp[i][j] = ans
		}
	}

	for _, eg := range edges {
		from, to, weight := eg[0], eg[1], eg[2]
		mp[from][to], mp[to][from] = weight, weight
	}

	for k := 0; k < n; k++ {
		mp[k][k] = 0
		for i := 0; i < n; i++ {
			for j := 0; j < n; j++ {
				mp[i][j] = min(mp[i][j], mp[i][k]+mp[k][j])
			}
		}
	}

	for i := 0; i < n; i++ {
		cnt := 0
		for j := 0; j < n; j++ {
			if mp[i][j] <= distanceThreshold {
				cnt++
			}
		}
		if cnt < ans {
			ans, ansIndex = cnt, i
		}
	}

	return ansIndex
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
