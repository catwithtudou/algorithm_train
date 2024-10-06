package leetcode

func canCompleteCircuit(gas []int, cost []int) int {
	var ans, s, minS int

	for i, v := range gas {
		s += v - cost[i]
		if s < minS {
			minS = s
			ans = i + 1
		}
	}

	if s < 0 {
		return -1
	}

	return ans
}
