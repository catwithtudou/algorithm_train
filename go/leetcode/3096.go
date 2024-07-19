package leetcode

func minimumLevels(possible []int) int {
	n := len(possible)
	s := 0
	for _, x := range possible {
		s += x
	}
	s = s*2 - n
	pre := 0
	for i, x := range possible[:n-1] {
		pre += 4*x - 2
		if pre > s {
			return i + 1
		}
	}

	return -1
}
