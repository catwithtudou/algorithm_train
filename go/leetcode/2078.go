package leetcode

func maxDistance2078(colors []int) int {
	n := len(colors)
	c := colors[0]
	if c != colors[n-1] {
		return n - 1
	}

	r := n - 2
	for colors[r] == c {
		r--
	}

	l := 1
	for colors[l] == c {
		l++
	}

	return max(r, n-l-1)
}
