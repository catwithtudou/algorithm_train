package leetcode

func carPooling(trips [][]int, capacity int) bool {
	idxMax := 0
	for _, trip := range trips {
		idxMax = max(idxMax, trip[2])
	}

	diff := make([]int, idxMax+1)
	for _, trip := range trips {
		diff[trip[1]] += trip[0]
		diff[trip[2]] -= trip[0]
	}

	count := 0
	for i := 0; i < idxMax; i++ {
		count += diff[i]
		if count > capacity {
			return false
		}
	}
	return true
}
