package leetcode

func numberOfArrays(differences []int, lower int, upper int) int {
	s := 0
	minS, maxS := 0, 0
	for _, d := range differences {
		s += d
		minS = min(minS, s)
		maxS = max(maxS, s)
	}
	return max(upper-lower-maxS+minS+1, 0)
}
