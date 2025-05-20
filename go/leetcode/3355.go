package leetcode

func isZeroArray(nums []int, queries [][]int) bool {
	diff := make([]int, len(nums)+1)

	for _, v := range queries {
		diff[v[0]]++
		diff[v[1]+1]--
	}

	sumD := 0
	for i, x := range nums {
		sumD += diff[i]
		if x > sumD {
			return false
		}
	}
	return true
}
