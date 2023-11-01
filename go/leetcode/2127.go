package leetcode

func maximumInvitations(favorite []int) int {
	n := len(favorite)
	deg := make([]int, n)
	// 统计基环数节点入度
	for _, in := range favorite {
		deg[in]++
	}

	maxDepth := make([]int, n)
	q := make([]int, 0, n)
	// 统计树枝节点索引
	for i, d := range deg {
		if d == 0 {
			q = append(q, i)
		}
	}

	// 拓扑排序（剪掉树枝）
	for len(q) > 0 {
		x := q[0]
		q = q[1:]
		y := favorite[x]
		maxDepth[y] = maxDepth[x] + 1
		if deg[y]--; deg[y] == 0 {
			q = append(q, y)
		}
	}

	maxRingSize, sumChinSize := 0, 0
	for i, d := range deg {
		if d == 0 {
			continue
		}

		deg[i] = 0
		// 统计环数长度
		ringSize := 1
		for x := favorite[i]; x != i; x = favorite[x] {
			deg[x] = 0
			ringSize++
		}

		if ringSize == 2 {
			sumChinSize += maxDepth[i] + maxDepth[favorite[i]] + 2
		} else {
			maxRingSize = max(maxRingSize, ringSize)
		}
	}

	return max(maxRingSize, sumChinSize)
}
