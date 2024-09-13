package leetcode

func maximumRobots(chargeTimes []int, runningCosts []int, budget int64) int {
	q := []int{0}
	left, sum, ans := 0, int64(0), 0
	for i, x := range chargeTimes {
		for len(q) > 0 && x >= chargeTimes[q[len(q)-1]] {
			q = q[:len(q)-1]
		}
		q = append(q, i)
		sum += int64(runningCosts[i])

		for len(q) > 0 && int64(chargeTimes[q[0]])+sum*int64(i-left+1) > budget {
			if left == q[0] {
				q = q[1:]
			}
			sum -= int64(runningCosts[left])
			left++
		}
		ans = max(ans, i-left+1)
	}

	return ans
}
