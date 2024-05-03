package leetcode

import "golang.org/x/exp/slices"

func average(salary []int) float64 {
	s := 0
	for _, x := range salary {
		s += x
	}
	m := slices.Min(salary)
	M := slices.Max(salary)
	n := len(salary)
	return float64(s-m-M) / float64(n-2)
}
