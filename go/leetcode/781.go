package leetcode

func numRabbits(answers []int) (ans int) {
	cnt := make(map[int]int)

	for _, v := range answers {
		cnt[v]++
	}

	for y, x := range cnt {
		ans += (x + y) / (y + 1) * (y + 1)
	}

	return
}
