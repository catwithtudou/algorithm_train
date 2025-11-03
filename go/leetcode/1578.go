package leetcode

func minCost1578(colors string, neededTime []int) (ans int) {
	maxT := 0
	for i, t := range neededTime {
		ans += t
		maxT = max(maxT, t)
		if i == len(neededTime)-1 || colors[i] != colors[i+1] {
			ans -= maxT
			maxT = 0
		}
	}

	return
}
