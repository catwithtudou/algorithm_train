package leetcode

func findWinningPlayer(skills []int, k int) int {
	win, maxI := 0, 0
	for i := 1; i < len(skills) && win < k; i++ {
		if skills[i] > skills[maxI] {
			maxI = i
			win = 0
		}
		win++
	}
	return maxI
}
