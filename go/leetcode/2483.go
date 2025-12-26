package leetcode

import "strings"

func bestClosingTime(customers string) (ans int) {

	penalty := strings.Count(customers, "Y")
	minPenalty := penalty

	for i, c := range customers {
		if c == 'N' {
			penalty++
		} else {
			penalty--
		}
		if penalty < minPenalty {
			minPenalty = penalty
			ans = i + 1
		}
	}

	return
}
