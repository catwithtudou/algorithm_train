package leetcode

func numberOfBoomerangs(points [][]int) int {
	ans := 0
	for _, p := range points {
		cnt := make(map[int]int)
		for _, q := range points {
			cnt[(((p[0]-q[0])*(p[0]-q[0]))+((p[1]-q[1])*(p[1]-q[1])))]++
		}
		for _, m := range cnt {
			ans += m * (m - 1)
		}

	}
	return ans
}
