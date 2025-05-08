package leetcode

import (
	"container/heap"
	"math"
)

func minTimeToReachII(moveTime [][]int) int {
	n, m := len(moveTime), len(moveTime[0])
	dp := make([][]int, n)
	for i := range dp {
		dp[i] = make([]int, m)
		for j := range dp[i] {
			dp[i][j] = math.MaxInt32
		}
	}
	dp[0][0] = 0

	dirs := [][]int{{0, 1}, {1, 0}, {0, -1}, {-1, 0}}

	h := hpDirs{{}}

	for {
		top := heap.Pop(&h).(tupleDirs)
		i, j := top.x, top.y
		if i == n-1 && j == m-1 {
			return top.dis
		}
		if top.dis > dp[i][j] {
			continue
		}

		time := (i+j)%2 + 1
		for _, d := range dirs {
			x, y := i+d[0], j+d[1]
			if x >= 0 && x < n && y >= 0 && y < m {
				newDis := max(top.dis, moveTime[x][y]) + time
				if newDis < dp[x][y] {
					dp[x][y] = newDis
					heap.Push(&h, tupleDirs{x, y, newDis})
				}
			}
		}

	}

}
