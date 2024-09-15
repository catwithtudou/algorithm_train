package leetcode

func numberOfPoints(nums [][]int) int {
	ans := 0
	maxLen := 0
	for _, v := range nums {
		maxLen = max(maxLen, v[1])
	}

	diff := make([]int, maxLen+2)
	for _, v := range nums {
		diff[v[0]]++
		diff[v[1]+1]--
	}

	s := 0
	for _, v := range diff {
		s += v
		if s > 0 {
			ans++
		}
	}

	return ans
}
