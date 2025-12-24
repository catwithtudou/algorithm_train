package leetcode

import "slices"

func minimumBoxes(apple []int, capacity []int) (ans int) {
	appleSum := 0
	for _, a := range apple {
		appleSum += a
	}

	slices.Sort(capacity)

	for i := len(capacity) - 1; i >= 0; i-- {
		appleSum -= capacity[i]
		ans++
		if appleSum <= 0 {
			return
		}
	}

	return
}
