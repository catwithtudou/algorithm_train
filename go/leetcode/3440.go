package leetcode

func maxFreeTimeII(eventTime int, startTime []int, endTime []int) (ans int) {
	n := len(startTime)
	free := make([]int, n+1)
	free[0] = startTime[0]
	for i := 1; i < n; i++ {
		free[i] = startTime[i] - endTime[i-1]
	}
	free[n] = eventTime - endTime[n-1]

	r := make([]int, n+1)
	for i := n - 1; i >= 0; i-- {
		r[i] = max(r[i+1], free[i+1])
	}

	l := 0
	for i := 0; i < n; i++ {
		delta := endTime[i] - startTime[i]
		if delta <= l || delta <= r[i+1] {
			ans = max(ans, free[i]+delta+free[i+1])
		} else {
			ans = max(ans, free[i]+free[i+1])
		}
		l = max(l, free[i])
	}

	return
}
