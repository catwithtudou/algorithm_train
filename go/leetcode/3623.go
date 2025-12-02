package leetcode

func countTrapezoids(points [][]int) (ans int) {
	const mod int = 1e9 + 7

	cnt := make(map[int]int, len(points))
	for _, p := range points {
		cnt[p[1]]++
	}

	s := 0
	for _, c := range cnt {
		k := c * (c - 1) / 2
		ans += s * k
		s += k
	}

	return ans % mod
}
