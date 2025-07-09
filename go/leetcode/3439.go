package leetcode

func maxFreeTime(eventTime int, k int, startTime []int, endTime []int) (ans int) {
	n := len(startTime)
	free := make([]int, n+1)
	free[0] = startTime[0]
	for i := 1; i < n; i++ {
		free[i] = startTime[i] - endTime[i-1]
	}
	free[n] = eventTime - endTime[n-1]

	s := 0
	for i, f := range free {
		s += f
		if i < k {
			continue
		}
		ans = max(ans, s)
		s -= free[i-k]
	}

	return
}
