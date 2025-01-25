package leetcode

func minimumMoney(transactions [][]int) int64 {
	totalLose, mx := 0, 0
	for _, t := range transactions {
		totalLose += max(t[0]-t[1], 0)
		mx = max(mx, min(t[0], t[1]))
	}
	return int64(totalLose + mx)
}
