package leetcode

func maxCandies(status []int, candies []int, keys [][]int, containedBoxes [][]int, initialBoxes []int) (ans int) {
	hasKey := status
	hasBox := make([]bool, len(status))
	for _, v := range initialBoxes {
		hasBox[v] = true
	}

	var dfs func(int)
	dfs = func(x int) {
		ans += candies[x]
		hasBox[x] = false

		for _, y := range keys[x] {
			hasKey[y] = 1
			if hasBox[y] {
				dfs(y)
			}
		}

		for _, y := range containedBoxes[x] {
			hasBox[y] = true
			if hasKey[y] == 1 {
				dfs(y)
			}
		}
	}

	for _, x := range initialBoxes {
		if hasKey[x] == 1 && hasBox[x] {
			dfs(x)
		}
	}

	return
}
